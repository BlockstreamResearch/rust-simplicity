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

if [ -d "$VENDORED_SIM_DIR" ]; then
    while true; do
        read -r -p "$VENDORED_SIM_DIR will be deleted [yn]: " yn
        case $yn in
            [Yy]* ) break;;
            [Nn]* ) exit;;
            * ) echo "Please answer y or n.";;
        esac
    done

    rm -rf "$VENDORED_SIM_DIR"
elif [ -e "$VENDORED_SIM_DIR" ]; then
    echo "'simplicity' inside depend directory exists but appears not to be a directory."
    echo "Please move or delete this file."
    exit 1
fi

## 2. Copy files from libsimplicity
pushd "$LIBSIM_PATH/C"

if test -n "$(git status --porcelain)"; then
    echo "WARNING: libsimplicity repo is not clean"
fi

HEAD=$(git rev-parse HEAD)
echo "# This file has been automatically generated." > "$DEPEND_PATH/simplicity-HEAD-revision.txt"
echo "$HEAD" >> "$DEPEND_PATH/simplicity-HEAD-revision.txt"

# Copy C folder to simplicity-sys/depend/simplicity
# Use rsync to copy only files tracked by git
git ls-files | rsync -av --files-from=- . "$VENDORED_SIM_DIR"

# Ensure directories are writable (rsync may preserve restrictive source permissions)
chmod -R u+w "$VENDORED_SIM_DIR"

# Normalize line endings in case source files came from a Windows mount.
find "$VENDORED_SIM_DIR" -type f \( -name "*.[ch]" -o -name "*.inc" -o -name "Makefile" \) | while IFS= read -r f; do
    tmp=$(mktemp)
    tr -d '\r' < "$f" > "$tmp" && cat "$tmp" > "$f"
    rm "$tmp"
done

popd

## 3. Patch things to include versions

# Helper: in-place sed that works on Windows bind mounts (avoids rename).
# Usage: ised [sed-flags] pattern file
ised() {
    local file="${*: -1}"
    local args=("${@:1:$#-1}")
    local tmp
    tmp=$(mktemp)
    sed "${args[@]}" < "$file" > "$tmp" && cat "$tmp" > "$file"
    rm -f "$tmp"
}

# a. patch our own drop/new functions for C structures.
find "$DEPEND_PATH/.." -name "*.rs" -type f | while IFS= read -r f; do
    ised "s/rust_[0-9_]*_free/rust_${SIMPLICITY_ALLOC_VERSION_CODE}_free/" "$f"
    ised "s/rust_[0-9_]*_malloc/rust_${SIMPLICITY_ALLOC_VERSION_CODE}_malloc/" "$f"
    ised "s/rust_[0-9_]*_calloc/rust_${SIMPLICITY_ALLOC_VERSION_CODE}_calloc/" "$f"
done

# b. patch the rust_{malloc,calloc,free} functions in simplicity_alloc.h
sed "s/rust_/rust_${SIMPLICITY_ALLOC_VERSION_CODE}_/" \
    < "$DEPEND_PATH/simplicity_alloc.h.patch" \
    | patch "$VENDORED_SIM_DIR/simplicity_alloc.h"

# c. patch every single simplicity_* symbol in the library (every instance except
#    those in #includes, which is overkill but doesn't hurt anything)
find "$DEPEND_PATH/simplicity" \( -name "*.[ch]" -o -name '*.inc' \) -type f | while IFS= read -r f; do
    ised "/^#include/! s/simplicity_/rustsimplicity_${SIMPLICITY_ALLOC_VERSION_CODE}_/g" "$f"
done
# ...ok, actually we didn't want to replace simplicity_err
find "$DEPEND_PATH/simplicity" \( -name "*.[ch]" -o -name '*.inc' \) -type f | while IFS= read -r f; do
    ised "s/rustsimplicity_${SIMPLICITY_ALLOC_VERSION_CODE}_err/simplicity_err/g" "$f"
done
# Special-case calls in depend/env.c and depend/warpper.h
ised -r "s/rustsimplicity_[0-9]+_[0-9]+_/rustsimplicity_${SIMPLICITY_ALLOC_VERSION_CODE}_/" "$DEPEND_PATH/env.c"
ised -r "s/rustsimplicity_[0-9]+_[0-9]+_/rustsimplicity_${SIMPLICITY_ALLOC_VERSION_CODE}_/" "$DEPEND_PATH/wrapper.h"

# d. ...also update the corresponding link_name= entries in the Rust source code
find "./src/" -name "*.rs" -type f | while IFS= read -r f; do
    ised -r "s/rustsimplicity_[0-9]+_[0-9]+_(.*)([\"\(])/rustsimplicity_${SIMPLICITY_ALLOC_VERSION_CODE}_\1\2/g" "$f"
done
# e. ...and the links= field in the manifest file
ised -r "s/^links = \".*\"$/links = \"rustsimplicity_${SIMPLICITY_ALLOC_VERSION_CODE}\"/" Cargo.toml

