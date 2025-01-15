{
  description = "Flake to manage node builds";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs?ref=nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs = {
        nixpkgs.follows = "nixpkgs";
        flake-utils.follows = "flake-utils";
      };
    };
  };

  outputs = {
    self,
    nixpkgs,
    flake-utils,
    rust-overlay,
  }:
    flake-utils.lib.eachDefaultSystem
    (
      system: let
        overlays = [(import rust-overlay)];
        pkgs = import nixpkgs {
          inherit system overlays;
        };
        rust-bin = pkgs.rust-bin.fromRustupToolchainFile ./rust-toolchain.toml;
        rustPlatform = pkgs.makeRustPlatform {
          cargo = rust-bin;
          rustc = rust-bin;
        };
      in {
        devShells.default = pkgs.mkShell {
          buildInputs = with pkgs; [
            rust-bin
            emscripten
          ];
          shellHook = ''
            export EM_CACHE=/tmp/emcache
          '';
        };
        packages = rec {
          godot-flappy-linux = rustPlatform.buildRustPackage {
            pname = "godot-neural-networks";
            version = "0.1.0";
            src = ./rust/godot-neural-networks;
            cargoLock = {
              lockFile = ./rust/godot-neural-networks/Cargo.lock;
              outputHashes = {
                "gdextension-api-0.2.1" = "sha256-YkMbzObJGnmQa1XGT4ApRrfqAeOz7CktJrhYks8z0RY=";
                "godot-0.2.2" = "sha256-k5gW74nRp8AUef2nJ5Fwcr2Vf5PVJqpEgrsacG6mp1g=";
              };
            };
            nativeBuildInputs = [
              rust-bin
            ];
          };
          default = godot-flappy-linux;
        };
      }
    );
}
