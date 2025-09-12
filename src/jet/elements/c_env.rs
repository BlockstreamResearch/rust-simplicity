// SPDX-License-Identifier: CC0-1.0

//! High level APIs for creating C FFI compatible environment.
//!

use elements::secp256k1_zkp::ffi::CPtr;
use std::os::raw::{c_uchar, c_uint};

use elements::{
    confidential,
    encode::serialize,
    secp256k1_zkp::{RangeProof, SurjectionProof},
    taproot::ControlBlock,
};
use simplicity_sys::c_jets::c_env::elements as c_elements;

use crate::merkle::cmr::Cmr;

use super::ElementsUtxo;

fn new_raw_output(
    out: &elements::TxOut,
    out_data: &c_elements::RawOutputData,
) -> c_elements::CRawOutput {
    unsafe {
        let mut raw_output = std::mem::MaybeUninit::<c_elements::CRawOutput>::uninit();
        c_elements::c_set_rawOutput(
            raw_output.as_mut_ptr(),
            asset_ptr(out.asset, &out_data.asset),
            value_ptr(out.value, &out_data.value),
            nonce_ptr(out.nonce, &out_data.nonce),
            &script_ptr(&out.script_pubkey),
            &surjection_proof_ptr(&out_data.surjection_proof),
            &range_proof_ptr(&out_data.range_proof),
        );
        raw_output.assume_init()
    }
}

fn new_raw_input(
    inp: &elements::TxIn,
    in_utxo: &ElementsUtxo,
    inp_data: &c_elements::RawInputData,
) -> c_elements::CRawInput {
    unsafe {
        let mut raw_input = std::mem::MaybeUninit::<c_elements::CRawInput>::uninit();

        let (issue_nonce_ptr, issue_entropy_ptr, issue_amt_ptr, issue_infl_key_ptr) =
            if inp.has_issuance() {
                (
                    inp.asset_issuance.asset_blinding_nonce.as_c_ptr(),
                    inp.asset_issuance.asset_entropy.as_ptr(),
                    value_ptr(inp.asset_issuance.amount, &inp_data.issuance_amount),
                    value_ptr(
                        inp.asset_issuance.inflation_keys,
                        &inp_data.issuance_inflation_keys,
                    ),
                )
            } else {
                (
                    std::ptr::null(),
                    std::ptr::null(),
                    std::ptr::null(),
                    std::ptr::null(),
                )
            };
        c_elements::c_set_rawInput(
            raw_input.as_mut_ptr(),
            opt_ptr(annex_ptr(&inp_data.annex).as_ref()),
            inp.pegin_data()
                .map(|x| AsRef::<[u8]>::as_ref(&x.genesis_hash).as_ptr())
                .unwrap_or(std::ptr::null()),
            &script_ptr(&inp.script_sig),
            AsRef::<[u8]>::as_ref(&inp.previous_output.txid).as_ptr(),
            inp.previous_output.vout as c_uint,
            asset_ptr(in_utxo.asset, &inp_data.asset),
            value_ptr(in_utxo.value, &inp_data.value),
            &script_ptr(&in_utxo.script_pubkey),
            inp.sequence.0 as c_uint,
            issue_nonce_ptr, // FIXME: CHECK ASSET ISSUANCE IS NOT NULL. EASIER WITH NEW ELEMENTS VERSION.
            issue_entropy_ptr,
            issue_amt_ptr,
            issue_infl_key_ptr,
            &range_proof_ptr(&inp_data.amount_range_proof),
            &range_proof_ptr(&inp_data.inflation_keys_range_proof),
        );
        raw_input.assume_init()
    }
}

fn new_tx_data(
    tx: &elements::Transaction,
    in_utxos: &[ElementsUtxo],
) -> c_elements::RawTransactionData {
    let mut tx_data = c_elements::RawTransactionData {
        inputs: Vec::with_capacity(tx.input.len()),
        outputs: Vec::with_capacity(tx.output.len()),
    };
    for (inp, in_utxo) in tx.input.iter().zip(in_utxos.iter()) {
        let inp_data = c_elements::RawInputData {
            annex: None, // Actually store annex
            issuance_amount: serialize(&inp.asset_issuance.amount),
            issuance_inflation_keys: serialize(&inp.asset_issuance.inflation_keys),
            amount_range_proof: serialize_rangeproof(&inp.witness.amount_rangeproof),
            inflation_keys_range_proof: serialize_rangeproof(
                &inp.witness.inflation_keys_rangeproof,
            ),
            asset: serialize(&in_utxo.asset),
            value: serialize(&in_utxo.value),
        };
        tx_data.inputs.push(inp_data);
    }
    for out in &tx.output {
        let out_data = c_elements::RawOutputData {
            asset: serialize(&out.asset),
            value: serialize(&out.value),
            nonce: serialize(&out.nonce),
            surjection_proof: serialize_surjection_proof(&out.witness.surjection_proof),
            range_proof: serialize_rangeproof(&out.witness.rangeproof),
        };
        tx_data.outputs.push(out_data);
    }
    tx_data
}

pub(super) fn new_tx(
    tx: &elements::Transaction,
    in_utxos: &[ElementsUtxo],
) -> *mut c_elements::CTransaction {
    let mut raw_inputs = Vec::new();
    let mut raw_outputs = Vec::new();
    let txid = tx.txid();
    let tx_data = new_tx_data(tx, in_utxos);
    for ((inp, in_utxo), inp_data) in tx
        .input
        .iter()
        .zip(in_utxos.iter())
        .zip(tx_data.inputs.iter())
    {
        let res = new_raw_input(inp, in_utxo, inp_data);
        raw_inputs.push(res);
    }
    for (out, out_data) in tx.output.iter().zip(tx_data.outputs.iter()) {
        raw_outputs.push(new_raw_output(out, out_data));
    }
    unsafe {
        let mut raw_tx = std::mem::MaybeUninit::<c_elements::CRawTransaction>::uninit();
        c_elements::c_set_rawTransaction(
            raw_tx.as_mut_ptr(),
            tx.version as c_uint,
            AsRef::<[u8]>::as_ref(&txid).as_ptr(),
            raw_inputs.as_ptr(),
            raw_inputs.len() as c_uint,
            raw_outputs.as_ptr(),
            raw_outputs.len() as c_uint,
            tx.lock_time.to_consensus_u32() as c_uint,
        );
        let raw_tx = raw_tx.assume_init();
        c_elements::simplicity_mallocTransaction(&raw_tx)
    }
}

pub(super) fn new_tap_env(
    control_block: &ControlBlock,
    script_cmr: Cmr,
) -> *mut c_elements::CTapEnv {
    unsafe {
        let mut raw_tap_env = std::mem::MaybeUninit::<c_elements::CRawTapEnv>::uninit();
        let cb_ser = control_block.serialize();
        c_elements::c_set_rawTapEnv(
            raw_tap_env.as_mut_ptr(),
            cb_ser.as_ptr(),
            control_block.merkle_branch.as_inner().len() as c_uchar,
            script_cmr.as_ref().as_ptr(),
        );
        let raw_tap_env = raw_tap_env.assume_init();
        c_elements::simplicity_mallocTapEnv(&raw_tap_env)
    }
}

pub(super) fn new_tx_env(
    tx: *const c_elements::CTransaction,
    taproot: *const c_elements::CTapEnv,
    genesis_hash: elements::BlockHash,
    ix: u32,
) -> c_elements::CTxEnv {
    unsafe {
        let mut tx_env = std::mem::MaybeUninit::<c_elements::CTxEnv>::uninit();
        c_elements::c_set_txEnv(
            tx_env.as_mut_ptr(),
            tx,
            taproot,
            AsRef::<[u8]>::as_ref(&genesis_hash).as_ptr(),
            ix,
        );
        tx_env.assume_init()
    }
}

fn asset_ptr(asset: confidential::Asset, data: &[u8]) -> *const c_uchar {
    if asset.is_null() {
        std::ptr::null()
    } else {
        data.as_ptr()
    }
}

fn value_ptr(value: confidential::Value, data: &[u8]) -> *const c_uchar {
    if value.is_null() {
        std::ptr::null()
    } else {
        data.as_ptr()
    }
}

fn nonce_ptr(nonce: confidential::Nonce, data: &[u8]) -> *const c_uchar {
    if nonce.is_null() {
        std::ptr::null()
    } else {
        data.as_ptr()
    }
}

fn opt_ptr<T>(t: Option<&T>) -> *const T {
    if let Some(t) = t {
        t
    } else {
        std::ptr::null()
    }
}

fn script_ptr(script: &elements::Script) -> c_elements::CRawBuffer {
    c_elements::CRawBuffer::new(script.as_bytes())
}

fn annex_ptr(annex: &Option<Vec<c_uchar>>) -> Option<c_elements::CRawBuffer> {
    annex
        .as_ref()
        .map(|annex| c_elements::CRawBuffer::new(annex))
}

fn surjection_proof_ptr(surjection_proof: &[c_uchar]) -> c_elements::CRawBuffer {
    c_elements::CRawBuffer::new(surjection_proof)
}

fn range_proof_ptr(rangeproof: &[c_uchar]) -> c_elements::CRawBuffer {
    c_elements::CRawBuffer::new(rangeproof)
}

fn serialize_rangeproof(rangeproof: &Option<Box<RangeProof>>) -> Vec<c_uchar> {
    rangeproof
        .as_ref()
        .map(|x| x.serialize())
        .unwrap_or_default()
}

fn serialize_surjection_proof(surjection_proof: &Option<Box<SurjectionProof>>) -> Vec<c_uchar> {
    surjection_proof
        .as_ref()
        .map(|x| x.serialize())
        .unwrap_or_default()
}
