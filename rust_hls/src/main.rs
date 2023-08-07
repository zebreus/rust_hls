use clap::Parser;
use rust_hls::generator_hls;

/// Search for a pattern in a file and display the lines that contain it.
#[derive(Parser)]
struct Cli {
    /// The pattern to look for
    directory: std::path::PathBuf,
}

fn main() {
    let args = Cli::parse();

    let root = args.directory.clone();

    generator_hls(&root).unwrap();
}
