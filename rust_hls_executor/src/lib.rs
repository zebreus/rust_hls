//! This crate is part of `rust_hls`.
//!
//! It provides functions to run synthesis on an extracted crate that contains a hls script.
//!
//! The main function takes a list of files, writes them to disk, tries to find and execute `script.sh`, and extracts the results. It also performs caching of the results, so synthesizing the same thing twice is a no-op. The temporary files are written to something like `~/.cache/rust_hls`.
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
mod cache_workspace;
mod caching;
mod rust_hls;

pub use crate::rust_hls::RustHls;
pub use crate::rust_hls::RustHlsError;
pub use crate::rust_hls::RustHlsResult;
