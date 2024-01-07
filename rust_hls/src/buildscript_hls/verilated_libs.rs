//! This file attempts to bundle all the functionalities needed for the verilator libs in one place.

use std::{
    fs::{create_dir_all, remove_dir_all, write},
    io::{self, Read},
    path::{Path, PathBuf},
};

use glob::PatternError;
use rust_hls_executor::{calculate_hash, CrateFile};
use rust_hls_verilator::{verilate_module, ObtainVerilatorLibsError, VerilateModuleError};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum VerilatedLibsError {
    #[error(transparent)]
    IoError(#[from] io::Error),
    #[error(transparent)]
    ObtainVerilatorLibsError(#[from] ObtainVerilatorLibsError),
    #[error("Verilated lib file has invalid path: {0:?}")]
    VerilatedLibFileHasInvalidPath(String),
    #[error("The verilated lib must be placed before it can be used")]
    VerilatedLibDoesNotExist(),
}

#[derive(Error, Debug)]
pub enum VerilatedModuleError {
    #[error(transparent)]
    IoError(#[from] io::Error),
    #[error(transparent)]
    VerilatedLibsError(#[from] VerilatedLibsError),
    #[error(transparent)]
    VerilateModuleError(#[from] VerilateModuleError),
    #[error(transparent)]
    PatternError(#[from] PatternError),
    #[error("Verilog source file has invalid path: {0:?}")]
    VerilogSourceFileHasInvalidPath(String),
}

pub fn assert_verilator_libs_exists(verilated_lib_path: &Path) -> Result<(), VerilatedLibsError> {
    if !verilated_lib_path.exists() {
        return Err(VerilatedLibsError::VerilatedLibDoesNotExist());
    }
    return Ok(());
}

/// Place the verilated libs in the crate root.
pub fn place_verilated_libs_in_crate(crate_root: &Path) -> Result<(), VerilatedLibsError> {
    let verilated_libs_path = get_verilated_libs_path(crate_root);

    place_verilated_libs(&verilated_libs_path)
}

/// Place the verilator libs in the given directory.
pub fn place_verilated_libs(path: &Path) -> Result<(), VerilatedLibsError> {
    let verilator_libs_path = path;

    if verilator_libs_path.exists() {
        return Ok(());
    }

    create_dir_all(verilator_libs_path)?;

    let verilated_libs = rust_hls_verilator::obtain_verilator_libs()?;

    let result: Result<Vec<()>, VerilatedLibsError> = verilated_libs
        .into_iter()
        .map(|file| -> Result<(), VerilatedLibsError> {
            let target_path = verilator_libs_path.join(file.path);
            let target_dir =
                target_path
                    .parent()
                    .ok_or(VerilatedLibsError::VerilatedLibFileHasInvalidPath(
                        target_path.to_string_lossy().to_string(),
                    ))?;

            create_dir_all(target_dir)?;

            Ok(std::fs::write(target_path, file.content)?)
        })
        .collect();

    match result {
        Ok(_) => Ok(()),
        Err(e) => Err(e)?,
    }
}

/// Gets the path of the verilated_module directory for the given verilog file.
pub fn get_verilated_module_path(
    verilog_file_path: &Path,
) -> Result<PathBuf, VerilatedModuleError> {
    let target_directory = verilog_file_path
        .parent()
        .ok_or(VerilatedModuleError::VerilogSourceFileHasInvalidPath(
            verilog_file_path.to_string_lossy().to_string(),
        ))?
        .join("verilated_module");

    return Ok(target_directory);
}

/// Get the path to the directory that contains the headers and source of the verilated runtime
/// library.
///
/// Currently always returns `rust_hls/verilated_libs`.
pub fn get_verilated_libs_path(crate_root: &Path) -> PathBuf {
    crate_root.join("rust_hls/verilated_libs")
}

fn read_hash_file(path: &Path) -> Option<String> {
    let mut file = std::fs::File::open(path.join("state.hash")).ok()?;
    let mut contents = String::new();
    file.read_to_string(&mut contents).ok()?;
    Some(contents.trim().to_string())
}

fn write_hash_file(path: &Path, new_hash: &str) -> Result<(), io::Error> {
    let file: PathBuf = path.join("state.hash");
    write(file, new_hash)
}

/// Places the verilator output for a Verilog file in a `verilated_module` directory beside it
pub fn place_verilated_module(
    verilog_file: &CrateFile,
    top_module: &str,
) -> Result<(), VerilatedModuleError> {
    let target_directory = get_verilated_module_path(&verilog_file.path)?;
    // if target_directory.exists() {
    //     return Ok(());
    // }

    let new_hash = calculate_hash(&vec![verilog_file.content.clone()]);
    let previous_hash = read_hash_file(&target_directory);

    if previous_hash == Some(new_hash.clone()) {
        return Ok(());
    }

    remove_dir_all(&target_directory).unwrap_or(());
    create_dir_all(&target_directory)?;

    verilate_module(&verilog_file.content, top_module)?
        .into_iter()
        .map(|mut file| {
            file.path = target_directory.join(file.path.file_name().unwrap());
            file
        })
        .map(|file| {
            println!("Writing file: {:?}", file.path.display());
            file.write()
        })
        .collect::<Result<Vec<_>, _>>()?;

    write_hash_file(&target_directory, &new_hash)?;

    return Ok(());
}

/// Finds all c and cpp files in the given directory.
pub fn get_verilated_module_files(
    verilated_module_path: &Path,
) -> Result<Vec<PathBuf>, VerilatedModuleError> {
    let files: Vec<_> = glob::glob(
        verilated_module_path
            .join("**/*.c*")
            .to_string_lossy()
            .to_string()
            .as_str(),
    )?
    .filter_map(|x| x.ok())
    .collect();

    Ok(files)
}

fn get_lib_include_dirs(verilated_lib_path: &Path) -> Result<Vec<PathBuf>, VerilatedLibsError> {
    assert_verilator_libs_exists(verilated_lib_path)?;
    let include_dirs: Vec<PathBuf> =
        vec![verilated_lib_path.into(), verilated_lib_path.join("vltstd")];

    Ok(include_dirs)
}

fn get_lib_files(verilated_lib_path: &Path) -> Result<Vec<PathBuf>, VerilatedLibsError> {
    assert_verilator_libs_exists(verilated_lib_path)?;
    let files: Vec<_> = vec![
        "verilated_cov.cpp",
        "verilated_dpi.cpp",
        "verilated_save.cpp",
        "verilated_vcd_c.cpp",
        "verilated_threads.cpp",
        "verilated_vpi.cpp",
        "verilated.cpp",
    ]
    .into_iter()
    .map(|p| verilated_lib_path.join(p))
    .collect();

    Ok(files)
}

pub fn compile_verilated_module(
    verilated_module_path: &Path,
    cpp_shim_path: &Path,
    verilated_lib_path: &Path,
    top_module: &str,
) -> Result<(), VerilatedModuleError> {
    let mut files = get_verilated_module_files(verilated_module_path)?;
    let mut lib_files = get_lib_files(verilated_lib_path)?;
    files.append(&mut lib_files);
    files.push(cpp_shim_path.into());

    let mut include_dirs = get_lib_include_dirs(verilated_lib_path)?;
    include_dirs.push(verilated_module_path.into());

    let mut cpp_cfg = cc::Build::new();
    cpp_cfg.cpp(true).define("VL_PRINTF", "printf");
    cpp_cfg.opt_level(3);

    let tool = cpp_cfg.get_compiler();
    if tool.is_like_clang() {
        cpp_cfg
            .flag("-faligned-new")
            .flag("-fbracket-depth=4096")
            .flag("-Qunused-arguments")
            .flag("-Wno-parentheses-equality")
            .flag("-Wno-sign-compare")
            .flag("-Wno-uninitialized")
            .flag("-Wno-unused-parameter")
            .flag("-Wno-unused-variable")
            .flag("-Wno-shadow");
    }
    if tool.is_like_gnu() {
        cpp_cfg
            .flag("-std=gnu++17")
            .flag("-faligned-new")
            .flag("-Wno-bool-operation")
            .flag("-Wno-sign-compare")
            .flag("-Wno-uninitialized")
            .flag("-Wno-unused-but-set-variable")
            .flag("-Wno-unused-parameter")
            .flag("-Wno-unused-variable")
            .flag("-Wno-shadow");
    }

    include_dirs.into_iter().for_each(|dir| {
        cpp_cfg.include(dir);
    });

    files.into_iter().for_each(|file| {
        cpp_cfg.file(file);
    });

    cpp_cfg.define("VM_TRACE", "1");

    cpp_cfg.compile(&format!("V{}__ALL", top_module));

    return Ok(());
}

#[allow(dead_code)]
pub fn compile_verilated_libs(verilated_lib_path: &Path) -> Result<(), VerilatedLibsError> {
    let (major, minor) = (4, 110);
    println!("cargo:rustc-cfg=verilator_version=\"{}.{}\"", major, minor);
    println!("cargo:rustc-cfg=verilator=\"flush_and_exit_cb\"");

    let files: Vec<PathBuf> = get_lib_files(verilated_lib_path)?;

    let mut cfg = cc::Build::new();
    let tool = cfg.get_compiler();
    cfg.cpp(true);
    cfg.opt_level(3);

    if tool.is_like_clang() {
        cfg.flag("-faligned-new")
            .flag("-fbracket-depth=4096")
            .flag("-Qunused-arguments")
            .flag("-Wno-parentheses-equality")
            .flag("-Wno-sign-compare")
            .flag("-Wno-uninitialized")
            .flag("-Wno-unused-parameter")
            .flag("-Wno-unused-variable")
            .flag("-Wno-shadow");
    }
    if tool.is_like_gnu() {
        cfg.flag("-std=gnu++17")
            .flag("-faligned-new")
            .flag("-Wno-bool-operation")
            .flag("-Wno-sign-compare")
            .flag("-Wno-uninitialized")
            .flag("-Wno-unused-but-set-variable")
            .flag("-Wno-unused-parameter")
            .flag("-Wno-unused-variable")
            .flag("-Wno-shadow");
    }

    cfg.define("VERILATOR_VERSION_MAJOR", major.to_string().as_str())
        .define("VERILATOR_VERSION_MINOR", minor.to_string().as_str());

    let include_dirs = get_lib_include_dirs(verilated_lib_path)?;
    include_dirs.iter().for_each(|include| {
        cfg.include(include);
    });
    cfg.files(files);
    cfg.compile("verilated_libs_cc");

    return Ok(());
}

#[cfg(test)]
mod tests {

    use std::fs::read_dir;

    use tempfile::TempDir;

    use super::*;

    #[test]
    fn places_verilated_lib_into_nonexistent_directory() {
        let dir = TempDir::new().unwrap();

        let target_dir = dir.path().join("output");

        place_verilated_libs(&target_dir).unwrap();

        let files = read_dir(&target_dir).unwrap();

        assert!(files.count() > 4);

        dir.close().unwrap();
    }

    #[test]
    fn places_verilated_module_into_nonexistent_directory() {
        let dir = TempDir::new().unwrap();

        // let target_dir = dir.path().join("output");
        const VERILOG_COUNTER: &str = r#"
        module counter
        (
            input clock,
            output [5:0] led
        );
        
        reg [23:0] clockCounter = 0;
        localparam WAIT_TIME = 1000;
        reg [5:0] ledCounter = 0;
        
        always @(posedge clock) begin
            clockCounter <= clockCounter + 1;
            if (clockCounter == WAIT_TIME) begin
                clockCounter <= 0;
                ledCounter <= ledCounter + 1;
            end
        end
        
        assign led = ~ledCounter;
        endmodule
        "#;

        let file = CrateFile {
            path: dir.path().join("counter.v"),
            content: VERILOG_COUNTER.to_string(),
        };
        file.write().unwrap();

        place_verilated_module(&file, "counter").unwrap();

        let files = read_dir(&dir.path().join("verilated_module")).unwrap();
        let count = files.count();
        println!("path: {}", dir.path().as_os_str().to_str().unwrap());

        assert!(count > 4);

        dir.close().unwrap();
    }

    #[test]
    fn skips_placing_verilated_lib_into_existing_directory() {
        let dir = TempDir::new().unwrap();

        let target_dir = dir.path();

        place_verilated_libs(&target_dir).unwrap();

        let files = read_dir(&target_dir).unwrap();

        assert_eq!(files.count(), 0);

        dir.close().unwrap();
    }

    // #[test]
    // fn compile_verilated_libs_works() {
    //     let dir = TempDir::new().unwrap();

    //     let target_dir = dir.path().join("output");

    //     compile_verilated_libs(&target_dir).unwrap();

    //     dir.close().unwrap();
    // }
}
