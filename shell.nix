{ pkgs ? import <nixpkgs> {} }:

let
  stable = import (builtins.fetchTarball https://nixos.org/channels/nixos-21.05/nixexprs.tar.xz) {};
  unstable = import (builtins.fetchTarball https://nixos.org/channels/nixos-unstable/nixexprs.tar.xz) {};
in

stable.mkShell {
  buildInputs = with stable; [
    # required openssl-sys crate
    pkg-config
    openssl
  ] ++ (with unstable; [
    cargo
    clippy
    rust-analyzer
    rustc
    rustfmt
  ]);
}
