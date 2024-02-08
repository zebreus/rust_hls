use crate::compile_verilated_libs::CompileVerilatedLibsError;
use rust_hls_core::{
    check_hls::{check_hls, CheckHlsError},
    find_modules, verilated_libs_directory, verilated_module_directory, FindModulesError,
};
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

    let mut compiler = Compiler::new(&root.join(verilated_libs_directory()));

    for module in found_modules {
        let source_file = module.source_file.to_string_lossy().to_string();
        println!("cargo:rerun-if-changed={}", source_file);
        println!("cargo:warning=Found HLS module in {}", source_file);

        let result = check_hls(&module)?;

        compiler.add_verilated_module(
            &verilated_module_directory(&module.absolute_module_path),
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
