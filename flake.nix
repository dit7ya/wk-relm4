# adapted from https://gitlab.com/scvalex/sixty-two/-/blob/flake-blogpost/flake.nix
{
  inputs = {
    naersk.url = "github:nmattia/naersk/master";
    # This must be the stable nixpkgs if you're running the app on a
    # stable NixOS install.  Mixing EGL library versions doesn't work.
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    utils.url = "github:numtide/flake-utils";
  };

  outputs = {
    self,
    nixpkgs,
    utils,
    naersk,
    ...
  }:
    utils.lib.eachDefaultSystem (system: let
      pkgs = import nixpkgs {inherit system;};
      naersk-lib = pkgs.callPackage naersk {};
      # libPath = with pkgs;
      #   lib.makeLibraryPath [
      #     libGL
      #     libxkbcommon
      #     wayland
      #     xorg.libX11
      #     xorg.libXcursor
      #     xorg.libXi
      #     xorg.libXrandr
      #   ];
    in {
      devShell = with pkgs;
        mkShell {
          buildInputs = [
            cairo
            gdk-pixbuf
            gobject-introspection
            graphene
            gtk4
            gtksourceview5
            libadwaita
            openssl
            pandoc
            pango
            pkgconfig
            wrapGAppsHook

            pkg-config
            openssl
            glib
            cairo
            pango
            atk
            gdk-pixbuf
            libsoup
            gtk3
            libappindicator
            webkitgtk

            cargo-edit
            clippy
          ];
          RUST_SRC_PATH = rustPlatform.rustLibSrc;
          # LD_LIBRARY_PATH = libPath;
        };
    });
}
