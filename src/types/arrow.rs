// SPDX-License-Identifier: CC0-1.0

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
use crate::types::{Bound, Context, Error, Final, Type};
use crate::{jet::Jet, Value};

use super::variable::new_name;

/// A container for an expression's source and target types, whether or not
/// these types are complete.
#[derive(Debug)]
pub struct Arrow {
    /// The source type
    pub source: Type,
    /// The target type
    pub target: Type,
    /// Type inference context for both types.
    pub inference_context: Context,
}

// Having `Clone` makes it easier to derive Clone on structures
// that contain Arrow, even though it is potentially confusing
// to use `.clone` to mean a shallow clone.
impl Clone for Arrow {
    fn clone(&self) -> Self {
        self.shallow_clone()
    }
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

    /// Same as [`Self::clone`] but named to make it clearer that this is cheap
    pub fn shallow_clone(&self) -> Self {
        Arrow {
            source: self.source.shallow_clone(),
            target: self.target.shallow_clone(),
            inference_context: self.inference_context.shallow_clone(),
        }
    }

    /// Create a unification arrow for a fresh `case` combinator
    ///
    /// Either child may be `None`, in which case the combinator is assumed to be
    /// an assertion, which for type-inference purposes means there are no bounds
    /// on the missing child.
    ///
    /// # Panics
    ///
    /// If neither child is provided, this function will panic.
    fn for_case(lchild_arrow: Option<&Arrow>, rchild_arrow: Option<&Arrow>) -> Result<Self, Error> {
        if let (Some(left), Some(right)) = (lchild_arrow, rchild_arrow) {
            left.inference_context.check_eq(&right.inference_context)?;
        }

        let ctx = match (lchild_arrow, rchild_arrow) {
            (Some(left), _) => left.inference_context.shallow_clone(),
            (_, Some(right)) => right.inference_context.shallow_clone(),
            (None, None) => panic!("called `for_case` with no children"),
        };

        let a = Type::free(&ctx, new_name("case_a_"));
        let b = Type::free(&ctx, new_name("case_b_"));
        let c = Type::free(&ctx, new_name("case_c_"));

        let sum_a_b = Type::sum(&ctx, a.shallow_clone(), b.shallow_clone());
        let prod_sum_a_b_c = Type::product(&ctx, sum_a_b, c.shallow_clone());

        let target = Type::free(&ctx, String::new());
        if let Some(lchild_arrow) = lchild_arrow {
            ctx.bind(
                &lchild_arrow.source,
                Bound::Product(a, c.shallow_clone()),
                "case combinator: left source = A × C",
            )?;
            ctx.unify(&target, &lchild_arrow.target, "").unwrap();
        }
        if let Some(rchild_arrow) = rchild_arrow {
            ctx.bind(
                &rchild_arrow.source,
                Bound::Product(b, c),
                "case combinator: left source = B × C",
            )?;
            ctx.unify(
                &target,
                &rchild_arrow.target,
                "case combinator: left target = right target",
            )?;
        }

        Ok(Arrow {
            source: prod_sum_a_b_c,
            target,
            inference_context: ctx,
        })
    }

    /// Helper function to combine code for the two `DisconnectConstructible` impls for [`Arrow`].
    fn for_disconnect(lchild_arrow: &Arrow, rchild_arrow: &Arrow) -> Result<Self, Error> {
        lchild_arrow
            .inference_context
            .check_eq(&rchild_arrow.inference_context)?;

        let ctx = lchild_arrow.inference_context();
        let a = Type::free(ctx, new_name("disconnect_a_"));
        let b = Type::free(ctx, new_name("disconnect_b_"));
        let c = rchild_arrow.source.shallow_clone();
        let d = rchild_arrow.target.shallow_clone();

        let prod_256_a = Bound::Product(Type::two_two_n(ctx, 8), a.shallow_clone());
        let prod_b_c = Bound::Product(b.shallow_clone(), c);
        let prod_b_d = Type::product(ctx, b, d);

        ctx.bind(
            &lchild_arrow.source,
            prod_256_a,
            "disconnect combinator: left source = 2^256 × A",
        )?;
        ctx.bind(
            &lchild_arrow.target,
            prod_b_c,
            "disconnect combinator: left target = B × C",
        )?;

        Ok(Arrow {
            source: a,
            target: prod_b_d,
            inference_context: lchild_arrow.inference_context.shallow_clone(),
        })
    }
}

impl CoreConstructible for Arrow {
    fn iden(inference_context: &Context) -> Self {
        // Throughout this module, when two types are the same, we reuse a
        // pointer to them rather than creating distinct types and unifying
        // them. This theoretically could lead to more confusing errors for
        // the user during type inference, but in practice type inference
        // is completely opaque and there's no harm in making it moreso.
        let new = Type::free(inference_context, new_name("iden_src_"));
        Arrow {
            source: new.shallow_clone(),
            target: new,
            inference_context: inference_context.shallow_clone(),
        }
    }

    fn unit(inference_context: &Context) -> Self {
        Arrow {
            source: Type::free(inference_context, new_name("unit_src_")),
            target: Type::unit(inference_context),
            inference_context: inference_context.shallow_clone(),
        }
    }

    fn injl(child: &Self) -> Self {
        Arrow {
            source: child.source.shallow_clone(),
            target: Type::sum(
                &child.inference_context,
                child.target.shallow_clone(),
                Type::free(&child.inference_context, new_name("injl_tgt_")),
            ),
            inference_context: child.inference_context.shallow_clone(),
        }
    }

    fn injr(child: &Self) -> Self {
        Arrow {
            source: child.source.shallow_clone(),
            target: Type::sum(
                &child.inference_context,
                Type::free(&child.inference_context, new_name("injr_tgt_")),
                child.target.shallow_clone(),
            ),
            inference_context: child.inference_context.shallow_clone(),
        }
    }

    fn take(child: &Self) -> Self {
        Arrow {
            source: Type::product(
                &child.inference_context,
                child.source.shallow_clone(),
                Type::free(&child.inference_context, new_name("take_src_")),
            ),
            target: child.target.shallow_clone(),
            inference_context: child.inference_context.shallow_clone(),
        }
    }

    fn drop_(child: &Self) -> Self {
        Arrow {
            source: Type::product(
                &child.inference_context,
                Type::free(&child.inference_context, new_name("drop_src_")),
                child.source.shallow_clone(),
            ),
            target: child.target.shallow_clone(),
            inference_context: child.inference_context.shallow_clone(),
        }
    }

    fn comp(left: &Self, right: &Self) -> Result<Self, Error> {
        left.inference_context.check_eq(&right.inference_context)?;
        left.inference_context.unify(
            &left.target,
            &right.source,
            "comp combinator: left target = right source",
        )?;
        Ok(Arrow {
            source: left.source.shallow_clone(),
            target: right.target.shallow_clone(),
            inference_context: left.inference_context.shallow_clone(),
        })
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
        left.inference_context.check_eq(&right.inference_context)?;
        left.inference_context.unify(
            &left.source,
            &right.source,
            "pair combinator: left source = right source",
        )?;
        Ok(Arrow {
            source: left.source.shallow_clone(),
            target: Type::product(
                &left.inference_context,
                left.target.shallow_clone(),
                right.target.shallow_clone(),
            ),
            inference_context: left.inference_context.shallow_clone(),
        })
    }

    fn fail(inference_context: &Context, _: crate::FailEntropy) -> Self {
        Arrow {
            source: Type::free(inference_context, new_name("fail_src_")),
            target: Type::free(inference_context, new_name("fail_tgt_")),
            inference_context: inference_context.shallow_clone(),
        }
    }

    fn const_word(inference_context: &Context, word: Arc<Value>) -> Self {
        let len = word.len();
        assert!(len > 0, "Words must not be the empty bitstring");
        assert!(len.is_power_of_two());
        let depth = word.len().trailing_zeros();
        Arrow {
            source: Type::unit(inference_context),
            target: Type::two_two_n(inference_context, depth as usize),
            inference_context: inference_context.shallow_clone(),
        }
    }

    fn inference_context(&self) -> &Context {
        &self.inference_context
    }
}

impl DisconnectConstructible<Arrow> for Arrow {
    fn disconnect(left: &Self, right: &Self) -> Result<Self, Error> {
        Self::for_disconnect(left, right)
    }
}

impl DisconnectConstructible<NoDisconnect> for Arrow {
    fn disconnect(left: &Self, _: &NoDisconnect) -> Result<Self, Error> {
        let source = Type::free(&left.inference_context, "disc_src".into());
        let target = Type::free(&left.inference_context, "disc_tgt".into());
        Self::for_disconnect(
            left,
            &Arrow {
                source,
                target,
                inference_context: left.inference_context.shallow_clone(),
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
    fn jet(inference_context: &Context, jet: J) -> Self {
        Arrow {
            source: jet.source_ty().to_type(inference_context),
            target: jet.target_ty().to_type(inference_context),
            inference_context: inference_context.shallow_clone(),
        }
    }
}

impl<W> WitnessConstructible<W> for Arrow {
    fn witness(inference_context: &Context, _: W) -> Self {
        Arrow {
            source: Type::free(inference_context, new_name("witness_src_")),
            target: Type::free(inference_context, new_name("witness_tgt_")),
            inference_context: inference_context.shallow_clone(),
        }
    }
}
