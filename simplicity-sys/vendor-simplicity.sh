#!/usr/bin/env bash
set -e

# Requires (nix-shell with) git, rsync

## 0. Parse command-line options
if [ -z "$1" ] || [ -z "$2" ]; then
  echo "Usage: $0 <path to depend directory> <path to libsimplicity repository root>"
  exit 1
fi

DEPEND_PATH=$(readlink -f "$1")
LIBSIM_PATH=$(readlink -f "$2")
VENDORED_SIM_DIR="$DEPEND_PATH/simplicity"

DEFAULT_VERSION_CODE=$(grep "^version" "$DEPEND_PATH/../Cargo.toml" | sed 's/\./_/g' | sed 's/.*"\(.*\)".*/\1/' | cut -d_ -f1-2)

: "${SIMPLICITY_ALLOC_VERSION_CODE:=$DEFAULT_VERSION_CODE}"

## 1. Sanity check environment.
if ! command -v git > /dev/null; then
    echo "Missing 'git' executable in evironment."
    exit 1
fi
if ! command -v rsync > /dev/null; then
    echo "Missing 'rsync' executable in evironment."
    exit 1
fi

if [ ! -d "$DEPEND_PATH" ]; then
    echo "Depend path '$DEPEND_PATH' does not appear to be a directory."
    exit 1
fi

if [ ! -d "$LIBSIM_PATH" ]; then
    echo "libsimplicity path '$LIBSIM_PATH' does not appear to be a directory."
    exit 1
fi

## 2. Set up output directory on Linux filesystem (avoids all NTFS permission issues)
OUT_DIR=/tmp/simplicity_vendor_out
rm -rf "$OUT_DIR"
mkdir -p "$OUT_DIR/depend" "$OUT_DIR/src"

## 3. Copy files from libsimplicity into output dir
pushd "$LIBSIM_PATH/C"

if test -n "$(git status --porcelain)"; then
    echo "WARNING: libsimplicity repo is not clean"
fi

HEAD=$(git rev-parse HEAD)
{
    echo "# This file has been automatically generated."
    echo "$HEAD"
} > "$OUT_DIR/depend/simplicity-HEAD-revision.txt"

git ls-files | rsync -av --files-from=- . "$OUT_DIR/depend/simplicity"

popd

## 4. Patch things to include versions (all on Linux fs, no NTFS issues)

# a. patch rust alloc function names in .rs files
find "$DEPEND_PATH/.." -name "*.rs" -type f | while IFS= read -r src; do
    # compute destination path relative to simplicity-sys root
    rel="${src#$DEPEND_PATH/../}"
    dest="$OUT_DIR/$rel"
    mkdir -p "$(dirname "$dest")"
    sed "s/rust_[0-9_]*_free/rust_${SIMPLICITY_ALLOC_VERSION_CODE}_free/;
         s/rust_[0-9_]*_malloc/rust_${SIMPLICITY_ALLOC_VERSION_CODE}_malloc/;
         s/rust_[0-9_]*_calloc/rust_${SIMPLICITY_ALLOC_VERSION_CODE}_calloc/" "$src" > "$dest"
done

# b. patch the rust_{malloc,calloc,free} functions in simplicity_alloc.h
sed "s/rust_/rust_${SIMPLICITY_ALLOC_VERSION_CODE}_/" \
    < "$DEPEND_PATH/simplicity_alloc.h.patch" \
    | patch "$OUT_DIR/depend/simplicity/simplicity_alloc.h"

# c. patch every single simplicity_* symbol in the library
find "$OUT_DIR/depend/simplicity" -not -path "*/secp256k1/*" \( -name "*.[ch]" -o -name '*.inc' \) -type f | while IFS= read -r f; do
    sed "/^#include/! s/simplicity_/rustsimplicity_${SIMPLICITY_ALLOC_VERSION_CODE}_/g" "$f" > /tmp/_sv_tmp && mv /tmp/_sv_tmp "$f"
done
# ...ok, actually we didn't want to replace simplicity_err
find "$OUT_DIR/depend/simplicity" -not -path "*/secp256k1/*" \( -name "*.[ch]" -o -name '*.inc' \) -type f | while IFS= read -r f; do
    sed "s/rustsimplicity_${SIMPLICITY_ALLOC_VERSION_CODE}_err/simplicity_err/g" "$f" > /tmp/_sv_tmp && mv /tmp/_sv_tmp "$f"
done

# Special-case calls in depend/env.c and depend/wrapper.h
sed "s/rustsimplicity_[0-9]\+_[0-9]\+_/rustsimplicity_${SIMPLICITY_ALLOC_VERSION_CODE}_/" \
    "$DEPEND_PATH/env.c" > "$OUT_DIR/depend/env.c"
sed "s/rustsimplicity_[0-9]\+_[0-9]\+_/rustsimplicity_${SIMPLICITY_ALLOC_VERSION_CODE}_/" \
    "$DEPEND_PATH/wrapper.h" > "$OUT_DIR/depend/wrapper.h"

# d. update link_name= entries in Rust source code
find "$DEPEND_PATH/../src" -name "*.rs" -type f | while IFS= read -r src; do
    rel="${src#$DEPEND_PATH/../}"
    dest="$OUT_DIR/$rel"
    mkdir -p "$(dirname "$dest")"
    sed "s/rustsimplicity_[0-9]\+_[0-9]\+_\(.*\)\([\"(]\)/rustsimplicity_${SIMPLICITY_ALLOC_VERSION_CODE}_\1\2/g" "$src" > "$dest"
done

# e. update the links= field in the manifest file
sed "s/^links = \".*\"$/links = \"rustsimplicity_${SIMPLICITY_ALLOC_VERSION_CODE}\"/" \
    "$DEPEND_PATH/../Cargo.toml" > "$OUT_DIR/Cargo.toml"

## 5. Copy results back to NTFS destination
if [ -d "$VENDORED_SIM_DIR" ]; then
    rm -rf "$VENDORED_SIM_DIR"
fi
cp -r "$OUT_DIR/depend/simplicity" "$VENDORED_SIM_DIR"
cp "$OUT_DIR/depend/simplicity-HEAD-revision.txt" "$DEPEND_PATH/simplicity-HEAD-revision.txt"
cp "$OUT_DIR/depend/env.c" "$DEPEND_PATH/env.c"
cp "$OUT_DIR/depend/wrapper.h" "$DEPEND_PATH/wrapper.h"
cp "$OUT_DIR/Cargo.toml" "$DEPEND_PATH/../Cargo.toml"
find "$OUT_DIR/src" -name "*.rs" -type f | while IFS= read -r f; do
    rel="${f#$OUT_DIR/}"
    cp "$f" "$DEPEND_PATH/../$rel"
done

echo "Done."
