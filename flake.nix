{
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixpkgs-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    fenix = {
      url = "github:nix-community/fenix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    crane = {
      url = "github:ipetkov/crane";
      inputs.nixpkgs.follows = "nixpkgs";
    };
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
    {
      self,
      nixpkgs,
      flake-utils,
      fenix,
      crane,
      nixvim,
      neovim-config,
      advisory-db,
    }:
    flake-utils.lib.eachDefaultSystem (
      system:
      let
        pkgs = import nixpkgs { inherit system; };
        toolchain =
          with fenix.packages.${system};
          combine [
            (fromToolchainFile {
              file = ./rust-toolchain.toml;
              sha256 = "sha256-yMuSb5eQPO/bHv+Bcf/US8LVMbf/G/0MSfiPwBhiPpk=";
            })
            default.rustfmt # rustfmt nightly
          ];
        craneLib = (crane.mkLib pkgs).overrideToolchain toolchain;
        src = craneLib.cleanCargoSource ./.;
        buildInputs = with pkgs; lib.optional stdenv.isDarwin libiconv;
        commonArgs = {
          inherit src buildInputs;
          strictDeps = true;
        };
        cargoArtifacts = craneLib.buildDepsOnly commonArgs;
        dassai = craneLib.buildPackage (
          commonArgs
          // {
            inherit cargoArtifacts;
            doCheck = false;
          }
        );
      in
      {
        checks = {
          inherit dassai;
          dassai-clippy = craneLib.cargoClippy (commonArgs // { inherit cargoArtifacts; });
          dassai-fmt = craneLib.cargoFmt { inherit src; };
          dassai-audit = craneLib.cargoAudit { inherit src advisory-db; };
          dassai-nextest = craneLib.cargoNextest (commonArgs // { inherit cargoArtifacts; });
        };

        packages = {
          neovim = nixvim.legacyPackages.${system}.makeNixvim {
            imports = with neovim-config.nixosModules; [
              default
              nix
              markdown
              rust
            ];
          };
          inherit dassai;
          default = dassai;
        };

        devShells.default = pkgs.mkShell {
          inherit buildInputs;
          packages = [
            pkgs.cargo-nextest
            toolchain
            (neovim-config.lib.customName {
              inherit pkgs;
              nvim = self.packages.${system}.neovim;
            })
          ];
        };
      }
    );
}
