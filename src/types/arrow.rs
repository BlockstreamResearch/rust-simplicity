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

use crate::node::{CoreConstructible, JetConstructible, WitnessConstructible};
use crate::types::{Bound, Error, Final, Type};
use crate::{jet::Jet, Context, Value};

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
    pub fn for_unit<J: Jet>(context: &mut Context<J>) -> Self {
        Arrow {
            source: Type::free(context.naming.new_name()),
            target: context.unit_ty(),
        }
    }

    /// Create a unification arrow for a fresh `iden` combinator
    pub fn for_iden<J: Jet>(context: &mut Context<J>) -> Self {
        // Throughout this module, when two types are the same, we reuse a
        // pointer to them rather than creating distinct types and unifying
        // them. This theoretically could lead to more confusing errors for
        // the user during type inference, but in practice type inference
        // is completely opaque and there's no harm in making it moreso.
        let new = Type::free(context.naming.new_name());
        Arrow {
            source: new.shallow_clone(),
            target: new,
        }
    }

    /// Create a unification arrow for a fresh `witness` combinator
    pub fn for_witness<J: Jet>(context: &mut Context<J>) -> Self {
        Arrow {
            source: Type::free(context.naming.new_name()),
            target: Type::free(context.naming.new_name()),
        }
    }

    /// Create a unification arrow for a fresh `fail` combinator
    pub fn for_fail<J: Jet>(context: &mut Context<J>) -> Self {
        Arrow {
            source: Type::free(context.naming.new_name()),
            target: Type::free(context.naming.new_name()),
        }
    }

    /// Create a unification arrow for a fresh jet combinator
    pub fn for_jet<J: Jet>(context: &mut Context<J>, jet: J) -> Self {
        Arrow {
            source: jet.source_ty().to_type(|n| context.nth_power_of_2(n)),
            target: jet.target_ty().to_type(|n| context.nth_power_of_2(n)),
        }
    }

    /// Create a unification arrow for a fresh const-word combinator
    pub fn for_const_word<J: Jet>(context: &mut Context<J>, word: &Value) -> Self {
        let len = word.len();
        assert_eq!(len.count_ones(), 1);
        let depth = word.len().trailing_zeros();
        Arrow {
            source: context.unit_ty(),
            target: context.nth_power_of_2(depth as usize),
        }
    }

    /// Create a unification arrow for a fresh `injl` combinator
    pub fn for_injl<J: Jet>(context: &mut Context<J>, child_arrow: &Arrow) -> Self {
        Arrow {
            source: child_arrow.source.shallow_clone(),
            target: Type::sum(
                child_arrow.target.shallow_clone(),
                Type::free(context.naming.new_name()),
            ),
        }
    }

    /// Create a unification arrow for a fresh `injr` combinator
    pub fn for_injr<J: Jet>(context: &mut Context<J>, child_arrow: &Arrow) -> Self {
        Arrow {
            source: child_arrow.source.shallow_clone(),
            target: Type::sum(
                Type::free(context.naming.new_name()),
                child_arrow.target.shallow_clone(),
            ),
        }
    }

    /// Create a unification arrow for a fresh `take` combinator
    pub fn for_take<J: Jet>(context: &mut Context<J>, child_arrow: &Arrow) -> Self {
        Arrow {
            source: Type::product(
                child_arrow.source.shallow_clone(),
                Type::free(context.naming.new_name()),
            ),
            target: child_arrow.target.shallow_clone(),
        }
    }

    /// Create a unification arrow for a fresh `drop` combinator
    pub fn for_drop<J: Jet>(context: &mut Context<J>, child_arrow: &Arrow) -> Self {
        Arrow {
            source: Type::product(
                Type::free(context.naming.new_name()),
                child_arrow.source.shallow_clone(),
            ),
            target: child_arrow.target.shallow_clone(),
        }
    }

    /// Create a unification arrow for a fresh `pair` combinator
    pub fn for_pair<J: Jet>(
        _: &mut Context<J>,
        lchild_arrow: &Arrow,
        rchild_arrow: &Arrow,
    ) -> Result<Self, Error> {
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
    pub fn for_comp<J: Jet>(
        _: &mut Context<J>,
        lchild_arrow: &Arrow,
        rchild_arrow: &Arrow,
    ) -> Result<Self, Error> {
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
    pub fn for_case<J: Jet>(
        context: &mut Context<J>,
        lchild_arrow: Option<&Arrow>,
        rchild_arrow: Option<&Arrow>,
    ) -> Result<Self, Error> {
        let a = Type::free(context.naming.new_name());
        let b = Type::free(context.naming.new_name());
        let c = Type::free(context.naming.new_name());

        let sum_a_b = Type::sum(a.shallow_clone(), b.shallow_clone());
        let prod_sum_a_b_c = Type::product(sum_a_b, c.shallow_clone());

        let target = Type::free(String::new());
        if let Some(lchild_arrow) = lchild_arrow {
            lchild_arrow.source.bind(
                Bound::Product(a, c.shallow_clone()),
                &None,
                "case combinator: left source = A × C",
            )?;
            target.unify(&lchild_arrow.target, "").unwrap();
        }
        if let Some(rchild_arrow) = rchild_arrow {
            rchild_arrow.source.bind(
                Bound::Product(b, c),
                &None,
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
    pub fn for_disconnect<J: Jet>(
        context: &mut Context<J>,
        lchild_arrow: &Arrow,
        rchild_arrow: &Arrow,
    ) -> Result<Self, Error> {
        let a = Type::free(context.naming.new_name());
        let b = Type::free(context.naming.new_name());
        let c = rchild_arrow.source.shallow_clone();
        let d = rchild_arrow.target.shallow_clone();

        let prod_256_a = Bound::Product(context.nth_power_of_2(8), a.shallow_clone());
        let prod_b_c = Bound::Product(b.shallow_clone(), c);
        let prod_b_d = Type::product(b, d);

        lchild_arrow.source.bind(
            prod_256_a,
            &None,
            "disconnect combinator: left source = 2^256 × A",
        )?;
        lchild_arrow.target.bind(
            prod_b_c,
            &None,
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

impl<J: Jet> CoreConstructible<J> for Arrow {
    fn iden(ctx: &mut Context<J>) -> Self {
        Self::for_iden(ctx)
    }

    fn unit(ctx: &mut Context<J>) -> Self {
        Self::for_unit(ctx)
    }

    fn injl(ctx: &mut Context<J>, child: &Self) -> Self {
        Self::for_injl(ctx, child)
    }

    fn injr(ctx: &mut Context<J>, child: &Self) -> Self {
        Self::for_injr(ctx, child)
    }

    fn take(ctx: &mut Context<J>, child: &Self) -> Self {
        Self::for_take(ctx, child)
    }

    fn drop_(ctx: &mut Context<J>, child: &Self) -> Self {
        Self::for_drop(ctx, child)
    }

    fn comp(ctx: &mut Context<J>, left: &Self, right: &Self) -> Result<Self, Error> {
        Self::for_comp(ctx, left, right)
    }

    fn case(ctx: &mut Context<J>, left: &Self, right: &Self) -> Result<Self, Error> {
        Self::for_case(ctx, Some(left), Some(right))
    }

    fn assertl(ctx: &mut Context<J>, left: &Self, _: crate::Cmr) -> Result<Self, Error> {
        Self::for_case(ctx, Some(left), None)
    }

    fn assertr(ctx: &mut Context<J>, _: crate::Cmr, right: &Self) -> Result<Self, Error> {
        Self::for_case(ctx, None, Some(right))
    }

    fn pair(ctx: &mut Context<J>, left: &Self, right: &Self) -> Result<Self, Error> {
        Self::for_pair(ctx, left, right)
    }

    fn disconnect(ctx: &mut Context<J>, left: &Self, right: &Self) -> Result<Self, Error> {
        Self::for_disconnect(ctx, left, right)
    }

    fn fail(ctx: &mut Context<J>, _: crate::FailEntropy) -> Self {
        Self::for_fail(ctx)
    }

    fn const_word(ctx: &mut Context<J>, word: Arc<Value>) -> Self {
        Self::for_const_word(ctx, &word)
    }
}

impl<J: Jet> JetConstructible<J> for Arrow {
    fn jet(ctx: &mut Context<J>, jet: J) -> Self {
        Self::for_jet(ctx, jet)
    }
}

impl<W, J: Jet> WitnessConstructible<W, J> for Arrow {
    fn witness(ctx: &mut Context<J>, _: W) -> Self {
        Self::for_witness(ctx)
    }
}
