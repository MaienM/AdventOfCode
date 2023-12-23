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
        devShell = pkgs.mkShell rec {
          buildInputs = with pkgs; [
            # Core.
            (fenixPkgs.combine [
              (fenixPkgs.latest.withComponents [
                "cargo"
                "clippy"
                "rust-src"
                "rustc"
                "rustfmt"
              ])
              # WASM platform for web version.
              (fenixPkgs.targets.wasm32-unknown-unknown.latest.withComponents [
                "rust-std"
              ])
            ])
            fenixPkgs.rust-analyzer
            gnumake

            # Tests.
            cargo-nextest

            # Benchmarks.
            critcmp
            gnuplot

            # Web version.
            wasm-pack
            dprint
            nodePackages.eslint_d
            nodePackages.npm
            nodePackages.typescript-language-server

            cmake
            pkg-config
            fontconfig
          ];
          NODE_OPTIONS = "--openssl-legacy-provider";
        };
      });
}
