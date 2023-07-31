//! Contains the `verilate_module` function, which runs Verilator on a given Verilog file

use rust_hls_executor::CrateFile;
use std::{
    fs::{create_dir_all, read_dir},
    path::PathBuf,
    process::Command,
};
use tempfile::TempDir;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum VerilateModuleError {
    #[error("Verilator did not execute successfully")]
    VerilatorDidNotExitSuccessfully(),
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

    run_verilator(&input_file_path, &output_path, top_module)?;

    let files = collect_results(&output_path)?;

    dir.close().unwrap();

    return Ok(files);
}

/// Create a command for running Verilator. The command has no arguments.
pub fn create_verilator_command() -> Command {
    Command::new("verilator")
}

// Run verilator on the given verilog file and place the output in the output directory.
fn run_verilator(
    input_file: &PathBuf,
    output_directory: &PathBuf,
    top_module: &str,
) -> Result<(), VerilateModuleError> {
    let mut cmd = create_verilator_command();
    cmd.arg("--cc")
        .arg("-Mdir")
        .arg(&output_directory)
        .arg("--top-module")
        .arg(&top_module)
        // TODO: Modify wrapper generator so --trace is not required
        .arg("--trace")
        .arg("-O3")
        .arg("-Wno-width")
        .arg("-Wno-pinmissing")
        .arg("-Wno-timescalemod")
        .arg("+1364-2005ext+v")
        .arg(&input_file);
    run(&mut cmd)?;
    return Ok(());
}

/// Run a command and return an error if it does not exit successfully.
fn run(cmd: &mut Command) -> Result<(), VerilateModuleError> {
    println!("running: {:?}", cmd);
    let status = cmd.status()?;
    if !status.success() {
        return Err(VerilateModuleError::VerilatorDidNotExitSuccessfully());
    }
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
        assert_eq!(files.len(), 5);
    }
}
