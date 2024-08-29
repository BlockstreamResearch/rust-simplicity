// SPDX-License-Identifier: CC0-1.0

//! Serialization

use crate::bit_encoding::BitCollector;
use hex::DisplayHex;
use std::fmt;

pub struct DisplayWord<'a>(pub &'a crate::Value);

impl<'a> fmt::Display for DisplayWord<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // The default value serialization shows the whole structure of
        // the value; but for words, the structure is always fixed by the
        // length, so it is fine to just serialize the bits.
        if let Ok(hex) = self.0.iter_compact().try_collect_bytes() {
            write!(f, "0x{}", hex.as_hex())?;
        } else {
            f.write_str("0b")?;
            for bit in self.0.iter_compact() {
                match bit {
                    false => f.write_str("0")?,
                    true => f.write_str("1")?,
                }
            }
        }
        Ok(())
    }
}
