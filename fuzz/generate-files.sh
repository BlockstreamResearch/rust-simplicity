#!/usr/bin/env bash

set -e

REPO_DIR=$(git rev-parse --show-toplevel)

# shellcheck source=./fuzz-util.sh
source "$REPO_DIR/fuzz/fuzz-util.sh"

# 1. Generate fuzz/Cargo.toml
cat > "$REPO_DIR/fuzz/Cargo.toml" <<EOF
[package]
name = "simplicity-fuzz"
edition = "2021"
rust-version = "1.78.0"
version = "0.0.1"
authors = ["Generated by fuzz/generate-files.sh"]
publish = false

[package.metadata]
cargo-fuzz = true

[lib]
path = "fuzz_lib/lib.rs"

[dependencies]
libfuzzer-sys = "0.4"
# We shouldn't need an explicit version on the next line, but Andrew's tools
# choke on it otherwise. See https://github.com/nix-community/crate2nix/issues/373
simplicity-lang = { path = "..", features = ["test-utils"], version = "0.4.0" }
old_simplicity = { package = "simplicity-lang", version = "0.3.1", default-features = false }

[dev-dependencies]
base64 = "0.22.1"

[lints.rust]
unexpected_cfgs = { level = "warn", check-cfg = ['cfg(fuzzing)'] }
EOF

for targetFile in $(listTargetFiles); do
    targetName=$(targetFileToName "$targetFile")
    cat >> "$REPO_DIR/fuzz/Cargo.toml" <<EOF

[[bin]]
name = "$targetName"
path = "$targetFile"
test = false
doc = false
bench = false
EOF
done

# 2. Generate .github/workflows/fuzz.yml
cat > "$REPO_DIR/.github/workflows/fuzz.yml" <<EOF
# Automatically generated by fuzz/generate-files.sh
name: Fuzz

on:
  push:
    branches:
      - master
      - 'test-ci/**'
  pull_request:

jobs:
  fuzz:
    name: Run Fuzz Target
    if: \${{ !github.event.act }}
    runs-on: ubuntu-latest
    strategy:
      fail-fast: false
      matrix:
        fuzz_target: [
$(for name in $(listTargetNames); do echo "$name,"; done)
        ]
    steps:
      - name: Checkout Crate
        uses: actions/checkout@v4

      - name: Use Rust Cache
        uses: actions/cache@v4
        id: cache-fuzz
        with:
          path: |
            ~/.cargo/bin
            fuzz/target
            target
          key: cache-\${{ matrix.target }}-\${{ hashFiles('**/Cargo.toml','**/Cargo.lock') }}

      - name: Install Toolchain
        uses: dtolnay/rust-toolchain@master
        with:
          toolchain: nightly-2024-07-01
          components: "llvm-tools-preview"

      - name: Install Dependencies
        run: cargo update && cargo update -p cc --precise 1.0.83 && cargo install --force cargo-fuzz

      - name: Run Fuzz Target
        run: ./fuzz/fuzz.sh "\${{ matrix.fuzz_target }}"

      - name: Prepare Artifact
        run: echo "\${{ matrix.fuzz_target }}" >executed_\${{ matrix.fuzz_target }}

      - name: Upload Artifact
        uses: actions/upload-artifact@v4
        with:
          name: executed_\${{ matrix.fuzz_target }}
          path: executed_\${{ matrix.fuzz_target }}

  verify-execution:
    name: Verify Execution of All Targets
    if: \${{ !github.event.act }}
    needs: fuzz
    runs-on: ubuntu-latest
    steps:
      - name: Checkout Crate
        uses: actions/checkout@v4

      - name: Download All Artifacts
        uses: actions/download-artifact@v4

      - name: Display Structure of Downloaded Files
        run: ls -R

      - name: Write File With All Executed Targets
        run: find executed_* -type f -exec cat {} + | sort > executed

      - name: Compare Executed Targets With Available Targets
        run: source ./fuzz/fuzz-util.sh && listTargetNames | sort | diff - executed
EOF
