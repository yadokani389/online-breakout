{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    flake-parts.url = "github:hercules-ci/flake-parts";
    treefmt-nix = {
      url = "github:numtide/treefmt-nix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    git-hooks = {
      url = "github:cachix/git-hooks.nix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs =
    inputs@{
      nixpkgs,
      flake-parts,
      rust-overlay,
      ...
    }:

    flake-parts.lib.mkFlake { inherit inputs; } {
      systems = [
        "x86_64-linux"
        "aarch64-linux"
      ];

      imports = with inputs; [
        git-hooks.flakeModule
        treefmt-nix.flakeModule
      ];

      perSystem =
        {
          config,
          pkgs,
          system,
          ...
        }:
        let
          toolchain = pkgs.rust-bin.stable.latest.default.override {
            targets = [ "wasm32-unknown-unknown" ];
          };
          rustPlatform = pkgs.makeRustPlatform {
            cargo = toolchain;
            rustc = toolchain;
          };

          wasm-server-runner = rustPlatform.buildRustPackage rec {
            pname = "wasm-server-runner";
            version = "1.0.0";
            src = pkgs.fetchFromGitHub {
              owner = "jakobhellermann";
              repo = pname;
              rev = "v${version}";
              sha256 = "sha256-3ARVVA+W9IS+kpV8j5lL/z6/ZImDaA+m0iEEQ2mSiTw=";
            };
            cargoHash = "sha256-FrnCmfSRAePZuWLC1/iRJ87CwLtgWRpbk6nJLyQQIT8=";
          };

          # This is an action to build to wasm
          # cc-wrapper is currently not designed with multi-target https://github.com/NixOS/nixpkgs/issues/395191
          # and clang-19 does not have include https://github.com/NixOS/nixpkgs/issues/351962
          # Someone please help me
          # -ffreestanding set __STDC_HOSTED__ to 0
          cc = "clang-19 -ffreestanding -isystem ${pkgs.libclang.lib}/lib/clang/19/include -isystem ${pkgs.glibc.dev}/include";

          cargoDeps = rustPlatform.importCargoLock {
            lockFile = ./Cargo.lock;
            outputHashes = {
              "bevy_ggrs-0.17.0" = "sha256-hLhfk7pyxEr9nqRkYg6maIIAhoUGDRXTCF7DXZTGTyc=";
              "ggrs-0.11.0" = "sha256-l24xHszLK9NrDil7LCwKlUbUMWPaBX2gYbAFb+21uoI=";
              "matchbox_protocol-0.11.0" = "sha256-diUxoSAruZ1RVJwpcyI1T9Erq68095jN0Tv340FD7+Y=";
              "bevy-wasm-tasks-0.16.0" = "sha256-8RBYwPmGiiXVkmIrV/n2UhIDEX8UzAwIUZV+PcSog5c=";
            };
          };

          commonAttrs = {
            version = "0.1.0";

            src = ./.;

            inherit cargoDeps;
          };

          online-breakout = rustPlatform.buildRustPackage (
            commonAttrs
            // {
              pname = "online-breakout";

              nativeBuildInputs = with pkgs; [
                makeWrapper
                pkg-config
              ];

              buildInputs = with pkgs; [
                zstd
                alsa-lib
                udev
                vulkan-loader
                wayland
                xorg.libX11
                xorg.libXcursor
                xorg.libXi
                xorg.libXrandr
              ];

              postFixup =
                with pkgs;
                lib.optionalString stdenv.hostPlatform.isLinux ''
                  patchelf $out/bin/online-breakout \
                    --add-rpath ${
                      lib.makeLibraryPath [
                        libxkbcommon
                        vulkan-loader
                      ]
                    }
                '';
            }
          );

          online-breakout-wasm = rustPlatform.buildRustPackage (
            commonAttrs
            // {
              pname = "online-breakout-wasm";

              buildPhase = ''
                CC='${cc}' cargo build --release --target=wasm32-unknown-unknown
              '';

              installPhase = ''
                mkdir -p $out/lib
                cp target/wasm32-unknown-unknown/release/*.wasm $out/lib/
              '';

              nativeBuildInputs = with pkgs; [
                pkg-config
                clang_19
              ];

              buildInputs = with pkgs; [
                alsa-lib
                udev
              ];
            }
          );
        in
        {
          _module.args.pkgs = import nixpkgs {
            inherit system;
            overlays = [ inputs.rust-overlay.overlays.default ];
          };

          packages = {
            inherit online-breakout online-breakout-wasm cargoDeps;
            default = online-breakout;
          };

          devShells.default = pkgs.mkShell {
            inputsFrom = [
              config.pre-commit.devShell
            ];
            inherit (online-breakout) nativeBuildInputs buildInputs;

            packages = [
              pkgs.clang_19
              pkgs.wasm-bindgen-cli
              wasm-server-runner
            ];

            LD_LIBRARY_PATH =
              with pkgs;
              lib.makeLibraryPath [
                libxkbcommon
                vulkan-loader
                udev
                alsa-lib
              ];

            shellHook = ''
              export CC='${cc}'
            '';
          };

          treefmt = {
            projectRootFile = "flake.nix";
            programs = {
              nixfmt.enable = true;
              rustfmt.enable = true;
              taplo.enable = true;
            };

            settings.formatter = {
              taplo.options = [
                "fmt"
                "-o"
                "reorder_keys=true"
              ];
            };
          };

          pre-commit = {
            check.enable = true;
            settings = {
              settings.rust.check.cargoDeps = cargoDeps;
              hooks = {
                ripsecrets.enable = true;
                typos.enable = true;
                treefmt.enable = true;
                cargo-check.enable = true;
                clippy = {
                  enable = true;
                  packageOverrides.cargo = toolchain;
                  packageOverrides.clippy = toolchain;
                  extraPackages = online-breakout.nativeBuildInputs ++ online-breakout.buildInputs;
                };
              };
            };
          };
        };
    };
}
