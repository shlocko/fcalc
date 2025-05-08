
{ pkgs ? import <nixpkgs> {} }:

pkgs.mkShell {
  buildInputs = with pkgs; [
    rustc
    cargo
    rustfmt
    rust-analyzer
  ];

  shellHook = ''
    echo "Rust dev shell loaded with: $(rustc --version)"

  export TMPDIR=/tmp
  '';
}
