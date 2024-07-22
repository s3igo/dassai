{
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixpkgs-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    fenix = {
      url = "github:nix-community/fenix";
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
      in
      {
        packages.neovim = neovim.withModules {
          inherit system pkgs;
          modules = with neovim.modules; [
            nix
            rust
          ];
        };

        devShells.default = pkgs.mkShell {
          packages = [
            toolchain
            fenix'.default.rustfmt # rustfmt nightly
            self.packages.${system}.neovim
          ];
        };
      }
    );
}
