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

use crate::Tmr;

use std::borrow::Cow;
use std::sync::Arc;
use std::{cmp, fmt};

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
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Debug, Hash)]
pub struct Final {
    /// Underlying type
    bound: CompleteBound,
    /// Width of the type, in bits, in the bit machine
    bit_width: usize,
    /// TMR of the type
    tmr: Tmr,
    /// Cached string representation of the type
    display: Cow<'static, str>,
}

impl Final {
    /// (Non-public) constructor for the final data of the unit type
    pub(super) const fn unit() -> Self {
        Final {
            bound: CompleteBound::Unit,
            bit_width: 0,
            tmr: Tmr::unit(),
            display: Cow::Borrowed("1"),
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
            display: if left.bound == CompleteBound::Unit && right.bound == CompleteBound::Unit {
                "2".into()
            } else {
                format!("({} + {})", left.display, right.display).into()
            },
            bound: CompleteBound::Sum(left, right),
        }
    }

    /// (Non-public) constructor for the final data of a product type
    pub(super) fn product(left: Arc<Self>, right: Arc<Self>) -> Self {
        Final {
            tmr: Tmr::product(left.tmr, right.tmr),
            bit_width: left.bit_width + right.bit_width,
            display: if left.display == right.display {
                match left.display.as_ref() {
                    "2" => "2^2".into(),
                    "2^2" => "2^4".into(),
                    "2^4" => "2^8".into(),
                    "2^8" => "2^16".into(),
                    "2^16" => "2^32".into(),
                    "2^32" => "2^64".into(),
                    "2^64" => "2^128".into(),
                    "2^128" => "2^256".into(),
                    "2^256" => "2^512".into(),
                    _ => format!("({} × {})", left.display, right.display).into(),
                }
            } else {
                format!("({} × {})", left.display, right.display).into()
            },
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

    /// Accessor for both children of the type, if they exist.
    pub fn split(&self) -> Option<(Arc<Self>, Arc<Self>)> {
        match &self.bound {
            CompleteBound::Unit => None,
            CompleteBound::Sum(left, right) | CompleteBound::Product(left, right) => {
                Some((Arc::clone(left), Arc::clone(right)))
            }
        }
    }
}

impl fmt::Display for Final {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(&self.display, f)
    }
}
