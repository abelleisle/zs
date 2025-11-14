{
  description = "Rust project with treefmt and pre-commit hooks";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";

    # Rust toolchain
    fenix = {
      url = "github:nix-community/fenix";
      inputs.nixpkgs.follows = "nixpkgs";
    };

    # Formatting
    treefmt-nix = {
      url = "github:numtide/treefmt-nix";
      inputs.nixpkgs.follows = "nixpkgs";
    };

    # Pre-commit hooks
    pre-commit-hooks = {
      url = "github:cachix/pre-commit-hooks.nix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = {
    self,
    nixpkgs,
    flake-utils,
    fenix,
    treefmt-nix,
    pre-commit-hooks,
  }:
    flake-utils.lib.eachDefaultSystem (system: let
      # pkgs = nixpkgs.legacyPackages.${system};
      pkgs = import nixpkgs {
        inherit system;
        overlays = [fenix.overlays.default];
        rustPlatform = nixpkgs.makeRustPlatform {
          cargo = rustToolchain;
          rust = rustToolchain;
        };
      };

      # Create Rust toolchain from fenix based on rust-toolchain.toml
      rustToolchain = fenix.packages.${system}.fromToolchainFile {
        file = ./rust-toolchain.toml;
        sha256 = "sha256-x+EWymRPcdfpK3I1N+Rr3RE0ld/KmNPEJGDnyxFyByE=";
      };

      # treefmt configuration
      treefmtEval = treefmt-nix.lib.evalModule pkgs {
        projectRootFile = "flake.nix";
        programs = {
          alejandra.enable = true;
          deadnix.enable = true;
          rustfmt.enable = false;
        };
      };

      # Pre-commit hooks configuration
      pre-commit-check = pre-commit-hooks.lib.${system}.run {
        src = ./.;
        hooks = {
          # Run treefmt for all formatting
          treefmt = {
            enable = true;
            package = treefmtEval.config.build.wrapper;
            entry = "${treefmtEval.config.build.wrapper}/bin/treefmt --fail-on-change";
          };

          # Run rustfmt for Rust code formatting
          rustfmt = {
            enable = true;
            entry = "${rustToolchain}/bin/cargo-fmt fmt";
          };

          # Run clippy for linting
          clippy = {
            enable = true;
            entry = "${rustToolchain}/bin/cargo-clippy clippy --all-targets --all-features -- -D warnings";
          };
        };
      };
    in {
      # Formatter for `nix fmt`
      formatter = treefmtEval.config.build.wrapper;

      # Development shell
      devShells.default = pkgs.mkShell {
        inherit (pre-commit-check) shellHook;

        buildInputs = with pkgs; [
          # Rust toolchain from fenix
          rustToolchain
          rust-analyzer
          rustPlatform.bindgenHook

          # Formatting tools
          treefmtEval.config.build.wrapper
          alejandra
          deadnix

          # Additional development tools
          cargo-watch
          rust-analyzer
        ];

        # Environment variables
        RUST_SRC_PATH = "${rustToolchain}/lib/rustlib/src/rust/library";
      };

      # Checks for `nix flake check`
      checks = {
        pre-commit = pre-commit-check;
        formatting = treefmtEval.config.build.check self;
      };
    });
}
