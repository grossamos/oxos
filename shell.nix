let
  moz_overlay = import (builtins.fetchTarball https://github.com/mozilla/nixpkgs-mozilla/archive/master.tar.gz);
  nixpkgs = import <nixpkgs> { overlays = [ moz_overlay ]; };
in
  with nixpkgs;
  stdenv.mkDerivation {
    name = "moz_overlay_shell";
    buildInputs = [
      ((nixpkgs.rustChannelOf { date = "2022-10-28"; channel = "nightly"; }).rust.override (old:
        { 
          extensions = ["rust-src" "rust-analysis"]; 
          targets = [ "aarch64-unknown-linux-gnu" ];
        }
      ))
      gdb
      rustup
    ];
    nativeBuildInputs = with pkgs.buildPackages; [ 
      libmpc
      gmp
      mpfr
    ];
    hardeningDisable = [ "all" ];
    shellHook = ''
      export PATH="$PWD/bin/gcc/out/bin:$PATH"
    '';
  }

