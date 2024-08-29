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
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct Value {
    inner: ValueInner,
    ty: Arc<Final>,
}

/// The inner structure of a Simplicity value.
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum ValueInner {
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
        match &self.inner {
            ValueInner::Unit => Dag::Nullary,
            ValueInner::Left(child) | ValueInner::Right(child) => Dag::Unary(child),
            ValueInner::Product(left, right) => Dag::Binary(left, right),
        }
    }
}

impl Value {
    /// Make a cheap copy of the value.
    pub fn shallow_clone(&self) -> Self {
        Self {
            inner: self.inner.clone(),
            ty: Arc::clone(&self.ty),
        }
    }

    /// Access the type of the value.
    pub fn ty(&self) -> &Final {
        &self.ty
    }

    /// Create the unit value.
    pub fn unit() -> Self {
        Self {
            inner: ValueInner::Unit,
            ty: Final::unit(),
        }
    }

    /// Create a left value that wraps the given `inner` value.
    pub fn left(inner: Self, right: Arc<Final>) -> Self {
        Self {
            ty: Final::sum(Arc::clone(&inner.ty), right),
            inner: ValueInner::Left(Arc::new(inner)),
        }
    }

    /// Create a right value that wraps the given `inner` value.
    pub fn right(left: Arc<Final>, inner: Self) -> Self {
        Self {
            ty: Final::sum(left, Arc::clone(&inner.ty)),
            inner: ValueInner::Right(Arc::new(inner)),
        }
    }

    /// Create a product value that wraps the given `left` and `right` values.
    pub fn product(left: Self, right: Self) -> Self {
        Self {
            ty: Final::product(Arc::clone(&left.ty), Arc::clone(&right.ty)),
            inner: ValueInner::Product(Arc::new(left), Arc::new(right)),
        }
    }

    /// Create a none value.
    pub fn none(right: Arc<Final>) -> Self {
        Self {
            ty: Final::sum(Final::unit(), right),
            inner: ValueInner::Left(Arc::new(Value::unit())),
        }
    }

    /// Create a some value.
    pub fn some(inner: Self) -> Self {
        Self {
            ty: Final::sum(Final::unit(), Arc::clone(&inner.ty)),
            inner: ValueInner::Right(Arc::new(inner)),
        }
    }

    /// Return the bit length of the value in compact encoding.
    pub fn compact_len(&self) -> usize {
        self.iter_compact().count()
    }

    /// Return the bit length of the value in padded encoding.
    pub fn padded_len(&self) -> usize {
        self.iter_padded().count()
    }

    /// Check if the value is a nested product of units.
    /// In this case, the value contains no information.
    pub fn is_empty(&self) -> bool {
        self.pre_order_iter::<NoSharing>()
            .all(|value| matches!(&value.inner, ValueInner::Unit | ValueInner::Product(..)))
    }

    /// Check if the value is a unit.
    pub fn is_unit(&self) -> bool {
        matches!(&self.inner, ValueInner::Unit)
    }

    /// Access the inner value of a left sum value.
    pub fn as_left(&self) -> Option<&Self> {
        match &self.inner {
            ValueInner::Left(inner) => Some(inner.as_ref()),
            _ => None,
        }
    }

    /// Access the inner value of a right sum value.
    pub fn as_right(&self) -> Option<&Self> {
        match &self.inner {
            ValueInner::Right(inner) => Some(inner.as_ref()),
            _ => None,
        }
    }

    /// Access the inner values of a product value.
    pub fn as_product(&self) -> Option<(&Self, &Self)> {
        match &self.inner {
            ValueInner::Product(left, right) => Some((left.as_ref(), right.as_ref())),
            _ => None,
        }
    }

    /// Encode a single bit as a value. Will panic if the input is out of range
    pub fn u1(n: u8) -> Self {
        match n {
            0 => Self::left(Self::unit(), Final::unit()),
            1 => Self::right(Final::unit(), Self::unit()),
            x => panic!("{} out of range for Value::u1", x),
        }
    }

    /// Encode a two-bit number as a value. Will panic if the input is out of range
    pub fn u2(n: u8) -> Self {
        let b0 = (n & 2) / 2;
        let b1 = n & 1;
        assert!(n <= 3, "{} out of range for Value::u2", n);
        Self::product(Self::u1(b0), Self::u1(b1))
    }

    /// Encode a four-bit number as a value. Will panic if the input is out of range
    pub fn u4(n: u8) -> Self {
        let w0 = (n & 12) / 4;
        let w1 = n & 3;
        assert!(n <= 15, "{} out of range for Value::u2", n);
        Self::product(Self::u2(w0), Self::u2(w1))
    }

    /// Encode an eight-bit number as a value
    pub fn u8(n: u8) -> Self {
        let w0 = n >> 4;
        let w1 = n & 0xf;
        Self::product(Self::u4(w0), Self::u4(w1))
    }

    /// Encode a 16-bit number as a value
    pub fn u16(n: u16) -> Self {
        let w0 = (n >> 8) as u8;
        let w1 = (n & 0xff) as u8;
        Self::product(Self::u8(w0), Self::u8(w1))
    }

    /// Encode a 32-bit number as a value
    pub fn u32(n: u32) -> Self {
        let w0 = (n >> 16) as u16;
        let w1 = (n & 0xffff) as u16;
        Self::product(Self::u16(w0), Self::u16(w1))
    }

    /// Encode a 64-bit number as a value
    pub fn u64(n: u64) -> Self {
        let w0 = (n >> 32) as u32;
        let w1 = (n & 0xffff_ffff) as u32;
        Self::product(Self::u32(w0), Self::u32(w1))
    }

    /// Encode a 128-bit number as a value
    pub fn u128(n: u128) -> Self {
        let w0 = (n >> 64) as u64;
        let w1 = n as u64; // Cast safety: picking last 64 bits
        Self::product(Self::u64(w0), Self::u64(w1))
    }

    /// Create a value from 32 bytes.
    pub fn u256(bytes: [u8; 32]) -> Self {
        Self::from_byte_array(bytes)
    }

    /// Create a value from 64 bytes.
    pub fn u512(bytes: [u8; 64]) -> Self {
        Self::from_byte_array(bytes)
    }

    /// Create a value from a byte array.
    ///
    /// ## Panics
    ///
    /// The array length is not a power of two.
    pub fn from_byte_array<const N: usize>(bytes: [u8; N]) -> Self {
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

    /// Return an iterator over the compact bit encoding of the value.
    ///
    /// This encoding is used for writing witness data and for computing IMRs.
    pub fn iter_compact(&self) -> impl Iterator<Item = bool> + '_ {
        self.pre_order_iter::<NoSharing>()
            .filter_map(|value| match &value.inner {
                ValueInner::Left(..) => Some(false),
                ValueInner::Right(..) => Some(true),
                _ => None,
            })
    }

    /// Return an iterator over the padded bit encoding of the value.
    ///
    /// This encoding is used to represent the value in the Bit Machine.
    pub fn iter_padded(&self) -> impl Iterator<Item = bool> + '_ {
        PaddedBitsIter::new(self)
    }

    /// Check if the value is of the given type.
    pub fn is_of_type(&self, ty: &Final) -> bool {
        self.ty.as_ref() == ty
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
            match &data.node.inner {
                ValueInner::Unit => {
                    if data.n_children_yielded == 0
                        && !matches!(
                            data.parent.map(|value| &value.inner),
                            Some(ValueInner::Left(_)) | Some(ValueInner::Right(_))
                        )
                    {
                        f.write_str("ε")?;
                    }
                }
                ValueInner::Left(..) => {
                    if data.n_children_yielded == 0 {
                        f.write_str("0")?;
                    }
                }
                ValueInner::Right(..) => {
                    if data.n_children_yielded == 0 {
                        f.write_str("1")?;
                    }
                }
                ValueInner::Product(..) => match data.n_children_yielded {
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

/// An iterator over the bits of the padded encoding of a [`Value`].
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
struct PaddedBitsIter<'a> {
    stack: Vec<&'a Value>,
    next_padding: Option<usize>,
}

impl<'a> PaddedBitsIter<'a> {
    /// Create an iterator over the bits of the padded encoding of the `value`.
    pub fn new(value: &'a Value) -> Self {
        Self {
            stack: vec![value],
            next_padding: None,
        }
    }
}

impl<'a> Iterator for PaddedBitsIter<'a> {
    type Item = bool;

    fn next(&mut self) -> Option<Self::Item> {
        match self.next_padding {
            Some(0) => {
                self.next_padding = None;
            }
            Some(n) => {
                self.next_padding = Some(n - 1);
                return Some(false);
            }
            None => {}
        }

        while let Some(value) = self.stack.pop() {
            if value.is_unit() {
                // NOP
            } else if let Some(l_value) = value.as_left() {
                let (l_ty, r_ty) = value.ty.as_sum().unwrap();
                self.stack.push(l_value);
                self.next_padding = Some(l_ty.pad_left(r_ty));
                return Some(false);
            } else if let Some(r_value) = value.as_right() {
                let (l_ty, r_ty) = value.ty.as_sum().unwrap();
                self.stack.push(r_value);
                self.next_padding = Some(l_ty.pad_right(r_ty));
                return Some(true);
            } else if let Some((l_value, r_value)) = value.as_product() {
                self.stack.push(r_value);
                self.stack.push(l_value);
            }
        }

        None
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
            (Value::left(Value::unit(), Final::unit()), TypeName(b"+11")),
            (Value::right(Final::unit(), Value::unit()), TypeName(b"+11")),
            (
                Value::left(Value::unit(), Final::two_two_n(8)),
                TypeName(b"+1h"),
            ),
            (
                Value::right(Final::two_two_n(8), Value::unit()),
                TypeName(b"+h1"),
            ),
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
