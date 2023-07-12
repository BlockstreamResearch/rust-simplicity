{
  pkgs ? import <nixpkgs> {}
}:
let
  cargoHfuzz = import ./honggfuzz-rs.nix { };
in
pkgs.mkShell {
  buildInputs = [
    cargoHfuzz
  ];
  shellHook = ''
    source fuzz-util.sh
  '';
}
