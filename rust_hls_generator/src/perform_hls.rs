use crate::generated_file::{generate_file, GenerateRustHdlStructError};
use itertools::Itertools;
use rust_hls_core::{
    generate_llvm_file_path, generate_log_file_path, generate_output_filename,
    generate_verilated_cpp_file_path, generate_verilog_file_path, process_module, ExtractHashError,
    ExtractModulePathError, MacroModule, PerformHlsResult, ProcessModuleError,
};
use rust_hls_executor::{CrateFile, RustHlsError};
use rust_hls_macro_lib::synthesized_struct_name;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum PerformHlsError {
    #[error(transparent)]
    ProcessModuleError(#[from] ProcessModuleError),
    #[error(transparent)]
    ExtractModulePathError(#[from] ExtractModulePathError),
    #[error(transparent)]
    ExtractHashError(#[from] ExtractHashError),
    #[error(transparent)]
    RustHlsError(#[from] RustHlsError),
    #[error(transparent)]
    FailedToGenerateOutputFile(#[from] GenerateRustHdlStructError),
}

/// Perform HLS on a given list of modules
pub fn perform_hls(module: &MacroModule) -> Result<PerformHlsResult, PerformHlsError> {
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

    let rust_hls = processed_module.to_rust_hls();

    let result = rust_hls.execute()?;

    let struct_name = synthesized_struct_name(&processed_module.function_name);

    let files = generate_file(
        &input_module_path,
        &result.verilog,
        &processed_module.function_name,
        &struct_name,
        &new_hash,
        &processed_module
            .parameters
            .into_iter()
            .map(|(name, _ty)| name)
            .collect_vec(),
    )?;

    let verilog_file = CrateFile {
        path: generate_verilog_file_path(input_module_path, &processed_module.function_name),
        content: result.verilog,
    };
    let log_file = if processed_module.rust_hls_args.include_logs.unwrap_or(false) {
        Some(CrateFile {
            path: generate_log_file_path(input_module_path, &processed_module.function_name),
            content: result.log,
        })
    } else {
        None
    };
    let llvm_file = if processed_module
        .rust_hls_args
        .include_llvm_ir
        .unwrap_or(false)
    {
        Some(CrateFile {
            path: generate_llvm_file_path(input_module_path, &processed_module.function_name),
            content: result.llvm,
        })
    } else {
        None
    };

    #[cfg(feature = "verilator")]
    {
        // let verilogShim
    }

    return Ok(PerformHlsResult::New {
        synthesized_file: files.0,
        verilog_file: verilog_file,
        llvm_file: llvm_file,
        log_file: log_file,
        function_name: processed_module.function_name,
        #[cfg(feature = "verilator")]
        verilated_cpp_file: files.1,
    });
}

#[cfg(test)]
mod tests {
    use super::*;
    use quote::quote;

    #[test]
    fn hls_seems_to_work_on_simple_example() {
        let module = MacroModule::new_for_tests(
            quote!(
                #[hls]
                mod toast {
                    #[hls]
                    #[no_mangle]
                    pub extern "C" fn add(a: u32, b: u32) -> u32 {
                        a + b
                    }
                }
            ),
            "src/lib.rs",
        )
        .0;

        perform_hls(&module).unwrap();
    }

    #[test]
    fn hls_can_output_llvm_ir() {
        let module = MacroModule::new_for_tests(
            quote!(
                #[hls]
                mod toast {
                    #[hls(include_llvm_ir)]
                    #[no_mangle]
                    pub extern "C" fn add(a: u32, b: u32) -> u32 {
                        a + b
                    }
                }
            ),
            "src/lib.rs",
        )
        .0;

        let result = perform_hls(&module).unwrap();
        assert_eq!(result.llvm_file_path().is_some(), true);
    }

    #[test]
    fn hls_seems_to_create_synthesized_output() {
        let module = MacroModule::new_for_tests(
            quote!(
                #[hls]
                mod toast {
                    #[hls]
                    #[no_mangle]
                    pub extern "C" fn add(a: u32, b: u32) -> u32 {
                        a + b
                    }
                }
            ),
            "src/lib.rs",
        )
        .0;

        perform_hls(&module).unwrap();
        // assert_eq!(result.len(), 1);
        // assert_eq!(
        //     result[0].path.to_str().unwrap(),
        //     format!(
        //         "src/{}.rs",
        //         rust_hls_macro_lib::synthesized_module_name("toast")
        //     )
        // );
        // assert!(result[0].content.contains("pub struct Add"));
        // assert!(result[0].content.contains("Code created using PandA"));
    }
}
