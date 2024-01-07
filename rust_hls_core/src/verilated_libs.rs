use std::{
    io::{self},
    path::{Path, PathBuf},
};

/// Gets the path of the verilated_module directory for the given verilog file.
pub fn get_verilated_module_path(verilog_file_path: &Path) -> Result<PathBuf, io::Error> {
    let target_directory = verilog_file_path
        .parent()
        .ok_or(io::Error::new(
            io::ErrorKind::NotFound,
            format!(
                "Verilog source file has invalid path: {:?}",
                verilog_file_path
            ),
        ))?
        .join("verilated_module");

    return Ok(target_directory);
}

/// Get the path to the directory that contains the headers and source of the verilated runtime
/// library.
///
/// Currently always returns `rust_hls/verilated_libs`.
pub fn get_verilated_libs_path(crate_root: &Path) -> PathBuf {
    crate_root.join("rust_hls/verilated_libs")
}
