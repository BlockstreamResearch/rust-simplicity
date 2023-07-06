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

//! Types Arrows
//!
//! Every Simplicity expression has two types associated with it: a source and
//! a target. See the `types` module for more information. We refer to this
//! pair of types as an "arrow", since the expression can be thought of as
//! mapping a value of the source type to a value of the target type.
//!
//! This module defines the specific arrows associated with each kind of node.
//!
//! See the `types` module above this one for more information.

use std::fmt;
use std::sync::Arc;

use crate::node::{
    CoreConstructible, DisconnectConstructible, JetConstructible, NoDisconnect,
    WitnessConstructible,
};
use crate::types::{Bound, Error, Final, Type};
use crate::{jet::Jet, Value};

use super::variable::new_name;

/// A container for an expression's source and target types, whether or not
/// these types are complete.
#[derive(Clone, Debug)]
pub struct Arrow {
    /// The source type
    pub source: Type,
    /// The target type
    pub target: Type,
}

impl fmt::Display for Arrow {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} → {}", self.source, self.target)
    }
}

/// A container for the type data associated with an expression's source and
/// target types, if both types are complete.
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Debug, Hash)]
pub struct FinalArrow {
    /// The source type
    pub source: Arc<Final>,
    /// The target type
    pub target: Arc<Final>,
}

impl fmt::Display for FinalArrow {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} → {}", self.source, self.target)
    }
}

impl FinalArrow {
    /// Same as [`Self::clone`] but named to make it clearer that this is cheap
    pub fn shallow_clone(&self) -> Self {
        FinalArrow {
            source: Arc::clone(&self.source),
            target: Arc::clone(&self.target),
        }
    }
}

impl Arrow {
    /// Finalize the source and target types in the arrow
    pub fn finalize(&self) -> Result<FinalArrow, Error> {
        Ok(FinalArrow {
            source: self.source.finalize()?,
            target: self.target.finalize()?,
        })
    }

    /// Create a unification arrow for a fresh `unit` combinator
    pub fn for_unit() -> Self {
        Arrow {
            source: Type::free(new_name("unit_src_")),
            target: Type::unit(),
        }
    }

    /// Create a unification arrow for a fresh `iden` combinator
    pub fn for_iden() -> Self {
        // Throughout this module, when two types are the same, we reuse a
        // pointer to them rather than creating distinct types and unifying
        // them. This theoretically could lead to more confusing errors for
        // the user during type inference, but in practice type inference
        // is completely opaque and there's no harm in making it moreso.
        let new = Type::free(new_name("iden_src_"));
        Arrow {
            source: new.shallow_clone(),
            target: new,
        }
    }

    /// Create a unification arrow for a fresh `witness` combinator
    pub fn for_witness() -> Self {
        Arrow {
            source: Type::free(new_name("witness_src_")),
            target: Type::free(new_name("witness_tgt_")),
        }
    }

    /// Create a unification arrow for a fresh `fail` combinator
    pub fn for_fail() -> Self {
        Arrow {
            source: Type::free(new_name("fail_src_")),
            target: Type::free(new_name("fail_tgt_")),
        }
    }

    /// Create a unification arrow for a fresh jet combinator
    pub fn for_jet<J: Jet>(jet: J) -> Self {
        Arrow {
            source: jet.source_ty().to_type(),
            target: jet.target_ty().to_type(),
        }
    }

    /// Create a unification arrow for a fresh const-word combinator
    pub fn for_const_word(word: &Value) -> Self {
        let len = word.len();
        assert_eq!(len.count_ones(), 1);
        let depth = word.len().trailing_zeros();
        Arrow {
            source: Type::unit(),
            target: Type::two_two_n(depth as usize),
        }
    }

    /// Create a unification arrow for a fresh `injl` combinator
    pub fn for_injl(child_arrow: &Arrow) -> Self {
        Arrow {
            source: child_arrow.source.shallow_clone(),
            target: Type::sum(
                child_arrow.target.shallow_clone(),
                Type::free(new_name("injl_tgt_")),
            ),
        }
    }

    /// Create a unification arrow for a fresh `injr` combinator
    pub fn for_injr(child_arrow: &Arrow) -> Self {
        Arrow {
            source: child_arrow.source.shallow_clone(),
            target: Type::sum(
                Type::free(new_name("injr_tgt_")),
                child_arrow.target.shallow_clone(),
            ),
        }
    }

    /// Create a unification arrow for a fresh `take` combinator
    pub fn for_take(child_arrow: &Arrow) -> Self {
        Arrow {
            source: Type::product(
                child_arrow.source.shallow_clone(),
                Type::free(new_name("take_src_")),
            ),
            target: child_arrow.target.shallow_clone(),
        }
    }

    /// Create a unification arrow for a fresh `drop` combinator
    pub fn for_drop(child_arrow: &Arrow) -> Self {
        Arrow {
            source: Type::product(
                Type::free(new_name("drop_src_")),
                child_arrow.source.shallow_clone(),
            ),
            target: child_arrow.target.shallow_clone(),
        }
    }

    /// Create a unification arrow for a fresh `pair` combinator
    pub fn for_pair(lchild_arrow: &Arrow, rchild_arrow: &Arrow) -> Result<Self, Error> {
        lchild_arrow.source.unify(
            &rchild_arrow.source,
            "pair combinator: left source = right source",
        )?;
        Ok(Arrow {
            source: lchild_arrow.source.shallow_clone(),
            target: Type::product(
                lchild_arrow.target.shallow_clone(),
                rchild_arrow.target.shallow_clone(),
            ),
        })
    }

    /// Create a unification arrow for a fresh `comp` combinator
    pub fn for_comp(lchild_arrow: &Arrow, rchild_arrow: &Arrow) -> Result<Self, Error> {
        lchild_arrow.target.unify(
            &rchild_arrow.source,
            "comp combinator: left target = right source",
        )?;
        Ok(Arrow {
            source: lchild_arrow.source.shallow_clone(),
            target: rchild_arrow.target.shallow_clone(),
        })
    }

    /// Create a unification arrow for a fresh `case` combinator
    ///
    /// Either child may be `None`, in which case the combinator is assumed to be
    /// an assertion, which for type-inference purposes means there are no bounds
    /// on the missing child.
    ///
    /// If neither child is provided, this function will not raise an error; it
    /// is the responsibility of the caller to detect this case and error elsewhere.
    pub fn for_case(
        lchild_arrow: Option<&Arrow>,
        rchild_arrow: Option<&Arrow>,
    ) -> Result<Self, Error> {
        let a = Type::free(new_name("case_a_"));
        let b = Type::free(new_name("case_b_"));
        let c = Type::free(new_name("case_c_"));

        let sum_a_b = Type::sum(a.shallow_clone(), b.shallow_clone());
        let prod_sum_a_b_c = Type::product(sum_a_b, c.shallow_clone());

        let target = Type::free(String::new());
        if let Some(lchild_arrow) = lchild_arrow {
            lchild_arrow.source.bind(
                Arc::new(Bound::Product(a, c.shallow_clone())),
                "case combinator: left source = A × C",
            )?;
            target.unify(&lchild_arrow.target, "").unwrap();
        }
        if let Some(rchild_arrow) = rchild_arrow {
            rchild_arrow.source.bind(
                Arc::new(Bound::Product(b, c)),
                "case combinator: left source = B × C",
            )?;
            target.unify(
                &rchild_arrow.target,
                "case combinator: left target = right target",
            )?;
        }

        Ok(Arrow {
            source: prod_sum_a_b_c,
            target,
        })
    }

    /// Create a unification arrow for a fresh `comp` combinator
    pub fn for_disconnect(lchild_arrow: &Arrow, rchild_arrow: &Arrow) -> Result<Self, Error> {
        let a = Type::free(new_name("disconnect_a_"));
        let b = Type::free(new_name("disconnect_b_"));
        let c = rchild_arrow.source.shallow_clone();
        let d = rchild_arrow.target.shallow_clone();

        let prod_256_a = Bound::Product(Type::two_two_n(8), a.shallow_clone());
        let prod_b_c = Bound::Product(b.shallow_clone(), c);
        let prod_b_d = Type::product(b, d);

        lchild_arrow.source.bind(
            Arc::new(prod_256_a),
            "disconnect combinator: left source = 2^256 × A",
        )?;
        lchild_arrow.target.bind(
            Arc::new(prod_b_c),
            "disconnect combinator: left target = B × C",
        )?;

        Ok(Arrow {
            source: a,
            target: prod_b_d,
        })
    }

    /// Same as [`Self::clone`] but named to make it clearer that this is cheap
    pub fn shallow_clone(&self) -> Self {
        Arrow {
            source: self.source.shallow_clone(),
            target: self.target.shallow_clone(),
        }
    }
}

impl CoreConstructible for Arrow {
    fn iden() -> Self {
        Self::for_iden()
    }

    fn unit() -> Self {
        Self::for_unit()
    }

    fn injl(child: &Self) -> Self {
        Self::for_injl(child)
    }

    fn injr(child: &Self) -> Self {
        Self::for_injr(child)
    }

    fn take(child: &Self) -> Self {
        Self::for_take(child)
    }

    fn drop_(child: &Self) -> Self {
        Self::for_drop(child)
    }

    fn comp(left: &Self, right: &Self) -> Result<Self, Error> {
        Self::for_comp(left, right)
    }

    fn case(left: &Self, right: &Self) -> Result<Self, Error> {
        Self::for_case(Some(left), Some(right))
    }

    fn assertl(left: &Self, _: crate::Cmr) -> Result<Self, Error> {
        Self::for_case(Some(left), None)
    }

    fn assertr(_: crate::Cmr, right: &Self) -> Result<Self, Error> {
        Self::for_case(None, Some(right))
    }

    fn pair(left: &Self, right: &Self) -> Result<Self, Error> {
        Self::for_pair(left, right)
    }

    fn fail(_: crate::FailEntropy) -> Self {
        Self::for_fail()
    }

    fn const_word(word: Arc<Value>) -> Self {
        Self::for_const_word(&word)
    }
}

impl DisconnectConstructible<Arrow> for Arrow {
    fn disconnect(left: &Self, right: &Self) -> Result<Self, Error> {
        Self::for_disconnect(left, right)
    }
}

impl DisconnectConstructible<NoDisconnect> for Arrow {
    fn disconnect(left: &Self, _: &NoDisconnect) -> Result<Self, Error> {
        Self::for_disconnect(
            left,
            &Arrow {
                source: Type::free("disc_src".into()),
                target: Type::free("disc_tgt".into()),
            },
        )
    }
}

impl DisconnectConstructible<Option<&Arrow>> for Arrow {
    fn disconnect(left: &Self, right: &Option<&Self>) -> Result<Self, Error> {
        match *right {
            Some(right) => Self::disconnect(left, right),
            None => Self::disconnect(left, &NoDisconnect),
        }
    }
}

impl<J: Jet> JetConstructible<J> for Arrow {
    fn jet(jet: J) -> Self {
        Self::for_jet(jet)
    }
}

impl<W> WitnessConstructible<W> for Arrow {
    fn witness(_: W) -> Self {
        Self::for_witness()
    }
}
