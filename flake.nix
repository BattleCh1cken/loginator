{
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/master";
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay.url = "github:oxalica/rust-overlay";
  };

  outputs = { self, nixpkgs, flake-utils, rust-overlay, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs
          {
            inherit system overlays;
          };

        libraries = with pkgs; [
          pkg-config
          dbus.lib
          dbus.dev
        ];

        packages = with pkgs; [
          rust-bin.nightly.latest.default
          rust-analyzer
        ];
      in
      {
        devShell = pkgs.mkShell
          {
            buildInputs = packages;
            LD_LIBRARY_PATH = pkgs.lib.makeLibraryPath libraries;
            RUST_LOG = "trace";
          };
      });
}

