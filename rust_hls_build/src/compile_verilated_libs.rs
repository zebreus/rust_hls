//! This file attempts to bundle all the functionalities needed for the verilator libs in one place.

use std::{
    io::{self},
    path::{Path, PathBuf},
};

use glob::PatternError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum CompileVerilatedLibsError {
    #[error(transparent)]
    IoError(#[from] io::Error),
    #[error("The verilated lib must be placed before it can be used")]
    VerilatedLibDoesNotExist(),
    #[error(transparent)]
    PatternError(#[from] PatternError),
}

pub fn assert_verilator_libs_exists(
    verilated_lib_path: &Path,
) -> Result<(), CompileVerilatedLibsError> {
    if !verilated_lib_path.exists() {
        return Err(CompileVerilatedLibsError::VerilatedLibDoesNotExist());
    }
    return Ok(());
}

/// Finds all c and cpp files in the given directory.
pub fn get_verilated_module_files(
    verilated_module_path: &Path,
) -> Result<Vec<PathBuf>, PatternError> {
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

fn get_lib_include_dirs(
    verilated_lib_path: &Path,
) -> Result<Vec<PathBuf>, CompileVerilatedLibsError> {
    assert_verilator_libs_exists(verilated_lib_path)?;
    let include_dirs: Vec<PathBuf> =
        vec![verilated_lib_path.into(), verilated_lib_path.join("vltstd")];

    Ok(include_dirs)
}

fn get_lib_files(verilated_lib_path: &Path) -> Result<Vec<PathBuf>, CompileVerilatedLibsError> {
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
) -> Result<(), CompileVerilatedLibsError> {
    let mut files = get_verilated_module_files(verilated_module_path)?;
    let mut lib_files = get_lib_files(verilated_lib_path)?;
    files.append(&mut lib_files);
    files.push(cpp_shim_path.into());

    let mut include_dirs = get_lib_include_dirs(verilated_lib_path)?;
    include_dirs.push(verilated_module_path.into());

    let mut cpp_cfg = cc::Build::new();
    cpp_cfg.cpp(true).define("VL_PRINTF", "printf");
    cpp_cfg.opt_level(3);

    cpp_cfg.flag("-pthread");

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
pub fn compile_verilated_libs(verilated_lib_path: &Path) -> Result<(), CompileVerilatedLibsError> {
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
    // #[test]
    // fn compile_verilated_libs_works() {
    //     let dir = TempDir::new().unwrap();

    //     let target_dir = dir.path().join("output");

    //     compile_verilated_libs(&target_dir).unwrap();

    //     dir.close().unwrap();
    // }
}
