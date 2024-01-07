//! Contains the `verilate_module` function, which runs Verilator on a given Verilog file

use rust_hls_executor::CrateFile;
use rust_hls_tools::{RustHlsCommand, RustHlsCommandError};
use std::{
    fs::{create_dir_all, read_dir},
    path::{Path, PathBuf},
};
use tempfile::TempDir;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum VerilateModuleError {
    #[error(transparent)]
    RustHlsCommandError(#[from] RustHlsCommandError),
    #[error(transparent)]
    IoError(#[from] std::io::Error),
}

/// Run verilator on the given verilog file and return all generated C files.
pub fn verilate_module(
    verilog: &str,
    top_module: &str,
) -> Result<Vec<CrateFile>, VerilateModuleError> {
    let dir = TempDir::new()?;

    let input_file_path = dir.path().join("test.v");
    let output_path = dir.path().join("output");
    create_dir_all(&output_path)?;

    std::fs::write(&input_file_path, verilog.to_string())?;

    run_verilator(&dir.path(), &input_file_path, &output_path, top_module)?;

    let files = collect_results(&output_path)?;

    dir.close().unwrap();

    return Ok(files);
}

/// Create a command for running Verilator. The command has no arguments.
// pub fn create_verilator_command(working_directory: &Path) -> Command {
//     let working_directory = working_directory.to_string_lossy().to_string();
//     let mut command = Command::new("docker");

//     command
//         .arg("run")
//         .arg("--rm")
//         .arg("-v")
//         .arg(format!("{working_directory}:{working_directory}"))
//         .arg("-w")
//         .arg(working_directory)
//         .arg("zebreus/rust_hls_tools:latest")
//         .arg("verilator_bin");

//     command
// }

// Run verilator on the given verilog file and place the output in the output directory.
fn run_verilator(
    working_directory: &Path,
    input_file: &Path,
    output_directory: &Path,
    top_module: &str,
) -> Result<(), VerilateModuleError> {
    let mut cmd = RustHlsCommand::new("verilator_bin");
    cmd.set_working_directory(working_directory);
    cmd.arg("--cc")
        .arg("-Mdir")
        .arg(output_directory.to_string_lossy())
        .arg("--top-module")
        .arg(top_module)
        // TODO: Modify wrapper generator so --trace is not required
        .arg("--trace")
        .arg("-O3")
        .arg("-Wno-width")
        .arg("-Wno-pinmissing")
        .arg("-Wno-timescalemod")
        .arg("+1364-2005ext+v")
        .arg(input_file.to_string_lossy());

    cmd.output()?;

    return Ok(());
}

pub fn collect_results(directory: &PathBuf) -> Result<Vec<CrateFile>, VerilateModuleError> {
    let all_files = read_dir(directory)?;

    let extra_modules = all_files
        .into_iter()
        .filter_map(|entry| match entry {
            Ok(path) => Some(path.file_name().to_string_lossy().to_string()),
            Err(_) => None,
        })
        .filter(|s| s.starts_with("V") && (s.ends_with(".cpp") || s.ends_with(".h")))
        .filter(|s| !s.contains("__Trace.cpp") && !s.contains("__Trace__Slow.cpp"))
        .map(|filename| directory.join(filename));

    let files = extra_modules
        .map(|file| -> Result<CrateFile, VerilateModuleError> {
            let content = std::fs::read_to_string(&file)?;
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
        let verilog = "
        module test(input clk, input reset, output reg [7:0] out);
            always @(posedge clk) begin
                if (reset) begin
                    out <= 0;
                end else begin
                    out <= out + 1;
                end
            end
        endmodule
        ";

        let files = verilate_module(verilog, "test").unwrap();
        assert!(files.len() >= 5);
    }
}
