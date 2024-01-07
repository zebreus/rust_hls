//! Buildscript entrypoint
//!
//! Used to compile and link the verilator modules

mod process_module;
use std::{io, path::PathBuf};

#[cfg(feature = "verilator")]
mod verilated_libs;

use process_module::*;
mod find_modules;
use find_modules::*;
mod perform_hls;

// #[cfg(feature = "verilator")]
// mod generate_verilator_shim;
// #[cfg(feature = "verilator")]
// pub use generate_verilator_shim::*;

use rust_hls_executor::CrateFile;
use thiserror::Error;

use crate::buildscript_hls::verilated_libs::{
    compile_verilated_module, get_verilated_module_path, place_verilated_module,
};

use self::{
    perform_hls::{CheckHlsError, PerformHlsError, PerformHlsResult},
    verilated_libs::{
        get_verilated_libs_path, place_verilated_libs_in_crate, VerilatedLibsError,
        VerilatedModuleError,
    },
};
#[derive(Error, Debug)]
pub enum HlsBuildscriptError {
    #[error(transparent)]
    FindModulesError(#[from] FindModulesError),
    #[error("Crate root does not exist {path}")]
    FailedToFindCrateRoot { path: String },
    #[error(transparent)]
    CheckHlsError(#[from] CheckHlsError),
    #[error(transparent)]
    VerilatedModuleError(#[from] VerilatedModuleError),
    #[error(transparent)]
    IoError(#[from] io::Error),
}

#[cfg(feature = "verilator")]
pub fn buildscript_hls(root: &PathBuf) -> Result<(), HlsBuildscriptError> {
    let root = root
        .canonicalize()
        .or(Err(HlsBuildscriptError::FailedToFindCrateRoot {
            path: root.to_string_lossy().to_string(),
        }))?;

    let found_modules = find_modules(&root)?;

    for module in found_modules {
        let source_file = module.source_file.to_string_lossy().to_string();
        println!("cargo:rerun-if-changed={}", source_file);
        println!("cargo:warning=Found HLS module in {}", source_file);

        let result = perform_hls::check_hls(&module)?;

        let verilog_file = result.verilog_file_path().to_string_lossy().to_string();

        let verilog_crate_file = CrateFile::from_file(root.join(verilog_file))?;

        compile_verilated_module(
            &get_verilated_module_path(&verilog_crate_file.path)?,
            &root.join(result.verilated_cpp_file_path()),
            &get_verilated_libs_path(&root),
            result.function_name(),
        )?;
    }

    Ok(())
}

#[cfg(not(feature = "verilator"))]
pub fn buildscript_hls(root: &PathBuf) -> Result<(), HlsBuildscriptError> {
    Ok(())
}

#[derive(Error, Debug)]
pub enum HlsGeneratorError {
    #[error(transparent)]
    FindModulesError(#[from] FindModulesError),
    #[error(transparent)]
    ProcessModulesError(#[from] ProcessModuleError),
    #[error("Crate root does not exist {path}")]
    FailedToFindCrateRoot { path: String },
    #[error(transparent)]
    PerformHlsError(#[from] PerformHlsError),
    #[error(transparent)]
    VerilatedLibsError(#[from] VerilatedLibsError),
    #[error(transparent)]
    VerilatedModuleError(#[from] VerilatedModuleError),
    #[error(transparent)]
    IoError(#[from] io::Error),
}

pub fn generator_hls(root: &PathBuf) -> Result<(), HlsGeneratorError> {
    let root = root
        .canonicalize()
        .or(Err(HlsGeneratorError::FailedToFindCrateRoot {
            path: root.to_string_lossy().to_string(),
        }))?;

    let found_modules = find_modules(&root)?;

    #[cfg(feature = "verilator")]
    place_verilated_libs_in_crate(&root)?;

    for module in found_modules {
        let mut result = perform_hls::perform_hls(&module)?;
        if let PerformHlsResult::New {
            synthesized_file,
            verilog_file,
            llvm_file,
            log_file,
            #[cfg(feature = "verilator")]
            verilated_cpp_file,
            ..
        } = &mut result
        {
            synthesized_file.path = root.join(&synthesized_file.path);
            synthesized_file.write()?;
            verilog_file.path = root.join(&verilog_file.path);
            verilog_file.write()?;
            if let Some(llvm_file) = llvm_file {
                llvm_file.path = root.join(&llvm_file.path);
                llvm_file.write()?;
            }
            if let Some(log_file) = log_file {
                log_file.path = root.join(&log_file.path);
                log_file.write()?;
            }
            #[cfg(feature = "verilator")]
            {
                verilated_cpp_file.path = root.join(&verilated_cpp_file.path);
                verilated_cpp_file.write()?;
            }
        }

        #[cfg(feature = "verilator")]
        {
            let verilog_file = result.verilog_file_path().to_string_lossy().to_string();

            let verilog_crate_file = CrateFile::from_file(root.join(verilog_file))?;
            let top_module = result.function_name();

            place_verilated_module(&verilog_crate_file, top_module)?;
        }
    }

    Ok(())
}
