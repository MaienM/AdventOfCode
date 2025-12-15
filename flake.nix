{
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixpkgs-unstable";

    fenix.url = "github:nix-community/fenix";

    flake-parts.url = "github:hercules-ci/flake-parts";

    git-hooks.url = "github:cachix/git-hooks.nix";
    git-hooks.inputs.nixpkgs.follows = "nixpkgs";
  };
  outputs =
    { flake-parts, ... }@inputs:
    flake-parts.lib.mkFlake
      {
        inherit inputs;
      }
      (_: {
        imports = [
          inputs.git-hooks.flakeModule
        ];
        systems = [
          "x86_64-linux"
          "aarch64-darwin"
        ];
        perSystem =
          {
            config,
            inputs',
            pkgs,
            system,
            ...
          }:
          let
            toolchain = inputs'.fenix.packages.fromToolchainName {
              name = "nightly-2025-09-28";
              sha256 = "sha256-F+nlO3ckY2zt5fqeBrKgO7gJ+8t5SUL8Jdu31kM6Q9k=";
            };
            rust = inputs'.fenix.packages.combine [
              (toolchain.withComponents [
                "cargo"
                "clippy"
                "rust-src"
                "rustc"
                "rustfmt"

                "llvm-tools-preview" # needed by cargo-llv-cov
              ])
              # WASM platform for web version.
              (inputs'.fenix.packages.targets.wasm32-unknown-unknown.latest.withComponents [
                "rust-std"
              ])
            ];
            pprofme =
              let
                artifacts = {
                  x86_64-linux = pkgs.fetchurl {
                    url = "https://github.com/polarsignals/pprofme/releases/download/v0.1.0/pprofme_Linux_x86_64";
                    hash = "sha256-tuS3DKJcPM3j1O1Fl1nAOSSpZOr2UdA0Of/fxPkG6nc=";
                  };
                  aarch64-darwin = pkgs.fetchurl {
                    url = "https://github.com/polarsignals/pprofme/releases/download/v0.1.0/pprofme_Darwin_arm64";
                    hash = "sha256-jeZtc1/6w93WbrKdCjXWEi71H/ec1PgzmDZpkGtVglI=";
                  };
                };
              in
              pkgs.runCommand "pprofme" { } ''
                mkdir -p $out/bin
                cp ${artifacts.${system}} $out/bin/pprofme
                chmod +x $out/bin/pprofme
              '';
          in
          {
            devShells.default = pkgs.mkShell {
              buildInputs = with pkgs; [
                cargo-edit
                cargo-expand
                cargo-machete
                inputs'.fenix.packages.rust-analyzer
                rust

                # Building.
                gnumake
                expect # unbuffer

                # Tests.
                cargo-llvm-cov
                cargo-nextest

                # Benchmarks.
                critcmp
                gnuplot
                pprofme

                # Web version.
                wasm-pack
                dprint
                nodePackages.eslint_d
                nodePackages.npm
                nodePackages.typescript-language-server
              ];

              shellHook = ''
                ${config.pre-commit.settings.shellHook}

                export AOC_SESSION_COOKIE_FILE="$PWD/.aoc-session"
              '';

              NODE_OPTIONS = "--openssl-legacy-provider";

              # See https://github.com/tikv/jemallocator/issues/108#issuecomment-2642756257 and
              # https://github.com/tikv/jemallocator/issues/108#issuecomment-3189533076.
              hardeningDisable = [ "fortify" ];
            };

            pre-commit.settings = {
              src = ./.;
              hooks = rec {
                # Github workflows.
                github-workflows = {
                  enable = true;
                  name = "github-workflows";
                  files = "^\\.github/workflows/.*\\.yaml$";
                  entry =
                    let
                      check-jsonschema = pkgs.check-jsonschema.overrideAttrs (old: {
                        propagatedBuildInputs = old.propagatedBuildInputs ++ [
                          pkgs.python3.pkgs.json5
                        ];
                      });
                    in
                    "${check-jsonschema}/bin/check-jsonschema --builtin-schema vendor.github-workflows";
                  pass_filenames = true;
                };

                # Nix.
                nixfmt-rfc-style.enable = true;

                # Rust.
                cargo-check = {
                  enable = true;
                  package = rust;
                };
                cargo-docs = {
                  enable = true;
                  entry = "make docs";
                  pass_filenames = false;
                };
                cargo-machete = {
                  enable = true;
                  entry = "cargo machete";
                  pass_filenames = false;
                  args = [ "--with-metadata" ];
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
                  inherit (eslint-custom) files;
                };
              };
            };
          };
      });
}
