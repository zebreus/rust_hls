//! This file attempts to bundle all the functionalities needed for the verilator libs in one place.

use std::{
    fs::{create_dir_all, remove_dir_all, write},
    io::{self, Read},
    path::{Path, PathBuf},
};

use glob::PatternError;
use itertools::Itertools;
use rust_hls_core::{calculate_hash, verilated_libs_directory, CrateFile};
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

/// Place the verilator libs in the verilated_libs_directory in crate_root
pub fn place_verilated_libs_in_crate(crate_root: &Path) -> Result<(), PlaceVerilatedLibsError> {
    if crate_root.join(verilated_libs_directory()).exists() {
        return Ok(());
    }

    eprintln!("Crateroot: {:?}", crate_root);

    let verilated_libs = rust_hls_verilator::obtain_verilator_libs()?;

    eprintln!(
        "Got files: {:?}",
        verilated_libs.iter().map(|f| &f.path).collect_vec()
    );

    let result: Result<_, io::Error> = verilated_libs
        .into_iter()
        .map(|file| -> Result<(), io::Error> { file.write_to_crate(crate_root) })
        .collect();

    // eprintln!("Got result: {:?}", result);

    result?;
    return Ok(());
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
    target_directory: &Path,
) -> Result<(), PlaceVerilatedModuleError> {
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

        place_verilated_libs_in_crate(&target_dir).unwrap();

        let files = read_dir(&target_dir.join(verilated_libs_directory())).unwrap();

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

        let target_directory = dir.path().join("verilated_module");

        place_verilated_module(&file, "counter", &target_directory).unwrap();

        let files = read_dir(&target_directory).unwrap();
        let count = files.count();

        assert!(count > 4);
        dir.close().unwrap();
    }

    #[test]
    fn skips_placing_verilated_lib_into_existing_directory() {
        let dir = TempDir::new().unwrap();

        let crate_root = dir.path();
        create_dir_all(crate_root.join(verilated_libs_directory())).unwrap();

        place_verilated_libs_in_crate(&crate_root).unwrap();

        let files = read_dir(&crate_root.join(verilated_libs_directory())).unwrap();

        assert_eq!(files.count(), 0);

        dir.close().unwrap();
    }
}
