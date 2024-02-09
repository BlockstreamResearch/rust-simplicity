extern crate cc;

use std::env;
use std::path::Path;

fn main() {
    // rerun if changes to the C code
    println!("cargo:rerun-if-changed=depend");
    let simplicity_path = Path::new("depend/simplicity");
    let jet_files: Vec<_> = vec![
        "frame.c",
        "jets.c",
        "jets-secp256k1.c",
        "rsort.c",
        "sha256.c",
        "primitive/elements/env.c",
        "primitive/elements/ops.c",
        "primitive/elements/jets.c",
    ]
    .into_iter()
    .map(|x| simplicity_path.join(x))
    .collect();

    let mut build = cc::Build::new();
    build
        .std("c11")
        .flag_if_supported("-fno-inline-functions")
        .opt_level(2)
        .files(jet_files)
        .file(Path::new("depend/wrapper.c"))
        .file(Path::new("depend/env.c"))
        .file(Path::new("depend/jets_wrapper.c"))
        .include(simplicity_path.join("include"));

    if cfg!(feature = "test-utils") {
        let test_files: Vec<_> = vec![
            "bitstream.c",
            "dag.c",
            "deserialize.c",
            "eval.c",
            "type.c",
            "typeInference.c",
            "primitive/elements/exec.c",
            "primitive/elements/primitive.c",
            "ctx8Pruned.c",
            "ctx8Unpruned.c",
            "hashBlock.c",
            "schnorr0.c",
            "schnorr6.c",
            "primitive/elements/checkSigHashAllTx1.c",
        ]
        .into_iter()
        .map(|x| simplicity_path.join(x))
        .collect();

        build.files(test_files);
    }

    if cfg!(not(fuzzing)) {
        build.define("PRODUCTION", None);
    }

    // Fix missing libc in WASM
    if env::var("CARGO_CFG_TARGET_ARCH").unwrap() == "wasm32" {
        build.include("wasm-sysroot");
    }

    build.compile("ElementsSimplicity");
}
