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

// #[test]
// fn sighash_all_cmr() {
//     let mut bits: BitIter<_> = sighash_all::ELEMENTS_CHECK_SIGHASH_ALL
//         .iter()
//         .cloned()
//         .into();
//     let program = RedeemNode::<Elements>::decode(&mut bits).expect("decoding program");

//     assert_eq!(program.cmr.into_inner(), sighash_all::SIGHASH_ALL_CMR);
//     // TODO: check IMR
// }

/*
TODO: Add c_jet_ptr for used jets and re-enable tests

#[test]
#[ignore] // too expensive to run. Run with -- --ignored to run this
fn exec_sighash_all() {
    /*
    rawTransaction testTx1 = (rawTransaction)
      { .input = (rawInput[])
                 { { .prevTxid = (unsigned char[32]){"\xeb\x04\xb6\x8e\x9a\x26\xd1\x16\x04\x6c\x76\xe8\xff\x47\x33\x2f\xb7\x1d\xda\x90\xff\x4b\xef\x53\x70\xf2\x52\x26\xd3\xbc\x09\xfc"}
                   , .prevIx = 0
                   , .sequence = 0xfffffffe
                   , .isPegin = false
                   , .issuance = {0}
                   , .txo = { .asset = (unsigned char[33]){"\x01\x23\x0f\x4f\x5d\x4b\x7c\x6f\xa8\x45\x80\x6e\xe4\xf6\x77\x13\x45\x9e\x1b\x69\xe8\xe6\x0f\xce\xe2\xe4\x94\x0c\x7a\x0d\x5d\xe1\xb2"}
                            , .value = (unsigned char[9]){"\x01\x00\x00\x00\x02\x54\x0b\xe4\x00"}
                            , .scriptPubKey = {0}
                 } }        }
      , .output = (rawOutput[])
                  { { .asset = (unsigned char[33]){"\x01\x23\x0f\x4f\x5d\x4b\x7c\x6f\xa8\x45\x80\x6e\xe4\xf6\x77\x13\x45\x9e\x1b\x69\xe8\xe6\x0f\xce\xe2\xe4\x94\x0c\x7a\x0d\x5d\xe1\xb2"}
                    , .value = (unsigned char[9]){"\x01\x00\x00\x00\x02\x54\x0b\xd7\x1c"}
                    , .nonce = NULL
                    , .scriptPubKey = { .code = (unsigned char [26]){"\x19\x76\xa9\x14\x48\x63\x3e\x2c\x0e\xe9\x49\x5d\xd3\xf9\xc4\x37\x32\xc4\x7f\x47\x02\xa3\x62\xc8\x88\xac"}
                                      , .len = 26
                                      }
                    }
                  , { .asset = (unsigned char[33]){"\x01\x23\x0f\x4f\x5d\x4b\x7c\x6f\xa8\x45\x80\x6e\xe4\xf6\x77\x13\x45\x9e\x1b\x69\xe8\xe6\x0f\xce\xe2\xe4\x94\x0c\x7a\x0d\x5d\xe1\xb2"}
                    , .value = (unsigned char[9]){"\x01\x00\x00\x00\x00\x00\x00\x0c\xe4"}
                    , .nonce = NULL
                    , .scriptPubKey = {0}
                  } }
      , .numInputs = 1
      , .numOutputs = 2
      , .version = 0x00000002
      , .lockTime = 0x00000000
      };
    */
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
        lock_time: PackedLockTime::ZERO,
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
                script_witness: vec![],
                pegin_witness: vec![],
            },
        }],
        output: vec![
            TxOut {
                asset: asset.clone(),
                value: confidential::Value::Explicit(0x00000002540bd71c),
                nonce: confidential::Nonce::Null,
                script_pubkey: hex_script(&"1976a91448633e2c0ee9495dd3f9c43732c47f4702a362c888ac"),
                witness: TxOutWitness {
                    surjection_proof: None,
                    rangeproof: None,
                },
            },
            TxOut {
                asset: asset.clone(),
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
    let script_cmr = Cmr::from(sighash_all::SIGHASH_ALL_CMR);
    let env = ElementsEnv::new(
        Arc::new(tx),
        vec![utxo],
        0,
        script_cmr,
        ctrl_block,
        None,
        BlockHash::all_zeros(),
    );

    let mut bits: BitIter<_> = sighash_all::ELEMENTS_CHECK_SIGHASH_ALL
        .iter()
        .cloned()
        .into();
    let program = RedeemNode::<Elements>::decode(&mut bits).expect("decoding program");

    let mut mac = BitMachine::for_program(&program);
    mac.exec(&program, &env).unwrap();
}
*/

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
                asset: asset.clone(),
                value: confidential::Value::Explicit(0x00000002540bd71c),
                nonce: confidential::Nonce::Null,
                script_pubkey: hex_script(&"1976a91448633e2c0ee9495dd3f9c43732c47f4702a362c888ac"),
                witness: TxOutWitness {
                    surjection_proof: None,
                    rangeproof: None,
                },
            },
            TxOut {
                asset: asset.clone(),
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
    let script_cmr = Cmr::from([0; 32]);
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
