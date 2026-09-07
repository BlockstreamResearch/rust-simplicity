// SPDX-License-Identifier: CC0-1.0

//! Tests that the Bitcoin backend actually executes (and prunes) tx-context /
//! crypto jets through the C implementations, using a *real* transaction
//! environment (`c_jet_env` returns the live `CTxEnv`, not the unit type).

use std::sync::Arc;

use bitcoin::taproot::ControlBlock;

use crate::jet::Bitcoin;
use crate::jet::BitcoinEnv;
use crate::node::{ConstructNode, CoreConstructible as _};
use crate::types::Context;
use crate::{BitMachine, Cmr, Value};

type Node<'brand> = Arc<ConstructNode<'brand>>;

/// A dummy but *structurally valid* spending context: one input (index 0), one
/// output, matching prevout utxo and a valid 33-byte control block. Building a
/// `BitcoinEnv` computes the sighash-all and tap-env-hash eagerly in C, so this
/// must not trip any C assertion.
fn dummy_env() -> BitcoinEnv<bitcoin::Transaction> {
    let tx = bitcoin::Transaction {
        version: bitcoin::transaction::Version(2),
        lock_time: bitcoin::absolute::LockTime::ZERO,
        input: vec![bitcoin::TxIn {
            previous_output: bitcoin::OutPoint::null(),
            script_sig: bitcoin::ScriptBuf::new(),
            sequence: bitcoin::Sequence::MAX,
            witness: bitcoin::Witness::new(),
        }],
        output: vec![bitcoin::TxOut {
            value: bitcoin::Amount::from_sat(0),
            script_pubkey: bitcoin::ScriptBuf::new(),
        }],
    };
    let utxo = bitcoin::TxOut {
        value: bitcoin::Amount::from_sat(0),
        script_pubkey: bitcoin::ScriptBuf::new(),
    };
    let cmr = Cmr::from_byte_array([0u8; 32]);
    let ctrl = ControlBlock::decode(&[
        0xc0, 0x50, 0x92, 0x9b, 0x74, 0xc1, 0xa0, 0x49, 0x54, 0xb7, 0x8b, 0x4b, 0x60, 0x35, 0xe9,
        0x7a, 0x5e, 0x07, 0x8a, 0x5a, 0x0f, 0x28, 0xec, 0x96, 0xd5, 0x47, 0xbf, 0xee, 0x9a, 0xce,
        0x80, 0x3a, 0xc0,
    ])
    .expect("control block");
    BitcoinEnv::new(tx, &[utxo], 0, cmr, ctrl)
}

/// Run a single-jet program (unit source) and return its output value.
fn run_jet(jet: &Bitcoin) -> Value {
    Context::with_context(|ctx| {
        let prog = Node::jet(&ctx, jet);
        let env = dummy_env();
        BitMachine::test_exec(prog, &env).expect("bit machine execution")
    })
}

/// `c_jet_env` must hand back the live transaction environment, not `()`.
#[test]
fn c_jet_env_is_the_tx_env() {
    let env = dummy_env();
    // Type is enforced at compile time: the trait method returns &CTxEnv.
    // We also confirm it points at the same struct the env exposes.
    let env_jet: &simplicity_sys::c_jets::c_env::bitcoin::CTxEnv =
        <BitcoinEnv<bitcoin::Transaction> as crate::jet::JetEnvironment>::c_jet_env(&env);
    assert_eq!(
        std::ptr::eq(env_jet, env.c_tx_env()),
        true,
        "c_jet_env should return the env's own CTxEnv"
    );
}

/// Every wired bitcoin env jet must yield a real C function pointer (i.e.
/// `c_jet_ptr` must not panic via `unimplemented!`) AND must execute
/// successfully against a live environment.
#[test]
fn env_jets_execute_with_real_environment() {
    // current_index (ONE -> 2^32): index of the input under scrutiny (0).
    assert_eq!(run_jet(&Bitcoin::CurrentIndex), Value::u32(0));
    // num_inputs (ONE -> 2^32): number of inputs (1).
    assert_eq!(run_jet(&Bitcoin::NumInputs), Value::u32(1));

    // tx_hash / tap_env_hash / sig_all_hash are ONE -> 2^256; just make sure
    // they execute (their C impls dereference the live txEnv, so a wrong/null
    // env would crash rather than return Ok).
    for jet in [Bitcoin::TxHash, Bitcoin::TapEnvHash, Bitcoin::SigAllHash] {
        let out = run_jet(&jet);
        assert_eq!(out.ty().to_string(), "2^256", "{jet:?} output type");
    }
}

/// bip_0340_verify is env-free on the bitcoin backend (C lives in the shared
/// jets-secp256k1.c); make sure its pointer resolves and it does not panic.
#[test]
fn bip_0340_verify_ptr_resolves() {
    let ptr = crate::jet::init::bitcoin::c_jet_ptr(&Bitcoin::Bip0340Verify);
    let ptr2 = crate::jet::init::bitcoin::c_jet_ptr(&Bitcoin::SigAllHash);
    let ptr3 = crate::jet::init::bitcoin::c_jet_ptr(&Bitcoin::CheckLockHeight);
    // Unwired jets still panic here; these must not.
    let _ = (ptr, ptr2, ptr3);
}

/// `check_lock_height` reads a 32-bit source and returns unit iff that height
/// is <= the transaction's lock height. Feed an in-range and an out-of-range
/// value and confirm the C jet consults the live environment.
#[test]
fn check_lock_height_executes_against_env() {
    use crate::node::SimpleFinalizer;

    let res_for = |x: u32| {
        Context::with_context(|ctx| {
            let prog = Node::jet(&ctx, &Bitcoin::CheckLockHeight); // 2^32 -> 1
            let prog = prog
                .finalize_types_non_program()
                .expect("finalizing types")
                .finalize(&mut SimpleFinalizer::new(None.into_iter()))
                .expect("finalizing");
            let env = dummy_env(); // lock_time = 0 => lock height 0
            let mut mac = BitMachine::for_program(&prog).expect("program bounds");
            mac.input(&Value::u32(x)).expect("input");
            mac.exec(&prog, &env)
        })
    };

    // 0 <= 0 -> Ok (unit), C returns true.
    assert_eq!(res_for(0).expect("should satisfy lock"), Value::unit());
    // 5 <= 0 is false -> C returns false -> JetFailed.
    assert!(
        res_for(5).is_err(),
        "height beyond lock should fail the jet"
    );
}
