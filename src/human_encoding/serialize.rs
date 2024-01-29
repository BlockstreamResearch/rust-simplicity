// SPDX-License-Identifier: CC0-1.0

//! Serialization

use hex::DisplayHex;
use std::fmt;

use crate::dag::{DagLike, NoSharing};
use crate::Value;

pub struct DisplayWord<'a>(pub &'a crate::Value);

impl<'a> fmt::Display for DisplayWord<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // The default value serialization shows the whole structure of
        // the value; but for words, the structure is always fixed by the
        // length, so it is fine to just serialize the bits.
        if let Ok(hex) = self.0.try_to_bytes() {
            write!(f, "0x{}", hex.as_hex())?;
        } else {
            f.write_str("0b")?;
            for comb in self.0.pre_order_iter::<NoSharing>() {
                match comb {
                    Value::SumL(..) => f.write_str("0")?,
                    Value::SumR(..) => f.write_str("1")?,
                    _ => {}
                }
            }
        }
        Ok(())
    }
}
