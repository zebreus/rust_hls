//! This module provides functionality for processing crates with rust-hls annotations.
//! You probably do not want to use this directly, but instead use the `rust_hls`crate
//! or the `rust_hls_cli` crate (I am not yet sure which one as the project is constantly restructured).

mod generated_file;
use std::{io, path::PathBuf};

#[cfg(feature = "verilator")]
mod verilated_libs;

use perform_hls::PerformHlsError;
pub mod perform_hls;

// #[cfg(feature = "verilator")]
// mod generate_verilator_shim;
// #[cfg(feature = "verilator")]
// pub use generate_verilator_shim::*;

use rust_hls_core::{
    find_modules, verilated_module_directory, CrateFile, FindModulesError, PerformHlsResult,
    ProcessModuleError,
};
use thiserror::Error;

use crate::verilated_libs::place_verilated_module;

use self::verilated_libs::{
    place_verilated_libs_in_crate, PlaceVerilatedLibsError, PlaceVerilatedModuleError,
};

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
    VerilatedLibsError(#[from] PlaceVerilatedLibsError),
    #[error(transparent)]
    VerilatedModuleError(#[from] PlaceVerilatedModuleError),
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

            let verilog_crate_file = CrateFile::from_file(&root.join(verilog_file))?;
            let top_module = result.function_name();

            place_verilated_module(
                &verilog_crate_file,
                top_module,
                &root.join(verilated_module_directory(&module.absolute_module_path)),
            )?;
        }
    }

    Ok(())
}
