{
  description = "Rust Function Name";

  inputs = {
    utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, utils }:
    utils.lib.eachDefaultSystem
      (system:
        let
          pkgs = nixpkgs.legacyPackages.${system};
        in
        {
          packages.${system}.default = pkgs.stdenv.mkDerivation {
            name = "rust-function_name";
            src = self;
          };
          devShells.default = import ./shell.nix { inherit pkgs; };
        }
      );

}
