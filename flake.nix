{
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixpkgs-unstable";

    fenix.url = "github:nix-community/fenix";
    fenix.inputs.nixpkgs.follows = "nixpkgs";

    flake-utils.url = "github:numtide/flake-utils";
  };
  outputs = { nixpkgs, fenix, flake-utils, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = nixpkgs.legacyPackages.${system};
        fenixPkgs = fenix.packages."${system}";
      in
      {
        defaultPackage = fenix.packages.x86_64-linux.minimal.toolchain;
        devShell = pkgs.mkShell {
          buildInputs = [
            (fenixPkgs.combine [
              (fenixPkgs.latest.withComponents [
                "cargo"
                "clippy"
                "rust-src"
                "rustc"
                "rustfmt"
              ])
              (fenixPkgs.targets.wasm32-unknown-unknown.latest.withComponents [
                "rust-std"
              ])
            ])
            fenixPkgs.rust-analyzer
            pkgs.cargo-nextest
            pkgs.critcmp
            pkgs.gnumake
            pkgs.gnuplot
            pkgs.wasm-pack

            pkgs.dprint
            pkgs.nodePackages.eslint_d
            pkgs.nodePackages.npm
            pkgs.nodePackages.typescript-language-server
          ];
          NODE_OPTIONS = "--openssl-legacy-provider";
        };
      });
}
