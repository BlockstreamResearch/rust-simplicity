use std::sync::Arc;

use crate::exec::BitMachine;
use crate::jet::elements::{ElementsEnv, ElementsUtxo};
use crate::jet::{Elements, Jet};
use crate::merkle::cmr::Cmr;
use bitcoin_hashes::sha256::Midstate;
use bitcoin_hashes::Hash;
use elements::secp256k1_zkp::Tweak;
use elements::taproot::ControlBlock;
use elements::{
    confidential, AssetId, AssetIssuance, BlockHash, OutPoint, PackedLockTime, Sequence,
    Transaction, TxIn, TxInWitness, TxOut, TxOutWitness,
};

#[test]
fn test_ffi_env() {
    let asset: [u8; 32] = [
        0x23, 0x0f, 0x4f, 0x5d, 0x4b, 0x7c, 0x6f, 0xa8, 0x45, 0x80, 0x6e, 0xe4, 0xf6, 0x77, 0x13,
        0x45, 0x9e, 0x1b, 0x69, 0xe8, 0xe6, 0x0f, 0xce, 0xe2, 0xe4, 0x94, 0x0c, 0x7a, 0x0d, 0x5d,
        0xe1, 0xb2,
    ];
    let tx_id: [u8; 32] = [
        0xeb, 0x04, 0xb6, 0x8e, 0x9a, 0x26, 0xd1, 0x16, 0x04, 0x6c, 0x76, 0xe8, 0xff, 0x47, 0x33,
        0x2f, 0xb7, 0x1d, 0xda, 0x90, 0xff, 0x4b, 0xef, 0x53, 0x70, 0xf2, 0x52, 0x26, 0xd3, 0xbc,
        0x09, 0xfc,
    ];
    let ctrl_blk: [u8; 33] = [
        0xc0, 0xeb, 0x04, 0xb6, 0x8e, 0x9a, 0x26, 0xd1, 0x16, 0x04, 0x6c, 0x76, 0xe8, 0xff, 0x47,
        0x33, 0x2f, 0xb7, 0x1d, 0xda, 0x90, 0xff, 0x4b, 0xef, 0x53, 0x70, 0xf2, 0x52, 0x26, 0xd3,
        0xbc, 0x09, 0xfc,
    ];
    let asset = confidential::Asset::Explicit(AssetId::from_inner(Midstate::from_inner(asset)));
    let tx = Transaction {
        version: 2,
        lock_time: PackedLockTime(100),
        input: vec![TxIn {
            previous_output: OutPoint {
                txid: elements::Txid::from_inner(tx_id),
                vout: 0,
            },
            sequence: Sequence::ENABLE_LOCKTIME_NO_RBF,
            is_pegin: false,
            // perhaps make this an option in elements upstream?
            asset_issuance: AssetIssuance {
                asset_blinding_nonce: Tweak::from_inner([0; 32]).expect("tweak from inner"),
                asset_entropy: [0; 32],
                amount: confidential::Value::Null,
                inflation_keys: confidential::Value::Null,
            },
            script_sig: elements::Script::new(),
            witness: TxInWitness {
                amount_rangeproof: None,
                inflation_keys_rangeproof: None,
                script_witness: vec![ctrl_blk.to_vec()],
                pegin_witness: vec![],
            },
        }],
        output: vec![
            TxOut {
                asset,
                value: confidential::Value::Explicit(0x00000002540bd71c),
                nonce: confidential::Nonce::Null,
                script_pubkey: hex_script("1976a91448633e2c0ee9495dd3f9c43732c47f4702a362c888ac"),
                witness: TxOutWitness {
                    surjection_proof: None,
                    rangeproof: None,
                },
            },
            TxOut {
                asset,
                value: confidential::Value::Explicit(0x0000000000000ce4),
                nonce: confidential::Nonce::Null,
                script_pubkey: elements::Script::new(),
                witness: TxOutWitness {
                    surjection_proof: None,
                    rangeproof: None,
                },
            },
        ],
    };
    let utxo = ElementsUtxo {
        script_pubkey: elements::Script::new(),
        asset,
        value: confidential::Value::Explicit(0x00000002540be400),
    };
    let ctrl_block = ControlBlock::from_slice(&ctrl_blk).expect("ctrl block from slice");
    let script_cmr = Cmr::from_byte_array([0; 32]);
    let env = ElementsEnv::new(
        Arc::new(tx),
        vec![utxo],
        0,
        script_cmr,
        ctrl_block,
        None,
        BlockHash::all_zeros(),
    );

    let mut mac = BitMachine {
        data: vec![0; 1000],
        next_frame_start: 0,
        read: Vec::with_capacity(5),
        write: Vec::with_capacity(5),
    };
    let jet = Elements::LockTime;
    mac.new_frame(100);
    mac.write_u32(2);
    mac.move_frame();
    mac.new_frame(35);
    jet.exec(&mut mac, &env).unwrap();
    mac.move_frame();
    let res = mac.read_u32();
    assert_eq!(res, 100);
}

fn hex_script(s: &str) -> elements::Script {
    let v: Vec<u8> = bitcoin_hashes::hex::FromHex::from_hex(s).unwrap();
    elements::Script::from(v)
}
