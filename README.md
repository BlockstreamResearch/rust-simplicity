<p>
  <a href="https://crates.io/crates/simplicity-lang"><img alt="Crate Info" src="https://img.shields.io/crates/v/simplicity-lang.svg"/></a>
  <a href="https://github.com/BlockstreamResearch/rust-simplicity/blob/master/LICENSE"><img alt="CC0 1.0 Universal Licensed" src="https://img.shields.io/badge/license-CC0--1.0-blue.svg"/></a>
  <a href="https://github.com/BlockstreamResearch/rust-simplicity/actions?query=workflow%3AContinuous%20integration"><img alt="CI Status" src="https://img.shields.io/github/actions/workflow/status/BlockstreamResearch/rust-simplicity/main.yml"></a>
  <a href="https://docs.rs/simplicity-lang"><img alt="API Docs" src="https://img.shields.io/badge/docs.rs-simplicity_lang-green"/></a>
</p>

# rust-simplicity

This is the official Rust library of the [Simplicity Language](https://simplicity-lang.org/).

Simplicity is a low-level, typed functional language designed to be a drop-in alternative
for Bitcoin's [Tapscript](https://github.com/bitcoin/bips/blob/master/bip-0342.mediawiki).
It offers static resource bounds and has a formal specification (in Rocq) which allows the
creation of machine-checkable proofs of program behavior.

It is currently deployed on Blockstream's Liquid, which is a sidechain resembling Bitcoin
in many ways; but which differs in many Script-relevant ways (e.g. supporting multiple
assets and using Confidential Transactions). We expect by the end of 2025 to directly
support Bitcoin, so that Simplicity can be used on a custom regtest chain.

Simplicity is a very low-level language. If you are simply looking to develop with the
language, you may wish to use [SimplicityHL](https://github.com/BlockstreamResearch/simplicityhl)
instead.

## rust-simplicity

The goal of rust-simplicity is to provide the tools to encode, decode, analyze, construct
and manipulate Simplicity programs at all stages of development. These stages are:

* **construction**, which is when a program is incomplete, and consists of a set of
  "Simplicity expressions" intended to be assembled into a full program. Such expressions
  are represented by the [`ConstructNode`](https://docs.rs/simplicity-lang/latest/simplicity/node/type.ConstructNode.html)
  type.
* **commitment**, which is when a program is complete but has not been satisfied. At this
  point, it is possible to derive addressses and receive funds to them, but to spend
  those funds requires the generation of witness data. Such programs are represented
  by [`CommitNode`](https://docs.rs/simplicity-lang/latest/simplicity/node/type.CommitNode.html)
* **redemption**, which is when a program has witness data attached. This is the only
  point at which a program may appear explicitly on the blockchain, and the only point
  at which the program has a canonical encoding. Such programs are represented by
  [`RedeemNode`](https://docs.rs/simplicity-lang/latest/simplicity/node/type.RedeemNode.html)

Generally speaking, the process of producing a Simplicity program involves moving
from one node type to the next.

## Installation

To use rust-simplicity, this to your Cargo.toml:

```toml
[dependencies]
simplicity-lang = "0.7"
```

## Quick Start

```rust
use simplicity::node::CoreConstructible;
use simplicity::types::Context;
use simplicity::{ConstructNode, jet::Core};
use std::sync::Arc;

// Create a trivial Simplicity program
let program = Context::with_context(|ctx| {
    let construct = Arc::<ConstructNode<Core>>::unit(&ctx);
    construct.finalize_types().unwrap()
});

// Encode the program to bytes
let encoded: Vec<u8> = simplicity::write_to_vec(|w| {
    program.encode_without_witness(w)
});
```

## Relationship to libsimplicity

This Rust library provides extensive tools for program construction and analysis, while
[libsimplicity](https://github.com/BlockstreamResearch/Simplicity) is a minimal C library
focused solely on decoding, validating, and executing programs in blockchain consensus code.

rust-simplicity includes a vendored copy of libsimplicity and uses it for:

- cryptographic and other "heavy" operations through optimized C jets
- cross-validating the C and Rust Bit Machine implementations

The libsimplicity repository also contains:
- A reference implementation written in Haskell
- A Rocq (Coq) implementation for formal verification and machine-checkable proofs

## Further Reading

- **[API Documentation](https://docs.rs/simplicity-lang/)** - Complete API reference
- **[Simplicity Tech Report](https://github.com/BlockstreamResearch/simplicity/blob/pdf/Simplicity-TR.pdf)** - Full informal language specification
- **[Simplicity Website](https://simplicity-lang.org/)** - Official language homepage
- **[libsimplicity](https://github.com/BlockstreamResearch/Simplicity)** - Reference implementation

# Contributing

The goals of this library are to support the low-level Simplicity language, and to
provide a basis for the [SimplicityHL](https://github.com/BlockstreamResearch/simplicityhl)
language implementation. If you wish to work on high-level programming language
abstractions, you may want to visit that repo instead.

Having said that, contributions are welcome! This library is in its early stages
and undergoing rapid development. We especially welcome improvements in:

* representing and displaying type inference information and errors
* representing and displaying information about Simplicity (sub)expressions;
  usefully and uniquely specifying subexpressions within larger expressions
* improvements to the debuggability and introspection of the Bit Machine, the
  Simplicity interpreter.
* testing, especially of programs that exhibit extreme behavior; improving test
  infrastructure and CI
* improving the ergonomics and reliability of our vendoring scripts (see below)
* documentation!

## Minimum Supported Rust Version

The MSRV of this crate is **1.74.0**. For now we will not increase this without
a major version bump, though as the library matures we may change that policy.

## Updating libsimplicity

This library includes a vendored copy of the libsimplicity C library, as well as
files generated by the `GenRustJets` utility from the libsimplicity Haskell library.
These files are clearly marked as being autogenerated, and may not be directly
changed.

Instead, to update this code you must use the included `vendor-simplicity.sh` and
`update_jets.sh` scripts.

`update_jets.sh` requires the user has `cabal` and all dependencies of the libsimplicity
Haskell library. Instructions for those can be found in the simplicity repository.

## Benchmarking

There are two sets of benchmarks in this codebase. First, there is the `jets-bench`
sub-crate which uses criterion to collect statistics about jet performance. These
benchmarks are specifically targeted at the C jets and are intended to estimate
consensus costs.

See `jets-bench/README.md` for information about running these benchmarks.

The second set of benchmarks are benchmarks for this library's performance. These
are used to track performance of this library itself. These can be run with

```
RUSTFLAGS=--cfg=bench cargo +nightly bench
```

The custom `cfg` flag is used to prevent nightly-only code from polluting ordinary
code.
