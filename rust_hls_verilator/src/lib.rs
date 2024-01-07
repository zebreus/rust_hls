//! The part of rust_hls that is responsible for running verilator to generated verilated C++ code

mod obtain_verilator_libs;
mod verilate_module;

pub use obtain_verilator_libs::{obtain_verilator_libs, ObtainVerilatorLibsError};
pub use verilate_module::{verilate_module, VerilateModuleError};
