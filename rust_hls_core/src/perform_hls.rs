use std::path::PathBuf;

use crate::{
    generated_file::{
        generate_llvm_file_path, generate_log_file_path, generate_output_filename,
        generate_verilated_cpp_file_path, generate_verilog_file_path,
    },
    process_module::process_module,
};
use rust_hls_executor::CrateFile;

use super::{find_modules::MacroModule, process_module::ProcessModuleError};

use thiserror::Error;

#[derive(Error, Debug)]
pub enum CheckHlsError {
    #[error(transparent)]
    ProcessModuleError(#[from] ProcessModuleError),
    #[error("Module {0} has not been processed yet. You need to run `cargo hls` first.")]
    ModuleNotProcessed(String),
}

pub enum PerformHlsResult {
    Cached {
        synthesized_file: PathBuf,
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

/// Check if HLS has been performed on a given list of modules
pub fn check_hls(module: &MacroModule) -> Result<PerformHlsResult, CheckHlsError> {
    let processed_module = process_module(&module)?;
    let new_hash = processed_module.calculate_hash();

    // Return empty vec if the hashes match
    let input_module_path = &module.absolute_module_path;
    let previous_hash = &module.previous_hash;
    if let Some(previous_hash) = previous_hash {
        if previous_hash == &new_hash {
            return Ok(PerformHlsResult::Cached {
                synthesized_file: generate_output_filename(input_module_path),
                verilog_file: generate_verilog_file_path(
                    input_module_path,
                    &processed_module.function_name,
                ),
                llvm_file: if processed_module
                    .rust_hls_args
                    .include_llvm_ir
                    .unwrap_or(false)
                {
                    Some(generate_llvm_file_path(
                        input_module_path,
                        &processed_module.function_name,
                    ))
                } else {
                    None
                },
                log_file: if processed_module.rust_hls_args.include_logs.unwrap_or(false) {
                    Some(generate_log_file_path(
                        input_module_path,
                        &processed_module.function_name,
                    ))
                } else {
                    None
                },
                function_name: processed_module.function_name.clone(),
                #[cfg(feature = "verilator")]
                verilated_cpp_file: generate_verilated_cpp_file_path(
                    input_module_path,
                    &processed_module.function_name,
                ),
            });
        }
    }

    return Err(CheckHlsError::ModuleNotProcessed(
        processed_module.function_name,
    ));
}

// TODO: Add test for check hls
// #[cfg(test)]
// mod tests {
//     use super::*;
//     use quote::quote;

//     #[test]
//     fn hls_seems_to_work_on_simple_example() {
//         let module = MacroModule::new_for_tests(
//             quote!(
//                 #[hls]
//                 mod toast {
//                     #[hls]
//                     #[no_mangle]
//                     pub extern "C" fn add(a: u32, b: u32) -> u32 {
//                         a + b
//                     }
//                 }
//             ),
//             "src/lib.rs",
//         )
//         .0;

//         perform_hls(&module).unwrap();
//     }

//     #[test]
//     fn hls_can_output_llvm_ir() {
//         let module = MacroModule::new_for_tests(
//             quote!(
//                 #[hls]
//                 mod toast {
//                     #[hls(include_llvm_ir)]
//                     #[no_mangle]
//                     pub extern "C" fn add(a: u32, b: u32) -> u32 {
//                         a + b
//                     }
//                 }
//             ),
//             "src/lib.rs",
//         )
//         .0;

//         let result = perform_hls(&module).unwrap();
//         assert_eq!(result.llvm_file_path().is_some(), true);
//     }

//     #[test]
//     fn hls_seems_to_create_synthesized_output() {
//         let module = MacroModule::new_for_tests(
//             quote!(
//                 #[hls]
//                 mod toast {
//                     #[hls]
//                     #[no_mangle]
//                     pub extern "C" fn add(a: u32, b: u32) -> u32 {
//                         a + b
//                     }
//                 }
//             ),
//             "src/lib.rs",
//         )
//         .0;

//         perform_hls(&module).unwrap();
//         // assert_eq!(result.len(), 1);
//         // assert_eq!(
//         //     result[0].path.to_str().unwrap(),
//         //     format!(
//         //         "src/{}.rs",
//         //         rust_hls_macro_lib::synthesized_module_name("toast")
//         //     )
//         // );
//         // assert!(result[0].content.contains("pub struct Add"));
//         // assert!(result[0].content.contains("Code created using PandA"));
//     }
// }
