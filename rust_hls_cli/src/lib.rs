//! This crate provides the cli utitlity for rust-hls.
//!
//! It is basically just a wrapper around rust_hsl_buildscript.
//!
//! If you just want to use rust-hls, you probably want to use the `rust_hls` crate
//! instead which re-exports this crate.

use clap::Parser;
use rust_hls_core::generator_hls;

/// Search for a pattern in a file and display the lines that contain it.
#[derive(Parser)]
struct Cli {
    /// The pattern to look for
    directory: std::path::PathBuf,

    /// Remove all generated files
    #[arg(short, long)]
    clean: bool,
}

pub fn main() {
    let args = Cli::parse();

    let root = args.directory.clone();

    generator_hls(&root).unwrap();
}
