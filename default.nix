with import <nixpkgs> {
	overlays = [ (import ./.nix/rust-overlay.nix) ];
};
let rust = rustChannels.nightly.rust; in
stdenv.mkDerivation {
	name = "game-service";
	buildInputs = [ rust pkgs.postgresql100 ];
}
