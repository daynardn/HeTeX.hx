# These are deps for libtexprintf

{
  pkgs ? import <nixpkgs> { },
}:

pkgs.mkShell {
  nativeBuildInputs = [
    pkgs.autoconf
    pkgs.automake
    pkgs.libtool
    pkgs.libclang
  ];
  LIBCLANG_PATH = "${pkgs.libclang.lib}/lib";
}
