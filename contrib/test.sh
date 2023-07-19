#!/bin/sh

set -ex

FEATURES="bitcoin elements test-utils serde"

if [ "$DO_COV" = true ]
then
    export RUSTFLAGS="-C link-dead-code"
fi

cargo --version
rustc --version

# Some tests require certain toolchain types.
NIGHTLY=false
STABLE=true
if cargo --version | grep nightly; then
    STABLE=false
    NIGHTLY=true
fi
if cargo --version | grep beta; then
    STABLE=false
fi

# Pin dependencies as required if we are using MSRV toolchain.
if cargo --version | grep "1\.48"; then
    # 1.0.100 uses syn 2.0 which uses edition 2021
    cargo update -p serde_json --precise 1.0.99
    # 1.0.157 uses syn 2.0
    cargo update -p serde --precise 1.0.156
    # 1.0.31 uses proc-macro 1.0.66
    cargo update -p quote --precise 1.0.30
    # 1.0.66 uses edition 2021
    cargo update -p proc-macro2 --precise 1.0.65
fi

if [ "$DO_LINT" = true ]
then
    cargo clippy --locked --all-features --all-targets -- -D warnings

    # We should not have any duplicate dependencies. This catches mistakes made upgrading dependencies
    # in one crate and not in another (e.g. upgrade bitcoin_hashes in bitcoin but not in secp).
    duplicate_dependencies=$(
        # Only show the actual duplicated deps, not their reverse tree, then
        # whitelist the 'syn' crate which is duplicated but it's not our fault.
        cargo tree  --target=all --all-features --duplicates \
            | grep '^[0-9A-Za-z]' \
            | grep -v 'syn' \
            | wc -l
                          )
    if [ "$duplicate_dependencies" -ne 0 ]; then
        echo "Dependency tree is broken, contains duplicates"
        cargo tree  --target=all --all-features --duplicates
        exit 1
    fi
fi

echo "********* Testing no features *************"
# Test without any features
cargo test --locked --verbose --no-default-features

echo "********* Testing default *************"
# Then test with the default features
cargo test --verbose

# Test each feature
for feature in ${FEATURES}
do
    echo "********* Testing $feature *************"
    cargo test --locked --verbose --features="$feature"
done

# Run ignored tests if told to
if [ "$DO_IGNORED" = true ]
then
    echo "********* Ignored tests *************"
    cargo test -- --ignored
fi

# Build the docs if told to (this only works with the nightly toolchain)
if [ "$DO_DOCSRS" = true ]; then
    RUSTDOCFLAGS="--cfg docsrs -D warnings -D rustdoc::broken-intra-doc-links" cargo +nightly doc --all-features
fi

# Build the docs with a stable toolchain, in unison with the DO_DOCSRS command
# above this checks that we feature guarded docs imports correctly.
if [ "$DO_DOCS" = true ]; then
    RUSTDOCFLAGS="-D warnings" cargo +stable doc --all-features
fi

# Run formatter if told to.
if [ "$DO_FMT" = true ]; then
    if [ "$NIGHTLY" = false ]; then
        echo "DO_FMT requires a nightly toolchain (consider using RUSTUP_TOOLCHAIN)"
        exit 1
    fi
    rustup component add rustfmt
    cargo fmt --check
fi

# Run benchmarks if told to
if [ "$DO_BENCH" = true ]
then
    echo "********* Benchmarks *************"
    cargo install cargo-criterion
    cd jets-bench
    cargo test
    cargo criterion --no-run
fi

# Run simpcli if told to
if [ "$DO_CLI" = true ]
then
    echo "********* CLI *************"
    cd simpcli
    cargo test
fi

# Use as dependency if told to
if [ "$AS_DEPENDENCY" = true ]
then
    cargo new dep_test 2> /dev/null # Mute warning about workspace, fixed below.
    cd dep_test
    echo 'bitcoin = { path = "..", features = ["serde"] }\n\n' >> Cargo.toml
    # Adding an empty workspace section excludes this crate from the rust-bitcoin workspace.
    echo '[workspace]\n\n' >> Cargo.toml

    cargo test --verbose
fi
