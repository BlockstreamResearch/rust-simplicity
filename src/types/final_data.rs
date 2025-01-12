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
                if data.node.tmr == Tmr::POWERS_OF_TWO[0] {
                    f.write_str("2")?;
                    skipping = Some(data.node.tmr);
                }
                for (n, tmr) in Tmr::POWERS_OF_TWO.iter().enumerate().skip(1) {
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
            tmr: Tmr::unit(),
        })
    }

    /// Create the type `2^(2^n)` for the given `n`.
    ///
    /// The type is precomputed and fast to access.
    pub fn two_two_n(n: usize) -> Arc<Self> {
        super::precomputed::nth_power_of_2(n)
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
        Arc::new(Final {
            tmr: Tmr::sum(left.tmr, right.tmr),
            bit_width: 1 + cmp::max(left.bit_width, right.bit_width),
            bound: CompleteBound::Sum(left, right),
        })
    }

    /// Create the product of the given `left` and `right` types.
    pub fn product(left: Arc<Self>, right: Arc<Self>) -> Arc<Self> {
        Arc::new(Final {
            tmr: Tmr::product(left.tmr, right.tmr),
            bit_width: left.bit_width + right.bit_width,
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
    pub fn as_sum(&self) -> Option<(&Self, &Self)> {
        match &self.bound {
            CompleteBound::Sum(left, right) => Some((left.as_ref(), right.as_ref())),
            _ => None,
        }
    }

    /// Access the inner types of a product type.
    pub fn as_product(&self) -> Option<(&Self, &Self)> {
        match &self.bound {
            CompleteBound::Product(left, right) => Some((left.as_ref(), right.as_ref())),
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
            self.tmr == Tmr::POWERS_OF_TWO[n as usize] // cast safety: 32-bit machine or higher
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn final_stringify() {
        let ty1 = Final::two_two_n(10);
        assert_eq!(ty1.to_string(), "2^1024");

        let sum = Final::sum(Final::two_two_n(5), Final::two_two_n(10));
        assert_eq!(sum.to_string(), "2^32 + 2^1024");

        let prod = Final::product(Final::two_two_n(5), Final::two_two_n(10));
        assert_eq!(prod.to_string(), "2^32 × 2^1024");

        let ty1 = Final::two_two_n(0);
        assert_eq!(ty1.to_string(), "2");

        let ty1 = Final::sum(Final::unit(), Final::two_two_n(2));
        assert_eq!(ty1.to_string(), "2^4?");
    }
}
