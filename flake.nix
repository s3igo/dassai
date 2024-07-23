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
  };

  outputs =
    {
      self,
      nixpkgs,
      flake-utils,
      fenix,
      crane,
      neovim,
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
      in
      {
        packages = {
          neovim = neovim.withModules {
            inherit system pkgs;
            modules = with neovim.modules; [
              im-select
              nix
              rust
              { plugins.lsp.servers.taplo.enable = true; }
            ];
          };
          dassai = craneLib.buildPackage (commonArgs // { inherit cargoArtifacts; });
          default = self.packages.${system}.dassai;
        };

        devShells.default = pkgs.mkShell {
          inherit buildInputs;
          packages = [
            toolchain
            fenix'.default.rustfmt # rustfmt nightly
            self.packages.${system}.neovim
          ];
        };
      }
    );
}
