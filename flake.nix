{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    flake-parts.url = "github:hercules-ci/flake-parts";
  };

  outputs =
    inputs@{
      nixpkgs,
      rust-overlay,
      flake-parts,
      self,
      ...
    }:
    flake-parts.lib.mkFlake { inherit inputs; } {
      systems = [
        "x86_64-linux"
        "aarch64-linux"
        "x86_64-darwin"
        "aarch64-darwin"
      ];

      flake = {
        nixosModules.default = import ./nix/module.nix {
          tableaux-playground = self.packages.x86_64-linux.playground;
        };
      };

      perSystem =
        { system, ... }:
        let
          toolchain = pkgs.rust-bin.beta.latest.default.override {
            targets = [ "wasm32-unknown-unknown" ];
          };
          overlays = [ (import rust-overlay) ];
          pkgs = import nixpkgs { inherit system overlays; };
        in
        {
          formatter = pkgs.nixfmt-rfc-style;
          packages = rec {
            wasm = pkgs.callPackage ./nix/tableaux-wasm.nix { };
            playground = pkgs.callPackage ./nix/tableaux-playground.nix { tableaux = wasm; };
          };
          devShells.default = pkgs.mkShell {
            packages = [
              toolchain
              pkgs.rust-analyzer
              pkgs.cargo-nextest
              pkgs.simple-completion-language-server

              pkgs.pnpm
              pkgs.nodejs_22
              pkgs.wasm-pack
            ];

            EXAMPLES_GRAHAM_PRIEST_PATH = ./examples-graham-priest.toml;
          };
        };
    };
}
