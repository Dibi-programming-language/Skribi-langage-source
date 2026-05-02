{
  description = "Building pipeline for the Skribi lang";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs?ref=nixos-unstable";
    utils.url = "github:numtide/flake-utils";
  };

  outputs =
    {
      nixpkgs,
      utils,
      ...
    }:
    utils.lib.eachDefaultSystem (
      system:
      let
        pkgs = nixpkgs.legacyPackages.${system};
        skribiBuild = pkgs.rustPlatform.buildRustPackage {
            pname = "skribi";
            version = "0.1";
            src = ./.;

            doCheck = true;
            cargoLock = {
              lockFile = ./Cargo.lock;
            };
          };
      in
      {
        packages = rec {
          skribi = skribiBuild;
          default = skribi;
        };
        devShells = pkgs.mkShell {
          inputsFrom = [skribiBuild];
          buildInputs = with pkgs; [
            rust-analyzer
            clippy
            rustfmt
          ];
        };
      }
    );
}
