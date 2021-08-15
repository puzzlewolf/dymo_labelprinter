let
  oxalica_overlay = import (builtins.fetchTarball
    "https://github.com/oxalica/rust-overlay/archive/master.tar.gz");
  nixpkgs = import <nixpkgs> {
    overlays =
      [ oxalica_overlay (import ~/.config/nixpkgs/overlays/overlays.nix) ];
  };
  #rust_channel = nixpkgs.rust-bin.fromRustupToolchainFile ./rust-toolchain;
  rust_channel = nixpkgs.rust-bin.stable."1.52.0".default;

  #moz_overlay = import (builtins.fetchTarball
  #  "https://github.com/mozilla/nixpkgs-mozilla/archive/master.tar.gz");
  #nixpkgs = import <nixpkgs> { overlays = [ moz_overlay ]; };
  #rust_channel = (nixpkgs.rustChannelOf { rustToolchain = ./rust-toolchain; }).rust;
in with nixpkgs;
pkgs.mkShell {
  buildInputs = [

    #    cargo-audit
    cargo-outdated
  ];

  nativeBuildInputs = with pkgs;
    [
      (rust_channel.override {
        extensions = [ "rust-src" "rust-std" "clippy-preview" ];
      })
      #  llvmPackages.libclang
      #  llvmPackages.clang
    ];
  # Set Environment Variables
  RUST_BACKTRACE = 1;
  IM_CONVERT = "${pkgs.imagemagick}/bin/convert";
}

