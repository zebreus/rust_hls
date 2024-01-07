//! This module provides common functionality for the rust_hls crates.
//! The focus is on module discovery and management. The actual processing of the
//! modules is done by the `rust_hls_generator` crate, so that this crate is lighter
//! and does not have a ton of dependencies.
//! This module provides functionality for processing crates with rust-hls annotations.
//! You probably do not want to use this directly, but instead use the `rust_hls`crate
//! or the `rust_hls_cli` crate (I am not yet sure which one as the project is constantly restructured).

mod darling_error_outside_macro;
mod generated_file;
mod process_module;

#[cfg(feature = "verilator")]
mod verilated_libs;

mod find_modules;
pub mod perform_hls;

pub use find_modules::{find_modules, FindModulesError, MacroModule};
pub use generated_file::{
    extract_file_hash, filename_to_module_path, generate_hash_comment, generate_hash_regex,
    generate_llvm_file_path, generate_log_file_path, generate_output_filename,
    generate_output_module_path, generate_verilated_cpp_file_path, generate_verilator_output_path,
    generate_verilog_file_path, module_path_to_filename, ExtractHashError, ExtractModulePathError,
};
pub use perform_hls::{check_hls, CheckHlsError, PerformHlsResult};
pub use process_module::{process_module, ProcessModuleError, ProcessedModule};
pub use verilated_libs::{get_verilated_libs_path, get_verilated_module_path};

// #[cfg(feature = "verilator")]
// mod generate_verilator_shim;
// #[cfg(feature = "verilator")]
// pub use generate_verilator_shim::*;
