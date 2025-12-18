// SPDX-License-Identifier: CC0-1.0

use crate::Cmr;

use bitcoin::hashes::Hash as _;
use bitcoin::taproot::ControlBlock;
use simplicity_sys::c_jets::c_env::bitcoin as c_bitcoin;

pub(super) fn new_tx(
    tx: &bitcoin::Transaction,
    in_utxos: &[bitcoin::TxOut],
) -> *mut c_bitcoin::CTransaction {
    let mut raw_inputs = Vec::with_capacity(tx.input.len());
    let mut raw_outputs = Vec::with_capacity(tx.output.len());
    // Allocate space for the raw annexes. This dumb `Vec::from_iter` construction is
    // equivalent to `vec![None; tx.input.len()]`, but that won't compile because it
    // requires Option::<CRawBuffer>::None to be cloneable, which it's not because
    // CRawBuffer isn't.

    // SAFETY: this allocation *must* live until after the `simplicity_mallocTransaction`
    //  at the bottom of this function. We convert the vector to a boxed slice to ensure
    //  it cannot be resized, which would potentially trigger a reallocation.
    let mut raw_annexes = Vec::from_iter((0..tx.input.len()).map(|_| None)).into_boxed_slice();

    for (n, (inp, utxo)) in tx.input.iter().zip(in_utxos.iter()).enumerate() {
        raw_annexes[n] = inp.witness.taproot_annex().map(c_bitcoin::CRawBuffer::new);
        raw_inputs.push(c_bitcoin::CRawInput {
            // This `as_ref().map_or()` construction converts an Option<&T> to a nullable *const T.
            // In theory it's a no-op.
            annex: raw_annexes[n]
                .as_ref()
                .map_or(core::ptr::null(), |ptr| ptr as *const _), // cast should be changed to ptr::from_ref in rust 1.76
            prev_txid: inp.previous_output.txid.as_byte_array(),
            txo: c_bitcoin::CRawOutput {
                value: utxo.value.to_sat(),
                script_pubkey: c_bitcoin::CRawBuffer::new(utxo.script_pubkey.as_bytes()),
            },
            script_sig: c_bitcoin::CRawBuffer::new(inp.script_sig.as_bytes()),
            prev_txout_index: inp.previous_output.vout,
            sequence: inp.sequence.to_consensus_u32(),
        });
    }
    for out in tx.output.iter() {
        raw_outputs.push(c_bitcoin::CRawOutput {
            value: out.value.to_sat(),
            script_pubkey: c_bitcoin::CRawBuffer::new(out.script_pubkey.as_bytes()),
        });
    }
    let txid = tx.compute_txid();

    let c_raw_tx = c_bitcoin::CRawTransaction {
        txid: txid.as_byte_array(),
        inputs: raw_inputs.as_ptr(),
        outputs: raw_outputs.as_ptr(),
        n_inputs: raw_inputs.len().try_into().expect("sane length"),
        n_outputs: raw_outputs.len().try_into().expect("sane length"),
        version: tx.version.0 as u32, // in 1.87.0 can use .cast_unsigned
        locktime: tx.lock_time.to_consensus_u32(),
    };
    let ret = unsafe {
        // SAFETY: this is a FFI call and we constructed its argument correctly.
        c_bitcoin::simplicity_mallocTransaction(&c_raw_tx)
    };
    // Explicitly drop raw_annexes so Rust doesn't try any funny business dropping it early.
    // Drop raw_inputs first since it contains pointers into raw_annexes and we don't want
    // them to dangle. (It'd be safe since they're raw pointers, but still bad mojo.)
    drop(raw_inputs);
    drop(raw_annexes);
    ret
}

pub(super) fn new_tap_env(
    control_block: &ControlBlock,
    script_cmr: Cmr,
) -> *mut c_bitcoin::CTapEnv {
    let cb_ser = control_block.serialize();
    let raw_tap_env = c_bitcoin::CRawTapEnv {
        control_block: cb_ser.as_ptr(),
        script_cmr: script_cmr.as_ref().as_ptr(),
        branch_len: control_block
            .merkle_branch
            .len()
            .try_into()
            .expect("sane length"),
    };

    unsafe {
        // SAFETY: this is a FFI call and we constructed its argument correctly.
        c_bitcoin::simplicity_mallocTapEnv(&raw_tap_env)
    }
}

pub(super) fn new_tx_env(
    tx: *const c_bitcoin::CTransaction,
    taproot: *const c_bitcoin::CTapEnv,
    ix: u32,
) -> c_bitcoin::CTxEnv {
    unsafe {
        let mut tx_env = std::mem::MaybeUninit::<c_bitcoin::CTxEnv>::uninit();
        c_bitcoin::c_set_txEnv(tx_env.as_mut_ptr(), tx, taproot, ix);
        tx_env.assume_init()
    }
}
