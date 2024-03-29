use super::{find_modules::MacroModule, process_module::ProcessModuleError};
use crate::CrateFile;
use crate::{
    llvm_file_store_filepath, log_file_store_filepath, process_module::process_module,
    synthesized_module_filepath, verilator_shim_file_path, verilog_file_store_filepath,
};
use std::path::PathBuf;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum CheckHlsError {
    #[error(transparent)]
    ProcessModuleError(#[from] ProcessModuleError),
    #[error("You need to run `cargo hls` to update the synthesized module for \"{0}\" because its source has changed.")]
    ModuleChanged(String),
    #[error("Module {0} has not been processed yet. You need to run `cargo hls` first.")]
    ModuleNotProcessed(String),
}

pub enum PerformHlsResult {
    Cached {
        synthesized_file: PathBuf,
        source_path: Vec<String>,
        verilog_file: PathBuf,
        llvm_file: Option<PathBuf>,
        log_file: Option<PathBuf>,
        function_name: String,
        #[cfg(feature = "verilator")]
        verilated_cpp_file: PathBuf,
    },
    New {
        /// The generated verilog module
        synthesized_file: CrateFile,
        source_path: Vec<String>,
        verilog_file: CrateFile,
        llvm_file: Option<CrateFile>,
        log_file: Option<CrateFile>,
        function_name: String,
        #[cfg(feature = "verilator")]
        verilated_cpp_file: CrateFile,
    },
}

impl PerformHlsResult {
    #[allow(unused, dead_code)]
    pub fn synthesized_file_path(&self) -> &PathBuf {
        match self {
            Self::Cached {
                synthesized_file, ..
            } => synthesized_file,
            Self::New {
                synthesized_file, ..
            } => &synthesized_file.path,
        }
    }
    #[allow(unused, dead_code)]
    pub fn verilog_file_path(&self) -> &PathBuf {
        match self {
            Self::Cached { verilog_file, .. } => verilog_file,
            Self::New { verilog_file, .. } => &verilog_file.path,
        }
    }
    #[allow(unused, dead_code)]
    pub fn llvm_file_path(&self) -> Option<&PathBuf> {
        match self {
            Self::Cached { llvm_file, .. } => llvm_file.as_ref(),
            Self::New { llvm_file, .. } => llvm_file.as_ref().map(|f| &f.path),
        }
    }
    #[allow(unused, dead_code)]
    pub fn log_file_path(&self) -> Option<&PathBuf> {
        match self {
            Self::Cached { log_file, .. } => log_file.as_ref(),
            Self::New { log_file, .. } => log_file.as_ref().map(|f| &f.path),
        }
    }
    #[allow(unused, dead_code)]
    pub fn function_name(&self) -> &String {
        match self {
            Self::Cached { function_name, .. } => function_name,
            Self::New { function_name, .. } => function_name,
        }
    }
    #[cfg(feature = "verilator")]
    pub fn verilated_cpp_file_path(&self) -> &PathBuf {
        match self {
            Self::Cached {
                verilated_cpp_file, ..
            } => verilated_cpp_file,
            Self::New {
                verilated_cpp_file, ..
            } => &verilated_cpp_file.path,
        }
    }
}

/// Check if HLS has been performed for a given module
pub fn check_hls(module: &MacroModule) -> Result<PerformHlsResult, CheckHlsError> {
    let processed_module: crate::ProcessedModule = process_module(&module)?;
    let new_hash = processed_module.calculate_hash();

    // Return empty vec if the hashes match
    let input_module_path = &module.absolute_module_path;
    let previous_hash = &module.previous_hash;

    let Some(previous_hash) = previous_hash else {
        return Err(CheckHlsError::ModuleNotProcessed(
            processed_module.function_name,
        ));
    };

    if previous_hash != &new_hash {
        return Err(CheckHlsError::ModuleChanged(processed_module.function_name));
    }

    return Ok(PerformHlsResult::Cached {
        synthesized_file: synthesized_module_filepath(input_module_path),
        source_path: module.absolute_module_path.clone(),
        verilog_file: verilog_file_store_filepath(
            input_module_path,
            &processed_module.function_name,
        ),
        llvm_file: if processed_module
            .rust_hls_args
            .include_llvm_ir
            .unwrap_or(false)
        {
            Some(llvm_file_store_filepath(
                input_module_path,
                &processed_module.function_name,
            ))
        } else {
            None
        },
        log_file: if processed_module.rust_hls_args.include_logs.unwrap_or(false) {
            Some(log_file_store_filepath(
                input_module_path,
                &processed_module.function_name,
            ))
        } else {
            None
        },
        function_name: processed_module.function_name.clone(),
        #[cfg(feature = "verilator")]
        verilated_cpp_file: verilator_shim_file_path(
            input_module_path,
            &processed_module.function_name,
        ),
    });
}

// TODO: Add test for check hls
