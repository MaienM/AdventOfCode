{
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixpkgs-unstable";

    fenix.url = "github:nix-community/fenix";
    # fenix.inputs.nixpkgs.follows = "nixpkgs";

    flake-utils.url = "github:numtide/flake-utils";
  };
  outputs = { nixpkgs, fenix, flake-utils, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = nixpkgs.legacyPackages.${system};
        fenixPkgs = fenix.packages."${system}";
      in
      {
        apps =
          let
            platform = pkgs.makeRustPlatform rec {
              cargo = fenixPkgs.minimal.toolchain;
              rustc = cargo;
            };
            aoc = platform.buildRustPackage {
              pname = "advent-of-code";
              version = "0.0.0";
              src = ./.;
              cargoLock.lockFile = ./Cargo.lock;
            };
            mkApp = name: {
              type = "app";
              program = "${aoc}/bin/${name}";
            };
            binaries = builtins.concatMap
              (name:
                let match = builtins.match "([[:digit:]]{2}-[[:digit:]]{2})\\.rs" name;
                in if match == null then [ ] else match
              )
              (builtins.attrNames (builtins.readDir ./src/bin));
            apps = builtins.listToAttrs (
              builtins.map
              (
                name: {
                  inherit name;
                  value = mkApp name;
                }
              )
              (binaries ++ [ "aoc" ])
            );
          in
          apps // {
            default = apps.aoc;
          };

        devShell = pkgs.mkShell {
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
