//! This module is responsible for compiling and linking the verilated module at compile time.
//!
//! If you do not want to use verilator for simulation, you can disable the `verilator` feature.
//! Then this module becomes a no-op.
//!
//! # Example
//!
//! ```rust
//! // In your build.rs
//!
//! fn main() {
//!   // You should probably import Build from the `rust_hls` crate and not use the `rust_hls_build` crate directly
//!   rust_hls_build::Build::new().synthesize();
//! }
//! ```

mod build;
pub mod buildscript;
#[cfg(feature = "verilator")]
mod compile_verilated_libs;

pub use build::{Build, Error};
