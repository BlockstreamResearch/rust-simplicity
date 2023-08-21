// Rust Simplicity Library
// Written in 2020 by
//   Andrew Poelstra <apoelstra@blockstream.com>
//   Sanket Kanjalkar <sanket1729@gmail.com>
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

//! # Simplicity values
//!
//! Simplicity processes data in terms of [`Value`]s,
//! i.e., inputs, intermediate results and outputs.

use crate::dag::{Dag, DagLike, NoSharing};

use std::convert::TryInto;
use std::fmt;
use std::hash::Hash;
use std::sync::Arc;

/// Simplicity value.
///
/// _Unit_ is the base value and contains no information.
///
/// The zero bit is represented as the _left sum_ of unit,
/// and the one bit is represented as the _right sum_ of unit.
///
/// Bitstrings are represented as a tree of _products_ (a.k.a. a tuple)
/// with sums of unit as its leaves (a.k.a. bits).
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

    /// Encode a 32-bit number as a value
    pub fn u64(n: u64) -> Arc<Self> {
        let w0 = (n >> 32) as u32;
        let w1 = (n & 0xffff_ffff) as u32;
        Value::prod(Value::u32(w0), Value::u32(w1))
    }

    pub fn u128(n: u128) -> Arc<Self> {
        let w0 = (n >> 64) as u64;
        let w1 = n as u64; // Cast safety: picking last 64 bits
        Value::prod(Value::u64(w0), Value::u64(w1))
    }

    /// Encode a 32 byte number into value. Useful for encoding 32 pubkeys/hashes
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

    /// Encode a 64(pair(32, 32)) byte number into value.
    /// Useful for encoding 64 byte signatures
    pub fn u512_from_slice(v: &[u8]) -> Arc<Self> {
        assert_eq!(64, v.len(), "Expect 64-byte slice");

        Value::prod(
            Value::u256_from_slice(&v[0..32]),
            Value::u256_from_slice(&v[32..64]),
        )
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
    pub fn try_to_bytes(&self) -> Result<Vec<u8>, &str> {
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
