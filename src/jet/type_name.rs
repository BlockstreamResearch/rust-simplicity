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

//! # Manual type specification
//!
//! Source and target types of jet nodes need to be specified manually.

use crate::types;
use std::cmp;

/// Byte-based specification of a Simplicity type.
///
/// Because jets are black boxes, the type inference engine has no access to their
/// source and target types. Therefore, these types need to be specified manually.
///
/// The types are written in prefix (aka Polish) notation,
/// where `+` and `*` represent sum and product types, respectively,
/// and base types are represented by the following:
///
/// | char | type         |
/// |------|--------------|
/// | `1`  | unit         |
/// | `2`  | single bit   |
/// | `c`  | 8-bit word   |
/// | `s`  | 16-bit word  |
/// | `i`  | 32-bit word  |
/// | `l`  | 64-bit word  |
/// | `h`  | 256-bit word |
///
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct TypeName(pub &'static [u8]);

impl TypeName {
    // b'1' = 49
    // b'2' = 50
    // b'c' = 99
    // b's' = 115
    // b'i' = 105
    // b'l' = 108
    // b'h' = 104
    // b'+' = 43
    // b'*' = 42
    /// Convert the type name into a type.
    pub fn to_type<F: FnMut(usize) -> types::Type>(&self, mut pow2s: F) -> types::Type {
        let it = self.0.iter().rev();
        let mut stack = Vec::with_capacity(16);

        for c in it {
            match c {
                b'1' => stack.push(types::Type::unit()),
                b'2' => stack.push(pow2s(0)),
                b'c' => stack.push(pow2s(3)),
                b's' => stack.push(pow2s(4)),
                b'i' => stack.push(pow2s(5)),
                b'l' => stack.push(pow2s(6)),
                b'h' => stack.push(pow2s(8)),
                b'+' | b'*' => {
                    let left = stack.pop().expect("Illegal type name syntax!");
                    let right = stack.pop().expect("Illegal type name syntax!");

                    match c {
                        b'+' => stack.push(types::Type::sum(left, right)),
                        b'*' => stack.push(types::Type::product(left, right)),
                        _ => unreachable!(),
                    }
                }
                _ => panic!("Illegal type name syntax!"),
            }
        }

        if stack.len() == 1 {
            stack.pop().unwrap()
        } else {
            panic!("Illegal type name syntax!")
        }
    }

    /// Convert the type name into a type's bitwidth.
    ///
    /// This is more efficient than creating the type and computing its bit-width
    pub fn to_bit_width(&self) -> usize {
        let mut stack = Vec::with_capacity(16);

        for c in self.0.iter().rev() {
            match c {
                b'1' => stack.push(0),
                b'2' => stack.push(1),
                b'c' => stack.push(8),
                b's' => stack.push(16),
                b'i' => stack.push(32),
                b'l' => stack.push(64),
                b'h' => stack.push(256),
                b'+' | b'*' => {
                    let left = stack.pop().expect("Illegal type name syntax!");
                    let right = stack.pop().expect("Illegal type name syntax!");

                    match c {
                        b'+' => stack.push(1 + cmp::max(left, right)),
                        b'*' => stack.push(left + right),
                        _ => unreachable!(),
                    }
                }
                _ => panic!("Illegal type name syntax!"),
            }
        }

        if stack.len() == 1 {
            stack.pop().unwrap()
        } else {
            panic!("Illegal type name syntax!")
        }
    }
}
