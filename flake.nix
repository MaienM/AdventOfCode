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
          in
          {
            devShells.default = pkgs.mkShell {
              buildInputs = with pkgs; [
                cargo-edit
                cargo-expand
                cargo-machete
                inputs'.fenix.packages.rust-analyzer
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
              ];

              shellHook = ''
                ${config.pre-commit.settings.shellHook}
              '';

              NODE_OPTIONS = "--openssl-legacy-provider";

              # See https://github.com/tikv/jemallocator/issues/108#issuecomment-2642756257 and
              # https://github.com/tikv/jemallocator/issues/108#issuecomment-3189533076.
              hardeningDisable = [ "fortify" ];
            };

            checks = {
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
                  cargo-machete = {
                    enable = true;
                    entry = "cargo machete";
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
                    inherit (eslint-custom) files;
                  };
                };
              };
            };
          };
      });
}
