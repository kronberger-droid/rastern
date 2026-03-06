{
  description = "Rastern – STM/AFM image analysis tool";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    fenix.url = "github:nix-community/fenix";
    rust-overlay.url = "github:oxalica/rust-overlay";
  };

  outputs = {
    self,
    nixpkgs,
    flake-utils,
    fenix,
    rust-overlay,
    ...
  }:
    flake-utils.lib.eachDefaultSystem (
      system: let
        pkgs = import nixpkgs {
          inherit system;
          overlays = [fenix.overlays.default rust-overlay.overlays.default];
        };

        rustTools = {
          stable = pkgs.rust-bin.stable."1.89.0".default.override {
            extensions = ["rust-src"];
          };
          analyzer = pkgs.rust-bin.stable."1.89.0".rust-analyzer;
        };

        devTools = with pkgs; [
          cargo-expand
          pkg-config
          gcc
        ];
      in {
        packages = {
          rastern = pkgs.rustPlatform.buildRustPackage {
            pname = "rastern";
            version = "0.1.0";
            src = ./.;
            cargoLock.lockFile = ./Cargo.lock;

            nativeBuildInputs = with pkgs; [pkg-config];

            doCheck = false;
          };

          default = self.packages.${system}.rastern;
        };

        devShells.default = pkgs.mkShell {
          name = "rastern-dev";
          buildInputs =
            [
              rustTools.stable
              rustTools.analyzer
            ]
            ++ devTools;

          shellHook = ''
            echo "Using Rust toolchain: $(rustc --version)"
            export CARGO_HOME="$HOME/.cargo"
            export RUSTUP_HOME="$HOME/.rustup"
            mkdir -p "$CARGO_HOME" "$RUSTUP_HOME"
          '';
        };
      }
    );
}
