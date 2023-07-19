{
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
    utils.url = "github:numtide/flake-utils";
    rust-overlay.url = "github:oxalica/rust-overlay";
  };

  outputs = { self, nixpkgs, utils, rust-overlay, ... }@inputs:
    utils.lib.eachDefaultSystem
      (system:
        let
          name = "grimace";
          pkgs = nixpkgs.legacyPackages.${system};
          allSystems = [
            "x86_64-linux" # 64-bit Intel/AMD Linux
            "aarch64-linux" # 64-bit ARM Linux
            "x86_64-darwin" # 64-bit Intel macOS
            "aarch64-darwin" # 64-bit ARM macOS
          ];

          # Helper to provide system-specific attributes
          forAllSystems = f: nixpkgs.lib.genAttrs allSystems (system: f {
            pkgs = import nixpkgs { inherit system; };
          });

          dependencies = with pkgs; [
            # for Rust
            cargo
            rustc
            rust-analyzer
            rustfmt
            cmake
            # for GTK
            cairo
            gdk-pixbuf
            atk
            gobject-introspection
            graphene
            gtk3.dev
            gtksourceview5
            libadwaita
            openssl_legacy.dev
            pkg-config
            appstream-glib
            polkit
            gettext
            desktop-file-utils
            meson
            git
            wrapGAppsHook4
          ];
        in
        rec {
          packages = forAllSystems ({ pkgs }: {
            default = pkgs.rustPlatform.buildRustPackage {
              name = "grimace";
              src = ./.;
              cargoLock = {
                lockFile = ./Cargo.lock;
              };
              # why is it like this
              nativeBuildInputs = dependencies;
              buildInputs = [
                pkgs.gdk-pixbuf
                pkgs.gtk3
                pkgs.openssl
              ];
              LIBCLANG_PATH = "${pkgs.llvmPackages.libclang.lib}/lib";
              GTK_THEME="Nordic";
              RUST_SRC_PATH = pkgs.rustPlatform.rustLibSrc;
              BINDGEN_EXTRA_CLANG_ARGS = "-isystem ${pkgs.llvmPackages.libclang.lib}/lib/clang/${pkgs.lib.getVersion pkgs.clang}/include";
            };
          });

          # `nix build`
          defaultPackage = packages.${system}.default;
        }
      );
}
