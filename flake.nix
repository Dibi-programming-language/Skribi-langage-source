{
  description = "Building pipeline for the Skribi lang";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs?ref=nixos-unstable";
    utils.url = "github:numtide/flake-utils";

    naersk = {
      # Rust build toolchain
      url = "github:nix-community/naersk";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs =
    {
      nixpkgs,
      utils,
      naersk,
      ...
    }:
    utils.lib.eachDefaultSystem (
      system:
      let
        pkgs = nixpkgs.legacyPackages.${system};
        skribiBuild = (pkgs.callPackage naersk {}).buildPackage {
          src = ./.;
          doCheck = true;
        };
      in
      {
        packages = rec {
          skribi = skribiBuild;
          default = skribi;
        };
        devShells = pkgs.mkShell {
          inputsFrom = [ skribiBuild ];
          buildInputs = with pkgs; [
            rust-analyzer
            clippy
            rustfmt
          ];
        };
      }
    );
}
