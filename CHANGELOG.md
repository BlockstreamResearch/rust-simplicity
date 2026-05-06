# 0.7.1 - 2026-05-06

* Add runtime jet type check in `Context` [#361](https://github.com/BlockstreamResearch/rust-simplicity/pull/361)
* Migrate execution to dynamic dispatch [#360](https://github.com/BlockstreamResearch/rust-simplicity/pull/360)
* Move the `Jet` codec and parser towards object-safety [#358](https://github.com/BlockstreamResearch/rust-simplicity/pull/358)
* Lock `human_encoding` module behind a feature flag [#357](https://github.com/BlockstreamResearch/rust-simplicity/pull/357)
* Fix `get_padding` for larger costs and padding lengths [#356](https://github.com/BlockstreamResearch/rust-simplicity/pull/356)
* Rewrite the `human_encoding` lexer and parser using `logos` [#354](https://github.com/BlockstreamResearch/rust-simplicity/pull/354)
* Separate `Jet` env from the `Jet` trait [#351](https://github.com/BlockstreamResearch/rust-simplicity/pull/351)
* Fix value bit corruption in `right_shift` [#348](https://github.com/BlockstreamResearch/rust-simplicity/pull/348)
* Fix FFI type layout for Android [#347](https://github.com/BlockstreamResearch/rust-simplicity/pull/347)
* Add API to output programs as Graphviz and Mermaid diagrams [#344](https://github.com/BlockstreamResearch/rust-simplicity/pull/344)
* Fix FFI for platform-specific `uint_fast` types on macOS and Android [#334](https://github.com/BlockstreamResearch/rust-simplicity/pull/334)

# 0.7.0 - 2025-12-11

* Improve `Display` for `Value` in several ways [#328](https://github.com/BlockstreamResearch/rust-simplicity/pull/328)
* Replace `ExecTracker` trait with a more general API and add `PruneTracker` extension trait [#325](https://github.com/BlockstreamResearch/rust-simplicity/pull/325)
* Add `CONTRIBUTING.md` [#321](https://github.com/BlockstreamResearch/rust-simplicity/pull/321)

# 0.6.0 - 2025-09-17

* Improve FFI API and fix soundness issues [#307](https://github.com/BlockstreamResearch/rust-simplicity/pull/288)
* Decrease MSRV to 1.74.0, matching rust-bitcoin [#308](https://github.com/BlockstreamResearch/rust-simplicity/pull/308)
* Use `ghost_cell` within `types::Context`, requiring users to construct programs within a single Rust lexical scope [#305](https://github.com/BlockstreamResearch/rust-simplicity/pull/305)

# 0.5.0 - 2025-07-29

* Add generic `CaseTracker` in bit machine for debugging [#288](https://github.com/BlockstreamResearch/rust-simplicity/pull/288)
* Add debugging support to `ExecTracker` [#293](https://github.com/BlockstreamResearch/rust-simplicity/pull/293)
* Separate linker symbols to allow multiple simplicity-sys versions in same tree [#297](https://github.com/BlockstreamResearch/rust-simplicity/pull/297)
* Add base64 encoding and decoding support to programs [#299](https://github.com/BlockstreamResearch/rust-simplicity/pull/299)

