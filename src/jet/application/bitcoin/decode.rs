use crate::bititer::BitIter;
use crate::jet::application::Bitcoin;
use crate::jet::JetNode;
use crate::{jet, Error};

/// Decode a Bitcoin jet primitive from bits
pub(super) fn decode_primitive<I: Iterator<Item = u8>>(
    iter: &mut BitIter<I>,
) -> Result<&'static JetNode<Bitcoin>, Error> {
    let code = iter.read_bits_be(4).ok_or(Error::EndOfStream)?;
    match code {
        0 => match iter.next() {
            Some(false) => Ok(&jet::bitcoin::VERSION),
            Some(true) => Ok(&jet::bitcoin::LOCK_TIME),
            None => Err(Error::EndOfStream),
        },
        1 => Ok(&jet::bitcoin::INPUTS_HASH),
        2 => Ok(&jet::bitcoin::OUTPUTS_HASH),
        6 => Ok(&jet::bitcoin::CURRENT_VALUE),
        7 => Ok(&jet::bitcoin::CURRENT_SEQUENCE),
        8 => match iter.next() {
            Some(false) => Ok(&jet::bitcoin::CURRENT_INDEX),
            Some(true) => unimplemented!("Decode Bitcoin primitive"),
            None => Err(Error::EndOfStream),
        },
        _ => unimplemented!("Decode Bitcoin primitive"),
    }
}

/// Decode a Bitcoin jet macro from bits
pub(super) fn decode_macro<I: Iterator<Item = u8>>(
    iter: &mut BitIter<I>,
) -> Result<&'static JetNode<Bitcoin>, Error> {
    match iter.next() {
        Some(false) => {
            let code = iter.read_bits_be(2).ok_or(Error::EndOfStream)?;
            match code {
                0 => match iter.next() {
                    Some(false) => Ok(&jet::bitcoin::ADD32),
                    Some(true) => Ok(&jet::bitcoin::SUB32),
                    None => Err(Error::EndOfStream),
                },
                1 => Ok(&jet::bitcoin::MUL32),
                2 => match iter.next() {
                    Some(false) => Ok(&jet::bitcoin::FULL_ADD32),
                    Some(true) => Ok(&jet::bitcoin::FULL_SUB32),
                    None => Err(Error::EndOfStream),
                },
                3 => Ok(&jet::bitcoin::FULL_MUL32),
                _ => Err(Error::ParseError("Illegal jet encoding")),
            }
        }
        Some(true) => match iter.next().ok_or(Error::EndOfStream)? {
            false => Ok(&jet::bitcoin::SHA256_BLOCK),
            true => {
                let code = iter.read_bits_be(4).ok_or(Error::EndOfStream)?;
                match code {
                    0 => Ok(&jet::bitcoin::BIP_0340_VERIFY),
                    1 => Ok(&jet::bitcoin::EQ256_VERIFY),
                    2 => Ok(&jet::bitcoin::SHA256),
                    3 => Ok(&jet::bitcoin::LT32_VERIFY),
                    4 => Ok(&jet::bitcoin::EQ32_VERIFY),
                    _ => Err(Error::ParseError("Illegal jet encoding")),
                }
            }
        },
        None => Err(Error::EndOfStream),
    }
}
