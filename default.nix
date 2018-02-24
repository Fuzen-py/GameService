let
  moz_overlay = import (builtins.fetchTarball https://github.com/mozilla/nixpkgs-mozilla/archive/master.tar.gz);
  nixpkgs = import <nixpkgs> { overlays = [ moz_overlay ]; };
in
  with nixpkgs;
  stdenv.mkDerivation {
    name = "game-service";
    buildInputs = [
      nixpkgs.latest.rustChannels.nightly.rust
      pkgs.postgresql100
      ];
  }