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
use crate::jet;
use crate::jet::application::Elements;
use crate::jet::JetNode;
use crate::Error;

/// Decode an Elements jet primitive from bits
pub(crate) fn decode_primitive<I: Iterator<Item = u8>>(
    iter: &mut BitIter<I>,
) -> Result<&'static JetNode<Elements>, Error> {
    let code = iter.read_bits_be(5).ok_or(Error::EndOfStream)?;
    match code {
        0 => match iter.next() {
            Some(false) => Ok(&jet::elements::VERSION),
            Some(true) => Ok(&jet::elements::LOCK_TIME),
            None => Err(Error::EndOfStream),
        },
        1 => Ok(&jet::elements::INPUT_IS_PEGIN),
        2 => Ok(&jet::elements::INPUT_PREV_OUTPOINT),
        3 => Ok(&jet::elements::INPUT_ASSET),
        4 => match iter.next() {
            Some(false) => Ok(&jet::elements::INPUT_AMOUNT),
            Some(true) => Ok(&jet::elements::INPUT_SCRIPT_HASH),
            None => Err(Error::EndOfStream),
        },
        5 => Ok(&jet::elements::INPUT_SEQUENCE),
        6 => Ok(&jet::elements::INPUT_ISSUANCE_BLINDING),
        7 => Ok(&jet::elements::INPUT_ISSUANCE_CONTRACT),
        8 => match iter.next() {
            Some(false) => Ok(&jet::elements::INPUT_ISSUANCE_ENTROPY),
            Some(true) => Ok(&jet::elements::INPUT_ISSUANCE_ASSET_AMOUNT),
            None => Err(Error::EndOfStream),
        },
        9 => Ok(&jet::elements::INPUT_ISSUANCE_TOKEN_AMOUNT),
        10 => Ok(&jet::elements::OUTPUT_ASSET),
        11 => Ok(&jet::elements::OUTPUT_AMOUNT),
        12 => match iter.next() {
            Some(false) => Ok(&jet::elements::OUTPUT_NONCE),
            Some(true) => Ok(&jet::elements::OUTPUT_SCRIPT_HASH),
            None => Err(Error::EndOfStream),
        },
        13 => Ok(&jet::elements::OUTPUT_NULL_DATUM),
        14 => Ok(&jet::elements::SCRIPT_CMR),
        15 => Ok(&jet::elements::CURRENT_INDEX),
        16 => Ok(&jet::elements::CURRENT_IS_PEGIN),
        17 => Ok(&jet::elements::CURRENT_PREV_OUTPOINT),
        18 => Ok(&jet::elements::CURRENT_ASSET),
        19 => Ok(&jet::elements::CURRENT_AMOUNT),
        20 => Ok(&jet::elements::CURRENT_SCRIPT_HASH),
        21 => Ok(&jet::elements::CURRENT_SEQUENCE),
        22 => Ok(&jet::elements::CURRENT_ISSUANCE_BLINDING),
        23 => Ok(&jet::elements::CURRENT_ISSUANCE_CONTRACT),
        24 => Ok(&jet::elements::CURRENT_ISSUANCE_ENTROPY),
        25 => Ok(&jet::elements::CURRENT_ISSUANCE_ASSET_AMOUNT),
        26 => Ok(&jet::elements::CURRENT_ISSUANCE_TOKEN_AMOUNT),
        27 => Ok(&jet::elements::INPUTS_HASH),
        28 => Ok(&jet::elements::OUTPUTS_HASH),
        29 => Ok(&jet::elements::NUM_INPUTS),
        30 => Ok(&jet::elements::NUM_OUTPUTS),
        31 => Ok(&jet::elements::FEE),
        _ => unimplemented!("Decode Elements primitive"),
    }
}

/// Decode an Elements jet macro from bits
pub(crate) fn decode_macro<I: Iterator<Item = u8>>(
    iter: &mut BitIter<I>,
) -> Result<&'static JetNode<Elements>, Error> {
    match iter.next() {
        Some(false) => {
            let code = iter.read_bits_be(2).ok_or(Error::EndOfStream)?;
            match code {
                0 => match iter.next() {
                    Some(false) => Ok(&jet::elements::ADD32),
                    Some(true) => Ok(&jet::elements::SUB32),
                    None => Err(Error::EndOfStream),
                },
                1 => Ok(&jet::elements::MUL32),
                2 => match iter.next() {
                    Some(false) => Ok(&jet::elements::FULL_ADD32),
                    Some(true) => Ok(&jet::elements::FULL_SUB32),
                    None => Err(Error::EndOfStream),
                },
                3 => Ok(&jet::elements::FULL_MUL32),
                _ => Err(Error::ParseError("Illegal jet encoding")),
            }
        }
        Some(true) => match iter.next().ok_or(Error::EndOfStream)? {
            false => Ok(&jet::elements::SHA256_BLOCK),
            true => {
                let code = iter.read_bits_be(4).ok_or(Error::EndOfStream)?;
                match code {
                    0 => Ok(&jet::elements::BIP_0340_VERIFY),
                    1 => Ok(&jet::elements::EQ256_VERIFY),
                    2 => Ok(&jet::elements::SHA256),
                    3 => Ok(&jet::elements::LT32_VERIFY),
                    4 => Ok(&jet::elements::EQ32_VERIFY),
                    _ => Err(Error::ParseError("Illegal jet encoding")),
                }
            }
        },
        None => Err(Error::EndOfStream),
    }
}
