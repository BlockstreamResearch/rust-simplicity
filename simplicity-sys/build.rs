extern crate cc;

use std::env;
use std::path::{Path, PathBuf};

// What we compile, from the lists vendor-simplicity.sh takes from libsimplicity's Makefile:
// - library sources (CORE, BITCOIN, ELEMENTS): always, except TEST_UTILS_ONLY (test-utils only);
// - test sources (TEST): with test-utils only;
// - never anything in EXCLUDED.
// Each source goes in the library named by its directory: bitcoin/, elements/, or common.
include!("depend/simplicity-sources.rs");

const SIMPLICITY_DIR: &str = "depend/simplicity";

/// Rust has its own decoder, type inference and evaluator, so the C versions are only used to
/// cross-check them in tests.
const TEST_UTILS_ONLY: &[&str] = &[
    "bitstream.c",
    "dag.c",
    "deserialize.c",
    "eval.c",
    "type.c",
    "typeInference.c",
    "elements/exec.c",
    "elements/primitive.c",
    // #385: uncomment along with the Bitcoin library below.
    // "bitcoin/exec.c",
    // "bitcoin/primitive.c",
];

/// Sources we never compile, and why.
const EXCLUDED: &[(&str, &str)] = &[
    ("elements/cmr.c", "Rust computes CMRs itself"),
    ("cmr.c", "Rust computes CMRs itself"),
    ("bitcoin/cmr.c", "Rust computes CMRs itself"),
    ("test.c", "libsimplicity's test runner; defines main()"),
    ("regression4.c", "only used by test.c"),
    ("typeSkipTest.c", "only used by test.c"),
];

#[derive(Clone, Copy, PartialEq, Eq)]
enum Library {
    Common,
    Bitcoin,
    Elements,
}

/// The library a source belongs in, by its directory in libsimplicity.
fn library(source: &str) -> Library {
    if source.starts_with("bitcoin/") {
        Library::Bitcoin
    } else if source.starts_with("elements/") {
        Library::Elements
    } else {
        Library::Common
    }
}

/// The sources to compile into `lib`, following the rules at the top of this file.
fn sources(lib: Library) -> Vec<PathBuf> {
    let test_utils = cfg!(feature = "test-utils");
    let library_sources = CORE_SOURCES
        .iter()
        .chain(BITCOIN_SOURCES)
        .chain(ELEMENTS_SOURCES)
        .copied()
        .filter(|source| test_utils || !TEST_UTILS_ONLY.contains(source));
    let test_sources = TEST_SOURCES.iter().copied().filter(|_| test_utils);

    library_sources
        .chain(test_sources)
        .filter(|source| !EXCLUDED.iter().any(|(excluded, _)| excluded == source))
        .filter(|source| library(source) == lib)
        .map(|source| Path::new(SIMPLICITY_DIR).join(source))
        .collect()
}

/// Fail the build if the tables above name a source that libsimplicity's Makefile doesn't build,
/// e.g. because of a typo or because upstream renamed or removed it. Otherwise the entry would
/// silently do nothing. (Whether the vendored tree matches upstream is checked in CI, by re-running
/// vendor-simplicity.sh.)
fn check_tables() {
    let upstream = || {
        CORE_SOURCES
            .iter()
            .chain(BITCOIN_SOURCES)
            .chain(ELEMENTS_SOURCES)
            .chain(TEST_SOURCES)
    };
    let stale: Vec<_> = TEST_UTILS_ONLY
        .iter()
        .chain(EXCLUDED.iter().map(|(source, _)| source))
        .filter(|source| !upstream().any(|upstream| upstream == *source))
        .collect();
    assert!(
        stale.is_empty(),
        "build.rs mentions {stale:?}, but libsimplicity's Makefile doesn't build them.",
    );
}

fn main() {
    // rerun if changes to the C code
    println!("cargo:rerun-if-changed=depend");
    check_tables();

    let mut build = cc::Build::new();
    build
        .std("c11")
        .flag_if_supported("-fno-inline-functions")
        .opt_level(2)
        .include(Path::new(SIMPLICITY_DIR).join("include"));

    if cfg!(not(fuzzing)) {
        build.define("PRODUCTION", None);
    }

    // Fix missing libc in WASM
    if env::var("CARGO_CFG_TARGET_ARCH").unwrap() == "wasm32" {
        build.include("wasm-sysroot");
    }

    // Chain-independent code goes in its own library, which each chain's library links against.
    // Compiling it into every chain's library instead breaks the MSRV build: Rust 1.74 links native
    // libraries with --whole-archive, so the duplicate objects collide.
    build
        .clone()
        .files(sources(Library::Common))
        .file("depend/wrapper.c")
        .file("depend/jets_wrapper.c")
        .compile("RustSimplicityCommon");

    // Build the Bitcoin transaction environment's library.
    build
        .clone()
        .files(sources(Library::Bitcoin))
        .file("depend/bitcoin_env.c")
        .compile("BitcoinSimplicity");

    build
        .files(sources(Library::Elements))
        .file("depend/env.c")
        .compile("ElementsSimplicity");
}
