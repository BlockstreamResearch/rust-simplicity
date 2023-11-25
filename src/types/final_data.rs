// Rust Simplicity Library
// Written in 2023 by
//   Andrew Poelstra <apoelstra@blockstream.com>
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
        for data in self.verbose_pre_order_iter::<NoSharing>() {
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

impl<'a> DagLike for &'a Final {
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

impl Final {
    /// (Non-public) constructor for the final data of the unit type
    pub(super) const fn unit() -> Self {
        Final {
            bound: CompleteBound::Unit,
            bit_width: 0,
            tmr: Tmr::unit(),
        }
    }

    /// Return a precomputed copy of 2^(2^n), for given n.
    pub fn two_two_n(n: usize) -> Arc<Self> {
        super::precomputed::nth_power_of_2(n).final_data().unwrap()
    }

    /// (Non-public) constructor for the final data of a sum type
    pub(super) fn sum(left: Arc<Self>, right: Arc<Self>) -> Self {
        Final {
            tmr: Tmr::sum(left.tmr, right.tmr),
            bit_width: 1 + cmp::max(left.bit_width, right.bit_width),
            bound: CompleteBound::Sum(left, right),
        }
    }

    /// (Non-public) constructor for the final data of a product type
    pub(super) fn product(left: Arc<Self>, right: Arc<Self>) -> Self {
        Final {
            tmr: Tmr::product(left.tmr, right.tmr),
            bit_width: left.bit_width + right.bit_width,
            bound: CompleteBound::Product(left, right),
        }
    }

    /// Accessor for the TMR
    pub fn tmr(&self) -> Tmr {
        self.tmr
    }

    /// Accessor for the Bit Machine bit-width of the type
    pub fn bit_width(&self) -> usize {
        self.bit_width
    }

    /// Accessor for the type bound
    pub fn bound(&self) -> &CompleteBound {
        &self.bound
    }

    /// Returns whether this is the unit type
    pub fn is_unit(&self) -> bool {
        self.bound == CompleteBound::Unit
    }

    /// Return both children, if the type is a sum type
    pub fn split_sum(&self) -> Option<(Arc<Self>, Arc<Self>)> {
        match &self.bound {
            CompleteBound::Sum(left, right) => Some((left.clone(), right.clone())),
            _ => None,
        }
    }

    /// Return both children, if the type is a product type
    pub fn split_product(&self) -> Option<(Arc<Self>, Arc<Self>)> {
        match &self.bound {
            CompleteBound::Product(left, right) => Some((left.clone(), right.clone())),
            _ => None,
        }
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

        let ty1 = Final::sum(Arc::new(Final::unit()), Final::two_two_n(2));
        assert_eq!(ty1.to_string(), "2^4?");
    }
}
