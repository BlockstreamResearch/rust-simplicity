use crate::exec::BitMachine;
use crate::jet;
use crate::jet::bitcoin::BitcoinEnv;
use crate::jet::JetFailed;
use bitcoin_hashes::{sha256, Hash, HashEngine};
use byteorder::{LittleEndian, WriteBytesExt};

pub(crate) fn version(mac: &mut BitMachine, env: &BitcoinEnv) -> Result<(), JetFailed> {
    mac.write_u32(env.tx.version as u32);
    Ok(())
}

pub(crate) fn lock_time(mac: &mut BitMachine, env: &BitcoinEnv) -> Result<(), JetFailed> {
    mac.write_u32(env.tx.lock_time.0);
    Ok(())
}

pub(crate) fn inputs_hash(mac: &mut BitMachine, env: &BitcoinEnv) -> Result<(), JetFailed> {
    let mut eng = sha256::Hash::engine();
    for input in &env.tx.input {
        eng.input(&input.previous_output.txid[..]);
        eng.write_u32::<LittleEndian>(input.previous_output.vout)
            .unwrap();
        eng.write_u64::<LittleEndian>(99_998_000).unwrap(); // value FIXME
        eng.write_u32::<LittleEndian>(input.sequence.0).unwrap();
    }
    mac.write_bytes(&sha256::Hash::from_engine(eng)[..]);
    Ok(())
}

pub(crate) fn outputs_hash(mac: &mut BitMachine, env: &BitcoinEnv) -> Result<(), JetFailed> {
    let mut eng = sha256::Hash::engine();
    for output in &env.tx.output {
        eng.write_u64::<LittleEndian>(output.value).unwrap();
        eng.input(&sha256::Hash::hash(&output.script_pubkey[..]));
    }
    mac.write_bytes(&sha256::Hash::from_engine(eng)[..]);
    Ok(())
}

// FIXME don't hardcode this
pub(crate) fn current_value(mac: &mut BitMachine, _env: &BitcoinEnv) -> Result<(), JetFailed> {
    mac.write_u64(99_998_000);
    Ok(())
}

pub(crate) fn current_index(mac: &mut BitMachine, _env: &BitcoinEnv) -> Result<(), JetFailed> {
    mac.write_u32(0);
    Ok(())
}

// Adapted Core jets

pub(crate) fn add_32(mac: &mut BitMachine, _env: &BitcoinEnv) -> Result<(), JetFailed> {
    jet::core::add_32(mac, &())
}

pub(crate) fn full_add_32(mac: &mut BitMachine, _env: &BitcoinEnv) -> Result<(), JetFailed> {
    jet::core::full_add_32(mac, &())
}

pub(crate) fn sub_32(mac: &mut BitMachine, _env: &BitcoinEnv) -> Result<(), JetFailed> {
    jet::core::sub_32(mac, &())
}

pub(crate) fn full_sub_32(mac: &mut BitMachine, _env: &BitcoinEnv) -> Result<(), JetFailed> {
    jet::core::full_sub_32(mac, &())
}

pub(crate) fn mul_32(mac: &mut BitMachine, _env: &BitcoinEnv) -> Result<(), JetFailed> {
    jet::core::mul_32(mac, &())
}

pub(crate) fn full_mul_32(mac: &mut BitMachine, _env: &BitcoinEnv) -> Result<(), JetFailed> {
    jet::core::full_mul_32(mac, &())
}

pub(crate) fn eq_32_verify(mac: &mut BitMachine, _env: &BitcoinEnv) -> Result<(), JetFailed> {
    jet::core::eq_32_verify(mac, &())
}

pub(crate) fn eq_256_verify(mac: &mut BitMachine, _env: &BitcoinEnv) -> Result<(), JetFailed> {
    jet::core::eq_256_verify(mac, &())
}

pub(crate) fn lt_32_verify(mac: &mut BitMachine, _env: &BitcoinEnv) -> Result<(), JetFailed> {
    jet::core::lt_32_verify(mac, &())
}

pub(crate) fn sha256(mac: &mut BitMachine, _env: &BitcoinEnv) -> Result<(), JetFailed> {
    jet::core::sha256(mac, &())
}

pub(crate) fn sha256_block(mac: &mut BitMachine, _env: &BitcoinEnv) -> Result<(), JetFailed> {
    jet::core::sha256_block(mac, &())
}
