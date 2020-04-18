with import <nixpkgs> {};
stdenv.mkDerivation {
  name = "rust-env";
  buildInputs = [
    # this includes, cargo, rustc, rls, rustfmt
#    (rustChannelOf (parseRustToolchain ./rust-toolchain)).rust
#    cargo-tree
#    cargo-audit
#    cargo-clippy
    rustup
    cargo-outdated
  ];

  nativeBuildInputs = with pkgs; [
  #  llvmPackages.libclang
  #  llvmPackages.clang
  ]; 
  # Set Environment Variables
  RUST_BACKTRACE = 1;
}

