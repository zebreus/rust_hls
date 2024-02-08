//! This module provides a function to extract the verilated library files from the docker image.

use rust_hls_core::{verilated_libs_directory, CrateFile};
use rust_hls_tools::{RustHlsCommand, RustHlsCommandError};
use std::{
    fs::{create_dir_all, read_dir},
    path::PathBuf,
};
use tempfile::TempDir;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ObtainVerilatorLibsError {
    #[error("Verilator did not execute successfully")]
    VerilatorDidNotExitSuccessfully(),
    #[error(transparent)]
    IoError(#[from] std::io::Error),
    #[error(transparent)]
    RustHlsCommandError(#[from] RustHlsCommandError),
}

/// Obtain the verilator libs files
///
/// The path corresponds to the correct path inside the crate
pub fn obtain_verilator_libs() -> Result<Vec<CrateFile>, ObtainVerilatorLibsError> {
    let dir = TempDir::new()?;

    let output_path = PathBuf::from(dir.path());
    create_dir_all(&output_path)?;

    copy_from_container(&output_path)?;

    let files = collect_results(&output_path)?;

    dir.close().unwrap();

    return Ok(files);
}

// Run verilator on the given verilog file and place the output in the output directory.
fn copy_from_container(output_directory: &PathBuf) -> Result<(), ObtainVerilatorLibsError> {
    let mut get_root_command = RustHlsCommand::new("verilator");
    get_root_command.arg("--getenv").arg("VERILATOR_ROOT");
    let output = get_root_command.output()?;
    let root_path = PathBuf::from(
        String::from_utf8_lossy(output.stdout.as_slice())
            .to_string()
            .trim(),
    );

    let mut copy_files_command = RustHlsCommand::new("sh");
    copy_files_command.add_directory(output_directory);
    copy_files_command.arg("-c");

    copy_files_command.arg(format!(
        r#"cp -r {}/include/*.cpp {}/include/*.h {} && mkdir {}/vltstd && cp {}/include/vltstd/*.h {}/vltstd"#,
        root_path.to_string_lossy(),
        root_path.to_string_lossy(),
        output_directory.to_string_lossy(),
        output_directory.to_string_lossy(),
        root_path.to_string_lossy(),
        output_directory.to_string_lossy(),
    ));

    let output = copy_files_command.output()?;
    if !output.status.success() {
        return Err(ObtainVerilatorLibsError::VerilatorDidNotExitSuccessfully());
    }
    return Ok(());
}

/// Copy
fn collect_results(
    system_verilated_libs_directory: &PathBuf,
) -> Result<Vec<CrateFile>, ObtainVerilatorLibsError> {
    let root_files = read_dir(system_verilated_libs_directory)?;
    let vltstd_files = read_dir(system_verilated_libs_directory.join("vltstd"))?;

    let all_files = root_files.chain(vltstd_files);

    // let all_files = read_dir(directory.join("vltstd"))?;

    let verilated_libs_files = all_files.filter_map(|entry| match entry {
        Ok(path) => {
            let Ok(file_type) = path.file_type() else {
                return None;
            };
            if !file_type.is_file() {
                return None;
            }
            Some(
                path.path()
                    .strip_prefix(&system_verilated_libs_directory)
                    .unwrap()
                    .to_owned(),
            )
        }
        Err(_) => None,
    });

    let files = verilated_libs_files
        .map(|file| -> Result<CrateFile, ObtainVerilatorLibsError> {
            let content = std::fs::read_to_string(&system_verilated_libs_directory.join(&file))?;
            Ok(CrateFile {
                path: verilated_libs_directory().join(file),
                content: content,
            })
        })
        .collect::<Result<Vec<_>, _>>()?;

    return Ok(files);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_verilator() {
        let files = obtain_verilator_libs().unwrap();
        // The number of files will change as the verilator version changes.
        assert!(files.len() >= 31);
    }
}
