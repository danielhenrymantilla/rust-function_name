{
  description = "Rust Function Name";

  inputs = {
    utils.url = "github:numtide/flake-utils";
    rust-overlay.url = "github:oxalica/rust-overlay";
  };

  outputs = { self, nixpkgs, utils, rust-overlay }:
    utils.lib.eachDefaultSystem
      (system:
        let
          overlays = [ (import rust-overlay) ];
          pkgs = import nixpkgs { inherit system overlays; };
          rust = pkgs.rust-bin.fromRustupToolchainFile ./rust-toolchain;
        
          baseInputs = with pkgs; [
            ## Rust
            rust
            pkg-config
            openssl
            openssl.dev
            llvm
            wllvm
            rust-bindgen
          ];

          rust-function_name = pkgs.rustPlatform.buildRustPackage rec {
            name = "rust-function_name-${version}";
            version = "master";

            src = pkgs.fetchFromGitHub {
              owner = "Unbox-infinity";
              repo = "rust-function_name";
              rev = "${version}";
              sha256 = "sha256-EVv6X2hhS5OVeNCRg7Ut8cVpF5FZ2RB4bMQDv/pgc58=";
            };

            cargoSha256 = "sha256-LmhCymOgUi9llEnR4ZZ4g/a+eSng0ZzFefY1wQtIRZ8=";
          };

        in
        {
          defaultPackage = rust-function_name;  
          devShells.default = import ./shell.nix { inherit pkgs; };
        }
      );

}
