{pkgs ? import <nixpkgs> {}}:
pkgs.mkShell {
  name = "mercado-sdk-rust";
  buildInputs = [
    pkgs.sccache
    pkgs.openssl.dev
    pkgs.pkg-config
    pkgs.rustup
    pkgs.rust-analyzer
    pkgs.lazygit
  ];
}

