{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-24.05";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, flake-utils }:
    flake-utils.lib.eachDefaultSystem
      (system:
        let
          pkgs = (import nixpkgs) {
            inherit system;
          };
        in
        {
          packages = rec {
            post = pkgs.callPackage ./derivation.nix {
              cargoToml = ./Cargo.toml;
            };
            default = post;
          };
        }
      ) // {
      overlays.default = _: prev: {
        post = self.packages."${prev.system}".default;
      };

      nixosModules = rec {
        post = import ./module.nix;
        default = post;
      };
    };
}
