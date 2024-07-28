// SPDX-License-Identifier: CC0-1.0

//! # Simplicity values
//!
//! Simplicity processes data in terms of [`Value`]s,
//! i.e., inputs, intermediate results and outputs.

use crate::dag::{Dag, DagLike, NoSharing};
use crate::types::Final;

use std::collections::VecDeque;
use std::fmt;
use std::hash::Hash;
use std::sync::Arc;

/// A Simplicity value.
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Value {
    /// The unit value.
    ///
    /// The unit value is the only value of the unit type `1`.
    /// It must be wrapped in left and right values to encode information.
    Unit,
    /// A left value.
    ///
    /// A left value wraps a value of type `A` and its type is the sum `A + B` for some `B`.
    /// A `false` bit encodes that a left value is wrapped.
    Left(Arc<Value>),
    /// A right value.
    ///
    /// A right value wraps a value of type `B` and its type is the sum `A + B` for some `A`.
    /// A `true` bit encodes that a right value is wrapped.
    Right(Arc<Value>),
    /// A product value.
    ///
    /// A product value wraps a left value of type `A` and a right value of type `B`,
    /// and its type is the product `A × B`.
    /// A product value combines the information of its inner values.
    Product(Arc<Value>, Arc<Value>),
}

impl<'a> DagLike for &'a Value {
    type Node = Value;

    fn data(&self) -> &Value {
        self
    }

    fn as_dag_node(&self) -> Dag<Self> {
        match self {
            Value::Unit => Dag::Nullary,
            Value::Left(child) | Value::Right(child) => Dag::Unary(child),
            Value::Product(left, right) => Dag::Binary(left, right),
        }
    }
}

impl Value {
    /// Create the unit value.
    pub fn unit() -> Arc<Self> {
        Arc::new(Self::Unit)
    }

    /// Create a left value that wraps the given `inner` value.
    pub fn left(inner: Arc<Self>) -> Arc<Self> {
        Arc::new(Value::Left(inner))
    }

    /// Create a right value that wraps the given `inner` value.
    pub fn right(inner: Arc<Self>) -> Arc<Self> {
        Arc::new(Value::Right(inner))
    }

    /// Create a product value that wraps the given `left` and `right` values.
    pub fn product(left: Arc<Self>, right: Arc<Self>) -> Arc<Self> {
        Arc::new(Value::Product(left, right))
    }

    /// The length, in bits, of the value when encoded in the Bit Machine
    pub fn len(&self) -> usize {
        self.pre_order_iter::<NoSharing>()
            .filter(|inner| matches!(inner, Value::Left(_) | Value::Right(_)))
            .count()
    }

    /// Check if the value is a nested product of units.
    /// In this case, the value contains no information.
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Check if the value is a unit.
    pub fn is_unit(&self) -> bool {
        matches!(self, Value::Unit)
    }

    /// Access the inner value of a left sum value.
    pub fn as_left(&self) -> Option<&Self> {
        match self {
            Value::Left(inner) => Some(inner.as_ref()),
            _ => None,
        }
    }

    /// Access the inner value of a right sum value.
    pub fn as_right(&self) -> Option<&Self> {
        match self {
            Value::Right(inner) => Some(inner.as_ref()),
            _ => None,
        }
    }

    /// Access the inner values of a product value.
    pub fn as_product(&self) -> Option<(&Self, &Self)> {
        match self {
            Value::Product(left, right) => Some((left.as_ref(), right.as_ref())),
            _ => None,
        }
    }

    /// Encode a single bit as a value. Will panic if the input is out of range
    pub fn u1(n: u8) -> Arc<Self> {
        match n {
            0 => Value::left(Value::unit()),
            1 => Value::right(Value::unit()),
            x => panic!("{} out of range for Value::u1", x),
        }
    }

    /// Encode a two-bit number as a value. Will panic if the input is out of range
    pub fn u2(n: u8) -> Arc<Self> {
        let b0 = (n & 2) / 2;
        let b1 = n & 1;
        assert!(n <= 3, "{} out of range for Value::u2", n);
        Value::product(Value::u1(b0), Value::u1(b1))
    }

    /// Encode a four-bit number as a value. Will panic if the input is out of range
    pub fn u4(n: u8) -> Arc<Self> {
        let w0 = (n & 12) / 4;
        let w1 = n & 3;
        assert!(n <= 15, "{} out of range for Value::u2", n);
        Value::product(Value::u2(w0), Value::u2(w1))
    }

    /// Encode an eight-bit number as a value
    pub fn u8(n: u8) -> Arc<Self> {
        let w0 = n >> 4;
        let w1 = n & 0xf;
        Value::product(Value::u4(w0), Value::u4(w1))
    }

    /// Encode a 16-bit number as a value
    pub fn u16(n: u16) -> Arc<Self> {
        let w0 = (n >> 8) as u8;
        let w1 = (n & 0xff) as u8;
        Value::product(Value::u8(w0), Value::u8(w1))
    }

    /// Encode a 32-bit number as a value
    pub fn u32(n: u32) -> Arc<Self> {
        let w0 = (n >> 16) as u16;
        let w1 = (n & 0xffff) as u16;
        Value::product(Value::u16(w0), Value::u16(w1))
    }

    /// Encode a 64-bit number as a value
    pub fn u64(n: u64) -> Arc<Self> {
        let w0 = (n >> 32) as u32;
        let w1 = (n & 0xffff_ffff) as u32;
        Value::product(Value::u32(w0), Value::u32(w1))
    }

    /// Encode a 128-bit number as a value
    pub fn u128(n: u128) -> Arc<Self> {
        let w0 = (n >> 64) as u64;
        let w1 = n as u64; // Cast safety: picking last 64 bits
        Value::product(Value::u64(w0), Value::u64(w1))
    }

    /// Create a value from 32 bytes.
    pub fn u256(bytes: [u8; 32]) -> Arc<Self> {
        Value::from_byte_array(bytes)
    }

    /// Create a value from 64 bytes.
    pub fn u512(bytes: [u8; 64]) -> Arc<Self> {
        Value::from_byte_array(bytes)
    }

    /// Create a value from a byte array.
    ///
    /// ## Panics
    ///
    /// The array length is not a power of two.
    pub fn from_byte_array<const N: usize>(bytes: [u8; N]) -> Arc<Self> {
        assert!(N.is_power_of_two(), "Array length must be a power of two");
        let mut values: VecDeque<_> = bytes.into_iter().map(Value::u8).collect();

        while values.len() > 1 {
            let mut alt_values = VecDeque::with_capacity(values.len() / 2);

            while let (Some(left), Some(right)) = (values.pop_front(), values.pop_front()) {
                alt_values.push_back(Value::product(left, right));
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
                Value::Left(..) => f(false),
                Value::Right(..) => f(true),
                Value::Product(..) => {}
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

    /// Check if the value is of the given type.
    pub fn is_of_type(&self, ty: &Final) -> bool {
        let mut stack = vec![(self, ty)];

        while let Some((value, ty)) = stack.pop() {
            if ty.is_unit() {
                if !value.is_unit() {
                    return false;
                }
            } else if let Some((ty_l, ty_r)) = ty.as_sum() {
                if let Some(value_l) = value.as_left() {
                    stack.push((value_l, ty_l));
                } else if let Some(value_r) = value.as_right() {
                    stack.push((value_r, ty_r));
                } else {
                    return false;
                }
            } else if let Some((ty_l, ty_r)) = ty.as_product() {
                if let Some((value_l, value_r)) = value.as_product() {
                    stack.push((value_r, ty_r));
                    stack.push((value_l, ty_l));
                } else {
                    return false;
                }
            }
        }

        true
    }
}

impl fmt::Debug for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(self, f)
    }
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for data in self.verbose_pre_order_iter::<NoSharing>(None) {
            match data.node {
                Value::Unit => {
                    if data.n_children_yielded == 0
                        && !matches!(data.parent, Some(Value::Left(_)) | Some(Value::Right(_)))
                    {
                        f.write_str("ε")?;
                    }
                }
                Value::Left(..) => {
                    if data.n_children_yielded == 0 {
                        f.write_str("0")?;
                    }
                }
                Value::Right(..) => {
                    if data.n_children_yielded == 0 {
                        f.write_str("1")?;
                    }
                }
                Value::Product(..) => match data.n_children_yielded {
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
    use crate::jet::type_name::TypeName;

    #[test]
    fn value_display() {
        // Only test a couple values becasue we probably want to change this
        // at some point and will have to redo this test.
        assert_eq!(Value::u1(0).to_string(), "0",);
        assert_eq!(Value::u1(1).to_string(), "1",);
        assert_eq!(Value::u4(6).to_string(), "((0,1),(1,0))",);
    }

    #[test]
    fn is_of_type() {
        let value_typename = [
            (Value::unit(), TypeName(b"1")),
            (Value::left(Value::unit()), TypeName(b"+11")),
            (Value::right(Value::unit()), TypeName(b"+11")),
            (Value::left(Value::unit()), TypeName(b"+1h")),
            (Value::right(Value::unit()), TypeName(b"+h1")),
            (
                Value::product(Value::unit(), Value::unit()),
                TypeName(b"*11"),
            ),
            (Value::u8(u8::MAX), TypeName(b"c")),
            (Value::u64(u64::MAX), TypeName(b"l")),
        ];

        for (value, typename) in value_typename {
            let ty = typename.to_final();
            assert!(value.is_of_type(ty.as_ref()));
        }
    }
}
