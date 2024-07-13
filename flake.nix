{
  inputs = {
    nixpkgs.url = "github:NixOs/nixpkgs";
    nci.url = "github:yusdacra/nix-cargo-integration";
    parts.url = "github:hercules-ci/flake-parts";
    systems.url = "github:nix-systems/default";
  };
  outputs = {
    parts,
    systems,
    nci,
    ...
  } @ inputs:
    parts.lib.mkFlake {inherit inputs;} {
      systems = import systems;
      imports = [nci.flakeModule];
      perSystem = {config, ...}: let
        inherit (config.nci.outputs.dawnstorm-textfront) packages devShell;
      in {
        nci.projects.default.path = ./.;
        packages.default = packages.release;
        devShells.default = devShell;
      };
    };
}
