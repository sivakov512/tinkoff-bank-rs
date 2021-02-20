{ pkgs ? import <nixpkgs> {} }:

pkgs.mkShell {
  buildInputs = with pkgs; [
    # needed to compile openssl-sys crate
    pkg-config
    openssl
    cargo
    clippy
    rust-analyzer
    rustc
    rustfmt
  ];
}
