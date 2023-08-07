mod process_module;
use std::{io, path::PathBuf};

#[cfg(feature = "verilator")]
mod verilated_libs;

use process_module::*;
mod find_modules;
use find_modules::*;
mod perform_hls;

// #[cfg(feature = "verilator")]
// mod generate_verilator_shim;
// #[cfg(feature = "verilator")]
// pub use generate_verilator_shim::*;

use rust_hls_executor::CrateFile;
use thiserror::Error;

use crate::buildscript_hls::verilated_libs::{
    compile_verilated_module, get_verilated_module_path, place_verilated_module,
    place_verilator_libs,
};

use self::{
    perform_hls::{PerformHlsError, PerformHlsResult},
    verilated_libs::{VerilatedLibsError, VerilatedModuleError},
};
#[derive(Error, Debug)]
pub enum HlsBuildscriptError {
    #[error(transparent)]
    FindModulesError(#[from] FindModulesError),
    #[error(transparent)]
    ProcessModulesError(#[from] ProcessModuleError),
    #[error("Crate root does not exist {path}")]
    FailedToFindCrateRoot { path: String },
    #[error(transparent)]
    PerformHlsError(#[from] PerformHlsError),
    #[error(transparent)]
    VerilatedLibsError(#[from] VerilatedLibsError),
    #[error(transparent)]
    VerilatedModuleError(#[from] VerilatedModuleError),
    #[error(transparent)]
    IoError(#[from] io::Error),
}

pub fn buildscript_hls(root: &PathBuf) -> Result<(), HlsBuildscriptError> {
    let root = root
        .canonicalize()
        .or(Err(HlsBuildscriptError::FailedToFindCrateRoot {
            path: root.to_string_lossy().to_string(),
        }))?;

    let found_modules = find_modules(&root)?;

    #[cfg(feature = "verilator")]
    let verilated_libs_path = root.join(PathBuf::from("rust_hls/verilated_libs"));
    #[cfg(feature = "verilator")]
    place_verilator_libs(&verilated_libs_path)?;

    for module in found_modules {
        let mut result = perform_hls::perform_hls(&module)?;
        println!(
            "cargo:rerun-if-changed={}",
            module.source_file.to_string_lossy().to_string()
        );
        if let PerformHlsResult::New {
            synthesized_file,
            verilog_file,
            llvm_file,
            log_file,
            #[cfg(feature = "verilator")]
            verilated_cpp_file,
            ..
        } = &mut result
        {
            synthesized_file.path = root.join(&synthesized_file.path);
            synthesized_file.write()?;
            verilog_file.path = root.join(&verilog_file.path);
            verilog_file.write()?;
            if let Some(llvm_file) = llvm_file {
                llvm_file.path = root.join(&llvm_file.path);
                llvm_file.write()?;
            }
            if let Some(log_file) = log_file {
                log_file.path = root.join(&log_file.path);
                log_file.write()?;
            }
            #[cfg(feature = "verilator")]
            {
                verilated_cpp_file.path = root.join(&verilated_cpp_file.path);
                verilated_cpp_file.write()?;
            }
        }

        #[cfg(feature = "verilator")]
        {
            // let out_dir = std::env::var("OUT_DIR").unwrap();
            // let out_dir = PathBuf::from(out_dir);
            // std::fs::create_dir_all(&out_dir);
            let verilog_file = result.verilog_file_path().to_string_lossy().to_string();
            // let cpp_file = result
            //     .verilated_cpp_file_path()
            //     .to_string_lossy()
            //     .to_string();

            println!("cargo:rerun-if-changed={}", verilog_file);

            let verilog_crate_file = CrateFile::from_file(root.join(verilog_file))?;
            let top_module = result.function_name();

            // Generate CPP from Verilog

            place_verilated_module(&verilog_crate_file, top_module)?;
            // println!("cargo:warning=############################################################################################################################################### {:?}", verilog_crate_file);

            // compile_verilated_libs(&verilated_libs_path)?;

            compile_verilated_module(
                &get_verilated_module_path(&verilog_crate_file.path)?,
                &root.join(result.verilated_cpp_file_path()),
                &verilated_libs_path,
                top_module,
            )?;
            // let mut verilator = Verilator::default();
            // verilator
            //     .with_coverage(false)
            //     .with_trace(true)
            //     .no_warn("width")
            //     .no_warn("pinmissing")
            //     .no_warn("timescalemod")
            //     .with_performance_optimizations(true)
            //     .file_with_standard(
            //         &root.join(result.verilog_file_path()),
            //         Standard::Verilog2005,
            //     )
            //     .file(&root.join(cpp_file))
            //     .build(result.function_name());
        }
    }

    Ok(())
}
