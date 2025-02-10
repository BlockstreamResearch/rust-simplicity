// SPDX-License-Identifier: CC0-1.0

//! # Simplicity values
//!
//! Simplicity processes data in terms of [`Value`]s,
//! i.e., inputs, intermediate results and outputs.

use crate::dag::{Dag, DagLike, NoSharing};
use crate::types::{CompleteBound, Final};

use crate::{BitCollector, EarlyEndOfStreamError};
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

impl DagLike for &'_ Value {
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

    /// Create a 1-bit integer.
    ///
    /// ## Panics
    ///
    /// The value is out of range.
    pub fn u1(value: u8) -> Self {
        match value {
            0 => Self::left(Self::unit(), Final::unit()),
            1 => Self::right(Final::unit(), Self::unit()),
            x => panic!("{} out of range for Value::u1", x),
        }
    }

    /// Create a 2-bit integer.
    ///
    /// ## Panics
    ///
    /// The value is out of range.
    pub fn u2(value: u8) -> Self {
        let b0 = (value & 2) / 2;
        let b1 = value & 1;
        assert!(value <= 3, "{} out of range for Value::u2", value);
        Self::product(Self::u1(b0), Self::u1(b1))
    }

    /// Create a 4-bit integer.
    ///
    /// ## Panics
    ///
    /// The value is ouf of range.
    pub fn u4(value: u8) -> Self {
        let w0 = (value & 12) / 4;
        let w1 = value & 3;
        assert!(value <= 15, "{} out of range for Value::u2", value);
        Self::product(Self::u2(w0), Self::u2(w1))
    }

    /// Create an 8-bit integer.
    pub fn u8(value: u8) -> Self {
        let w0 = value >> 4;
        let w1 = value & 0xf;
        Self::product(Self::u4(w0), Self::u4(w1))
    }

    /// Create a 16-bit integer.
    pub fn u16(bytes: u16) -> Self {
        Self::from_byte_array(bytes.to_be_bytes())
    }

    /// Create a 32-bit integer.
    pub fn u32(bytes: u32) -> Self {
        Self::from_byte_array(bytes.to_be_bytes())
    }

    /// Create a 64-bit integer.
    pub fn u64(bytes: u64) -> Self {
        Self::from_byte_array(bytes.to_be_bytes())
    }

    /// Create a 128-bit integer.
    pub fn u128(bytes: u128) -> Self {
        Self::from_byte_array(bytes.to_be_bytes())
    }

    /// Create a 256-bit integer.
    pub fn u256(bytes: [u8; 32]) -> Self {
        Self::from_byte_array(bytes)
    }

    /// Create a 512-bit integer.
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
        enum Task<'a> {
            Prune(&'a Value, &'a Final),
            MakeLeft(Arc<Final>),
            MakeRight(Arc<Final>),
            MakeProduct,
        }

        let mut stack = vec![Task::Prune(self, pruned_ty)];
        let mut output = vec![];

        while let Some(task) = stack.pop() {
            match task {
                Task::Prune(value, pruned_ty) => match pruned_ty.bound() {
                    CompleteBound::Unit => output.push(Value::unit()),
                    CompleteBound::Sum(l_ty, r_ty) => {
                        if let Some(l_value) = value.as_left() {
                            stack.push(Task::MakeLeft(Arc::clone(r_ty)));
                            stack.push(Task::Prune(l_value, l_ty));
                        } else {
                            let r_value = value.as_right()?;
                            stack.push(Task::MakeRight(Arc::clone(l_ty)));
                            stack.push(Task::Prune(r_value, r_ty));
                        }
                    }
                    CompleteBound::Product(l_ty, r_ty) => {
                        let (l_value, r_value) = value.as_product()?;
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

impl Iterator for PaddedBitsIter<'_> {
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
    fn from_bits<I: Iterator<Item = bool>, P: Padding>(
        bits: &mut I,
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
        while let Some(state) = stack.pop() {
            match state {
                State::ProcessType(ty) => match ty.bound() {
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
                },
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
    pub fn from_compact_bits<I: Iterator<Item = bool>>(
        bits: &mut I,
        ty: &Final,
    ) -> Result<Self, EarlyEndOfStreamError> {
        Self::from_bits::<_, CompactEncoding>(bits, ty)
    }

    /// Decode a value of the given type from its padded bit encoding.
    pub fn from_padded_bits<I: Iterator<Item = bool>>(
        bits: &mut I,
        ty: &Final,
    ) -> Result<Self, EarlyEndOfStreamError> {
        Self::from_bits::<_, PaddedEncoding>(bits, ty)
    }
}

/// A Simplicity word. A value of type `TWO^(2^n)` for some `0 ≤ n < 32`.
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
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
    pub fn from_bits<I: Iterator<Item = bool>>(
        bits: &mut I,
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
