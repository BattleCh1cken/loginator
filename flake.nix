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

        packages = with pkgs; [
          dbus.lib
          dbus.dev

          pkg-config
          rust-bin.nightly.latest.default
          rust-analyzer
        ];
      in
      {
        devShell = pkgs.mkShell
          {
            buildInputs = packages;
            RUST_LOG = "trace";
          };
      });
}

