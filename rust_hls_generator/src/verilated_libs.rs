//! This file attempts to bundle all the functionalities needed for the verilator libs in one place.

use std::{
    fs::{create_dir_all, remove_dir_all, write},
    io::{self, Read},
    path::{Path, PathBuf},
};

use glob::PatternError;
use rust_hls_core::{get_verilated_libs_path, get_verilated_module_path};
use rust_hls_executor::{calculate_hash, CrateFile};
use rust_hls_verilator::{verilate_module, ObtainVerilatorLibsError, VerilateModuleError};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum PlaceVerilatedLibsError {
    #[error(transparent)]
    IoError(#[from] io::Error),
    #[error(transparent)]
    ObtainVerilatorLibsError(#[from] ObtainVerilatorLibsError),
    #[error("Verilated lib file has invalid path: {0:?}")]
    VerilatedLibFileHasInvalidPath(String),
}

#[derive(Error, Debug)]
pub enum PlaceVerilatedModuleError {
    #[error(transparent)]
    IoError(#[from] io::Error),
    #[error(transparent)]
    VerilateModuleError(#[from] VerilateModuleError),
    #[error(transparent)]
    PatternError(#[from] PatternError),
}

/// Place the verilated libs in the crate root.
pub fn place_verilated_libs_in_crate(crate_root: &Path) -> Result<(), PlaceVerilatedLibsError> {
    let verilated_libs_path = get_verilated_libs_path(crate_root);

    place_verilated_libs(&verilated_libs_path)
}

/// Place the verilator libs in the given directory.
fn place_verilated_libs(path: &Path) -> Result<(), PlaceVerilatedLibsError> {
    let verilator_libs_path = path;

    if verilator_libs_path.exists() {
        return Ok(());
    }

    create_dir_all(verilator_libs_path)?;

    let verilated_libs = rust_hls_verilator::obtain_verilator_libs()?;

    let result: Result<Vec<()>, PlaceVerilatedLibsError> = verilated_libs
        .into_iter()
        .map(|file| -> Result<(), PlaceVerilatedLibsError> {
            let target_path = verilator_libs_path.join(file.path);
            let target_dir = target_path.parent().ok_or(
                PlaceVerilatedLibsError::VerilatedLibFileHasInvalidPath(
                    target_path.to_string_lossy().to_string(),
                ),
            )?;

            create_dir_all(target_dir)?;

            Ok(std::fs::write(target_path, file.content)?)
        })
        .collect();

    match result {
        Ok(_) => Ok(()),
        Err(e) => Err(e)?,
    }
}

pub fn read_hash_file(path: &Path) -> Option<String> {
    let mut file = std::fs::File::open(path.join("state.hash")).ok()?;
    let mut contents = String::new();
    file.read_to_string(&mut contents).ok()?;
    Some(contents.trim().to_string())
}

pub fn write_hash_file(path: &Path, new_hash: &str) -> Result<(), io::Error> {
    let file: PathBuf = path.join("state.hash");
    write(file, new_hash)
}

/// Places the verilator output for a Verilog file in a `verilated_module` directory beside it
pub fn place_verilated_module(
    verilog_file: &CrateFile,
    top_module: &str,
) -> Result<(), PlaceVerilatedModuleError> {
    let target_directory = get_verilated_module_path(&verilog_file.path)?;

    let new_hash = calculate_hash(&vec![verilog_file.content.clone()]);
    let previous_hash = read_hash_file(&target_directory);

    if previous_hash == Some(new_hash.clone()) {
        return Ok(());
    }

    remove_dir_all(&target_directory).unwrap_or(());
    create_dir_all(&target_directory)?;

    verilate_module(&verilog_file.content, top_module)?
        .into_iter()
        .map(|mut file| {
            file.path = target_directory.join(file.path.file_name().unwrap());
            file
        })
        .map(|file| {
            println!("Writing file: {:?}", file.path.display());
            file.write()
        })
        .collect::<Result<Vec<_>, _>>()?;

    write_hash_file(&target_directory, &new_hash)?;

    return Ok(());
}

#[cfg(test)]
mod tests {

    use std::fs::read_dir;

    use tempfile::TempDir;

    use super::*;

    #[test]
    fn places_verilated_lib_into_nonexistent_directory() {
        let dir = TempDir::new().unwrap();

        let target_dir = dir.path().join("output");

        place_verilated_libs(&target_dir).unwrap();

        let files = read_dir(&target_dir).unwrap();

        assert!(files.count() > 4);

        dir.close().unwrap();
    }

    #[test]
    fn places_verilated_module_into_nonexistent_directory() {
        let dir = TempDir::new().unwrap();

        // let target_dir = dir.path().join("output");
        const VERILOG_COUNTER: &str = r#"
        module counter
        (
            input clock,
            output [5:0] led
        );
        
        reg [23:0] clockCounter = 0;
        localparam WAIT_TIME = 1000;
        reg [5:0] ledCounter = 0;
        
        always @(posedge clock) begin
            clockCounter <= clockCounter + 1;
            if (clockCounter == WAIT_TIME) begin
                clockCounter <= 0;
                ledCounter <= ledCounter + 1;
            end
        end
        
        assign led = ~ledCounter;
        endmodule
        "#;

        let file = CrateFile {
            path: dir.path().join("counter.v"),
            content: VERILOG_COUNTER.to_string(),
        };
        file.write().unwrap();

        place_verilated_module(&file, "counter").unwrap();

        let files = read_dir(&dir.path().join("verilated_module")).unwrap();
        let count = files.count();
        println!("path: {}", dir.path().as_os_str().to_str().unwrap());

        assert!(count > 4);

        dir.close().unwrap();
    }

    #[test]
    fn skips_placing_verilated_lib_into_existing_directory() {
        let dir = TempDir::new().unwrap();

        let target_dir = dir.path();

        place_verilated_libs(&target_dir).unwrap();

        let files = read_dir(&target_dir).unwrap();

        assert_eq!(files.count(), 0);

        dir.close().unwrap();
    }
}
