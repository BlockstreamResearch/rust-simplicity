// Rust Simplicity Library
// Written in 2020 by
//   Andrew Poelstra <apoelstra@blockstream.com>
//
// To the extent possible under law, the author(s) have dedicated all
// copyright and related and neighboring rights to this software to
// the public domain worldwide. This software is distributed without
// any warranty.
//
// You should have received a copy of the CC0 Public Domain Dedication
// along with this software.
// If not, see <http://creativecommons.org/publicdomain/zero/1.0/>.
//

use crate::exec::BitMachine;
use crate::jet::application::elements::environment::{
    is_asset_new_issue, is_asset_reissue, ElementsEnv, SimplicityEncodable,
};
use crate::jet::application::elements::ElementsError;
use crate::jet::application::{Core, Elements};
use crate::jet::init::elements::ElementsJetName;
use crate::jet::{Application, JetNode};
use bitcoin_hashes::{sha256, Hash};
use elements::confidential::Value;
use std::convert::TryInto;

/// Execute an Elements jet on the Bit Machine
pub fn exec_jet(
    jet: &JetNode<Elements>,
    mac: &mut BitMachine,
    env: &ElementsEnv,
) -> Result<(), ElementsError> {
    // Environment must always be valid
    assert_eq!(env.tx.input.len(), env.utxos.len());

    let curr_idx = env.ix as usize;
    let curr_inp = &env.tx.input[curr_idx];
    let curr_utxo = &env.utxos[curr_idx];

    match jet.name {
        ElementsJetName::Version => mac.write_u32(env.tx.version),
        ElementsJetName::LockTime => mac.write_u32(env.tx.lock_time),
        ElementsJetName::InputIsPegin => {
            let idx = mac.read_u32() as usize;
            let is_valid_idx = idx < env.tx.input.len();
            mac.write_bit(is_valid_idx);
            if is_valid_idx {
                mac.write_bit(env.tx.input[idx].is_pegin);
            } else {
                mac.skip(1);
            }
        }
        ElementsJetName::InputPrevOutpoint => {
            let idx = mac.read_u32() as usize;
            let is_valid_idx = idx < env.tx.input.len();
            mac.write_bit(is_valid_idx);
            if is_valid_idx {
                mac.write_bytes(&env.tx.input[idx].previous_output.txid);
                mac.write_u32(env.tx.input[idx].previous_output.vout);
            } else {
                mac.skip(256 + 32);
            }
        }
        ElementsJetName::InputAsset => {
            let idx = mac.read_u32() as usize;
            let is_valid_idx = idx < env.tx.input.len();
            mac.write_bit(is_valid_idx);
            if is_valid_idx {
                let asset = env.utxos[idx].asset;
                asset.simplicity_encode(mac)?
            } else {
                // 2 bits for prefix and 256 bits for hash.
                mac.skip(2 + 256);
            }
        }
        ElementsJetName::InputAmount => {
            let idx = mac.read_u32() as usize;
            let is_valid_idx = idx < env.tx.input.len();
            mac.write_bit(is_valid_idx);
            if is_valid_idx {
                let amt = env.utxos[idx].value;
                amt.simplicity_encode(mac)?
            } else {
                // 2 bits for prefix and 256 bits for hash.
                mac.skip(2 + 256);
            }
        }
        ElementsJetName::InputScriptHash => {
            let idx = mac.read_u32() as usize;
            let is_valid_idx = idx < env.tx.input.len();
            mac.write_bit(is_valid_idx);
            if is_valid_idx {
                let script_pubkey = env.utxos[idx].script_pubkey;
                mac.write_bytes(&script_pubkey);
            } else {
                // 256 bits for hash.
                mac.skip(256);
            }
        }
        ElementsJetName::InputSequence => {
            let idx = mac.read_u32() as usize;
            let is_valid_idx = idx < env.tx.input.len();
            mac.write_bit(is_valid_idx);
            if is_valid_idx {
                let seq = env.tx.input[idx].sequence;
                mac.write_u32(seq);
            } else {
                // 32 bits for sequence.
                mac.skip(32);
            }
        }
        ElementsJetName::InputIssuanceBlinding => {
            let idx = mac.read_u32() as usize;
            let is_valid_idx = idx < env.tx.input.len();
            mac.write_bit(is_valid_idx);
            if is_valid_idx {
                assert!(env.tx.input[idx].has_issuance());
                blinding_issuance(mac, &env.tx.input[idx].asset_issuance);
            } else {
                // issuance_type + 256 hash bits.
                mac.skip(1 + 256);
            }
        }
        ElementsJetName::InputIssuanceContract => {
            let idx = mac.read_u32() as usize;
            let is_valid_idx = idx < env.tx.input.len();
            mac.write_bit(is_valid_idx);
            if is_valid_idx {
                assert!(env.tx.input[idx].has_issuance());
                contract_issuance(mac, &env.tx.input[idx].asset_issuance);
            } else {
                // issuance type + 256 bits for hash.
                mac.skip(1 + 256);
            }
        }
        ElementsJetName::InputIssuanceEntropy => {
            let idx = mac.read_u32() as usize;
            let is_valid_idx = idx < env.tx.input.len();
            mac.write_bit(is_valid_idx);
            if is_valid_idx {
                assert!(env.tx.input[idx].has_issuance());
                let asset = env.tx.input[idx].asset_issuance;
                entropy_issuance(mac, &asset);
            } else {
                // 1 + 256 bits for hash.
                mac.skip(1 + 256);
            }
        }
        ElementsJetName::InputIssuanceAssetAmount => {
            let idx = mac.read_u32() as usize;
            let is_valid_idx = idx < env.tx.input.len();
            mac.write_bit(is_valid_idx);
            if is_valid_idx {
                let asset = env.tx.input[idx].asset_issuance;
                asset_amt_issuance(mac, &asset, env.tx.input[idx].has_issuance())?
            } else {
                // 1 + 258 bits for conf value.
                mac.skip(1 + 258);
            }
        }
        ElementsJetName::InputIssuanceTokenAmount => {
            let idx = mac.read_u32() as usize;
            let is_valid_idx = idx < env.tx.input.len();
            mac.write_bit(is_valid_idx);
            if is_valid_idx {
                let asset = env.tx.input[idx].asset_issuance;
                inflation_amt_issuance(mac, &asset)?
            } else {
                // 1 + 258 bits for conf value.
                mac.skip(1 + 258);
            }
        }
        ElementsJetName::OutputAsset => {
            let idx = mac.read_u32() as usize;
            let is_valid_idx = idx < env.tx.output.len();
            mac.write_bit(is_valid_idx);
            if is_valid_idx {
                let asset = env.tx.output[idx].asset;
                asset.simplicity_encode(mac)?
            } else {
                // 258 bits for conf value.
                mac.skip(258);
            }
        }
        ElementsJetName::OutputAmount => {
            let idx = mac.read_u32() as usize;
            let is_valid_idx = idx < env.tx.output.len();
            mac.write_bit(is_valid_idx);
            if is_valid_idx {
                let value = env.tx.output[idx].value;
                value.simplicity_encode(mac)?
            } else {
                // 258 bits for conf value.
                mac.skip(258);
            }
        }
        ElementsJetName::OutputNonce => {
            let idx = mac.read_u32() as usize;
            let is_valid_idx = idx < env.tx.output.len();
            mac.write_bit(is_valid_idx);
            if is_valid_idx {
                let nonce = env.tx.output[idx].nonce;
                nonce.simplicity_encode(mac)?
            } else {
                // 259 bits for conf nonce.
                mac.skip(259);
            }
        }
        ElementsJetName::OutputScriptHash => {
            let idx = mac.read_u32() as usize;
            let is_valid_idx = idx < env.tx.output.len();
            mac.write_bit(is_valid_idx);
            if is_valid_idx {
                let output_script_pubkey = &env.tx.output[idx].script_pubkey;
                mac.write_bytes(&sha256::Hash::hash(&output_script_pubkey.to_bytes()));
            } else {
                // 256 bits of hash.
                mac.skip(256);
            }
        }
        ElementsJetName::OutputNullDatum => unimplemented!(),
        ElementsJetName::ScriptCmr => {
            mac.write_bytes(env.script_cmr.as_ref());
        }
        ElementsJetName::CurrentIndex => {
            mac.write_u32(env.ix);
        }
        ElementsJetName::CurrentIsPegin => {
            mac.write_bit(curr_inp.is_pegin());
        }
        ElementsJetName::CurrentPrevOutpoint => {
            mac.write_bytes(&curr_inp.previous_output.txid);
            mac.write_u32(curr_inp.previous_output.vout);
        }
        ElementsJetName::CurrentAsset => curr_utxo.asset.simplicity_encode(mac)?,
        ElementsJetName::CurrentAmount => curr_utxo.value.simplicity_encode(mac)?,
        ElementsJetName::CurrentScriptHash => {
            // TODO: cache these while creating utxo
            mac.write_bytes(&curr_utxo.script_pubkey);
        }
        ElementsJetName::CurrentSequence => {
            mac.write_u32(curr_inp.sequence);
        }
        ElementsJetName::CurrentIssuanceBlinding => {
            assert!(curr_inp.has_issuance());
            blinding_issuance(mac, &curr_inp.asset_issuance)
        }
        ElementsJetName::CurrentIssuanceContract => {
            assert!(curr_inp.has_issuance());
            contract_issuance(mac, &curr_inp.asset_issuance);
        }
        ElementsJetName::CurrentIssuanceEntropy => {
            assert!(curr_inp.has_issuance());
            entropy_issuance(mac, &curr_inp.asset_issuance);
        }
        ElementsJetName::CurrentIssuanceAssetAmount => {
            asset_amt_issuance(mac, &curr_inp.asset_issuance, curr_inp.has_issuance())?
        }
        ElementsJetName::CurrentIssuanceTokenAmount => {
            assert!(curr_inp.has_issuance());
            inflation_amt_issuance(mac, &curr_inp.asset_issuance)?
        }
        /*
        inputHash(l) :=
        BE256(LE[prevOutpoint.txid]),LE32(prevOutpoint.vout),LE32(sequence),encIssuance(l[issuance])
        */
        ElementsJetName::InputsHash => {
            mac.write_bytes(&env.inputs_hash);
        }
        ElementsJetName::OutputsHash => {
            mac.write_bytes(&env.outputs_hash);
        }
        ElementsJetName::NumInputs => {
            mac.write_u32(env.tx.input.len() as u32);
        }
        ElementsJetName::NumOutputs => {
            mac.write_u32(env.tx.output.len() as u32);
        }
        ElementsJetName::Fee => unimplemented!(),
        x => {
            let core_jet: &'static JetNode<Core> =
                x.try_into().expect("Convert Element jet into Core jet");
            Core::exec_jet(core_jet, mac, &())?;
        }
    };
    Ok(())
}

/// Write an optional 'blindingNonce' from an 'assetIssuance' when reissuing an asset;
/// writes 257 bits
fn blinding_issuance(mac: &mut BitMachine, issuance: &elements::AssetIssuance) {
    let is_reissue = is_asset_reissue(issuance);
    mac.write_bit(is_reissue);
    if is_reissue {
        mac.write_bytes(issuance.asset_blinding_nonce.as_ref());
    } else {
        mac.skip(256);
    }
}

/// Write an optional 'contractHash' from an 'assetIssuance' when issuing new asset
fn contract_issuance(mac: &mut BitMachine, issuance: &elements::AssetIssuance) {
    let is_new_issue = is_asset_new_issue(issuance);
    mac.write_bit(is_new_issue);
    if is_new_issue {
        mac.write_bytes(&issuance.asset_entropy);
    } else {
        mac.skip(256);
    }
}

/// Write an optional 'entropy' from an 'assetIssuance' when reissuing an asset
fn entropy_issuance(mac: &mut BitMachine, issuance: &elements::AssetIssuance) {
    let is_reissue = is_asset_reissue(issuance);
    mac.write_bit(is_reissue);
    if is_reissue {
        mac.write_bytes(&issuance.asset_entropy);
    } else {
        mac.skip(256);
    }
}

/// Write an optional confidential asset amount 'amount' from an 'assetIssuance'
fn asset_amt_issuance(
    mac: &mut BitMachine,
    issuance: &elements::AssetIssuance,
    has_issuance: bool,
) -> Result<(), ElementsError> {
    let is_null_amt = matches!(issuance.amount, Value::Null);
    mac.write_bit(has_issuance && !is_null_amt);
    if has_issuance {
        issuance.amount.simplicity_encode(mac)?;
    } else {
        // confidential value 258 bits
        mac.skip(2 + 256);
    }
    Ok(())
}

/// Write an optional confidential new token amount 'amount' from an 'assetIssuance'
fn inflation_amt_issuance(
    mac: &mut BitMachine,
    issuance: &elements::AssetIssuance,
) -> Result<(), ElementsError> {
    let is_null_amt = matches!(issuance.amount, Value::Null);
    let is_new_issue = is_asset_new_issue(issuance);
    mac.write_bit(is_new_issue && !is_null_amt);
    if is_new_issue {
        issuance.inflation_keys.simplicity_encode(mac)?;
    } else {
        // confidential value 258 bits
        mac.skip(2 + 256);
    }
    Ok(())
}
