// SPDX-License-Identifier: CC0-1.0

//! # Manual type specification
//!
//! Source and target types of jet nodes need to be specified manually.

use crate::types::Type;
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

trait TypeConstructible {
    fn two_two_n(n: Option<u32>) -> Self;
    fn sum(left: Self, right: Self) -> Self;
    fn product(left: Self, right: Self) -> Self;
}

impl TypeConstructible for Type {
    fn two_two_n(n: Option<u32>) -> Self {
        match n {
            None => Type::unit(),
            Some(m) => Type::two_two_n(m as usize), // cast safety: 32-bit arch or higher
        }
    }

    fn sum(left: Self, right: Self) -> Self {
        Type::sum(left, right)
    }

    fn product(left: Self, right: Self) -> Self {
        Type::product(left, right)
    }
}

struct BitWidth(usize);

impl TypeConstructible for BitWidth {
    fn two_two_n(n: Option<u32>) -> Self {
        match n {
            None => BitWidth(0),
            Some(m) => BitWidth(usize::pow(2, m)),
        }
    }

    fn sum(left: Self, right: Self) -> Self {
        BitWidth(1 + cmp::max(left.0, right.0))
    }

    fn product(left: Self, right: Self) -> Self {
        BitWidth(left.0 + right.0)
    }
}

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
    fn construct<T: TypeConstructible>(&self) -> T {
        let mut stack = Vec::with_capacity(16);

        for c in self.0.iter().rev() {
            match c {
                b'1' => stack.push(T::two_two_n(None)),
                b'2' => stack.push(T::two_two_n(Some(0))),
                b'c' => stack.push(T::two_two_n(Some(3))),
                b's' => stack.push(T::two_two_n(Some(4))),
                b'i' => stack.push(T::two_two_n(Some(5))),
                b'l' => stack.push(T::two_two_n(Some(6))),
                b'h' => stack.push(T::two_two_n(Some(8))),
                b'+' | b'*' => {
                    let left = stack.pop().expect("Illegal type name syntax!");
                    let right = stack.pop().expect("Illegal type name syntax!");

                    match c {
                        b'+' => stack.push(T::sum(left, right)),
                        b'*' => stack.push(T::product(left, right)),
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

    /// Convert the type name into a type.
    pub fn to_type(&self) -> Type {
        self.construct()
    }

    /// Convert the type name into a type's bitwidth.
    ///
    /// This is more efficient than creating the type and computing its bit-width
    pub fn to_bit_width(&self) -> usize {
        self.construct::<BitWidth>().0
    }
}
