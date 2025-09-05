// SPDX-License-Identifier: CC0-1.0

//! High level APIs for creating C FFI compatible environment.
//!

use hashes::Hash;
use std::os::raw::c_uchar;

use elements::{
    confidential,
    encode::serialize,
    secp256k1_zkp::{RangeProof, SurjectionProof},
    taproot::ControlBlock,
};
use simplicity_sys::c_jets::c_env::elements as c_elements;

use crate::merkle::cmr::Cmr;

use super::ElementsUtxo;

fn new_raw_output<'raw>(
    out: &elements::TxOut,
    out_data: &'raw c_elements::RawOutputData,
) -> c_elements::CRawOutput<'raw> {
    c_elements::CRawOutput {
        asset: out_data.asset.as_ref(),
        value: value_ptr(out.value, &out_data.value),
        nonce: out_data.nonce.as_ref(),
        script_pubkey: c_elements::CRawBuffer::new(out.script_pubkey.as_bytes()),
        surjection_proof: c_elements::CRawBuffer::new(&out_data.surjection_proof),
        range_proof: c_elements::CRawBuffer::new(&out_data.range_proof),
    }
}

fn new_raw_input<'raw>(
    inp: &'raw elements::TxIn,
    in_utxo: &'raw ElementsUtxo,
    inp_data: &'raw c_elements::RawInputData,
) -> c_elements::CRawInput<'raw> {
    c_elements::CRawInput {
        // FIXME actually pass the annex in; see https://github.com/BlockstreamResearch/simplicity/issues/311 for some difficulty here.
        annex: core::ptr::null(),
        prev_txid: inp.previous_output.txid.as_ref(),
        pegin: inp_data.genesis_hash.as_ref(),
        issuance: if inp.has_issuance() {
            c_elements::CRawInputIssuance {
                blinding_nonce: Some(inp.asset_issuance.asset_blinding_nonce.as_ref()),
                asset_entropy: Some(&inp.asset_issuance.asset_entropy),
                amount: value_ptr(inp.asset_issuance.amount, &inp_data.issuance_amount),
                inflation_keys: value_ptr(
                    inp.asset_issuance.inflation_keys,
                    &inp_data.issuance_inflation_keys,
                ),
                amount_range_proof: c_elements::CRawBuffer::new(&inp_data.amount_range_proof),
                inflation_keys_range_proof: c_elements::CRawBuffer::new(
                    &inp_data.inflation_keys_range_proof,
                ),
            }
        } else {
            c_elements::CRawInputIssuance::no_issuance()
        },
        txo: c_elements::CRawInputTxo {
            asset: inp_data.asset.as_ref(),
            value: value_ptr(in_utxo.value, &inp_data.value),
            script_pubkey: c_elements::CRawBuffer::new(in_utxo.script_pubkey.as_bytes()),
        },
        script_sig: c_elements::CRawBuffer::new(inp.script_sig.as_bytes()),
        prev_txout_index: inp.previous_output.vout,
        sequence: inp.sequence.to_consensus_u32(),
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
            genesis_hash: inp
                .pegin_data()
                .map(|x| x.genesis_hash.to_raw_hash().to_byte_array()),
            issuance_amount: serialize(&inp.asset_issuance.amount),
            issuance_inflation_keys: serialize(&inp.asset_issuance.inflation_keys),
            amount_range_proof: serialize_rangeproof(&inp.witness.amount_rangeproof),
            inflation_keys_range_proof: serialize_rangeproof(
                &inp.witness.inflation_keys_rangeproof,
            ),
            asset: asset_array(&in_utxo.asset),
            value: serialize(&in_utxo.value),
        };
        tx_data.inputs.push(inp_data);
    }
    for out in &tx.output {
        let out_data = c_elements::RawOutputData {
            asset: asset_array(&out.asset),
            value: serialize(&out.value),
            nonce: nonce_array(&out.nonce),
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

    let c_raw_tx = c_elements::CRawTransaction {
        txid: txid.as_raw_hash().as_byte_array(),
        inputs: raw_inputs.as_ptr(),
        outputs: raw_outputs.as_ptr(),
        n_inputs: raw_inputs.len().try_into().expect("sane length"),
        n_outputs: raw_outputs.len().try_into().expect("sane length"),
        version: tx.version,
        locktime: tx.lock_time.to_consensus_u32(),
    };
    unsafe {
        // SAFETY: this is a FFI call and we constructed its argument correctly.
        c_elements::simplicity_mallocTransaction(&c_raw_tx)
    }
}

pub(super) fn new_tap_env(
    control_block: &ControlBlock,
    script_cmr: Cmr,
) -> *mut c_elements::CTapEnv {
    let cb_ser = control_block.serialize();
    let raw_tap_env = c_elements::CRawTapEnv {
        control_block: cb_ser.as_ptr(),
        script_cmr: script_cmr.as_ref().as_ptr(),
        branch_len: control_block
            .merkle_branch
            .as_inner()
            .len()
            .try_into()
            .expect("sane length"),
    };

    unsafe {
        // SAFETY: this is a FFI call and we constructed its argument correctly.
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

fn asset_array(asset: &confidential::Asset) -> Option<[u8; 33]> {
    (!asset.is_null()).then(|| {
        serialize(asset)
            .try_into()
            .expect("non-null asset is 33 bytes")
    })
}

fn nonce_array(nonce: &confidential::Nonce) -> Option<[u8; 33]> {
    (!nonce.is_null()).then(|| {
        serialize(nonce)
            .try_into()
            .expect("non-null asset is 33 bytes")
    })
}

fn value_ptr(value: confidential::Value, data: &[u8]) -> *const c_uchar {
    if value.is_null() {
        std::ptr::null()
    } else {
        data.as_ptr()
    }
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
