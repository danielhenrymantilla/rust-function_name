{ pkgs ? import <nixpkgs> { } }:
let
in
with pkgs;

stdenv.mkDerivation {
  name = "rust-function_name";
  buildInputs = [
    ## Rust
    cargo
    rustup
    rustc
    pkg-config
    openssl
    openssl.dev
    llvm
    wllvm
    rust-bindgen
    rust-analyzer
    rustfmt
    # extra tools
    cargo-audit
    cargo-edit
    cargo-outdated
    cargo-watch
    ## Nix
    nixpkgs-fmt
  ];
  # OPENSSL_DEV=openssl.dev;
  # IS_DEV=1;
  RUST_LOG = "info";
}
