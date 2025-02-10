// SPDX-License-Identifier: CC0-1.0

//! # Simplicity values
//!
//! Simplicity processes data in terms of [`Value`]s,
//! i.e., inputs, intermediate results and outputs.

use crate::dag::{Dag, DagLike};
use crate::types::{CompleteBound, Final};
use crate::BitIter;

use crate::{BitCollector, EarlyEndOfStreamError, Tmr};
use core::{cmp, fmt, iter};
use std::collections::VecDeque;
use std::hash::Hash;
use std::sync::Arc;

/// A Simplicity value.
#[derive(Clone)]
pub struct Value {
    /// The underlying data, in "padded" bit-encoded form, with the leftmost
    /// bits encoded in the most-significant bits. So e.g. right(unit) is 0x80.
    ///
    /// We use the padded form, even though it is space-inefficient (sometimes
    /// significantly so), for a couple reasons: first, this is the representation
    /// used by the bit machine, so we want encoding/decoding to be as fast as
    /// possible. Secondly, when destructuring a value (which we need to do during
    /// scribing and pruning, at least) it is quite difficult to destructure a
    /// compact-encoded product, since we would need to determine the compact
    /// bitlengths of the children, and these quantities are not readily available.
    inner: Arc<[u8]>,
    /// An offset, in bits, at which the actual data starts. This is useful
    /// because it allows constructing sub-values of a value without needing
    /// to construct a new `inner` with all the bits offset.
    bit_offset: usize,
    /// The Simplicity type of the value.
    ty: Arc<Final>,
}

/// Private structure used for iterating over a value in pre-order
#[derive(Debug, Clone)]
struct ValueRef<'v> {
    inner: &'v [u8],
    bit_offset: usize,
    ty: &'v Final,
}

// Because two equal values may have different bit offsets, we must manually
// implement the comparison traits. We do so by first comparing types, which
// is constant overhead (this just compares TMRs). If those match, we know
// the lengths and structures match, so we then compare the underlying byte
// iterators.
impl PartialEq for Value {
    fn eq(&self, other: &Self) -> bool {
        self.ty == other.ty && self.raw_byte_iter().eq(other.raw_byte_iter())
    }
}
impl Eq for Value {}

impl PartialOrd for Value {
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl Ord for Value {
    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
        self.ty
            .cmp(&other.ty)
            .then_with(|| self.raw_byte_iter().cmp(other.raw_byte_iter()))
    }
}

impl core::hash::Hash for Value {
    fn hash<H: core::hash::Hasher>(&self, h: &mut H) {
        b"Simplicity\x1fValue".hash(h);
        self.ty.hash(h);
        for val in self.raw_byte_iter() {
            val.hash(h);
        }
    }
}

impl DagLike for ValueRef<'_> {
    type Node = Self;

    fn data(&self) -> &Self {
        self
    }

    fn as_dag_node(&self) -> Dag<Self> {
        if let Some((left, right)) = self.as_product() {
            Dag::Binary(left, right)
        } else if let Some(left) = self.as_left() {
            Dag::Unary(left)
        } else if let Some(right) = self.as_right() {
            Dag::Unary(right)
        } else {
            assert!(self.ty.is_unit());
            Dag::Nullary
        }
    }
}

impl ValueRef<'_> {
    /// Check if the value is a unit.
    fn is_unit(&self) -> bool {
        self.ty.is_unit()
    }

    /// Helper function to read the first bit of a value
    ///
    /// If the first bit is not available (e.g. if the value has zero size)
    /// then returns None.
    fn first_bit(&self) -> Option<bool> {
        let mask = if self.bit_offset % 8 == 0 {
            0x80
        } else {
            1 << (7 - self.bit_offset % 8)
        };
        let res = self
            .inner
            .get(self.bit_offset / 8)
            .map(|x| x & mask == mask);
        res
    }

    /// Access the inner value of a left sum value.
    pub fn as_left(&self) -> Option<Self> {
        if self.first_bit() == Some(false) {
            if let Some((lty, rty)) = self.ty.as_sum() {
                let sum_width = 1 + cmp::max(lty.bit_width(), rty.bit_width());
                Some(Self {
                    inner: self.inner,
                    bit_offset: self.bit_offset + sum_width - lty.bit_width(),
                    ty: lty,
                })
            } else {
                None
            }
        } else {
            None
        }
    }

    /// Access the inner value of a right sum value.
    pub fn as_right(&self) -> Option<Self> {
        if self.first_bit() == Some(true) {
            if let Some((lty, rty)) = self.ty.as_sum() {
                let sum_width = 1 + cmp::max(lty.bit_width(), rty.bit_width());
                Some(Self {
                    inner: self.inner,
                    bit_offset: self.bit_offset + sum_width - rty.bit_width(),
                    ty: rty,
                })
            } else {
                None
            }
        } else {
            None
        }
    }

    /// Access the inner values of a product value.
    pub fn as_product(&self) -> Option<(Self, Self)> {
        if let Some((lty, rty)) = self.ty.as_product() {
            Some((
                Self {
                    inner: self.inner,
                    bit_offset: self.bit_offset,
                    ty: lty,
                },
                Self {
                    inner: self.inner,
                    bit_offset: self.bit_offset + lty.bit_width(),
                    ty: rty,
                },
            ))
        } else {
            None
        }
    }
}

pub struct RawByteIter<'v> {
    value: &'v Value,
    yielded_bytes: usize,
}

impl Iterator for RawByteIter<'_> {
    type Item = u8;
    fn next(&mut self) -> Option<Self::Item> {
        if 8 * self.yielded_bytes >= self.value.ty.bit_width() {
            None
        } else if self.value.bit_offset % 8 == 0 {
            self.yielded_bytes += 1;

            Some(self.value.inner[self.value.bit_offset / 8 + self.yielded_bytes - 1])
        } else {
            self.yielded_bytes += 1;

            let ret1 = self.value.inner[self.value.bit_offset / 8 + self.yielded_bytes - 1];
            let ret2 = self
                .value
                .inner
                .get(self.value.bit_offset / 8 + self.yielded_bytes)
                .copied()
                .unwrap_or(0);
            let bit_offset = self.value.bit_offset % 8;
            Some((ret1 << bit_offset) | (ret2 >> (8 - bit_offset)))
        }
    }
}

pub struct PreOrderIter<'v> {
    inner: iter::Take<BitIter<RawByteIter<'v>>>,
}

impl Iterator for PreOrderIter<'_> {
    type Item = bool;
    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next()
    }
}

/// Helper function to right-shift a value by one bit.
///
/// This is used for putting a value into a sum type. It takes the value's
/// bit encoding and current bit offset, and returns a new bit-encoding with
/// a new bit inserted upfront and a new bit offset.
fn right_shift_1(inner: &Arc<[u8]>, bit_offset: usize, new_bit: bool) -> (Arc<[u8]>, usize) {
    // If the current bit offset is nonzero this is super easy: we just
    // lower the bit offset and call that a fix.
    if bit_offset > 0 {
        if new_bit {
            let new_bit_offset = bit_offset - 1;
            let mut bx: Box<[u8]> = inner.as_ref().into();
            bx[new_bit_offset / 8] |= 1 << (7 - new_bit_offset % 8);
            (bx.into(), new_bit_offset)
        } else {
            // ...and if we are inserting a 0 we don't even need to allocate a new [u8]
            (Arc::clone(inner), bit_offset - 1)
        }
    } else {
        // If the current bit offset is 0, we just shift everything right by 8
        // and then do pretty-much the same thing as above. This sometimes will
        // waste 7 bits, but it avoids needing to iterate through all of `inner`.
        let mut new = Vec::with_capacity(inner.len() + 1);
        new.push(u8::from(new_bit));
        new.extend(inner.iter().copied());
        (new.into(), 7)
    }
}

/// Helper function to copy `nbits` bits from `src`, starting at bit-offset `src_offset`,
/// into `dst`, starting at bit-offset `dst_offset`.
///
/// Thanks ChatGPT for suggesting we extract this function.
fn copy_bits(src: &[u8], src_offset: usize, dst: &mut [u8], dst_offset: usize, nbits: usize) {
    // For each bit i in 0..nbits, extract the bit from `src`
    // and insert it into `dst`.
    for i in 0..nbits {
        let bit = (src[(src_offset + i) / 8] >> (7 - (src_offset + i) % 8)) & 1;
        dst[(dst_offset + i) / 8] |= bit << (7 - (dst_offset + i) % 8);
    }
}

/// Helper function to take the product of two values (i.e. their concatenation).
///
/// If either `left_inner` or `right_inner` is not provided, it is assumed to be
/// padding and will be stored as all zeros.
///
/// Returns the new bit data and the offset (NOT the length) of the data.
fn product(
    left: Option<(&Arc<[u8]>, usize)>,
    left_bit_length: usize,
    right: Option<(&Arc<[u8]>, usize)>,
    right_bit_length: usize,
) -> (Arc<[u8]>, usize) {
    if left_bit_length == 0 {
        if let Some((right, right_bit_offset)) = right {
            (Arc::clone(right), right_bit_offset)
        } else if right_bit_length == 0 {
            (Arc::new([]), 0)
        } else {
            (Arc::from(vec![0; (right_bit_length + 7) / 8]), 0)
        }
    } else if right_bit_length == 0 {
        if let Some((lt, left_bit_offset)) = left {
            (Arc::clone(lt), left_bit_offset)
        } else {
            (Arc::from(vec![0; (left_bit_length + 7) / 8]), 0)
        }
    } else {
        // Both left and right have nonzero lengths. This is the only "real" case
        // in which we have to do something beyond cloning Arcs or allocating
        // zeroed vectors. In this case we left-shift both as much as possible.
        let mut bx = Box::<[u8]>::from(vec![0; (left_bit_length + right_bit_length + 7) / 8]);
        if let Some((left, left_bit_offset)) = left {
            copy_bits(left, left_bit_offset, &mut bx, 0, left_bit_length);
        }
        if let Some((right, right_bit_offset)) = right {
            copy_bits(
                right,
                right_bit_offset,
                &mut bx,
                left_bit_length,
                right_bit_length,
            );
        }

        (bx.into(), 0)
    }
}

impl Value {
    /// Make a cheap copy of the value.
    pub fn shallow_clone(&self) -> Self {
        Self {
            inner: Arc::clone(&self.inner),
            bit_offset: self.bit_offset,
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
            inner: Arc::new([]),
            bit_offset: 0,
            ty: Final::unit(),
        }
    }

    /// Create a left value that wraps the given `inner` value.
    pub fn left(inner: Self, right: Arc<Final>) -> Self {
        let total_width = cmp::max(inner.ty.bit_width(), right.bit_width());

        let (concat, concat_offset) = product(
            Some((&inner.inner, inner.bit_offset)),
            inner.ty.bit_width(),
            None,
            total_width - inner.ty.bit_width(),
        );
        let (new_inner, new_bit_offset) = right_shift_1(&concat, concat_offset, false);
        Self {
            inner: new_inner,
            bit_offset: new_bit_offset,
            ty: Final::sum(Arc::clone(&inner.ty), right),
        }
    }

    /// Create a right value that wraps the given `inner` value.
    pub fn right(left: Arc<Final>, inner: Self) -> Self {
        let total_width = cmp::max(left.bit_width(), inner.ty.bit_width());

        let (concat, concat_offset) = product(
            None,
            total_width - inner.ty.bit_width(),
            Some((&inner.inner, inner.bit_offset)),
            inner.ty.bit_width(),
        );
        let (new_inner, new_bit_offset) = right_shift_1(&concat, concat_offset, true);
        Self {
            inner: new_inner,
            bit_offset: new_bit_offset,
            ty: Final::sum(left, Arc::clone(&inner.ty)),
        }
    }

    /// Create a product value that wraps the given `left` and `right` values.
    pub fn product(left: Self, right: Self) -> Self {
        let (new_inner, new_bit_offset) = product(
            Some((&left.inner, left.bit_offset)),
            left.ty.bit_width(),
            Some((&right.inner, right.bit_offset)),
            right.ty.bit_width(),
        );
        Self {
            inner: new_inner,
            bit_offset: new_bit_offset,
            ty: Final::product(Arc::clone(&left.ty), Arc::clone(&right.ty)),
        }
    }

    /// Create a none value.
    pub fn none(right: Arc<Final>) -> Self {
        Self::left(Value::unit(), right)
    }

    /// Create a some value.
    pub fn some(inner: Self) -> Self {
        Self::right(Final::unit(), inner)
    }

    /// Return the bit length of the value in compact encoding.
    pub fn compact_len(&self) -> usize {
        self.iter_compact().count()
    }

    /// Return the bit length of the value in padded encoding.
    pub fn padded_len(&self) -> usize {
        self.ty.bit_width()
    }

    /// Check if the value is a nested product of units.
    /// In this case, the value contains no information.
    pub fn is_empty(&self) -> bool {
        self.ty.bit_width() == 0
    }

    /// Check if the value is a unit.
    pub fn is_unit(&self) -> bool {
        self.ty.is_unit()
    }

    /// Helper function to convert the value to a reference
    fn as_value(&self) -> ValueRef {
        ValueRef {
            inner: self.inner.as_ref(),
            bit_offset: self.bit_offset,
            ty: self.ty.as_ref(),
        }
    }

    /// Helper function to read the first bit of a value
    ///
    /// If the first bit is not available (e.g. if the value has zero size)
    /// then returns None.
    fn first_bit(&self) -> Option<bool> {
        let mask = if self.bit_offset % 8 == 0 {
            0x80
        } else {
            1 << (7 - self.bit_offset % 8)
        };
        let res = self
            .inner
            .get(self.bit_offset / 8)
            .map(|x| x & mask == mask);
        res
    }

    /// Access the inner value of a left sum value.
    pub fn to_left(&self) -> Option<Self> {
        if self.first_bit() == Some(false) {
            if let Some((lty, _)) = self.ty.as_sum() {
                Some(Self {
                    inner: Arc::clone(&self.inner),
                    bit_offset: self.bit_offset + self.ty.bit_width() - lty.bit_width(),
                    ty: Arc::clone(lty),
                })
            } else {
                None
            }
        } else {
            None
        }
    }

    /// Access the inner value of a right sum value.
    pub fn to_right(&self) -> Option<Self> {
        if self.first_bit() == Some(true) {
            if let Some((_, rty)) = self.ty.as_sum() {
                Some(Self {
                    inner: Arc::clone(&self.inner),
                    bit_offset: self.bit_offset + self.ty.bit_width() - rty.bit_width(),
                    ty: Arc::clone(rty),
                })
            } else {
                None
            }
        } else {
            None
        }
    }

    /// Access the inner values of a product value.
    pub fn to_product(&self) -> Option<(Self, Self)> {
        if let Some((lty, rty)) = self.ty.as_product() {
            Some((
                Self {
                    inner: Arc::clone(&self.inner),
                    bit_offset: self.bit_offset,
                    ty: Arc::clone(lty),
                },
                Self {
                    inner: Arc::clone(&self.inner),
                    bit_offset: self.bit_offset + lty.bit_width(),
                    ty: Arc::clone(rty),
                },
            ))
        } else {
            None
        }
    }

    /// Create a 1-bit integer.
    ///
    /// ## Panics
    ///
    /// The value is out of range.
    pub fn u1(value: u8) -> Self {
        assert!(value <= 1, "{} out of range for Value::u1", value);
        Self {
            inner: Arc::new([value]),
            bit_offset: 7,
            ty: Final::two_two_n(0),
        }
    }

    /// Create a 2-bit integer.
    ///
    /// ## Panics
    ///
    /// The value is out of range.
    pub fn u2(value: u8) -> Self {
        assert!(value <= 3, "{} out of range for Value::u2", value);
        Self {
            inner: Arc::new([value]),
            bit_offset: 6,
            ty: Final::two_two_n(1),
        }
    }

    /// Create a 4-bit integer.
    ///
    /// ## Panics
    ///
    /// The value is ouf of range.
    pub fn u4(value: u8) -> Self {
        assert!(value <= 15, "{} out of range for Value::u2", value);
        Self {
            inner: Arc::new([value]),
            bit_offset: 4,
            ty: Final::two_two_n(2),
        }
    }

    /// Create an 8-bit integer.
    pub fn u8(value: u8) -> Self {
        Self {
            inner: Arc::new([value]),
            bit_offset: 0,
            ty: Final::two_two_n(3),
        }
    }

    /// Create a 16-bit integer.
    pub fn u16(bytes: u16) -> Self {
        Self {
            inner: Arc::new(bytes.to_be_bytes()),
            bit_offset: 0,
            ty: Final::two_two_n(4),
        }
    }

    /// Create a 32-bit integer.
    pub fn u32(bytes: u32) -> Self {
        Self {
            inner: Arc::new(bytes.to_be_bytes()),
            bit_offset: 0,
            ty: Final::two_two_n(5),
        }
    }

    /// Create a 64-bit integer.
    pub fn u64(bytes: u64) -> Self {
        Self {
            inner: Arc::new(bytes.to_be_bytes()),
            bit_offset: 0,
            ty: Final::two_two_n(6),
        }
    }

    /// Create a 128-bit integer.
    pub fn u128(bytes: u128) -> Self {
        Self {
            inner: Arc::new(bytes.to_be_bytes()),
            bit_offset: 0,
            ty: Final::two_two_n(7),
        }
    }

    /// Create a 256-bit integer.
    pub fn u256(bytes: [u8; 32]) -> Self {
        Self {
            inner: Arc::new(bytes),
            bit_offset: 0,
            ty: Final::two_two_n(8),
        }
    }

    /// Create a 512-bit integer.
    pub fn u512(bytes: [u8; 64]) -> Self {
        Self {
            inner: Arc::new(bytes),
            bit_offset: 0,
            ty: Final::two_two_n(9),
        }
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

    /// Yields an iterator over the "raw bytes" of the value.
    ///
    /// The returned bytes match the padded bit-encoding of the value. You
    /// may wish to call [`Self::iter_padded`] instead to obtain the bits,
    /// but this method is more efficient in some contexts.
    pub fn raw_byte_iter(&self) -> RawByteIter {
        RawByteIter {
            value: self,
            yielded_bytes: 0,
        }
    }

    /// Return an iterator over the compact bit encoding of the value.
    ///
    /// This encoding is used for writing witness data and for computing IMRs.
    pub fn iter_compact(&self) -> CompactBitsIter {
        CompactBitsIter::new(self.as_value())
    }

    /// Return an iterator over the padded bit encoding of the value.
    ///
    /// This encoding is used to represent the value in the Bit Machine.
    pub fn iter_padded(&self) -> PreOrderIter {
        PreOrderIter {
            inner: BitIter::new(self.raw_byte_iter()).take(self.ty.bit_width()),
        }
    }

    /// Check if the value is of the given type.
    pub fn is_of_type(&self, ty: &Final) -> bool {
        self.ty.as_ref() == ty
    }

    /// Get the zero value for the given type.
    ///
    /// The zero value serializes to a string of zeroes.
    ///
    /// ## Construction
    ///
    /// - `zero( 1 )` = `()`
    /// - `zero( A + B )` = `zero(A)`
    /// - `zero( A × B )` = `zero(A) × zero(B)`
    pub fn zero(ty: &Final) -> Self {
        enum Task<'a> {
            ZeroValue(&'a Final),
            MakeLeft(Arc<Final>),
            MakeProduct,
        }

        let mut output = vec![];
        let mut stack = vec![Task::ZeroValue(ty)];

        while let Some(task) = stack.pop() {
            match task {
                Task::ZeroValue(ty) => match ty.bound() {
                    CompleteBound::Unit => output.push(Value::unit()),
                    CompleteBound::Sum(l_ty, r_ty) => {
                        stack.push(Task::MakeLeft(Arc::clone(r_ty)));
                        stack.push(Task::ZeroValue(l_ty));
                    }
                    CompleteBound::Product(l_ty, r_ty) => {
                        stack.push(Task::MakeProduct);
                        stack.push(Task::ZeroValue(r_ty));
                        stack.push(Task::ZeroValue(l_ty));
                    }
                },
                Task::MakeLeft(r_ty) => {
                    let l_value = output.pop().unwrap();
                    output.push(Value::left(l_value, r_ty));
                }
                Task::MakeProduct => {
                    let r_value = output.pop().unwrap();
                    let l_value = output.pop().unwrap();
                    output.push(Value::product(l_value, r_value));
                }
            }
        }

        debug_assert_eq!(output.len(), 1);
        output.pop().unwrap()
    }

    /// Try to convert the value into a word.
    ///
    /// The value is cheaply cloned.
    pub fn to_word(&self) -> Option<Word> {
        self.ty.as_word().map(|n| Word {
            value: self.shallow_clone(),
            n,
        })
    }

    /// Prune the value down to the given type.
    ///
    /// The pruned type must be _smaller than or equal to_ the current type of the value.
    /// Otherwise, this method returns `None`.
    ///
    /// ## Smallness
    ///
    /// - `T` ≤ `T` for all types `T`
    /// - `1` ≤ `T` for all types `T`
    /// - `A1 + B1` ≤ `A2 + B2` if `A1` ≤ `A2` and `B1` ≤ `B2`
    /// - `A1 × B1` ≤ `A2 × B2` if `A1` ≤ `A2` and `B1` ≤ `B2`
    ///
    /// ## Pruning
    ///
    /// - `prune( v: T, 1 )` = `(): 1`
    /// - `prune( L(l): A1 + B1, A2 + B2 )` = `prune(l: A1, A2) : A2 + B2`
    /// - `prune( R(r): A1 + B1, A2 + B2 )` = `prune(r: B1, B2) : A2 + B2`
    /// - `prune( (l, r): A1 × B1, A2 × B2 )` = `( prune(l: A1, A2), prune(r: B1, B2): A2 × B2`
    pub fn prune(&self, pruned_ty: &Final) -> Option<Self> {
        enum Task<'ty> {
            Prune(Value, &'ty Final),
            MakeLeft(Arc<Final>),
            MakeRight(Arc<Final>),
            MakeProduct,
        }

        let mut stack = vec![Task::Prune(self.shallow_clone(), pruned_ty)];
        let mut output = vec![];

        while let Some(task) = stack.pop() {
            match task {
                Task::Prune(value, pruned_ty) => match pruned_ty.bound() {
                    CompleteBound::Unit => output.push(Value::unit()),
                    CompleteBound::Sum(l_ty, r_ty) => {
                        if let Some(l_value) = value.to_left() {
                            stack.push(Task::MakeLeft(Arc::clone(r_ty)));
                            stack.push(Task::Prune(l_value, l_ty));
                        } else {
                            let r_value = value.to_right()?;
                            stack.push(Task::MakeRight(Arc::clone(l_ty)));
                            stack.push(Task::Prune(r_value, r_ty));
                        }
                    }
                    CompleteBound::Product(l_ty, r_ty) => {
                        let (l_value, r_value) = value.to_product()?;
                        stack.push(Task::MakeProduct);
                        stack.push(Task::Prune(r_value, r_ty));
                        stack.push(Task::Prune(l_value, l_ty));
                    }
                },
                Task::MakeLeft(r_ty) => {
                    let l_value = output.pop().unwrap();
                    output.push(Value::left(l_value, r_ty));
                }
                Task::MakeRight(l_ty) => {
                    let r_value = output.pop().unwrap();
                    output.push(Value::right(l_ty, r_value));
                }
                Task::MakeProduct => {
                    let r_value = output.pop().unwrap();
                    let l_value = output.pop().unwrap();
                    output.push(Value::product(l_value, r_value));
                }
            }
        }

        debug_assert_eq!(output.len(), 1);
        output.pop()
    }
}

impl fmt::Debug for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("Value")
            .field("value", &format_args!("{}", self))
            .field("ty", &self.ty)
            .field("raw_value", &self.inner)
            .field("raw_bit_offset", &self.bit_offset)
            .finish()
    }
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // This is a copy of the logic inside `CompactBitsIter` except
        // that we handle products more explicitly.
        enum S<'v> {
            Disp(ValueRef<'v>),
            DispUnlessUnit(ValueRef<'v>),
            DispCh(char),
        }

        let mut stack = Vec::with_capacity(1024);
        // Next node to visit, and a boolean indicating whether we should
        // display units explicitly (turned off for sums, since a sum of
        // a unit is displayed simply as 0 or 1.
        stack.push(S::Disp(self.as_value()));

        while let Some(next) = stack.pop() {
            let value = match next {
                S::Disp(ref value) | S::DispUnlessUnit(ref value) => value,
                S::DispCh(ch) => {
                    write!(f, "{}", ch)?;
                    continue;
                }
            };

            if value.is_unit() {
                if !matches!(next, S::DispUnlessUnit(..)) {
                    f.write_str("ε")?;
                }
            } else if let Some(l_value) = value.as_left() {
                f.write_str("0")?;
                stack.push(S::DispUnlessUnit(l_value));
            } else if let Some(r_value) = value.as_right() {
                f.write_str("1")?;
                stack.push(S::DispUnlessUnit(r_value));
            } else if let Some((l_value, r_value)) = value.as_product() {
                stack.push(S::DispCh(')'));
                stack.push(S::Disp(r_value));
                stack.push(S::DispCh(','));
                stack.push(S::Disp(l_value));
                stack.push(S::DispCh('('));
            } else {
                unreachable!()
            }
        }
        Ok(())
    }
}

/// An iterator over the bits of the compact encoding of a [`Value`].
#[derive(Debug, Clone)]
pub struct CompactBitsIter<'v> {
    stack: Vec<ValueRef<'v>>,
}

impl<'v> CompactBitsIter<'v> {
    /// Create an iterator over the bits of the compact encoding of the `value`.
    fn new(value: ValueRef<'v>) -> Self {
        Self { stack: vec![value] }
    }
}

impl Iterator for CompactBitsIter<'_> {
    type Item = bool;

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(value) = self.stack.pop() {
            if value.is_unit() {
                // NOP
            } else if let Some(l_value) = value.as_left() {
                self.stack.push(l_value);
                return Some(false);
            } else if let Some(r_value) = value.as_right() {
                self.stack.push(r_value);
                return Some(true);
            } else if let Some((l_value, r_value)) = value.as_product() {
                self.stack.push(r_value);
                self.stack.push(l_value);
            }
        }

        None
    }
}

trait Padding {
    fn read_left_padding<I: Iterator<Item = bool>>(
        bits: &mut I,
        ty_l: &Final,
        ty_r: &Final,
    ) -> Result<(), EarlyEndOfStreamError>;

    fn read_right_padding<I: Iterator<Item = bool>>(
        bits: &mut I,
        ty_l: &Final,
        ty_r: &Final,
    ) -> Result<(), EarlyEndOfStreamError>;
}

enum CompactEncoding {}
enum PaddedEncoding {}

impl Padding for CompactEncoding {
    fn read_left_padding<I: Iterator<Item = bool>>(
        _: &mut I,
        _: &Final,
        _: &Final,
    ) -> Result<(), EarlyEndOfStreamError> {
        // no padding
        Ok(())
    }

    fn read_right_padding<I: Iterator<Item = bool>>(
        _: &mut I,
        _: &Final,
        _: &Final,
    ) -> Result<(), EarlyEndOfStreamError> {
        // no padding
        Ok(())
    }
}

impl Padding for PaddedEncoding {
    fn read_left_padding<I: Iterator<Item = bool>>(
        bits: &mut I,
        ty_l: &Final,
        ty_r: &Final,
    ) -> Result<(), EarlyEndOfStreamError> {
        for _ in 0..ty_l.pad_left(ty_r) {
            let _padding = bits.next().ok_or(EarlyEndOfStreamError)?;
        }
        Ok(())
    }

    fn read_right_padding<I: Iterator<Item = bool>>(
        bits: &mut I,
        ty_l: &Final,
        ty_r: &Final,
    ) -> Result<(), EarlyEndOfStreamError> {
        for _ in 0..ty_l.pad_right(ty_r) {
            let _padding = bits.next().ok_or(EarlyEndOfStreamError)?;
        }
        Ok(())
    }
}

impl Value {
    fn from_bits<I: Iterator<Item = u8>, P: Padding>(
        bits: &mut BitIter<I>,
        ty: &Final,
    ) -> Result<Self, EarlyEndOfStreamError> {
        enum State<'a> {
            ProcessType(&'a Final),
            DoSumL(Arc<Final>),
            DoSumR(Arc<Final>),
            DoProduct,
        }

        let mut stack = vec![State::ProcessType(ty)];
        let mut result_stack = vec![];
        'stack_loop: while let Some(state) = stack.pop() {
            match state {
                State::ProcessType(ty) if ty.tmr() == Tmr::POWERS_OF_TWO[0] => {
                    result_stack.push(Value::u1(bits.read_bit()?.into()));
                }
                State::ProcessType(ty) if ty.tmr() == Tmr::POWERS_OF_TWO[1] => {
                    result_stack.push(Value::u2(bits.read_u2()?.into()));
                }
                State::ProcessType(ty) if ty.tmr() == Tmr::POWERS_OF_TWO[2] => {
                    let u4 = (u8::from(bits.read_u2()?) << 2) + u8::from(bits.read_u2()?);
                    result_stack.push(Value::u4(u4));
                }
                State::ProcessType(ty) => {
                    // The POWERS_OF_TWO array is somewhat misnamed; the ith index contains
                    // the TMR of TWO^(2^n). So e.g. the 0th index is 2 (a bit), the 1st is
                    // u2, then u4, and the 3rd is u8.
                    for (logn, tmr) in Tmr::POWERS_OF_TWO.iter().skip(3).enumerate() {
                        if ty.tmr() == *tmr {
                            let mut blob = Vec::with_capacity(1 << logn);
                            for _ in 0..blob.capacity() {
                                blob.push(bits.read_u8()?);
                            }
                            result_stack.push(Value {
                                inner: blob.into(),
                                bit_offset: 0,
                                ty: Final::two_two_n(logn + 3),
                            });
                            continue 'stack_loop;
                        }
                    }

                    match ty.bound() {
                        CompleteBound::Unit => result_stack.push(Value::unit()),
                        CompleteBound::Sum(ref l, ref r) => {
                            if !bits.next().ok_or(EarlyEndOfStreamError)? {
                                P::read_left_padding(bits, l, r)?;
                                stack.push(State::DoSumL(Arc::clone(r)));
                                stack.push(State::ProcessType(l));
                            } else {
                                P::read_right_padding(bits, l, r)?;
                                stack.push(State::DoSumR(Arc::clone(l)));
                                stack.push(State::ProcessType(r));
                            }
                        }
                        CompleteBound::Product(ref l, ref r) => {
                            stack.push(State::DoProduct);
                            stack.push(State::ProcessType(r));
                            stack.push(State::ProcessType(l));
                        }
                    }
                }
                State::DoSumL(r) => {
                    let val = result_stack.pop().unwrap();
                    result_stack.push(Value::left(val, r));
                }
                State::DoSumR(l) => {
                    let val = result_stack.pop().unwrap();
                    result_stack.push(Value::right(l, val));
                }
                State::DoProduct => {
                    let val_r = result_stack.pop().unwrap();
                    let val_l = result_stack.pop().unwrap();
                    result_stack.push(Value::product(val_l, val_r));
                }
            }
        }
        debug_assert_eq!(result_stack.len(), 1);
        Ok(result_stack.pop().unwrap())
    }

    /// Decode a value of the given type from its compact bit encoding.
    pub fn from_compact_bits<I: Iterator<Item = u8>>(
        bits: &mut BitIter<I>,
        ty: &Final,
    ) -> Result<Self, EarlyEndOfStreamError> {
        Self::from_bits::<_, CompactEncoding>(bits, ty)
    }

    /// Decode a value of the given type from its padded bit encoding.
    pub fn from_padded_bits<I: Iterator<Item = u8>>(
        bits: &mut BitIter<I>,
        ty: &Final,
    ) -> Result<Self, EarlyEndOfStreamError> {
        Self::from_bits::<_, PaddedEncoding>(bits, ty)
    }
}

/// A Simplicity word. A value of type `TWO^(2^n)` for some `0 ≤ n < 32`.
#[derive(Clone, Eq, PartialEq, PartialOrd, Ord, Hash)]
pub struct Word {
    /// Value of type `TWO^(2^n)`.
    value: Value,
    /// 0 ≤ n < 32.
    n: u32,
}

macro_rules! construct_word_fallible {
    ($name: ident, $n: expr, $text: expr) => {
        #[doc = "Create"]
        #[doc = $text]
        #[doc = "word.\n\n"]
        #[doc = "## Panics\n"]
        #[doc = "The value is ouf of range."]
        pub fn $name(bit: u8) -> Self {
            Self {
                value: Value::$name(bit),
                n: $n,
            }
        }
    };
}

macro_rules! construct_word {
    ($name: ident, $ty: ty, $n: expr, $text: expr) => {
        #[doc = "Create"]
        #[doc = $text]
        #[doc = "word."]
        pub fn $name(bit: $ty) -> Self {
            Self {
                value: Value::$name(bit),
                n: $n,
            }
        }
    };
}

impl Word {
    /// Concatenate two words into a larger word.
    ///
    /// Both words have to have the same length, which is 2^n bits.
    /// The resulting word will be 2^(n + 1) bits long.
    ///
    /// Returns `None` if the words differ in length.
    ///
    /// Returns `None` if the words are already 2^31 bits long
    /// _(the resulting word would be longer than 2^31 bits, which is not supported)_.
    pub fn product(self, right: Self) -> Option<Self> {
        if self.n == right.n && self.n < 30 {
            Some(Self {
                value: Value::product(self.value, right.value),
                n: self.n + 1,
            })
        } else {
            None
        }
    }

    construct_word_fallible!(u1, 0, "a 1-bit");
    construct_word_fallible!(u2, 1, "a 2-bit");
    construct_word_fallible!(u4, 2, "a 4-bit");
    construct_word!(u8, u8, 3, "an 8-bit");
    construct_word!(u16, u16, 4, "a 16-bit");
    construct_word!(u32, u32, 5, "a 32-bit");
    construct_word!(u64, u64, 6, "a 64-bit");
    construct_word!(u128, u128, 7, "a 128-bit");
    construct_word!(u256, [u8; 32], 8, "a 256-bit");
    construct_word!(u512, [u8; 64], 9, "a 512-bit");

    /// Make a cheap copy of the word.
    pub fn shallow_clone(&self) -> Self {
        Self {
            value: self.value.shallow_clone(),
            n: self.n,
        }
    }

    /// Access the value of the word.
    pub fn as_value(&self) -> &Value {
        &self.value
    }

    /// The word is of type `TWO^(2^n)`. Return `n`.
    pub fn n(&self) -> u32 {
        self.n
    }

    /// Return the bit length of the word.
    ///
    /// The word is of type `TWO^(2^n)`. Return `2^n`.
    #[allow(clippy::len_without_is_empty)]
    pub fn len(&self) -> usize {
        2usize.pow(self.n)
    }

    /// Return an iterator over the bit encoding of the word.
    ///
    /// Words have no padding, so their compact encoding is the same as the padded encoding.
    /// The universal encoding can be used in all situations.
    pub fn iter(&self) -> impl Iterator<Item = bool> + '_ {
        self.value.iter_compact()
    }

    /// Decode a word of type `TWO^(2^n)` from bits.
    ///
    /// ## Panics
    ///
    /// n is greater than 31.
    pub fn from_bits<I: Iterator<Item = u8>>(
        bits: &mut BitIter<I>,
        n: u32,
    ) -> Result<Self, EarlyEndOfStreamError> {
        assert!(n < 32, "TWO^(2^{n}) is not supported as a word type");
        let ty = Final::two_two_n(n as usize); // cast safety: 32-bit machine or higher
        let value = Value::from_compact_bits(bits, &ty)?;
        Ok(Self { value, n })
    }
}

impl fmt::Debug for Word {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(self, f)
    }
}

impl fmt::Display for Word {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use hex::DisplayHex;

        if let Ok(hex) = self.iter().try_collect_bytes() {
            write!(f, "0x{}", hex.as_hex())
        } else {
            f.write_str("0b")?;
            for bit in self.iter() {
                match bit {
                    false => f.write_str("0")?,
                    true => f.write_str("1")?,
                }
            }
            Ok(())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::bit_encoding::{BitCollector as _, BitIter};
    use crate::jet::type_name::TypeName;

    #[test]
    fn value_len() {
        let v = Value::u4(6);
        let s_v = Value::some(v.shallow_clone());
        let n_v = Value::none(Final::two_two_n(2));

        assert_eq!(v.compact_len(), 4);
        assert_eq!(v.padded_len(), 4);
        assert_eq!(s_v.compact_len(), 5);
        assert_eq!(s_v.padded_len(), 5);
        assert_eq!(n_v.compact_len(), 1);
        assert_eq!(n_v.padded_len(), 5);
    }

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

    #[test]
    fn prune_regression_1() {
        // Found this when fuzzing Elements; unsure how to reduce it further.
        let nontrivial_sum = Value::product(
            Value::right(Final::two_two_n(4), Value::u16(0)),
            Value::u8(0),
        );
        // Formatting should succeed and have no effect.
        let _ = format!("{nontrivial_sum}");
        // Pruning should succeed and have no effect.
        assert_eq!(
            nontrivial_sum.prune(nontrivial_sum.ty()),
            Some(nontrivial_sum)
        );
    }

    #[test]
    fn prune() {
        let test_vectors = [
            (Value::unit(), Value::unit()),
            (Value::u64(42), Value::unit()),
            (
                Value::left(Value::u64(42), Final::u64()),
                Value::left(Value::u64(42), Final::unit()),
            ),
            (
                Value::right(Final::u64(), Value::u64(1337)),
                Value::right(Final::unit(), Value::u64(1337)),
            ),
            (
                Value::product(Value::u64(42), Value::u64(1337)),
                Value::product(Value::u64(42), Value::unit()),
            ),
            (
                Value::product(Value::u64(42), Value::u64(1337)),
                Value::product(Value::unit(), Value::u64(1337)),
            ),
        ];

        for (value, expected_pruned_value) in test_vectors {
            assert_eq!(
                value.prune(expected_pruned_value.ty()),
                Some(expected_pruned_value)
            );
        }

        let bad_test_vectors = [
            (Value::unit(), Final::u1()),
            (
                Value::product(Value::unit(), Value::unit()),
                Final::sum(Final::unit(), Final::unit()),
            ),
            (
                Value::left(Value::unit(), Final::unit()),
                Final::product(Final::unit(), Final::unit()),
            ),
            (
                Value::right(Final::unit(), Value::unit()),
                Final::product(Final::unit(), Final::unit()),
            ),
        ];

        for (value, pruned_ty) in bad_test_vectors {
            assert_eq!(value.prune(&pruned_ty), None);
        }
    }

    #[test]
    fn zero_value() {
        let test_vectors = [
            (Final::unit(), Value::unit()),
            (Final::u8(), Value::u8(0)),
            (Final::u64(), Value::u64(0)),
            (
                Final::product(Final::u16(), Final::u32()),
                Value::product(Value::u16(0), Value::u32(0)),
            ),
            (
                Final::sum(Final::unit(), Final::u64()),
                Value::left(Value::unit(), Final::u64()),
            ),
            (
                Final::product(Final::unit(), Final::unit()),
                Value::product(Value::unit(), Value::unit()),
            ),
        ];

        for (ty, expected_default_value) in test_vectors {
            assert_eq!(Value::zero(&ty), expected_default_value);
        }
    }

    #[test]
    fn compact_round_trip() {
        let v = Value::u64(0xff00_00ff_ff00_00ff);
        let (bits, _) = v.iter_compact().collect_bits();
        let mut iter = BitIter::new(bits.into_iter());
        let new_v = Value::from_compact_bits(&mut iter, &v.ty).unwrap();
        assert_eq!(v, new_v);
    }

    #[test]
    fn padded_round_trip() {
        let v = Value::u64(0xff00_00ff_ff00_00ff);
        let (bits, _) = v.iter_padded().collect_bits();
        let mut iter = BitIter::new(bits.into_iter());
        let new_v = Value::from_padded_bits(&mut iter, &v.ty).unwrap();
        assert_eq!(v, new_v);
    }
}

#[cfg(bench)]
mod benches {
    use super::*;

    use crate::bit_encoding::{BitCollector as _, BitIter};

    use test::{black_box, Bencher};

    // Create values directly
    #[bench]
    fn bench_value_create_u64(bh: &mut Bencher) {
        bh.iter(|| black_box(Value::u64(0xff00_00ff_ff00_00ff)))
    }

    #[bench]
    fn bench_value_create_u2048(bh: &mut Bencher) {
        bh.iter(|| black_box(Value::from_byte_array([0xcd; 2048])));
    }

    #[bench]
    fn bench_value_create_deep_some(bh: &mut Bencher) {
        bh.iter(|| {
            let mut kilo = Value::from_byte_array([0xab; 1024]);
            for _ in 0..1000 {
                kilo = Value::some(kilo.shallow_clone());
            }
            black_box(kilo)
        });
    }

    #[bench]
    fn bench_value_create_64k(bh: &mut Bencher) {
        bh.iter(|| {
            let mut kilo = Value::from_byte_array([0xab; 1024]);
            for _ in 0..5 {
                kilo = Value::product(kilo.shallow_clone(), kilo.shallow_clone());
            }
            black_box(kilo)
        });
    }

    // Create values from padded bits
    fn padded_bits(v: &Value) -> (Vec<u8>, &Final) {
        let (bits, _) = v.iter_padded().collect_bits();
        (bits, v.ty.as_ref())
    }

    #[bench]
    fn bench_value_create_u64_padded(bh: &mut Bencher) {
        let v = Value::u64(0xff00_00ff_ff00_00ff);
        let (bits, ty) = padded_bits(&v);
        bh.iter(|| {
            black_box(Value::from_padded_bits(
                &mut BitIter::new(bits.iter().copied()),
                ty,
            ))
        })
    }

    #[bench]
    fn bench_value_create_u2048_padded(bh: &mut Bencher) {
        let v = Value::from_byte_array([0xcd; 2048]);
        let (bits, ty) = padded_bits(&v);
        bh.iter(|| {
            black_box(Value::from_padded_bits(
                &mut BitIter::new(bits.iter().copied()),
                ty,
            ))
        })
    }

    #[bench]
    fn bench_value_create_deep_some_padded(bh: &mut Bencher) {
        let mut kilo = Value::from_byte_array([0xab; 1024]);
        for _ in 0..1000 {
            kilo = Value::some(kilo.shallow_clone());
        }
        let (bits, ty) = padded_bits(&kilo);
        bh.iter(|| {
            black_box(Value::from_padded_bits(
                &mut BitIter::new(bits.iter().copied()),
                ty,
            ))
        })
    }

    #[bench]
    fn bench_value_create_64k_padded(bh: &mut Bencher) {
        let mut kilo = Value::from_byte_array([0xab; 1024]);
        for _ in 0..5 {
            kilo = Value::product(kilo.shallow_clone(), kilo.shallow_clone());
        }
        let (bits, ty) = padded_bits(&kilo);
        bh.iter(|| {
            black_box(Value::from_padded_bits(
                &mut BitIter::new(bits.iter().copied()),
                ty,
            ))
        })
    }

    // Create values from compact bits
    fn compact_bits(v: &Value) -> (Vec<u8>, &Final) {
        let (bits, _) = v.iter_compact().collect_bits();
        (bits, v.ty.as_ref())
    }

    #[bench]
    fn bench_value_create_u64_compact(bh: &mut Bencher) {
        let v = Value::u64(0xff00_00ff_ff00_00ff);
        let (bits, ty) = compact_bits(&v);
        bh.iter(|| {
            black_box(Value::from_compact_bits(
                &mut BitIter::new(bits.iter().copied()),
                ty,
            ))
        })
    }

    #[bench]
    fn bench_value_create_u2048_compact(bh: &mut Bencher) {
        let v = Value::from_byte_array([0xcd; 2048]);
        let (bits, ty) = compact_bits(&v);
        bh.iter(|| {
            black_box(Value::from_compact_bits(
                &mut BitIter::new(bits.iter().copied()),
                ty,
            ))
        })
    }

    #[bench]
    fn bench_value_create_deep_some_compact(bh: &mut Bencher) {
        let mut kilo = Value::from_byte_array([0xab; 1024]);
        for _ in 0..1000 {
            kilo = Value::some(kilo.shallow_clone());
        }
        let (bits, ty) = compact_bits(&kilo);
        bh.iter(|| {
            black_box(Value::from_compact_bits(
                &mut BitIter::new(bits.iter().copied()),
                ty,
            ))
        })
    }

    #[bench]
    fn bench_value_create_64k_compact(bh: &mut Bencher) {
        let mut kilo = Value::from_byte_array([0xab; 1024]);
        for _ in 0..5 {
            kilo = Value::product(kilo.shallow_clone(), kilo.shallow_clone());
        }
        let (bits, ty) = compact_bits(&kilo);
        bh.iter(|| {
            black_box(Value::from_compact_bits(
                &mut BitIter::new(bits.iter().copied()),
                ty,
            ))
        })
    }

    // Display values
    #[bench]
    fn bench_value_display_u64(bh: &mut Bencher) {
        let v = Value::u64(0xff00_00ff_ff00_00ff);
        bh.iter(|| black_box(format!("{}", v)))
    }

    #[bench]
    fn bench_value_display_u2024(bh: &mut Bencher) {
        let v = Value::from_byte_array([0xcd; 2048]);
        bh.iter(|| black_box(format!("{}", v)))
    }

    #[bench]
    fn bench_value_display_deep_some(bh: &mut Bencher) {
        let mut kilo = Value::from_byte_array([0xab; 1024]);
        for _ in 0..1000 {
            kilo = Value::some(kilo.shallow_clone());
        }
        bh.iter(|| black_box(format!("{}", kilo)))
    }

    #[bench]
    fn bench_value_display_64k(bh: &mut Bencher) {
        let mut kilo = Value::from_byte_array([0xab; 1024]);
        for _ in 0..5 {
            kilo = Value::product(kilo.shallow_clone(), kilo.shallow_clone());
        }
        bh.iter(|| black_box(format!("{}", kilo)))
    }
}
