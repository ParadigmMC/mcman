{
  description = "Powerful Minecraft Server Manager CLI";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-23.11";
    flake-utils.url = "github:numtide/flake-utils";
    gitignore = { url = "github:hercules-ci/gitignore.nix"; flake = false; };
  };

  outputs = { self, nixpkgs, flake-utils, ... }:
  flake-utils.lib.eachDefaultSystem (system:
    let
      pkgs = nixpkgs.legacyPackages.${system};
      cargoToml = builtins.fromTOML (builtins.readFile ./Cargo.toml);
    in rec {
      legacyPackages.mcman = pkgs.rustPlatform.buildRustPackage {
        inherit (cargoToml.package) name version;
        src = ./.;
        cargoLock.lockFile = ./Cargo.lock;
        cargoLock.outputHashes = {
          "mcapi-0.2.0" = "sha256-wHXA+4DQVQpfSCfJLFuc9kUSwyqo6T4o0PypYdhpp5s=";
          "pathdiff-0.2.1" = "sha256-+X1afTOLIVk1AOopQwnjdobKw6P8BXEXkdZOieHW5Os=";
          "rpackwiz-0.1.0" = "sha256-pOotNPIZS/BXiJWZVECXzP1lkb/o9J1tu6G2OqyEnI8=";
        };
      };
      defaultPackage = legacyPackages.mcman;
    }
  );
}
