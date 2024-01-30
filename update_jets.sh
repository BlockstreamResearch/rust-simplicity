#!/bin/bash
set -e

# Requires (nix-shell with) cabal

if [ -z "$1" ]; then
  echo "\$1 path to libsimplicity repository root"
  exit 1
fi

C_DIR="$1"
RUST_DIR="$(pwd)"
VENDORED_HEAD=$(sed -n '2p' "$RUST_DIR"/simplicity-sys/depend/simplicity-HEAD-revision.txt)

cd "$C_DIR"
REV=$(git rev-parse HEAD)

if [ "$REV" != "$VENDORED_HEAD" ]; then
    echo "WARNING: Haskell and Rust have different libsimplicity version"
    echo "Haskell: $REV"
    echo "Rust:    $VENDORED_HEAD"
fi

if test -n "$(git status --porcelain)"; then
    echo "WARNING: libsimplicity repo is not clean"
fi

cabal build -j8
cabal exec GenRustJets

cd "$RUST_DIR"
mv "${C_DIR}/jets_ffi.rs" "./simplicity-sys/src/c_jets/jets_ffi.rs"
mv "${C_DIR}/jets_wrapper.rs" "./simplicity-sys/src/c_jets/jets_wrapper.rs"
mv "${C_DIR}/jets_wrapper.c" "./simplicity-sys/depend/jets_wrapper.c"

mv "${C_DIR}/core.rs" "./src/jet/init/"
mv "${C_DIR}/bitcoin.rs" "./src/jet/init/"
mv "${C_DIR}/elements.rs" "./src/jet/init/"
