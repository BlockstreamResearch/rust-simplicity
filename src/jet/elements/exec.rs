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
use crate::jet;
use crate::jet::elements::environment::{
    is_asset_new_issue, is_asset_reissue, ElementsEnv, SimplicityEncodable,
};
use crate::jet::JetFailed;
use bitcoin_hashes::{sha256, Hash};
use elements::confidential::Value;

pub(crate) fn version(mac: &mut BitMachine, env: &ElementsEnv) -> Result<(), JetFailed> {
    mac.write_u32(env.tx.version);
    Ok(())
}

pub(crate) fn lock_time(mac: &mut BitMachine, env: &ElementsEnv) -> Result<(), JetFailed> {
    mac.write_u32(env.tx.lock_time);
    Ok(())
}

pub(crate) fn input_is_pegin(mac: &mut BitMachine, env: &ElementsEnv) -> Result<(), JetFailed> {
    let idx = mac.read_u32() as usize;
    let is_valid_idx = idx < env.tx.input.len();
    mac.write_bit(is_valid_idx);

    if is_valid_idx {
        mac.write_bit(env.tx.input[idx].is_pegin);
    } else {
        mac.skip(1);
    }

    Ok(())
}

pub(crate) fn input_prev_outpoint(
    mac: &mut BitMachine,
    env: &ElementsEnv,
) -> Result<(), JetFailed> {
    let idx = mac.read_u32() as usize;
    let is_valid_idx = idx < env.tx.input.len();
    mac.write_bit(is_valid_idx);

    if is_valid_idx {
        mac.write_bytes(&env.tx.input[idx].previous_output.txid);
        mac.write_u32(env.tx.input[idx].previous_output.vout);
    } else {
        mac.skip(256 + 32);
    }

    Ok(())
}

pub(crate) fn input_asset(mac: &mut BitMachine, env: &ElementsEnv) -> Result<(), JetFailed> {
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

    Ok(())
}

pub(crate) fn input_amount(mac: &mut BitMachine, env: &ElementsEnv) -> Result<(), JetFailed> {
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

    Ok(())
}

pub(crate) fn input_script_hash(mac: &mut BitMachine, env: &ElementsEnv) -> Result<(), JetFailed> {
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

    Ok(())
}

pub(crate) fn input_sequence(mac: &mut BitMachine, env: &ElementsEnv) -> Result<(), JetFailed> {
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

    Ok(())
}

pub(crate) fn input_issuance_blinding(
    mac: &mut BitMachine,
    env: &ElementsEnv,
) -> Result<(), JetFailed> {
    let idx = mac.read_u32() as usize;
    let is_valid_idx = idx < env.tx.input.len();
    mac.write_bit(is_valid_idx);

    if is_valid_idx {
        if !env.tx.input[idx].has_issuance() {
            return Err(JetFailed);
        }
        blinding_issuance(mac, &env.tx.input[idx].asset_issuance);
    } else {
        // issuance_type + 256 hash bits.
        mac.skip(1 + 256);
    }

    Ok(())
}

pub(crate) fn input_issuance_contract(
    mac: &mut BitMachine,
    env: &ElementsEnv,
) -> Result<(), JetFailed> {
    let idx = mac.read_u32() as usize;
    let is_valid_idx = idx < env.tx.input.len();
    mac.write_bit(is_valid_idx);

    if is_valid_idx {
        if !env.tx.input[idx].has_issuance() {
            return Err(JetFailed);
        }
        contract_issuance(mac, &env.tx.input[idx].asset_issuance);
    } else {
        // issuance type + 256 bits for hash.
        mac.skip(1 + 256);
    }

    Ok(())
}

pub(crate) fn input_issuance_entropy(
    mac: &mut BitMachine,
    env: &ElementsEnv,
) -> Result<(), JetFailed> {
    let idx = mac.read_u32() as usize;
    let is_valid_idx = idx < env.tx.input.len();
    mac.write_bit(is_valid_idx);

    if is_valid_idx {
        if !env.tx.input[idx].has_issuance() {
            return Err(JetFailed);
        }
        let asset = env.tx.input[idx].asset_issuance;
        entropy_issuance(mac, &asset);
    } else {
        // 1 + 256 bits for hash.
        mac.skip(1 + 256);
    }

    Ok(())
}

pub(crate) fn input_issuance_asset_amount(
    mac: &mut BitMachine,
    env: &ElementsEnv,
) -> Result<(), JetFailed> {
    let idx = mac.read_u32() as usize;
    let is_valid_idx = idx < env.tx.input.len();
    mac.write_bit(is_valid_idx);

    if is_valid_idx {
        let asset = env.tx.input[idx].asset_issuance;
        asset_amt_issuance(mac, &asset, env.tx.input[idx].has_issuance())?;
    } else {
        // 1 + 258 bits for conf value.
        mac.skip(1 + 258);
    }

    Ok(())
}

pub(crate) fn input_issuance_token_amount(
    mac: &mut BitMachine,
    env: &ElementsEnv,
) -> Result<(), JetFailed> {
    let idx = mac.read_u32() as usize;
    let is_valid_idx = idx < env.tx.input.len();
    mac.write_bit(is_valid_idx);

    if is_valid_idx {
        let asset = env.tx.input[idx].asset_issuance;
        inflation_amt_issuance(mac, &asset)?;
    } else {
        // 1 + 258 bits for conf value.
        mac.skip(1 + 258);
    }

    Ok(())
}

pub(crate) fn output_asset(mac: &mut BitMachine, env: &ElementsEnv) -> Result<(), JetFailed> {
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

    Ok(())
}

pub(crate) fn output_amount(mac: &mut BitMachine, env: &ElementsEnv) -> Result<(), JetFailed> {
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

    Ok(())
}

pub(crate) fn output_nonce(mac: &mut BitMachine, env: &ElementsEnv) -> Result<(), JetFailed> {
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

    Ok(())
}

pub(crate) fn output_script_hash(mac: &mut BitMachine, env: &ElementsEnv) -> Result<(), JetFailed> {
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

    Ok(())
}

pub(crate) fn script_cmr(mac: &mut BitMachine, env: &ElementsEnv) -> Result<(), JetFailed> {
    mac.write_bytes(env.script_cmr.as_ref());
    Ok(())
}

pub(crate) fn current_index(mac: &mut BitMachine, env: &ElementsEnv) -> Result<(), JetFailed> {
    mac.write_u32(env.ix);
    Ok(())
}

pub(crate) fn current_is_pegin(mac: &mut BitMachine, env: &ElementsEnv) -> Result<(), JetFailed> {
    let curr_idx = env.ix as usize;
    let curr_inp = &env.tx.input[curr_idx];
    mac.write_bit(curr_inp.is_pegin());
    Ok(())
}

pub(crate) fn current_prev_outpoint(
    mac: &mut BitMachine,
    env: &ElementsEnv,
) -> Result<(), JetFailed> {
    let curr_idx = env.ix as usize;
    let curr_inp = &env.tx.input[curr_idx];
    mac.write_bytes(&curr_inp.previous_output.txid);
    mac.write_u32(curr_inp.previous_output.vout);
    Ok(())
}

pub(crate) fn current_asset(mac: &mut BitMachine, env: &ElementsEnv) -> Result<(), JetFailed> {
    let curr_idx = env.ix as usize;
    let curr_utxo = &env.utxos[curr_idx];
    curr_utxo.asset.simplicity_encode(mac)
}

pub(crate) fn current_amount(mac: &mut BitMachine, env: &ElementsEnv) -> Result<(), JetFailed> {
    let curr_idx = env.ix as usize;
    let curr_utxo = &env.utxos[curr_idx];
    curr_utxo.value.simplicity_encode(mac)
}

pub(crate) fn current_script_hash(
    mac: &mut BitMachine,
    env: &ElementsEnv,
) -> Result<(), JetFailed> {
    let curr_idx = env.ix as usize;
    let curr_utxo = &env.utxos[curr_idx];
    // TODO: cache these while creating utxo
    mac.write_bytes(&curr_utxo.script_pubkey);
    Ok(())
}

pub(crate) fn current_sequence(mac: &mut BitMachine, env: &ElementsEnv) -> Result<(), JetFailed> {
    let curr_idx = env.ix as usize;
    let curr_inp = &env.tx.input[curr_idx];
    mac.write_u32(curr_inp.sequence);
    Ok(())
}

pub(crate) fn current_issuance_blinding(
    mac: &mut BitMachine,
    env: &ElementsEnv,
) -> Result<(), JetFailed> {
    let curr_idx = env.ix as usize;
    let curr_inp = &env.tx.input[curr_idx];

    if curr_inp.has_issuance {
        blinding_issuance(mac, &curr_inp.asset_issuance);
        Ok(())
    } else {
        Err(JetFailed)
    }
}

pub(crate) fn current_issuance_contract(
    mac: &mut BitMachine,
    env: &ElementsEnv,
) -> Result<(), JetFailed> {
    let curr_idx = env.ix as usize;
    let curr_inp = &env.tx.input[curr_idx];

    if curr_inp.has_issuance {
        contract_issuance(mac, &curr_inp.asset_issuance);
        Ok(())
    } else {
        Err(JetFailed)
    }
}

pub(crate) fn current_issuance_entropy(
    mac: &mut BitMachine,
    env: &ElementsEnv,
) -> Result<(), JetFailed> {
    let curr_idx = env.ix as usize;
    let curr_inp = &env.tx.input[curr_idx];

    if curr_inp.has_issuance {
        entropy_issuance(mac, &curr_inp.asset_issuance);
        Ok(())
    } else {
        Err(JetFailed)
    }
}

pub(crate) fn current_issuance_asset_amount(
    mac: &mut BitMachine,
    env: &ElementsEnv,
) -> Result<(), JetFailed> {
    let curr_idx = env.ix as usize;
    let curr_inp = &env.tx.input[curr_idx];
    asset_amt_issuance(mac, &curr_inp.asset_issuance, curr_inp.has_issuance())
}

pub(crate) fn current_issuance_token_amount(
    mac: &mut BitMachine,
    env: &ElementsEnv,
) -> Result<(), JetFailed> {
    let curr_idx = env.ix as usize;
    let curr_inp = &env.tx.input[curr_idx];

    if curr_inp.has_issuance {
        inflation_amt_issuance(mac, &curr_inp.asset_issuance)
    } else {
        Err(JetFailed)
    }
}

pub(crate) fn num_inputs(mac: &mut BitMachine, env: &ElementsEnv) -> Result<(), JetFailed> {
    mac.write_u32(env.tx.input.len() as u32);
    Ok(())
}

pub(crate) fn num_outputs(mac: &mut BitMachine, env: &ElementsEnv) -> Result<(), JetFailed> {
    mac.write_u32(env.tx.output.len() as u32);
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
) -> Result<(), JetFailed> {
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
) -> Result<(), JetFailed> {
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

// Adapted Core jets

pub(crate) fn add_32(mac: &mut BitMachine, _env: &ElementsEnv) -> Result<(), JetFailed> {
    jet::core::add_32(mac, &())
}

pub(crate) fn full_add_32(mac: &mut BitMachine, _env: &ElementsEnv) -> Result<(), JetFailed> {
    jet::core::full_add_32(mac, &())
}

pub(crate) fn sub_32(mac: &mut BitMachine, _env: &ElementsEnv) -> Result<(), JetFailed> {
    jet::core::sub_32(mac, &())
}

pub(crate) fn full_sub_32(mac: &mut BitMachine, _env: &ElementsEnv) -> Result<(), JetFailed> {
    jet::core::full_sub_32(mac, &())
}

pub(crate) fn mul_32(mac: &mut BitMachine, _env: &ElementsEnv) -> Result<(), JetFailed> {
    jet::core::mul_32(mac, &())
}

pub(crate) fn full_mul_32(mac: &mut BitMachine, _env: &ElementsEnv) -> Result<(), JetFailed> {
    jet::core::full_mul_32(mac, &())
}

pub(crate) fn eq_32_verify(mac: &mut BitMachine, _env: &ElementsEnv) -> Result<(), JetFailed> {
    jet::core::eq_32_verify(mac, &())
}

pub(crate) fn eq_256_verify(mac: &mut BitMachine, _env: &ElementsEnv) -> Result<(), JetFailed> {
    jet::core::eq_256_verify(mac, &())
}

pub(crate) fn lt_32_verify(mac: &mut BitMachine, _env: &ElementsEnv) -> Result<(), JetFailed> {
    jet::core::lt_32_verify(mac, &())
}

pub(crate) fn sha256(mac: &mut BitMachine, _env: &ElementsEnv) -> Result<(), JetFailed> {
    jet::core::sha256(mac, &())
}

pub(crate) fn sha256_block(mac: &mut BitMachine, _env: &ElementsEnv) -> Result<(), JetFailed> {
    jet::core::sha256_block(mac, &())
}
