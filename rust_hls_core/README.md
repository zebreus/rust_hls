# rust_hls_core

<!-- cargo-rdme start -->

This module provides common functionality for the rust_hls crates.
The focus is on module discovery and management. The actual processing of the
modules is done by the `rust_hls_generator` crate, so that this crate is lighter
and does not have a ton of dependencies.
This module provides functionality for processing crates with rust-hls annotations.
You probably do not want to use this directly, but instead use the `rust_hls`crate
or the `rust_hls_cli` crate (I am not yet sure which one as the project is constantly restructured).

<!-- cargo-rdme end -->
