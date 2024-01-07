//! The part of rust_hls that is responsible for generating a proxy to verilated verilog modules

mod generate_verilator_shim;
pub use generate_verilator_shim::{
    generate_verilator_shim_from_rusthdl_module, GenerateVerilatorShimError, GeneratedVerilatorShim,
};
