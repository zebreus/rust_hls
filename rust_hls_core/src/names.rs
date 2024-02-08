//! Contains the functions that determine how things are named and where they are placed
//!
//! files?
//!
//! `path` in the name of functions refers to a rust module path (e.g. `test::something::blarg`)
//!
//! `filepath` in the name of functions refers to a filesystem path relative to the crate root
//!
//! The `rust_hls` directory in your repo will be called the rust_hls store or just store

use crate::module_path_to_filename;
use convert_case::{Case, Casing};
use std::path::PathBuf;

/// Get the name of the synthesized struct based on the function name in a source module
pub fn synthesized_struct_name(original_function_name: &str) -> String {
    return format!("{}", original_function_name.to_case(Case::Pascal));
}

/// Get the name of the synthesized module based on a source module
pub fn synthesized_module_name(original_module_name: &str) -> String {
    return format!("{}_synthesized", original_module_name);
}

/// Get the name of the synthesized module from the module path of a source module
///
/// The result will be relative to the crate root
pub fn synthesized_module_filepath(source_module_path: &Vec<String>) -> PathBuf {
    module_path_to_filename(&synthesized_module_path(source_module_path))
}

/// Get the rust module path of the synthesized module from the module path of a source module
pub fn synthesized_module_path(source_module_path: &Vec<String>) -> Vec<String> {
    let mut synthesized_module_path = source_module_path.clone();
    let last_module = synthesized_module_path.pop().unwrap_or("lib".into());
    synthesized_module_path.push(synthesized_module_name(&last_module));
    return synthesized_module_path;
}

/// Get the path to the rust_hls store relative to the crate root
///
/// The rust_hls store can be used to store artifacts. It should get committed to the repository
pub fn store_directory() -> PathBuf {
    return PathBuf::from("rust_hls/");
}

/// Get the directory for a single module in the rust_hls store
///
/// Every synthesized module has a place for its artifacts
pub fn module_store_directory(source_module_path: &Vec<String>) -> PathBuf {
    let synthesized_module_path = synthesized_module_path(source_module_path);
    let synthesized_module_filepath = PathBuf::from(synthesized_module_path.join("/"));
    let filepath = store_directory().join(synthesized_module_filepath);
    return PathBuf::from(filepath);
}

/// Get the directory for the verilator output for a given verilog source file
pub fn verilated_module_directory(source_module_path: &Vec<String>) -> PathBuf {
    let store_directory = module_store_directory(source_module_path);
    return store_directory.join("verilated_module");
}

/// Get the path to the directory that contains the headers and source of the verilated runtime library.
///
/// The path points into the rust_hls store. `rust_hls_generator` should populate it with the system libs.
pub fn verilated_libs_directory() -> PathBuf {
    store_directory().join(PathBuf::from("verilated_libs"))
}

/// Get the path to the verilator shim generated from a given source module
pub fn verilator_shim_file_path(source_module_path: &Vec<String>, function_name: &str) -> PathBuf {
    let base_path = module_store_directory(source_module_path);
    let file_path = base_path.join(format!("{}.cpp", function_name));
    return PathBuf::from(file_path);
}

/// Get the path to the LLVM generated from a given source module
pub fn llvm_file_store_filepath(source_module_path: &Vec<String>, function_name: &str) -> PathBuf {
    let base_path = module_store_directory(source_module_path);
    let file_path = base_path.join(format!("{}.ll", function_name));
    return PathBuf::from(file_path);
}

/// Get the path to the build log generated from a given source module
pub fn log_file_store_filepath(source_module_path: &Vec<String>, function_name: &str) -> PathBuf {
    let base_path = module_store_directory(source_module_path);
    let file_path = base_path.join(format!("{}.log", function_name));
    return PathBuf::from(file_path);
}

/// Get the path to synthesized verilog file for a given source module
pub fn verilog_file_store_filepath(
    source_module_path: &Vec<String>,
    function_name: &str,
) -> PathBuf {
    let base_path = module_store_directory(source_module_path);
    let file_path = base_path.join(format!("{}.v", function_name));
    return PathBuf::from(file_path);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generate_filename_seems_to_work() {
        let filename = synthesized_module_filepath(&vec!["test".to_string(), "test2".to_string()]);
        assert!(filename.starts_with("src"));
        assert_eq!(filename.extension().unwrap(), "rs");
    }

    #[test]
    fn generate_module_path_seems_to_work() {
        let module_path = synthesized_module_path(&vec!["test".to_string(), "test2".to_string()]);
        assert_eq!(module_path.first().unwrap(), "test");
    }
}
