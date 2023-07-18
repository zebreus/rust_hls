//! Perform HLS from Rust
//!
//! This crate uses [bambu](https://github.com/ferrandi/PandA-bambu) to synthesize Rust functions into HDL.
//!
//! This crate is quite experimental, things may break. If you encounter problems please open an issue.
//!
//! ## How it works
//!
//! High-level synthesis is performed by ripping out a module that is annoted for hls and creating a new (temporary) crate that only contains that module.
//!
//! The module is then compiled to LLVM IR.
//!
//! The LLVM IR is then passed to bambu, which generates Verilog.
//!
//! The Verilog file is then parsed by this crate and converted into a rust module containing a rust-hdl struct that wraps the HDL.
//!
//! The rust_hdl_macro crate then inserts the mod and use statements to use the synthsized module into the original crate.
//!
//! ## Setup
//!
//! You should add this crate to your `[build-dependencies]` in Cargo.toml and add `rust_hls_macro` to your `[dependencies]`.
//!
//! In your buildscript you should call `rust_hls::Build::new().synthesize();` to synthesize all functions that are annotated with the hls macro (`#[hls]`).
//!
//! ## Simulation
//!
//! rust_hdl does not support to simulating embedded verilog.
//!
//! To enable simulation you need to enable the `"verilator"` feature on this crate.
//! This will integrate Verilator into the build process to allow simulation.
//!
//! ## Dependencies
//!
//! You need to have [bambu](https://github.com/ferrandi/PandA-bambu) installed locally to synthesize Verilog.
//!
//! If you want to use verilator for simulations you need an old version of verilator installed. `v4.108` works for me.
//!
//! Especially the verilator version is quite tricky to get.
//!
//! To get all dependencies you can use the nix package manager. `nix develop github:zebreus/bachelor-thesis` to enter a shell with all dependencies installed. This may take a while as bambu will be compiled from source.
//!
//! ## Example
//!
//! Build script:
//! ```no_run
//! fn main() {
//!     // Finds all modules annotated with `#[hls]` and generates synthesized versions of them
//!     rust_hls::Build::new().synthesize();
//! }
//! ```
//!
//!
//! ```ignore
//! #[rust_hls_macro::hls]
//! pub mod your_module {
//!     #[hls]
//!     pub extern "C" fn your_function(a: u32, b: u32) -> u32 {
//!         // You can insert any Rust code here
//!         // The main restriction is that the code must never panic
//!         // You cannot access things in your crate that are outside of `your_module`
//!         // Accessing external crates is supported
//!         a * b
//!     }
//! }
//! // The macro will insert `mod your_module_synthesized;` and `use your_module_synthesized::YourFunction` here.
//! // The `your_module_synthesized.rs` module is generated by the build script.
//!
//! pub fn main() {
//!     // The generated module is the function name in PascalCase
//!     let mut device = YourFunction::new();
//!     device.connect_all();
//!     let data = generate_verilog(&device);
//!     std::fs::write("./multiplier.v", data).unwrap();
//! }
//! ```
//!
//! You can find a full example at [rust_hdl_example](https://github.com/zebreus/bachelor-thesis/tree/master/bambu-macro/rust_hls_example).
//!
//! ## HLS Macro
//!
//! The hls macro has options for configuring `rust_flags` and `hls_flags`.
//!
//! ## Documentation
//!
//! Currently there is not much more documentation than the rustdoc.
//! If you need more documentation open an issue or an PR.
//!
//! ## Contributing
//!
//! I am happy about any major or minor contributions.
//!
//! ## Bugs
//!
//! There are probably lots of bugs in the code.
mod build;
mod buildscript_hls;
mod darling_error_outside_macro;
mod generated_file;
// pub(crate) use darling_error_outside_macro::DarlingErrorOutsideMacro;

pub use buildscript_hls::buildscript_hls;

pub use build::Build;
pub use build::Error;
