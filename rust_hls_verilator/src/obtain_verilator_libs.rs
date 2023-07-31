//! This module provides a function to extract the verilated library files from the docker image.

// docker run --rm -it zebreus/rust_hls_tools verilator_bin --getenv VERILATOR_ROOT

use rust_hls_executor::CrateFile;
use std::{
    fs::{create_dir_all, read_dir},
    path::PathBuf,
    process::Command,
};
use tempfile::TempDir;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ObtainVerilatorLibsError {
    #[error("Verilator did not execute successfully")]
    VerilatorDidNotExitSuccessfully(),
    #[error(transparent)]
    IoError(#[from] std::io::Error),
}

pub fn obtain_verilator_libs() -> Result<Vec<CrateFile>, ObtainVerilatorLibsError> {
    let dir = TempDir::new()?;

    let output_path = PathBuf::from(dir.path());
    create_dir_all(&output_path)?;

    copy_from_container(&output_path)?;

    let files = collect_results(&output_path)?;

    dir.close().unwrap();

    return Ok(files);
}

/// Create a command for running Verilator. The command has no arguments.
fn create_verilator_command() -> Command {
    Command::new("verilator")
}

// Run verilator on the given verilog file and place the output in the output directory.
fn copy_from_container(output_directory: &PathBuf) -> Result<(), ObtainVerilatorLibsError> {
    let mut get_root_command = create_verilator_command();
    get_root_command.arg("--getenv").arg("VERILATOR_ROOT");
    let output = get_root_command.output()?;
    let root_path = PathBuf::from(
        String::from_utf8_lossy(output.stdout.as_slice())
            .to_string()
            .trim(),
    );

    let mut copy_files_command = Command::new("sh");
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

    let status = copy_files_command.status()?;
    if !status.success() {
        return Err(ObtainVerilatorLibsError::VerilatorDidNotExitSuccessfully());
    }
    return Ok(());
}

fn collect_results(directory: &PathBuf) -> Result<Vec<CrateFile>, ObtainVerilatorLibsError> {
    let root_files = read_dir(directory)?;
    let vltstd_files = read_dir(directory.join("vltstd"))?;

    let all_files = root_files.chain(vltstd_files);

    // let all_files = read_dir(directory.join("vltstd"))?;

    let extra_modules = all_files.filter_map(|entry| match entry {
        Ok(path) => {
            let Ok(file_type) = path.file_type() else {
                return None;
            };
            if !file_type.is_file() {
                return None;
            }
            Some(
                path.path()
                    .strip_prefix(&directory)
                    .unwrap()
                    .to_string_lossy()
                    .to_string(),
            )
        }
        Err(_) => None,
    });

    let files = extra_modules
        .map(|file| -> Result<CrateFile, ObtainVerilatorLibsError> {
            let content = std::fs::read_to_string(&directory.join(&file))?;
            Ok(CrateFile {
                path: PathBuf::from(file),
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
        assert_eq!(files.len(), 31);
    }
}
