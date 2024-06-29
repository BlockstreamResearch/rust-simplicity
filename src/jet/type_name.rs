// SPDX-License-Identifier: CC0-1.0

//! # Manual type specification
//!
//! Source and target types of jet nodes need to be specified manually.

use crate::types::{Final, Type};
use std::cmp;
use std::sync::Arc;

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
    /// Convert the type name into a type.
    pub fn to_type(&self) -> Type {
        Type::complete(self.to_final())
    }

    /// Convert the type name into a finalized type.
    pub fn to_final(&self) -> Arc<Final> {
        let mut stack = Vec::with_capacity(16);

        for c in self.0.iter().rev() {
            match c {
                b'1' => stack.push(Final::unit()),
                b'2' => stack.push(Final::two_two_n(0)),
                b'c' => stack.push(Final::two_two_n(3)),
                b's' => stack.push(Final::two_two_n(4)),
                b'i' => stack.push(Final::two_two_n(5)),
                b'l' => stack.push(Final::two_two_n(6)),
                b'h' => stack.push(Final::two_two_n(8)),
                b'+' | b'*' => {
                    let left = stack.pop().expect("Illegal type name syntax!");
                    let right = stack.pop().expect("Illegal type name syntax!");

                    match c {
                        b'+' => stack.push(Final::sum(left, right)),
                        b'*' => stack.push(Final::product(left, right)),
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
