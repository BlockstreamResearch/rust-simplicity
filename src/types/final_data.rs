// SPDX-License-Identifier: CC0-1.0

//! Finalized (Complete) Type Data
//!
//! Once a type is complete (has no free variables), it can be represented as
//! a much simpler data structure than [`super::Type`], which we call [`Final`].
//! This contains a recursively-defined [`CompleteBound`] which specifies what
//! the type is, as well as a cached Merkle root (the TMR) and bit-width.
//!
//! We refer to types as "finalized" when they are represented by this data
//! structure, since this structure is immutable.
//!

use crate::dag::{Dag, DagLike, NoSharing};
use crate::Tmr;

use std::sync::Arc;
use std::{cmp, fmt, hash};

/// A finalized type bound, whose tree is accessible without any mutex locking
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Debug, Hash)]
pub enum CompleteBound {
    /// The unit type
    Unit,
    /// A sum of two other types
    Sum(Arc<Final>, Arc<Final>),
    /// A product of two other types
    Product(Arc<Final>, Arc<Final>),
}

/// Data related to a finalized type, which can be extracted from a [`super::Type`]
/// if (and only if) it is finalized.
#[derive(Clone)]
pub struct Final {
    /// Underlying type
    bound: CompleteBound,
    /// Width of the type, in bits, in the bit machine
    bit_width: usize,
    /// Whether the type's bit representation has any padding. If this is true,
    /// then its "compact" witness-encoded bit-width may be lower than its "padded"
    /// bit-machine bit-width.
    has_padding: bool,
    /// TMR of the type
    tmr: Tmr,
}

impl PartialEq for Final {
    fn eq(&self, other: &Self) -> bool {
        self.tmr == other.tmr
    }
}
impl Eq for Final {}

impl PartialOrd for Final {
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl Ord for Final {
    fn cmp(&self, other: &Self) -> cmp::Ordering {
        self.tmr.cmp(&other.tmr)
    }
}
impl hash::Hash for Final {
    fn hash<H: hash::Hasher>(&self, hasher: &mut H) {
        self.tmr.hash(hasher)
    }
}

impl fmt::Debug for Final {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{{ tmr: {}, bit_width: {}, bound: {} }}",
            self.tmr, self.bit_width, self
        )
    }
}

impl fmt::Display for Final {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut skipping: Option<Tmr> = None;
        for data in self.verbose_pre_order_iter::<NoSharing>(None) {
            if let Some(skip) = skipping {
                if data.is_complete && data.node.tmr == skip {
                    skipping = None;
                }
                continue;
            } else {
                if data.node.tmr == Tmr::TWO_TWO_N[0] {
                    f.write_str("2")?;
                    skipping = Some(data.node.tmr);
                }
                for (n, tmr) in Tmr::TWO_TWO_N.iter().enumerate().skip(1) {
                    if data.node.tmr == *tmr {
                        write!(f, "2^{}", 1 << n)?;
                        skipping = Some(data.node.tmr);
                    }
                }
            }
            if skipping.is_some() {
                continue;
            }

            match (&data.node.bound, data.n_children_yielded) {
                (CompleteBound::Unit, _) => {
                    f.write_str("1")?;
                }
                // special-case 1 + A as A?
                (CompleteBound::Sum(ref left, _), 0)
                    if matches!(left.bound, CompleteBound::Unit) =>
                {
                    skipping = Some(Tmr::unit());
                }
                (CompleteBound::Sum(ref left, _), 1)
                    if matches!(left.bound, CompleteBound::Unit) => {}
                (CompleteBound::Sum(ref left, _), 2)
                    if matches!(left.bound, CompleteBound::Unit) =>
                {
                    f.write_str("?")?;
                }
                // other sums and products
                (CompleteBound::Sum(..), 0) | (CompleteBound::Product(..), 0) => {
                    if data.index > 0 {
                        f.write_str("(")?;
                    }
                }
                (CompleteBound::Sum(..), 2) | (CompleteBound::Product(..), 2) => {
                    if data.index > 0 {
                        f.write_str(")")?;
                    }
                }
                (CompleteBound::Sum(..), _) => f.write_str(" + ")?,
                (CompleteBound::Product(..), _) => f.write_str(" × ")?,
            }
        }
        Ok(())
    }
}

impl DagLike for &'_ Final {
    type Node = Final;
    fn data(&self) -> &Final {
        self
    }
    fn as_dag_node(&self) -> Dag<Self> {
        match self.bound {
            CompleteBound::Unit => Dag::Nullary,
            CompleteBound::Sum(ref left, ref right)
            | CompleteBound::Product(ref left, ref right) => Dag::Binary(left, right),
        }
    }
}

macro_rules! construct_final_two_two_n {
    ($name: ident, $n: expr, $text: expr) => {
        #[doc = "Create the type of"]
        #[doc = $text]
        #[doc = "words.\n\nThe type is precomputed and fast to access."]
        #[inline]
        pub fn $name() -> Arc<Self> {
            super::precomputed::nth_power_of_2($n)
        }
    };
}

impl Final {
    /// Create the unit type.
    pub fn unit() -> Arc<Self> {
        Arc::new(Final {
            bound: CompleteBound::Unit,
            bit_width: 0,
            has_padding: false,
            tmr: Tmr::unit(),
        })
    }

    /// Computes the successor of the type.
    ///
    /// Given a type `X`, we define its successor `S X` as `1 + X`.
    /// In Rust notation this would be `Option<X>`.
    pub fn successor(self: Arc<Self>) -> Arc<Self> {
        Self::sum(Self::unit(), self)
    }

    /// Create the type `2^(2^n)` for the given `n`.
    ///
    /// The type is precomputed and fast to access.
    #[inline]
    pub fn two_two_n(n: usize) -> Result<Arc<Self>, TypeTooLargeError> {
        let maximum = Tmr::TWO_TWO_N.len();
        if n < maximum {
            Ok(super::precomputed::nth_power_of_2(n))
        } else {
            Err(TypeTooLargeError {
                ty: "2^(2^n)",
                n,
                maximum,
            })
        }
    }

    /// Create the type `2^(2^N)` for the compile-time constant `N`.
    ///
    /// Will fail to compile if `N` exceeds 31.
    #[inline]
    pub fn two_two_n_fixed<const N: usize>() -> Arc<Self> {
        // This crazy construction amounts to a compile-time assertion that N is less than the max.
        struct Hack<const N: usize>;
        impl<const N: usize> Hack<N> {
            const IS_IN_RANGE: () = {
                assert!(N < Tmr::TWO_TWO_N.len());
            };
        }
        let () = Hack::<N>::IS_IN_RANGE;

        super::precomputed::nth_power_of_2(N)
    }

    /// Create the type `(TWO^8)^<2^(n+1)` for the given `n`.
    ///
    /// Here
    /// * The notation X^<2 is notation for the type (S X)
    /// * The notation X^<(2*n) is notation for the type S (X^n) * X^<n
    ///
    /// And `S X` is the successor of `X`, i.e. `Option<X>`
    ///
    /// The type is precomputed and fast to access.
    #[inline]
    pub fn buffer8_two_n_plus_one(n: usize) -> Result<Arc<Self>, TypeTooLargeError> {
        let maximum = Tmr::BUFFER8_TWO_N_PLUS_ONE.len();
        if n < maximum {
            Ok(super::precomputed::buffer8_two_n_plus_one(n))
        } else {
            // This is arguably a programming error and a panic would be justified, but it's
            // hard to say how SimplicityHL will use this. I also think the current maximum
            // may be too small and we could bump into this with real code, so better to let
            // the caller decide how to handle that.
            Err(TypeTooLargeError {
                ty: "(TWO^8)^<2^(n+1)",
                n,
                maximum,
            })
        }
    }

    /// Create the `Ctx8` type used by the SHA256 jets.
    ///
    /// The type is precomputed and fast to access.
    pub fn ctx8() -> Arc<Self> {
        super::precomputed::ctx8()
    }

    construct_final_two_two_n!(u1, 0, "1-bit");
    construct_final_two_two_n!(u2, 1, "2-bit");
    construct_final_two_two_n!(u4, 2, "4-bit");
    construct_final_two_two_n!(u8, 3, "8-bit");
    construct_final_two_two_n!(u16, 4, "16-bit");
    construct_final_two_two_n!(u32, 5, "32-bit");
    construct_final_two_two_n!(u64, 6, "64-bit");
    construct_final_two_two_n!(u128, 7, "128-bit");
    construct_final_two_two_n!(u256, 8, "256-bit");
    construct_final_two_two_n!(u512, 9, "512-bit");

    /// Create the sum of the given `left` and `right` types.
    pub fn sum(left: Arc<Self>, right: Arc<Self>) -> Arc<Self> {
        // Use saturating_add for bitwidths. If the user has overflowed usize, even on a 32-bit
        // system this means that they have a 4-gigabit type and their program should be rejected
        // by a sanity check somewhere. However, if we panic here, the user cannot finalize their
        // program and cannot even tell that this resource limit has been hit before panicking.
        Arc::new(Final {
            tmr: Tmr::sum(left.tmr, right.tmr),
            bit_width: cmp::max(left.bit_width, right.bit_width).saturating_add(1),
            has_padding: left.has_padding || right.has_padding || left.bit_width != right.bit_width,
            bound: CompleteBound::Sum(left, right),
        })
    }

    /// Create the product of the given `left` and `right` types.
    pub fn product(left: Arc<Self>, right: Arc<Self>) -> Arc<Self> {
        // See comment in `sum` about use of saturating add.
        Arc::new(Final {
            tmr: Tmr::product(left.tmr, right.tmr),
            bit_width: left.bit_width.saturating_add(right.bit_width),
            has_padding: left.has_padding || right.has_padding,
            bound: CompleteBound::Product(left, right),
        })
    }

    /// Accessor for the TMR
    pub fn tmr(&self) -> Tmr {
        self.tmr
    }

    /// Accessor for the Bit Machine bit-width of the type
    pub fn bit_width(&self) -> usize {
        self.bit_width
    }

    /// Whether the type's bit representation has any padding.
    ///
    /// If this is true, then its "compact" witness-encoded bit-width may be lower
    /// than its "padded" bit-machine bit-width.
    pub fn has_padding(&self) -> bool {
        self.has_padding
    }

    /// Check if the type is a nested product of units.
    /// In this case, values contain no information.
    pub fn is_empty(&self) -> bool {
        self.bit_width() == 0
    }

    /// Accessor for the type bound
    pub fn bound(&self) -> &CompleteBound {
        &self.bound
    }

    /// Check if the type is a unit.
    pub fn is_unit(&self) -> bool {
        self.bound == CompleteBound::Unit
    }

    /// Access the inner types of a sum type.
    pub fn as_sum(&self) -> Option<(&Arc<Self>, &Arc<Self>)> {
        match &self.bound {
            CompleteBound::Sum(left, right) => Some((left, right)),
            _ => None,
        }
    }

    /// Access the inner types of a product type.
    pub fn as_product(&self) -> Option<(&Arc<Self>, &Arc<Self>)> {
        match &self.bound {
            CompleteBound::Product(left, right) => Some((left, right)),
            _ => None,
        }
    }

    /// If the type is of the form `TWO^(2^n)`, then return `n`.
    ///
    /// ## Post condition
    ///
    /// 0 ≤ n < 32.
    pub fn as_word(&self) -> Option<u32> {
        (0..32u32).find(|&n| {
            self.tmr == Tmr::TWO_TWO_N[n as usize] // cast safety: 32-bit machine or higher
        })
    }

    /// Compute the padding of left values of the sum type `Self + Other`.
    pub fn pad_left(&self, other: &Self) -> usize {
        cmp::max(self.bit_width, other.bit_width) - self.bit_width
    }

    /// Compute the padding of right values of the sum type `Self + Other`.
    pub fn pad_right(&self, other: &Self) -> usize {
        cmp::max(self.bit_width, other.bit_width) - other.bit_width
    }
}

/// Attempted to produce a `(TWO^8)^(2^(n+1))` type exceeding the maximum size.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct TypeTooLargeError {
    ty: &'static str,
    n: usize,
    maximum: usize,
}

impl fmt::Display for TypeTooLargeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "maximum n for {} is {}; got {}",
            self.ty, self.maximum, self.n
        )
    }
}

impl std::error::Error for TypeTooLargeError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        let Self {
            ty: _,
            n: _,
            maximum: _,
        } = self;
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn two_two_n_consistent() {
        // This test probably does not need to be exhaustive.
        assert_eq!(Final::two_two_n_fixed::<0>(), Final::two_two_n(0).unwrap());
        assert_eq!(Final::two_two_n_fixed::<1>(), Final::two_two_n(1).unwrap());
        assert_eq!(Final::two_two_n_fixed::<2>(), Final::two_two_n(2).unwrap());
        assert_eq!(Final::two_two_n_fixed::<3>(), Final::two_two_n(3).unwrap());
        assert_eq!(Final::two_two_n_fixed::<4>(), Final::two_two_n(4).unwrap());
        assert_eq!(Final::two_two_n_fixed::<5>(), Final::two_two_n(5).unwrap());
        assert_eq!(Final::two_two_n_fixed::<6>(), Final::two_two_n(6).unwrap());
        assert_eq!(Final::two_two_n_fixed::<7>(), Final::two_two_n(7).unwrap());
    }

    #[test]
    fn final_stringify() {
        let ty1 = Final::two_two_n_fixed::<10>();
        assert_eq!(ty1.to_string(), "2^1024");

        let sum = Final::sum(Final::two_two_n_fixed::<5>(), Final::two_two_n(10).unwrap());
        assert_eq!(sum.to_string(), "2^32 + 2^1024");

        let prod = Final::product(
            Final::two_two_n_fixed::<5>(),
            Final::two_two_n_fixed::<10>(),
        );
        assert_eq!(prod.to_string(), "2^32 × 2^1024");

        let ty1 = Final::two_two_n_fixed::<0>();
        assert_eq!(ty1.to_string(), "2");

        let ty1 = Final::sum(Final::unit(), Final::two_two_n_fixed::<2>());
        assert_eq!(ty1.to_string(), "2^4?");
    }
}
