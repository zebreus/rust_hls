//! This crate is part of `rust_hls`.
//!
//! The part of `rust_hls` that is responsible for generating bash scripts that perform HLS using native tools
//!
//! For now it only supports PandA bambu as a HLS backend
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
mod generate_hls_script;

pub use generate_hls_script::{
    generate_hls_script, GenerateHlsOptions, DEFAULT_HLS_FLAGS, DEFAULT_RUST_FLAGS,
};
