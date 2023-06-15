![Build](https://github.com/apoelstra/rust-simplicity/workflows/Continuous%20integration/badge.svg)

# rust-simplicity
Under development....

# Minimum Supported Rust Version

The MSRV of this crate is **1.48.0**.

# Updating jets code

Some of the jet files in the library are auto-generated from Haskell code. These can be updated `update_jets.sh`. This requires the user has `cabal` and other necessary things that are required to build simplicity haskell. Instructions for those can be found in the simplicity repository.

This script also checks that the internal vendored version of simplicity has the same git hash as of the version from which we are auto-generating the code. If this is not the case, the script will fail. This is because we only vendor minimal required C simplicity code and not the entire simplicity repo.
