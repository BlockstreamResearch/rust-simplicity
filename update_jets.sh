#!/bin/bash
set -e


if [ -z "$1" ]; then
  echo "\$1 parameter must be the simplicity directory"
  echo "\$2 parameter (optional) can be the revision to check out"
  exit 1
fi

SRC_DIR=$1
REV=$2
ORIGDIR=$(pwd)

cd "$SRC_DIR" || exit 1
if [ -n "$REV" ]; then
    git checkout "$REV"
fi
HEAD1=$(git rev-parse HEAD)
cd $ORIGDIR
HEAD2=$(sed -n '2p' simplicity-sys/depend/simplicity-HEAD-revision.txt)


if [ $HEAD1 != $HEAD2 ]; then
    echo "WARNING: Simplicity GenRustJets and simplicity depend must have the same git rev"
    echo $HEAD1
    echo $HEAD2
    # exit 1. UNCOMMENT THIS BEFORE MERGING. THIS IS ONLY COMMENTED FOR TESTING
    # AS TILL THE PR THAT IS BEING TESTED IS NOT MERGED.
fi

cd $SRC_DIR
cabal build -j8
cabal exec GenRustJets

cd $ORIGDIR
# TODO: Change the name on Haskell side
mv "${SRC_DIR}/jets_ffi.rs" "./simplicity-sys/src/c_jets/jets_ffi.rs"
mv "${SRC_DIR}/jets_wrapper.rs" "./simplicity-sys/src/c_jets/jets_wrapper.rs"

mv "${SRC_DIR}/core.rs" "./src/jet/init/"
mv "${SRC_DIR}/bitcoin.rs" "./src/jet/init/"
mv "${SRC_DIR}/elements.rs" "./src/jet/init/"
