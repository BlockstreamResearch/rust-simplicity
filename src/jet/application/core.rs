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

use crate::bititer::BitIter;
use crate::encode::BitWriter;
use crate::exec::BitMachine;
use crate::jet;
use crate::jet::core::CoreJetName;
use crate::jet::{AppError, Application, JetNode};
use crate::Error;
use bitcoin_hashes::{sha256, Hash, HashEngine};
use std::io::Write;

/// # Core Simplicity
///
/// The Core application provides macros such as adders, subtractors, hashes,
/// equality checks and signature checks.
/// There is no introspection into an environment.
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Debug, Hash)]
pub struct Core {}

/// Core error
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Debug, Hash)]
pub enum CoreError {
    AssertionFailed,
}

impl std::fmt::Display for CoreError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            CoreError::AssertionFailed => f.write_str("Assertion failed during jet execution"),
        }
    }
}

impl std::error::Error for CoreError {}
impl AppError for CoreError {}

impl std::fmt::Display for CoreJetName {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(self, f)
    }
}

impl Application for Core {
    type Environment = ();
    type Error = CoreError;
    type JetName = CoreJetName;

    fn decode_jet<I: Iterator<Item = u8>>(
        iter: &mut BitIter<I>,
    ) -> Result<&'static JetNode<Self>, Error> {
        match iter.next() {
            // There are no Core primitives
            Some(false) => Err(Error::ParseError("Illegal jet encoding")),
            Some(true) => self::decode_macro(iter),
            None => Err(Error::EndOfStream),
        }
    }

    fn encode_jet<W: Write>(jet: &JetNode<Self>, w: &mut BitWriter<W>) -> std::io::Result<usize> {
        self::encode_macro(jet, w)
    }

    fn exec_jet(
        jet: &JetNode<Self>,
        mac: &mut BitMachine,
        _env: &Self::Environment,
    ) -> Result<(), Self::Error> {
        match jet.name {
            CoreJetName::Add32 => {
                let a = mac.read_u32();
                let b = mac.read_u32();
                let (res, overflow) = a.overflowing_add(b);
                mac.write_bit(overflow);
                mac.write_u32(res);
            }
            CoreJetName::FullAdd32 => {
                let a = mac.read_u32();
                let b = mac.read_u32();
                let carry = mac.read_bit();
                let (res, overflow_1) = a.overflowing_add(b);
                let (res, overflow_2) = res.overflowing_add(carry as u32);
                mac.write_bit(overflow_1 || overflow_2);
                mac.write_u32(res);
            }
            CoreJetName::Sub32 => {
                let a = mac.read_u32();
                let b = mac.read_u32();
                let (res, overflow) = a.overflowing_sub(b);
                mac.write_bit(overflow);
                mac.write_u32(res);
            }
            CoreJetName::FullSub32 => {
                let a = mac.read_u32();
                let b = mac.read_u32();
                let carry = mac.read_bit();
                let (res, overflow_1) = a.overflowing_sub(b);
                let (res, overflow_2) = res.overflowing_sub(carry as u32);
                mac.write_bit(overflow_1 || overflow_2);
                mac.write_u32(res);
            }
            CoreJetName::Mul32 => {
                let a = mac.read_u32() as u64;
                let b = mac.read_u32() as u64;
                mac.write_u64(a * b);
            }
            CoreJetName::FullMul32 => {
                let a = mac.read_u32() as u64;
                let b = mac.read_u32() as u64;
                let c = mac.read_u32() as u64;
                let d = mac.read_u32() as u64;
                mac.write_u64(a * b + c + d);
            }
            CoreJetName::Eq32Verify => {
                let a = mac.read_u32();
                let b = mac.read_u32();

                if a != b {
                    return Err(CoreError::AssertionFailed);
                }
            }
            CoreJetName::Eq256Verify => {
                let a = mac.read_32bytes();
                let b = mac.read_32bytes();

                if a != b {
                    return Err(CoreError::AssertionFailed);
                }
            }
            CoreJetName::Lt32Verify => {
                let a = mac.read_u32();
                let b = mac.read_u32();

                if a >= b {
                    return Err(CoreError::AssertionFailed);
                }
            }
            CoreJetName::Sha256 => {
                let data = mac.read_32bytes();
                let h = sha256::Hash::hash(&data);
                mac.write_bytes(&h);
            }
            CoreJetName::Sha256Block => {
                let hash = mac.read_32bytes();
                let block = mac.read_bytes(64);
                let sha2_midstate = sha256::Midstate::from_inner(hash);
                let mut engine = sha256::HashEngine::from_midstate(sha2_midstate, 0);
                engine.input(&block);
                let h = engine.midstate();
                mac.write_bytes(&h.into_inner());
            }
            CoreJetName::Bip0340Verify => {
                unimplemented!("BIP 340 Schnorr signature verification");
            }
        }

        Ok(())
    }
}

/// Decode a Core jet macro from bits
fn decode_macro<I: Iterator<Item = u8>>(
    iter: &mut BitIter<I>,
) -> Result<&'static JetNode<Core>, Error> {
    match iter.next() {
        Some(false) => {
            let code = iter.read_bits_be(2).ok_or(Error::EndOfStream)?;
            match code {
                0 => match iter.next() {
                    Some(false) => Ok(&jet::core::ADD32),
                    Some(true) => Ok(&jet::core::SUB32),
                    None => Err(Error::EndOfStream),
                },
                1 => Ok(&jet::core::MUL32),
                2 => match iter.next() {
                    Some(false) => Ok(&jet::core::FULL_ADD32),
                    Some(true) => Ok(&jet::core::FULL_SUB32),
                    None => Err(Error::EndOfStream),
                },
                3 => Ok(&jet::core::FULL_MUL32),
                _ => Err(Error::ParseError("Illegal jet encoding")),
            }
        }
        Some(true) => match iter.next().ok_or(Error::EndOfStream)? {
            false => Ok(&jet::core::SHA256_BLOCK),
            true => {
                let code = iter.read_bits_be(4).ok_or(Error::EndOfStream)?;
                match code {
                    0 => Ok(&jet::core::BIP_0340_VERIFY),
                    1 => Ok(&jet::core::EQ256_VERIFY),
                    2 => Ok(&jet::core::SHA256),
                    3 => Ok(&jet::core::LT32_VERIFY),
                    4 => Ok(&jet::core::EQ32_VERIFY),
                    _ => Err(Error::ParseError("Illegal jet encoding")),
                }
            }
        },
        None => Err(Error::EndOfStream),
    }
}

fn encode_macro<W: Write>(jet: &JetNode<Core>, w: &mut BitWriter<W>) -> std::io::Result<usize> {
    match jet.name {
        CoreJetName::Add32 => w.write_bits_be(48 + 0, 6),
        CoreJetName::FullAdd32 => w.write_bits_be(48 + 4, 6),
        CoreJetName::Sub32 => w.write_bits_be(48 + 1, 6),
        CoreJetName::FullSub32 => w.write_bits_be(48 + 5, 6),
        CoreJetName::Mul32 => w.write_bits_be(24 + 1, 5),
        CoreJetName::FullMul32 => w.write_bits_be(24 + 3, 5),
        CoreJetName::Eq32Verify => w.write_bits_be(15 * 16 + 4, 8),
        CoreJetName::Eq256Verify => w.write_bits_be(15 * 16 + 1, 8),
        CoreJetName::Lt32Verify => w.write_bits_be(15 * 16 + 3, 8),
        CoreJetName::Sha256 => w.write_bits_be(15 * 16 + 2, 8),
        CoreJetName::Sha256Block => w.write_bits_be(14, 4),
        CoreJetName::Bip0340Verify => w.write_bits_be(15 * 16 + 0, 8),
    }
}
