#!/bin/bash
set -e


if [ -z "$1" ]; then
  echo "\$1 parameter must be the rust-simplicity-sys depend directory"
  echo "\$2 parameter (optional) can be the revision to check out"
  exit 1
fi

PARENT_DIR=$1
REV=$2
DIR=simplicity
ORIGDIR=$(pwd)

while true; do
    read -r -p "$PARENT_DIR/$DIR will be deleted [yn]: " yn
    case $yn in
        [Yy]* ) break;;
        [Nn]* ) exit;;
        * ) echo "Please answer yes or no.";;
    esac
done

cd "$PARENT_DIR" || exit 1
rm -rf "$DIR"
git clone https://github.com/ElementsProject/simplicity.git "${DIR}"

cd "$DIR"
if [ -n "$REV" ]; then
    git checkout "$REV"
fi
HEAD=$(git rev-parse HEAD)
cd ..
echo "# This file was automatically created by $0" > ./simplicity-HEAD-revision.txt
echo "$HEAD" >> ./simplicity-HEAD-revision.txt

# Only get the C directory. move C into parent folder, and then move it back
mv "${DIR}/C" "./${DIR}_C"
rm -rf "${DIR}"
mv "${DIR}_C" "${DIR}"

# We don't make any automatic source changes here. Optionally patch files
# can be added here to automate the process. In future, we can have support for
# symbols if required.