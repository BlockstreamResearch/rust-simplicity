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
#[cfg(test)]
mod tests;

use crate::bititer::BitIter;
use crate::encode::BitWriter;
use crate::exec::BitMachine;
use crate::jet::application::{Core, CoreError};
use crate::jet::elements::ElementsJetName;
use crate::jet::{AppError, Application, JetNode};
use crate::{jet, Error};
pub use environment::{ElementsEnv, ElementsUtxo};
use std::convert::{TryFrom, TryInto};
use std::io::Write;

/// # Elements Simplicity
///
/// The Elements application provides everything that Core provides,
/// plus introspection into an Elements environment.
///
/// The environment contains an Elements transaction,
/// information on the input and outpoints,
/// as well as input/output/script hashes.
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Debug, Hash)]
pub struct Elements {}

/// Elements error
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Debug, Hash)]
pub enum ElementsError {
    AssertionFailed,
    NullAssetEncoding,
    NullValueEncoding,
}

impl std::fmt::Display for ElementsError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            ElementsError::AssertionFailed => f.write_str("Assertion failed during jet evaluation"),
            ElementsError::NullAssetEncoding => f.write_str("Null asset cannot be encoded"),
            ElementsError::NullValueEncoding => f.write_str("Null value cannot be encoded"),
        }
    }
}

impl std::error::Error for ElementsError {}
impl AppError for ElementsError {}

impl std::fmt::Display for ElementsJetName {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(self, f)
    }
}

// Identify (some) Core jets by Elements jet names
impl TryFrom<ElementsJetName> for &'static JetNode<Core> {
    type Error = ();

    fn try_from(name: ElementsJetName) -> Result<Self, Self::Error> {
        match name {
            ElementsJetName::Add32 => Ok(&jet::core::ADD32),
            ElementsJetName::FullAdd32 => Ok(&jet::core::FULL_ADD32),
            ElementsJetName::Sub32 => Ok(&jet::core::SUB32),
            ElementsJetName::FullSub32 => Ok(&jet::core::FULL_SUB32),
            ElementsJetName::Mul32 => Ok(&jet::core::MUL32),
            ElementsJetName::FullMul32 => Ok(&jet::core::FULL_MUL32),
            ElementsJetName::Eq32Verify => Ok(&jet::core::EQ32_VERIFY),
            ElementsJetName::Eq256Verify => Ok(&jet::core::EQ256_VERIFY),
            ElementsJetName::Lt32Verify => Ok(&jet::core::LT32_VERIFY),
            ElementsJetName::Sha256 => Ok(&jet::core::SHA256),
            ElementsJetName::Sha256Block => Ok(&jet::core::SHA256_BLOCK),
            ElementsJetName::Bip0340Verify => Ok(&jet::core::BIP_0340_VERIFY),
            _ => Err(()),
        }
    }
}

// Map Core errors onto Elements errors
impl From<CoreError> for ElementsError {
    fn from(e: CoreError) -> Self {
        match e {
            CoreError::AssertionFailed => ElementsError::AssertionFailed,
        }
    }
}

impl Application for Elements {
    type Environment = ElementsEnv;
    type Error = ElementsError;
    type JetName = ElementsJetName;

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
