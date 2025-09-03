extern crate cc;

use std::env;
use std::path::Path;

fn main() {
    // rerun if changes to the C code
    println!("cargo:rerun-if-changed=depend");
    let simplicity_path = Path::new("depend/simplicity");

    let mut files = vec![];

    // 1. Base files.
    files.extend(
        [
            "frame.c",
            "jets.c",
            "jets-secp256k1.c",
            "rsort.c",
            "sha256.c",
        ]
        .into_iter()
        .map(|x| simplicity_path.join(x)),
    );
    // 2. Test files.
    if cfg!(feature = "test-utils") {
        files.extend(
            [
                "bitstream.c",
                "dag.c",
                "deserialize.c",
                "eval.c",
                "type.c",
                "typeInference.c",
                "ctx8Pruned.c",
                "ctx8Unpruned.c",
                "hashBlock.c",
                "schnorr0.c",
                "schnorr6.c",
            ]
            .into_iter()
            .map(|x| simplicity_path.join(x)),
        );
    }

    // Common files are compiled once into their own static library; Bitcoin
    // and Elements each get their own library with only the chain-specific
    // files. Duplicating the common files into both the Bitcoin and Elements
    // libraries (as before) builds fine on stable (the linker only pulls the
    // archive members it needs), but MSRV (1.74.0) forces `--whole-archive`
    // on every native static lib, which pulls every object from both copies
    // and fails with "multiple definition of" for every common symbol.
    let common_files = files;

    // 3B. Bitcoin base files.
    let mut bitcoin_files = vec![];
    bitcoin_files.extend(
        [
            "bitcoin/env.c",
            "bitcoin/ops.c",
            "bitcoin/bitcoinJets.c",
            "bitcoin/txEnv.c",
        ]
        .into_iter()
        .map(|x| simplicity_path.join(x)),
    );
    bitcoin_files.push("depend/bitcoin_env.c".into());

    // 3E. Elements base files.
    let mut elements_files = vec![];
    elements_files.extend(
        [
            "elements/env.c",
            "elements/ops.c",
            "elements/elementsJets.c",
            "elements/txEnv.c",
        ]
        .into_iter()
        .map(|x| simplicity_path.join(x)),
    );
    elements_files.push("depend/env.c".into());

    if cfg!(feature = "test-utils") {
        // 4B. Bitcoin base files.
        bitcoin_files.extend(
            [
                "bitcoin/exec.c",
                "bitcoin/primitive.c",
                // "bitcoin/checkSigHashAllTx1.c", // no sighashall test
            ]
            .into_iter()
            .map(|x| simplicity_path.join(x)),
        );

        // 4E. Elements base files.
        elements_files.extend(
            [
                "elements/exec.c",
                "elements/primitive.c",
                "elements/checkSigHashAllTx1.c",
            ]
            .into_iter()
            .map(|x| simplicity_path.join(x)),
        );
    }

    // General build
    let mut build = cc::Build::new();
    build
        .std("c11")
        .flag_if_supported("-fno-inline-functions")
        .opt_level(2)
        .include(simplicity_path.join("include"));

    if cfg!(not(fuzzing)) {
        build.define("PRODUCTION", None);
    }

    // Fix missing libc in WASM
    if env::var("CARGO_CFG_TARGET_ARCH").unwrap() == "wasm32" {
        build.include("wasm-sysroot");
    }

    let mut common_build = build.clone();
    let mut bitcoin_build = build.clone();
    let mut elements_build = build;

    // Common build: files shared between Bitcoin and Elements, including the
    // wrapper files. Compiled once into its own static library so neither
    // the Bitcoin nor the Elements library contains these object files --
    // see the comment above `common_files` for why that matters.
    common_build
        .file(Path::new("depend/wrapper.c"))
        .file(Path::new("depend/jets_wrapper.c"))
        .files(common_files)
        .compile("RustSimplicityCommon");

    // Bitcoin build
    bitcoin_build
        .files(bitcoin_files)
        .compile("BitcoinSimplicity");

    // Elements build
    elements_build
        .files(elements_files)
        .compile("ElementsSimplicity");
}
