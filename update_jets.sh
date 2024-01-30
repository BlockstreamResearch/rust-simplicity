#!/bin/bash
set -e


if [ -z "$1" ]; then
  echo "\$1 parameter must be the simplicity directory"
  echo "\$2 parameter (optional) can be the revision to check out. If this is not specified, we
  use the simplicity revision specified in the simplicity-sys/depend/simplicity-HEAD-revision.txt.
  If this is specified the revision must be available in the simplicity directory."
  exit 1
fi

C_DIR=$1
REV=$2
RUST_DIR=$(pwd)
VENDORED_HEAD=$(sed -n '2p' "$RUST_DIR"/simplicity-sys/depend/simplicity-HEAD-revision.txt)

cd "$C_DIR" || exit 1
if [ -n "$REV" ]; then
    if [ "$REV" != "$VENDORED_HEAD" ]; then
        echo "WARNING: Simplicity GenRustJets and simplicity depend do not have the same git rev"
        echo "$REV"
        echo "$VENDORED_HEAD"
    fi
    git checkout "$REV"
else
    git checkout "$VENDORED_HEAD"
fi

cd "$C_DIR"
cabal build -j8
cabal exec GenRustJets

cd "$RUST_DIR"
mv "${C_DIR}/jets_ffi.rs" "./simplicity-sys/src/c_jets/jets_ffi.rs"
mv "${C_DIR}/jets_wrapper.rs" "./simplicity-sys/src/c_jets/jets_wrapper.rs"

mv "${C_DIR}/core.rs" "./src/jet/init/"
mv "${C_DIR}/bitcoin.rs" "./src/jet/init/"
mv "${C_DIR}/elements.rs" "./src/jet/init/"
