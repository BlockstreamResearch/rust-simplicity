#!/usr/bin/env bash
set -e

# Requires (nix-shell with) git, rsync

if [ -z "$1" ] || [ -z "$2" ]; then
  echo "\$1 path to depend directory inside rust-simplicity-sys"
  echo "\$2 path to libsimplicity repository root"
  exit 1
fi

ORIGINAL_PATH=$(pwd)
DEPEND_PATH=$1
LIBSIM_PATH=$2
VENDORED_SIM_DIR=simplicity

while true; do
    read -r -p "$DEPEND_PATH/$VENDORED_SIM_DIR will be deleted [yn]: " yn
    case $yn in
        [Yy]* ) break;;
        [Nn]* ) exit;;
        * ) echo "Please answer yes or no.";;
    esac
done

rm -rf "$DEPEND_PATH/$VENDORED_SIM_DIR"
cd "$LIBSIM_PATH"

if test -n "$(git status --porcelain)"; then
    echo "WARNING: libsimplicity repo is not clean"
fi

HEAD=$(git rev-parse HEAD)
echo "# This file has been automatically generated." > "$ORIGINAL_PATH/$DEPEND_PATH/simplicity-HEAD-revision.txt"
echo "$HEAD" >> "$ORIGINAL_PATH/$DEPEND_PATH/simplicity-HEAD-revision.txt"

# Copy C folder to simplicity-sys/depend/simplicity
# Use rsync to copy only files tracked by git
cd C
git ls-files | rsync -av --files-from=- . "$ORIGINAL_PATH/$DEPEND_PATH/$VENDORED_SIM_DIR"
cd ..

# We don't make any automatic source changes here. Optionally patch files
# can be added here to automate the process. In future, we can have support for
# symbols if required.

cd "$ORIGINAL_PATH"
