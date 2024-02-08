{
  description = "High-level synthesis from Rust";

  inputs = {
    fenix = {
      url = "github:nix-community/fenix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    nixpkgs.url = "github:NixOS/nixpkgs?ref=c96725682c394613822d477540b4e36abffb8354";
    old-nixpkgs.url = "nixpkgs/nixos-21.05";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, fenix, flake-utils, old-nixpkgs }:
    flake-utils.lib.eachDefaultSystem (system:
      rec {
        old-pkgs = import old-nixpkgs { inherit system; };
        pkgs = import nixpkgs { inherit system; };

        name = "rust_hls";

        packages.default = with pkgs;
          mkShell {
            buildInputs = [
              # For nix
              pkgs.nixpkgs-fmt
              pkgs.nil

              # For rust
              (fenix.packages.${system}.complete.withComponents [
                "cargo"
                "clippy"
                "rust-src"
                "rustc"
                "rustfmt"
              ])
              fenix.packages.${system}.rust-analyzer
              pkgs.lldb_14

              # # For rust_hls
              # # Required commands in path: verilator, bambu, llvm-extract, llvm-link, llvm-dis, jq, grep, sed, tr, export, sh
              # # Also a old version of verilator
              # old-pkgs.verilator
              verilator
              llvmPackages_16.libllvm # Required for rust-hls-lib
              jq
              bambu
            ];
          };
        packages.rust-hls-container = pkgs.dockerTools.buildLayeredImage
          {
            name = "zebreus/rust_hls_tools";
            tag = "latest";
            contents = [
              # bambu
              pkgs.bambu
              # c compiler
              pkgs.stdenv.cc
              # other tools for rust_hls
              pkgs.verilator
              pkgs.llvmPackages_16.libllvm # Required for rust-hls-lib
              pkgs.jq
              # make the container usable interactivly
              pkgs.nix
              pkgs.bashInteractive
              pkgs.coreutils-full
              pkgs.gnutar
              pkgs.gzip
              pkgs.gnugrep
              pkgs.which
              pkgs.curl
              pkgs.less
              pkgs.wget
              pkgs.man
              pkgs.cacert.out
              pkgs.findutils
            ];
            extraCommands = "mkdir -m 0777 tmp";
            config = {
              Cmd = [
                "${pkgs.lib.getExe pkgs.bambu}"
              ];
              WorkingDir = "/src";
            };
          };
      }
    );
}
