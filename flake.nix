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
          nativeBuildInputs = with pkgs; [
            llvm_22
          ];
        };
      in
      {
        packages = rec {
          skribi = skribiBuild;
          default = skribi;
        };
        devShells.default = pkgs.mkShell {
          inputsFrom = [ skribiBuild ];
          LLVM_SYS_221_PREFIX=pkgs.llvm_22.dev;
          buildInputs = with pkgs; [
            rust-analyzer
            clippy
            rustfmt
            rustc
          ];
        };
      }
    );
}
