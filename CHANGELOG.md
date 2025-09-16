# 0.6.0 - 2025-09-17

* Improve FFI API and fix soundness issues [#307](https://github.com/BlockstreamResearch/rust-simplicity/pull/288)
* Decrease MSRV to 1.74.0, matching rust-bitcoin [#308](https://github.com/BlockstreamResearch/rust-simplicity/pull/308)
* Use `ghost_cell` within `types::Context`, requiring users to construct programs within a single Rust lexical scope [#305](https://github.com/BlockstreamResearch/rust-simplicity/pull/305)

# 0.5.0 - 2025-07-29

* Add generic `CaseTracker` in bit machine for debugging [#288](https://github.com/BlockstreamResearch/rust-simplicity/pull/288)
* Add debugging support to `ExecTracker` [#293](https://github.com/BlockstreamResearch/rust-simplicity/pull/293)
* Separate linker symbols to allow multiple simplicity-sys versions in same tree [#297](https://github.com/BlockstreamResearch/rust-simplicity/pull/297)
* Add base64 encoding and decoding support to programs [#299](https://github.com/BlockstreamResearch/rust-simplicity/pull/299)

