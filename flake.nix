{
  description = "Building pipeline for the Skribi lang";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs?ref=nixos-unstable";
  };

  outputs = { self, nixpkgs }:
  let
    pkgs = nixpkgs.legacyPackages.x86_64-linux;
  in
  {
    packages.x86_64-linux.skribi = pkgs.rustPlatform.buildRustPackage {
      pname = "skribi";
      version = "0.1";
      src = ./.;

      doCheck = true;
      cargoLock = {
          lockFile = ./Cargo.lock;
      };
    };

    packages.x86_64-linux.default = self.packages.x86_64-linux.skribi;
  };
}
