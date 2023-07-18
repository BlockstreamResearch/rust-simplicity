// Simplicity "Human-Readable" Language
// Written in 2023 by
//   Andrew Poelstra <simplicity@wpsoftware.net>
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

//! Serialization

use std::fmt;

use crate::dag::{DagLike, NoSharing};
use crate::hex::ToHex;
use crate::Value;

pub struct DisplayWord<'a>(pub &'a crate::Value);

impl<'a> fmt::Display for DisplayWord<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // The default value serialization shows the whole structure of
        // the value; but for words, the structure is always fixed by the
        // length, so it is fine to just serialize the bits.
        if let Ok(hex) = self.0.try_to_bytes() {
            f.write_str("0x")?;
            f.write_str(&hex.to_hex())?;
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
