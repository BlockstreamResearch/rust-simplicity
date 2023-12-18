// SPDX-License-Identifier: CC0-1.0

//! # Simplicity values
//!
//! Simplicity processes data in terms of [`Value`]s,
//! i.e., inputs, intermediate results and outputs.

use crate::dag::{Dag, DagLike, NoSharing};

use std::collections::VecDeque;
use std::convert::TryInto;
use std::fmt;
use std::hash::Hash;
use std::sync::Arc;

/// Value of some type.
///
/// The _unit value_ is the only value of the _unit type_.
/// This is the basis for everything we are doing.
/// Because there is only a single unit value, there is no information contained in it.
/// Instead, we wrap unit values in sum and product values to encode information.
///
/// A _sum value_ wraps another value.
/// The _left sum value_ `L(a)` wraps a value `a` from the _left type_ `A`.
/// The _right sum value_ `R(b)` wraps a value `b` from the _right type_ `B`.
/// The type of the sum value is the _sum type_ `A + B` of the left type and the right type.
///
/// We represent the false bit as a left value that wraps a unit value.
/// The true bit is represented as a right value that wraps a unit value.
///
/// A _product value_ `(a, b)` wraps two values:
/// a value `a` from the _left type_ `A` and a value `b` from the _right type_ `B`.
/// The type of the product value is the _product type_ `A × B` of the left type and the right type.
///
/// We represent bit strings (tuples of bits) as trees of nested product values
/// that have bit values (sum values wrapping the unit value) at their leaves.
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Value {
    /// Unit value
    Unit,
    /// Sum value that wraps a left value
    SumL(Arc<Value>),
    /// Sum value that wraps a right value
    SumR(Arc<Value>),
    /// Product value that wraps a left and a right value
    Prod(Arc<Value>, Arc<Value>),
}

impl<'a> DagLike for &'a Value {
    type Node = Value;

    fn data(&self) -> &Value {
        self
    }

    fn as_dag_node(&self) -> Dag<Self> {
        match self {
            Value::Unit => Dag::Nullary,
            Value::SumL(child) | Value::SumR(child) => Dag::Unary(child),
            Value::Prod(left, right) => Dag::Binary(left, right),
        }
    }
}

impl Value {
    /// Create the unit value.
    pub fn unit() -> Arc<Self> {
        Arc::new(Self::Unit)
    }

    /// Create a sum value that wraps a left value.
    pub fn sum_l(left: Arc<Self>) -> Arc<Self> {
        Arc::new(Value::SumL(left))
    }

    /// Create a sum value that wraps a right value.
    pub fn sum_r(right: Arc<Self>) -> Arc<Self> {
        Arc::new(Value::SumR(right))
    }

    /// Create a product value that wraps a left and a right value.
    pub fn prod(left: Arc<Self>, right: Arc<Self>) -> Arc<Self> {
        Arc::new(Value::Prod(left, right))
    }

    #[allow(clippy::len_without_is_empty)]
    /// The length, in bits, of the value when encoded in the Bit Machine
    pub fn len(&self) -> usize {
        self.pre_order_iter::<NoSharing>()
            .filter(|inner| matches!(inner, Value::SumL(_) | Value::SumR(_)))
            .count()
    }

    /// Encode a single bit as a value. Will panic if the input is out of range
    pub fn u1(n: u8) -> Arc<Self> {
        match n {
            0 => Value::sum_l(Value::unit()),
            1 => Value::sum_r(Value::unit()),
            x => panic!("{} out of range for Value::u1", x),
        }
    }

    /// Encode a two-bit number as a value. Will panic if the input is out of range
    pub fn u2(n: u8) -> Arc<Self> {
        let b0 = (n & 2) / 2;
        let b1 = n & 1;
        assert!(n <= 3, "{} out of range for Value::u2", n);
        Value::prod(Value::u1(b0), Value::u1(b1))
    }

    /// Encode a four-bit number as a value. Will panic if the input is out of range
    pub fn u4(n: u8) -> Arc<Self> {
        let w0 = (n & 12) / 4;
        let w1 = n & 3;
        assert!(n <= 15, "{} out of range for Value::u2", n);
        Value::prod(Value::u2(w0), Value::u2(w1))
    }

    /// Encode an eight-bit number as a value
    pub fn u8(n: u8) -> Arc<Self> {
        let w0 = n >> 4;
        let w1 = n & 0xf;
        Value::prod(Value::u4(w0), Value::u4(w1))
    }

    /// Encode a 16-bit number as a value
    pub fn u16(n: u16) -> Arc<Self> {
        let w0 = (n >> 8) as u8;
        let w1 = (n & 0xff) as u8;
        Value::prod(Value::u8(w0), Value::u8(w1))
    }

    /// Encode a 32-bit number as a value
    pub fn u32(n: u32) -> Arc<Self> {
        let w0 = (n >> 16) as u16;
        let w1 = (n & 0xffff) as u16;
        Value::prod(Value::u16(w0), Value::u16(w1))
    }

    /// Encode a 64-bit number as a value
    pub fn u64(n: u64) -> Arc<Self> {
        let w0 = (n >> 32) as u32;
        let w1 = (n & 0xffff_ffff) as u32;
        Value::prod(Value::u32(w0), Value::u32(w1))
    }

    /// Encode a 128-bit number as a value
    pub fn u128(n: u128) -> Arc<Self> {
        let w0 = (n >> 64) as u64;
        let w1 = n as u64; // Cast safety: picking last 64 bits
        Value::prod(Value::u64(w0), Value::u64(w1))
    }

    /// Encode a 32-byte number as a value
    ///
    /// Useful for encoding public keys and hashes
    pub fn u256_from_slice(v: &[u8]) -> Arc<Self> {
        assert_eq!(32, v.len(), "Expect 32-byte slice");

        Value::prod(
            Value::prod(
                Value::u64(u64::from_be_bytes(v[0..8].try_into().unwrap())),
                Value::u64(u64::from_be_bytes(v[8..16].try_into().unwrap())),
            ),
            Value::prod(
                Value::u64(u64::from_be_bytes(v[16..24].try_into().unwrap())),
                Value::u64(u64::from_be_bytes(v[24..32].try_into().unwrap())),
            ),
        )
    }

    /// Encode a 64-byte number as a value
    ///
    /// Useful for encoding signatures
    pub fn u512_from_slice(v: &[u8]) -> Arc<Self> {
        assert_eq!(64, v.len(), "Expect 64-byte slice");

        Value::prod(
            Value::u256_from_slice(&v[0..32]),
            Value::u256_from_slice(&v[32..64]),
        )
    }

    /// Encode a byte slice as a value.
    ///
    /// The length of the slice must be a power of two.
    pub fn power_of_two(v: &[u8]) -> Arc<Self> {
        assert!(
            v.len().is_power_of_two(),
            "Slice length must be a power of two"
        );
        let mut values: VecDeque<_> = v.iter().map(|b| Value::u8(*b)).collect();

        while values.len() > 1 {
            let mut alt_values = VecDeque::with_capacity(values.len() / 2);

            while let (Some(left), Some(right)) = (values.pop_front(), values.pop_front()) {
                alt_values.push_back(Value::prod(left, right));
            }

            values = alt_values;
        }

        values.into_iter().next().unwrap()
    }

    /// Execute function `f` on each bit of the encoding of the value.
    pub fn do_each_bit<F>(&self, mut f: F)
    where
        F: FnMut(bool),
    {
        for val in self.pre_order_iter::<NoSharing>() {
            match val {
                Value::Unit => {}
                Value::SumL(..) => f(false),
                Value::SumR(..) => f(true),
                Value::Prod(..) => {}
            }
        }
    }

    /// Encode value as big-endian byte string.
    /// Fails if underlying bit string has length not divisible by 8
    pub fn try_to_bytes(&self) -> Result<Vec<u8>, &'static str> {
        let (bytes, bit_length) = self.to_bytes_len();

        if bit_length % 8 == 0 {
            Ok(bytes)
        } else {
            Err("Length of bit string that encodes this value is not divisible by 8!")
        }
    }

    /// Encode value as big-endian byte string.
    /// Trailing zeroes are added as padding if underlying bit string has length not divisible by 8.
    /// The length of said bit string is returned as second argument
    pub fn to_bytes_len(&self) -> (Vec<u8>, usize) {
        let mut bytes = vec![];
        let mut unfinished_byte = Vec::with_capacity(8);
        let update_bytes = |bit: bool| {
            unfinished_byte.push(bit);

            if unfinished_byte.len() == 8 {
                bytes.push(
                    unfinished_byte
                        .iter()
                        .fold(0, |acc, &b| acc * 2 + u8::from(b)),
                );
                unfinished_byte.clear();
            }
        };

        self.do_each_bit(update_bytes);
        let bit_length = bytes.len() * 8 + unfinished_byte.len();

        if !unfinished_byte.is_empty() {
            unfinished_byte.resize(8, false);
            bytes.push(
                unfinished_byte
                    .iter()
                    .fold(0, |acc, &b| acc * 2 + u8::from(b)),
            );
        }

        (bytes, bit_length)
    }
}

impl fmt::Debug for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(self, f)
    }
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for data in self.verbose_pre_order_iter::<NoSharing>() {
            match data.node {
                Value::Unit => {
                    if data.n_children_yielded == 0
                        && !matches!(data.parent, Some(Value::SumL(_)) | Some(Value::SumR(_)))
                    {
                        f.write_str("ε")?;
                    }
                }
                Value::SumL(..) => {
                    if data.n_children_yielded == 0 {
                        f.write_str("0")?;
                    }
                }
                Value::SumR(..) => {
                    if data.n_children_yielded == 0 {
                        f.write_str("1")?;
                    }
                }
                Value::Prod(..) => match data.n_children_yielded {
                    0 => f.write_str("(")?,
                    1 => f.write_str(",")?,
                    2 => f.write_str(")")?,
                    _ => unreachable!(),
                },
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn value_display() {
        // Only test a couple values becasue we probably want to change this
        // at some point and will have to redo this test.
        assert_eq!(Value::u1(0).to_string(), "0",);
        assert_eq!(Value::u1(1).to_string(), "1",);
        assert_eq!(Value::u4(6).to_string(), "((0,1),(1,0))",);
    }
}
