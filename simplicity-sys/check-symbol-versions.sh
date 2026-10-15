#!/usr/bin/env bash
set -euo pipefail

# Check that every versioned symbol prefix in simplicity-sys matches the crate version.
#
# vendor-simplicity.sh renames every libsimplicity symbol to `rustsimplicity_<major>_<minor>_*`
# and the allocator hooks to `rust_<major>_<minor>_*`, so that semver-incompatible versions of
# this crate can be linked into the same binary. A file that keeps an older prefix (for example
# one copied in by hand instead of vendored) silently defeats that, so fail if any prefix other
# than the current one appears.

cd "$(dirname "$0")"

VERSION_CODE=$(grep "^version" Cargo.toml | sed 's/\./_/g' | sed 's/.*"\(.*\)".*/\1/' | cut -d_ -f1-2)

# Each output line is `file:line:match`; keep only matches that are not the current version.
STALE=$(grep -rnoE '(rustsimplicity|rust)_[0-9]+_[0-9]+' Cargo.toml depend src \
    | grep -vE ":(rustsimplicity|rust)_${VERSION_CODE}\$" || true)

if [ -n "$STALE" ]; then
    echo "Found symbol prefixes that do not match the simplicity-sys version code ${VERSION_CODE}:"
    echo "$STALE"
    echo
    echo "Expected only rustsimplicity_${VERSION_CODE}_* and rust_${VERSION_CODE}_*."
    echo "Re-run vendor-simplicity.sh rather than copying libsimplicity files in by hand."
    exit 1
fi

echo "All symbol prefixes match version code ${VERSION_CODE}."
