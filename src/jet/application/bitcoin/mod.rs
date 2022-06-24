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

mod decode;
mod encode;
mod environment;
mod exec;

use crate::bititer::BitIter;
use crate::encode::BitWriter;
use crate::exec::BitMachine;
use crate::jet::application::core::CoreError;
use crate::jet::application::Core;
use crate::jet::bitcoin::BitcoinJetName;
use crate::jet::{AppError, Application, JetNode};
use crate::{jet, Error};
pub use environment::BitcoinEnv;
use std::convert::{TryFrom, TryInto};
use std::io::Write;

/// # Bitcoin Simplicity
///
/// The Bitcoin application provides everything that Core provides,
/// plus introspection into a Bitcoin environment.
///
/// The environment contains a Bitcoin transaction.
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Debug, Hash)]
pub struct Bitcoin {}

/// Bitcoin error
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Debug, Hash)]
pub enum BitcoinError {
    AssertionFailed,
}

impl std::fmt::Display for BitcoinError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            BitcoinError::AssertionFailed => f.write_str("Assertion failed during jet evaluation"),
        }
    }
}

impl std::error::Error for BitcoinError {}
impl AppError for BitcoinError {}

impl std::fmt::Display for BitcoinJetName {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(self, f)
    }
}

// Identify (some) Core jets by Bitcoin jet names
impl TryFrom<BitcoinJetName> for &'static JetNode<Core> {
    type Error = ();

    fn try_from(name: BitcoinJetName) -> Result<Self, Self::Error> {
        match name {
            BitcoinJetName::Add32 => Ok(&jet::core::ADD32),
            BitcoinJetName::FullAdd32 => Ok(&jet::core::FULL_ADD32),
            BitcoinJetName::Sub32 => Ok(&jet::core::SUB32),
            BitcoinJetName::FullSub32 => Ok(&jet::core::FULL_SUB32),
            BitcoinJetName::Mul32 => Ok(&jet::core::MUL32),
            BitcoinJetName::FullMul32 => Ok(&jet::core::FULL_MUL32),
            BitcoinJetName::Eq32Verify => Ok(&jet::core::EQ32_VERIFY),
            BitcoinJetName::Eq256Verify => Ok(&jet::core::EQ256_VERIFY),
            BitcoinJetName::Lt32Verify => Ok(&jet::core::LT32_VERIFY),
            BitcoinJetName::Sha256 => Ok(&jet::core::SHA256),
            BitcoinJetName::Sha256Block => Ok(&jet::core::SHA256_BLOCK),
            BitcoinJetName::Bip0340Verify => Ok(&jet::core::BIP_0340_VERIFY),
            _ => Err(()),
        }
    }
}

// Map Core errors onto Bitcoin errors
impl From<CoreError> for BitcoinError {
    fn from(e: CoreError) -> Self {
        match e {
            CoreError::AssertionFailed => BitcoinError::AssertionFailed,
        }
    }
}

impl Application for Bitcoin {
    type Environment = BitcoinEnv;
    type Error = BitcoinError;
    type JetName = BitcoinJetName;

    fn decode_jet<I: Iterator<Item = u8>>(
        iter: &mut BitIter<I>,
    ) -> Result<&'static JetNode<Self>, Error> {
        match iter.next() {
            Some(false) => self::decode::decode_primitive(iter),
            Some(true) => self::decode::decode_macro(iter),
            None => Err(Error::EndOfStream),
        }
    }

    fn encode_jet<W: Write>(jet: &JetNode<Self>, w: &mut BitWriter<W>) -> std::io::Result<usize> {
        if let Ok(jet) = jet.name.try_into() {
            Core::encode_jet(jet, w)
        } else {
            self::encode::encode_primitive(jet, w)
        }
    }

    fn exec_jet(
        jet: &JetNode<Self>,
        mac: &mut BitMachine,
        env: &Self::Environment,
    ) -> Result<(), Self::Error> {
        self::exec::exec_jet(jet, mac, env)
    }
}
