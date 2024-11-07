{
  description = "CLI tool for converting source code into Markdown code blocks";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixpkgs-unstable";
    systems.url = "github:nix-systems/default";
    flake-parts = {
      url = "github:hercules-ci/flake-parts";
      inputs.nixpkgs-lib.follows = "nixpkgs";
    };
    fenix = {
      url = "github:nix-community/fenix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    crane.url = "github:ipetkov/crane";
    nixvim = {
      url = "github:nix-community/nixvim";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    neovim-config.url = "github:s3igo/dotfiles?dir=neovim-config";
    advisory-db = {
      url = "github:rustsec/advisory-db";
      flake = false;
    };
  };

  outputs =
    inputs:
    inputs.flake-parts.lib.mkFlake { inherit inputs; } {
      imports = [ inputs.flake-parts.flakeModules.easyOverlay ];

      systems = import inputs.systems;

      perSystem =
        {
          config,
          pkgs,
          inputs',
          ...
        }:

        let
          toolchain =
            with inputs'.fenix.packages;
            combine [
              (fromToolchainFile {
                file = ./rust-toolchain.toml;
                sha256 = "sha256-yMuSb5eQPO/bHv+Bcf/US8LVMbf/G/0MSfiPwBhiPpk=";
              })
              default.rustfmt # rustfmt nightly
            ];
          craneLib = (inputs.crane.mkLib pkgs).overrideToolchain toolchain;
          src = craneLib.cleanCargoSource ./.;
          buildInputs = with pkgs; lib.optional stdenv.isDarwin libiconv;
          commonArgs = {
            inherit src buildInputs;
            strictDeps = true;
          };
          cargoArtifacts = craneLib.buildDepsOnly commonArgs;
        in

        {
          packages = {
            neovim = inputs'.nixvim.legacyPackages.makeNixvim {
              imports = with inputs.neovim-config.nixosModules; [
                default
                nix
                markdown
                rust
              ];
            };
            dassai = craneLib.buildPackage (
              commonArgs
              // {
                inherit cargoArtifacts;
                doCheck = false;
              }
            );
            default = config.packages.dassai;
          };

          checks = {
            dassai-build = config.packages.dassai;
            dassai-clippy = craneLib.cargoClippy (commonArgs // { inherit cargoArtifacts; });
            dassai-fmt = craneLib.cargoFmt { inherit src; };
            dassai-audit = craneLib.cargoAudit {
              inherit src;
              inherit (inputs) advisory-db;
            };
            dassai-nextest = craneLib.cargoNextest (commonArgs // { inherit cargoArtifacts; });
          };

          devShells.default = pkgs.mkShellNoCC {
            inherit buildInputs;
            packages = [
              toolchain
              pkgs.cargo-nextest
              (inputs.neovim-config.lib.customName {
                inherit pkgs;
                nvim = config.packages.neovim;
              })
            ];
            shellHook = ''
              export RUST_BACKTRACE=1
            '';
          };

          overlayAttrs = {
            inherit (config.packages) dassai;
          };
        };
    };
}
