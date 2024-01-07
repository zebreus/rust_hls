use crate::compile_verilated_libs::CompileVerilatedLibsError;
use rust_hls_core::{
    find_modules, get_verilated_libs_path, get_verilated_module_path,
    perform_hls::{check_hls, CheckHlsError},
    FindModulesError,
};
use rust_hls_executor::CrateFile;
use std::{io, path::PathBuf};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum HlsBuildscriptError {
    #[error(transparent)]
    FindModulesError(#[from] FindModulesError),
    #[error("Crate root does not exist {path}")]
    FailedToFindCrateRoot { path: String },
    #[error(transparent)]
    CheckHlsError(#[from] CheckHlsError),
    #[error(transparent)]
    CompileVerilatedLibsError(#[from] CompileVerilatedLibsError),
    #[error(transparent)]
    IoError(#[from] io::Error),
}

#[cfg(feature = "verilator")]
pub fn buildscript_hls(root: &PathBuf) -> Result<(), HlsBuildscriptError> {
    use crate::compile_verilated_libs::Compiler;

    let root = root
        .canonicalize()
        .or(Err(HlsBuildscriptError::FailedToFindCrateRoot {
            path: root.to_string_lossy().to_string(),
        }))?;

    let found_modules = find_modules(&root)?;

    if found_modules.is_empty() {
        println!("cargo:warning=No HLS modules found");
        return Ok(());
    }

    let mut compiler = Compiler::new(&get_verilated_libs_path(&root));

    for module in found_modules {
        let source_file = module.source_file.to_string_lossy().to_string();
        println!("cargo:rerun-if-changed={}", source_file);
        println!("cargo:warning=Found HLS module in {}", source_file);

        let result = check_hls(&module)?;

        let verilog_file = result.verilog_file_path().to_string_lossy().to_string();

        let verilog_crate_file = CrateFile::from_file(root.join(verilog_file))?;

        compiler.add_verilated_module(
            &get_verilated_module_path(&verilog_crate_file.path)?,
            &root.join(result.verilated_cpp_file_path()),
        );
    }

    compiler.compile();

    Ok(())
}

#[cfg(not(feature = "verilator"))]
pub fn buildscript_hls(root: &PathBuf) -> Result<(), HlsBuildscriptError> {
    Ok(())
}
