{
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixpkgs-unstable";

    fenix.url = "github:nix-community/fenix";
    # fenix.inputs.nixpkgs.follows = "nixpkgs";

    flake-utils.url = "github:numtide/flake-utils";

    pre-commit-hooks.url = "github:cachix/pre-commit-hooks.nix";
    pre-commit-hooks.inputs.nixpkgs.follows = "nixpkgs";
  };
  outputs =
    {
      self,
      nixpkgs,
      fenix,
      flake-utils,
      pre-commit-hooks,
      ...
    }:
    flake-utils.lib.eachDefaultSystem (
      system:
      let
        pkgs = nixpkgs.legacyPackages.${system};
        fenixPkgs = fenix.packages."${system}";
        rust = fenixPkgs.combine [
          (fenixPkgs.latest.withComponents [
            "cargo"
            "clippy"
            "rust-src"
            "rustc"
            "rustfmt"

            "llvm-tools-preview" # needed by cargo-llv-cov
          ])
          # WASM platform for web version.
          (fenixPkgs.targets.wasm32-unknown-unknown.latest.withComponents [
            "rust-std"
          ])
        ];
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
            binaries = builtins.concatMap (
              name:
              let
                match = builtins.match "([[:digit:]]{2}-[[:digit:]]{2})\\.rs" name;
              in
              if match == null then [ ] else match
            ) (builtins.attrNames (builtins.readDir ./src/bin));
            apps = builtins.listToAttrs (
              builtins.map (name: {
                inherit name;
                value = mkApp name;
              }) (binaries ++ [ "aoc" ])
            );
          in
          apps
          // {
            default = apps.aoc;
          };

        checks = {
          pre-commit-check =
            let
              check-jsonschema = pkgs.check-jsonschema.overrideAttrs (old: {
                propagatedBuildInputs = old.propagatedBuildInputs ++ [
                  pkgs.python3.pkgs.json5
                ];
              });
            in
            pre-commit-hooks.lib.${system}.run rec {
              src = ./.;
              hooks = {
                # Github workflows.
                github-workflows = {
                  enable = true;
                  name = "github-workflows";
                  files = "^\\.github/workflows/.*\\.yaml$";
                  entry = "${check-jsonschema}/bin/check-jsonschema --builtin-schema vendor.github-workflows";
                  pass_filenames = true;
                };

                # Nix.
                nixfmt-rfc-style.enable = true;

                # Rust.
                cargo-check = {
                  enable = true;
                  package = rust;
                };
                cargo-udeps = {
                  enable = true;
                  entry = "cargo udeps";
                  pass_filenames = false;
                  files = "Cargo\\.toml$|.*\\.rs$";
                };
                clippy = {
                  enable = true;
                  packageOverrides.cargo = rust;
                  packageOverrides.clippy = rust;
                };
                rustfmt = {
                  enable = true;
                  packageOverrides.cargo = rust;
                  packageOverrides.rustfmt = rust;
                };

                # Typescript.
                eslint-custom = {
                  enable = true;
                  name = "eslint";
                  entry = "sh -c 'cd web && ./node_modules/.bin/eslint --fix'";
                  files = "web/.*\\.(tsx?|jsx?|mjs|cjs)$";
                };
                dprint = {
                  enable = true;
                  name = "dprint";
                  entry = "sh -c 'cd web && dprint check'";
                  inherit (hooks.eslint-custom) files;
                };
              };
            };
        };

        inherit rust;

        devShell = pkgs.mkShell {
          inherit (self.checks.${system}.pre-commit-check) shellHook;
          buildInputs =
            with pkgs;
            [
              cargo-udeps
              fenixPkgs.rust-analyzer
              gnumake
              rust

              # Tests.
              cargo-llvm-cov
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
            ]
            ++ self.checks.${system}.pre-commit-check.enabledPackages;
          NODE_OPTIONS = "--openssl-legacy-provider";
        };
      }
    );
}
