// Rust Simplicity Library
// Written in 2022 by
//   Christian Lewe <clewe@blockstream.com>
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
use crate::jet::{Core, JetFailed};
use bitcoin_hashes::{sha256, Hash, HashEngine};

impl std::fmt::Display for Core {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(self, f)
    }
}

pub(crate) fn add_32(mac: &mut BitMachine, _env: &()) -> Result<(), JetFailed> {
    let a = mac.read_u32();
    let b = mac.read_u32();
    let (res, overflow) = a.overflowing_add(b);
    mac.write_bit(overflow);
    mac.write_u32(res);
    Ok(())
}

pub(crate) fn full_add_32(mac: &mut BitMachine, _env: &()) -> Result<(), JetFailed> {
    let a = mac.read_u32();
    let b = mac.read_u32();
    let carry = mac.read_bit();
    let (res, overflow_1) = a.overflowing_add(b);
    let (res, overflow_2) = res.overflowing_add(carry as u32);
    mac.write_bit(overflow_1 || overflow_2);
    mac.write_u32(res);
    Ok(())
}

pub(crate) fn sub_32(mac: &mut BitMachine, _env: &()) -> Result<(), JetFailed> {
    let a = mac.read_u32();
    let b = mac.read_u32();
    let (res, overflow) = a.overflowing_sub(b);
    mac.write_bit(overflow);
    mac.write_u32(res);
    Ok(())
}

pub(crate) fn full_sub_32(mac: &mut BitMachine, _env: &()) -> Result<(), JetFailed> {
    let a = mac.read_u32();
    let b = mac.read_u32();
    let carry = mac.read_bit();
    let (res, overflow_1) = a.overflowing_sub(b);
    let (res, overflow_2) = res.overflowing_sub(carry as u32);
    mac.write_bit(overflow_1 || overflow_2);
    mac.write_u32(res);
    Ok(())
}

pub(crate) fn mul_32(mac: &mut BitMachine, _env: &()) -> Result<(), JetFailed> {
    let a = mac.read_u32() as u64;
    let b = mac.read_u32() as u64;
    mac.write_u64(a * b);
    Ok(())
}

pub(crate) fn full_mul_32(mac: &mut BitMachine, _env: &()) -> Result<(), JetFailed> {
    let a = mac.read_u32() as u64;
    let b = mac.read_u32() as u64;
    let c = mac.read_u32() as u64;
    let d = mac.read_u32() as u64;
    mac.write_u64(a * b + c + d);
    Ok(())
}

pub(crate) fn eq_32_verify(mac: &mut BitMachine, _env: &()) -> Result<(), JetFailed> {
    let a = mac.read_u32();
    let b = mac.read_u32();

    if a == b {
        Ok(())
    } else {
        Err(JetFailed)
    }
}

pub(crate) fn eq_256_verify(mac: &mut BitMachine, _env: &()) -> Result<(), JetFailed> {
    let a = mac.read_32bytes();
    let b = mac.read_32bytes();

    if a == b {
        Ok(())
    } else {
        Err(JetFailed)
    }
}

pub(crate) fn lt_32_verify(mac: &mut BitMachine, _env: &()) -> Result<(), JetFailed> {
    let a = mac.read_u32();
    let b = mac.read_u32();

    if a < b {
        Ok(())
    } else {
        Err(JetFailed)
    }
}

pub(crate) fn sha256(mac: &mut BitMachine, _env: &()) -> Result<(), JetFailed> {
    let data = mac.read_32bytes();
    let h = sha256::Hash::hash(&data);
    mac.write_bytes(&h);
    Ok(())
}

pub(crate) fn sha256_block(mac: &mut BitMachine, _env: &()) -> Result<(), JetFailed> {
    let hash = mac.read_32bytes();
    let block = mac.read_bytes(64);
    let sha2_midstate = sha256::Midstate::from_inner(hash);
    let mut engine = sha256::HashEngine::from_midstate(sha2_midstate, 0);
    engine.input(&block);
    let h = engine.midstate();
    mac.write_bytes(&h.into_inner());
    Ok(())
}
