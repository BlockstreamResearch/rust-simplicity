// SPDX-License-Identifier: CC0-1.0

use crate::jet::bitcoin::BitcoinEnv;
use crate::hashes::{sha256, Hash, HashEngine};
use simplicity_sys::c_jets::frame_ffi::{c_writeBit, CFrameItem, c_readBit};

/// Write a u32 value to the frame (big-endian).
fn write_u32(frame: &mut CFrameItem, val: u32) {
    for i in (0..32).rev() {
        unsafe { c_writeBit(frame, (val >> i) & 1 == 1) };
    }
}

/// Write a u64 value to the frame (big-endian).
fn write_u64(frame: &mut CFrameItem, val: u64) {
    for i in (0..64).rev() {
        unsafe { c_writeBit(frame, (val >> i) & 1 == 1) };
    }
}

/// Write a 256-bit hash to the frame.
fn write_hash(frame: &mut CFrameItem, hash: &[u8; 32]) {
    for byte in hash {
        for i in (0..8).rev() {
            unsafe { c_writeBit(frame, (byte >> i) & 1 == 1) };
        }
    }
}

/// Read a u32 value from the frame (big-endian).
fn read_u32(frame: &mut CFrameItem) -> u32 {
    let mut res = 0u32;
    for _ in 0..32 {
        res <<= 1;
        if unsafe { c_readBit(frame) } {
            res |= 1;
        }
    }
    res
}

/// Jet num_inputs: returns the number of inputs in the transaction.
pub fn num_inputs(frame: &mut CFrameItem, _src: CFrameItem, env: &BitcoinEnv) -> bool {
    write_u32(frame, env.tx.input.len() as u32);
    true
}

/// Jet num_outputs: returns the number of outputs in the transaction.
pub fn num_outputs(frame: &mut CFrameItem, _src: CFrameItem, env: &BitcoinEnv) -> bool {
    write_u32(frame, env.tx.output.len() as u32);
    true
}

/// Jet input_value: returns the value of the input at the given index.
/// Returns Option<u64>.
pub fn input_value(frame: &mut CFrameItem, mut src: CFrameItem, env: &BitcoinEnv) -> bool {
    let index = read_u32(&mut src);
    if index < env.spent_outputs.len() as u32 {
        unsafe { c_writeBit(frame, true) }; // Some
        write_u64(frame, env.spent_outputs[index as usize].value.to_sat());
    } else {
        unsafe { c_writeBit(frame, false) }; // None
        for _ in 0..64 {
            unsafe { c_writeBit(frame, false) }; // Padding
        }
    }
    true
}

/// Jet input_utxos_hash: returns the hash of all spent outputs (values and scripts).
/// Follows the Simplicity-C precomputation hierarchy.
pub fn input_utxos_hash(frame: &mut CFrameItem, _src: CFrameItem, env: &BitcoinEnv) -> bool {
    // 1. inputValuesHash
    let mut values_engine = sha256::Hash::engine();
    for output in &env.spent_outputs {
        values_engine.input(&output.value.to_sat().to_be_bytes());
    }
    let values_hash = sha256::Hash::from_engine(values_engine);

    // 2. inputScriptsHash
    let mut scripts_engine = sha256::Hash::engine();
    for output in &env.spent_outputs {
        let script_hash = sha256::Hash::hash(output.script_pubkey.as_bytes());
        scripts_engine.input(script_hash.as_ref());
    }
    let scripts_hash = sha256::Hash::from_engine(scripts_engine);

    // 3. inputUTXOsHash
    let mut utxos_engine = sha256::Hash::engine();
    utxos_engine.input(values_hash.as_ref());
    utxos_engine.input(scripts_hash.as_ref());
    let utxos_hash = sha256::Hash::from_engine(utxos_engine);

    write_hash(frame, utxos_hash.as_ref());
    true
}
