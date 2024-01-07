#!/usr/bin/env bash

# This script updates the README.md files in the subdirectories of this directory.

packages=(
    "extract_rust_hdl_interface"
    "rust_hls"
    "rust_hls_core"
    "rust_hls_cli"
    "rust_hls_build"
    "examples/multiplier"
    "examples/adder"
    "examples/using_rust_hls_modules"
    "rust_hls_executor"
    "rust_hls_macro"
    "rust_hls_macro_lib"
    "rust_hls_script_generator"
    "rust_hls_tools"
    "rust_hls_verilator"
    "rust_hls_verilator_shim"
    "spanned_error_message"
    "wrap_verilog_in_rust_hdl_macro"
)

for package in "${packages[@]}"; do
    cd $package
    package_name=$(basename $package)
    cat <<EOF >README
# $package_name

<!-- cargo-rdme -->
EOF
    cargo rdme --force
    cd - >/dev/null
done
