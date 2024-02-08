//! This module provides common functionality for the rust_hls crates.
//! The focus is on module discovery and management. The actual processing of the
//! modules is done by the `rust_hls_generator` crate, so that this crate is lighter
//! and does not have a ton of dependencies.
//! This module provides functionality for processing crates with rust-hls annotations.
//! You probably do not want to use this directly, but instead use the `rust_hls`crate
//! or the `rust_hls_cli` crate (I am not yet sure which one as the project is constantly restructured).

/// Helper functions for calculating hashes
mod calculate_hash;
pub mod check_hls;
/// Represents a file in a crate
mod crate_file;
mod darling_error_outside_macro;
mod find_modules;
/// Defines the hash comment in generated rust files
mod hash_comment;
/// Convert rust module paths to filesystem paths and back
mod module_path_helpers;
/// Source of truth for file paths and names
mod names;
mod process_module;

pub use calculate_hash::calculate_hash;
pub use check_hls::{check_hls, CheckHlsError, PerformHlsResult};
pub use crate_file::CrateFile;
pub use find_modules::{find_modules, FindModulesError, MacroModule};
pub use hash_comment::{extract_file_hash, hash_comment, hash_comment_regex, ExtractHashError};
pub use module_path_helpers::{
    filename_to_module_path, module_path_to_filename, ExtractModulePathError,
};
pub use names::{
    llvm_file_store_filepath, log_file_store_filepath, module_store_directory,
    synthesized_module_filepath, synthesized_module_name, synthesized_module_path,
    synthesized_struct_name, verilated_libs_directory, verilated_module_directory,
    verilator_shim_file_path, verilog_file_store_filepath,
};
pub use process_module::{process_module, ProcessModuleError, ProcessedModule};
