{ pkgs ? import (fetchTarball "https://github.com/NixOS/nixpkgs/archive/b7d8c687782c8f9a1d425a7e486eb989654f6468.tar.gz") {} }:
  pkgs.mkShell {
    nativeBuildInputs = [ 
      pkgs.gcc-arm-embedded-8
    ];
}
