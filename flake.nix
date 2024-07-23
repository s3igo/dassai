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
    neovim.url = "github:s3igo/dotfiles?dir=neovim";
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
      neovim,
      advisory-db,
    }:
    flake-utils.lib.eachDefaultSystem (
      system:
      let
        pkgs = import nixpkgs { inherit system; };
        fenix' = fenix.packages.${system};
        toolchain = fenix'.fromToolchainFile {
          file = ./rust-toolchain.toml;
          sha256 = "sha256-Ngiz76YP4HTY75GGdH2P+APE/DEIx2R/Dn+BwwOyzZU=";
        };
        craneLib = (crane.mkLib pkgs).overrideToolchain toolchain;
        src = craneLib.cleanCargoSource ./.;
        buildInputs = with pkgs; lib.optionals stdenv.isDarwin [ libiconv ];
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
        inherit (fenix'.default) rustfmt; # rustfmt nightly
      in
      {
        checks = {
          inherit dassai;
          dassai-clippy = craneLib.cargoClippy (commonArgs // { inherit cargoArtifacts; });
          dassai-fmt = craneLib.cargoFmt {
            inherit src;
            buildInputs = [ rustfmt ];
          };
          dassai-audit = craneLib.cargoAudit { inherit src advisory-db; };
          dassai-nextest = craneLib.cargoNextest (commonArgs // { inherit cargoArtifacts; });
        };

        packages = {
          neovim = neovim.withModules {
            inherit system pkgs;
            modules = with neovim.modules; [
              im-select
              markdown
              nix
              rust
              { plugins.lsp.servers.taplo.enable = true; }
            ];
          };
          inherit dassai;
          default = dassai;
        };

        devShells.default = pkgs.mkShell {
          inherit buildInputs;
          packages = [
            toolchain
            rustfmt
            self.packages.${system}.neovim
          ];
        };
      }
    );
}
