/// There can only be one HLS function per module
use rust_hls_macro::hls;

#[hls]
pub mod adder {}

fn main() {}
