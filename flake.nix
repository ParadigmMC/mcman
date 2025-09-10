{
  description = "Powerful Minecraft Server Manager CLI";
  inputs = {
    nixpkgs.url = "nixpkgs/nixos-unstable";
    crane.url = "github:ipetkov/crane";
  };

  outputs = {
    nixpkgs,
    crane,
    ...
  }: {
    packages = nixpkgs.lib.genAttrs nixpkgs.legacyPackages.x86_64-linux.rustc.meta.platforms (system: let
      pkgs = nixpkgs.legacyPackages.${system};
      craneLib = crane.mkLib pkgs;

      src = pkgs.lib.cleanSourceWith {
        src = ./.;
        filter = path: type: (craneLib.filterCargoSources path type) || (builtins.match ".*res/.*" path != null);
        name = "mcman";
      };
      common = {
        inherit src;
        strictDeps = true;
        doCheck = false;
      };
      cargoArtifacts =
        craneLib.buildDepsOnly common;
    in rec {
      mcman = craneLib.buildPackage (common // {inherit cargoArtifacts;});
      default = mcman;
    });
  };
}
