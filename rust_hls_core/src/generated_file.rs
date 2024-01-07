//! The filenames of the generated files are generated here
use std::path::PathBuf;

use regex::Regex;
use thiserror::Error;

mod filename_to_module_path;
pub use filename_to_module_path::{
    filename_to_module_path, module_path_to_filename, ExtractModulePathError,
};
// mod generate_verilator_rust_hdl_struct;

/// Generate the name of the generated file from the rust module path of the input file.
///
/// Relative to the crate root
pub fn generate_output_filename(source_module_path: &Vec<String>) -> PathBuf {
    module_path_to_filename(&generate_output_module_path(source_module_path))
}

/// Generate the module path to the generated file
pub fn generate_output_module_path(source_module_path: &Vec<String>) -> Vec<String> {
    let mut synthesized_module_path = source_module_path.clone();
    let last_module = synthesized_module_path.pop().unwrap_or("lib".into());
    synthesized_module_path.push(rust_hls_macro_lib::synthesized_module_name(&last_module));
    return synthesized_module_path;
}

pub fn generate_verilator_output_path(source_module_path: &Vec<String>) -> PathBuf {
    let synthesized_module_path = generate_output_module_path(source_module_path);
    let file_path = format!("rust_hls/{}", synthesized_module_path.join("/"));
    return PathBuf::from(file_path);
}

pub fn generate_verilated_cpp_file_path(
    source_module_path: &Vec<String>,
    function_name: &str,
) -> PathBuf {
    let base_path = generate_verilator_output_path(source_module_path);
    let file_path = base_path.join(format!("{}.cpp", function_name));
    return PathBuf::from(file_path);
}

pub fn generate_llvm_file_path(source_module_path: &Vec<String>, function_name: &str) -> PathBuf {
    let base_path = generate_verilator_output_path(source_module_path);
    let file_path = base_path.join(format!("{}.ll", function_name));
    return PathBuf::from(file_path);
}

pub fn generate_log_file_path(source_module_path: &Vec<String>, function_name: &str) -> PathBuf {
    let base_path = generate_verilator_output_path(source_module_path);
    let file_path = base_path.join(format!("{}.log", function_name));
    return PathBuf::from(file_path);
}

pub fn generate_verilog_file_path(
    source_module_path: &Vec<String>,
    function_name: &str,
) -> PathBuf {
    let base_path = generate_verilator_output_path(source_module_path);
    let file_path = base_path.join(format!("{}.v", function_name));
    return PathBuf::from(file_path);
}

pub fn generate_hash_comment(hash: &str) -> String {
    format!("// rust_hls hash: \"{}\"", hash)
}

pub fn generate_hash_regex() -> Regex {
    let regex = Regex::new(r#"(?m)^[\s]*// rust_hls hash: "([^"]*)"[\s]*$"#).unwrap();
    return regex;
}

#[derive(Error, Debug)]
pub enum ExtractHashError {
    #[error(
        "Failed to find a rust_hls hash in an existing output file. Maybe it was modified by hand?"
    )]
    FailedToFindHashInExistingFile,
}

/// Extracts the file hash from a generated file
pub fn extract_file_hash(content: &str) -> Result<String, ExtractHashError> {
    let regex = generate_hash_regex();
    let captures = regex
        .captures(content)
        .ok_or(ExtractHashError::FailedToFindHashInExistingFile)?;
    let hash = captures
        .get(1)
        .ok_or(ExtractHashError::FailedToFindHashInExistingFile)?;
    return Ok(hash.as_str().to_string());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generate_filename_seems_to_work() {
        let filename = generate_output_filename(&vec!["test".to_string(), "test2".to_string()]);
        assert!(filename.starts_with("src"));
        assert_eq!(filename.extension().unwrap(), "rs");
    }

    #[test]
    fn generate_module_path_seems_to_work() {
        let module_path =
            generate_output_module_path(&vec!["test".to_string(), "test2".to_string()]);
        assert_eq!(module_path.first().unwrap(), "test");
    }

    #[test]
    fn extract_and_generate_hash_work_together() {
        let original_hash = "sdfsadkfjsjdklf823rfew";
        let generated_hash_comment = generate_hash_comment(original_hash);

        assert!(generated_hash_comment.contains(original_hash));

        let extracted_hash = extract_file_hash(&generated_hash_comment).unwrap();

        assert_eq!(extracted_hash, original_hash);
    }

    #[test]
    fn extract_hash_regex_works() {
        let original_hash = "sdfsadkfjsjdklf823rfew";
        let generated_hash_comment = generate_hash_comment(original_hash);

        let file = format!(
            r##"
        stuff
        
{}

other stuff
        "##,
            generated_hash_comment
        );
        let extracted_hash = extract_file_hash(&file).unwrap();

        assert_eq!(extracted_hash, original_hash);
    }

    #[test]
    fn extract_hash_regex_works_with_leading_and_trailing_spaces() {
        let original_hash = "sdfsadkfjsjdklf823rfew";
        let generated_hash_comment = generate_hash_comment(original_hash);

        let file = format!(
            r##"
        stuff
        
     {}            

other stuff
        "##,
            generated_hash_comment
        );
        let extracted_hash = extract_file_hash(&file).unwrap();

        assert_eq!(extracted_hash, original_hash);
    }
}
