// SPDX-License-Identifier: CC0-1.0

//! Types and Type Inference
//!
//! Every Simplicity expression has two types associated with it: a source and
//! a target. We refer to this pair of types as an "arrow". The types are
//! inferred from the structure of the program.
//!
//! Simplicity types are one of three things
//!   * A unit type, which has one value
//!   * A sum of two other types
//!   * A product of two other types
//!
//! During type inference, types are initially "free", meaning that there are
//! no constraints on what they will eventually be. The program structure then
//! imposes additional constraints (for example, the `comp` combinator requires
//! that its left child's target type be the same as its right child's source
//! type), and by unifying all these constraints, all types can be inferred.
//!
//! In this module, during inference types are characterized by their [`Bound`],
//! which describes the constraints on the type. The bound of a type can be
//! obtained by the [`Type::bound`] method, and is an enum with four variants:
//!
//! * [`Bound::Free`] means that the type has no constraints; it is a free
//!   variable. The type has a name which can be used to identify it in error
//!   messages.
//! * [`Bound::Sum`] and [`Bound::Product`] means that the the type is a sum
//!   (resp. product) of two other types, which are characterized by their
//!   own bounds.
//! * [`Bound::Complete`] means that the type has no free variables at all,
//!   and has an already-computed [`Final`] structure suitable for use in
//!   contexts that require complete types. (Unit types are always complete,
//!   and therefore use this variant rather than getting their own.)
//!
//! During inference, it is possible for a type to be complete, in the sense
//! of having no free variables, without its bound being [`Bound::Complete`].
//! This occurs, for example, if a type is a sum of two incomplete types, then
//! the child types are completed during type inference on an unrelated part
//! of the type hierarchy. The type would then have a [`Bound::Sum`] with two
//! children, both of which are complete.
//!
//! The inference engine makes an effort to notice when this happens and set
//! the bound of complete types to [`Bound::Complete`], but since type inference
//! is inherently non-local this cannot always be done.
//!
//! When the distinction matters, we say a type is "finalized" only if its bound
//! is `Complete` and "complete" if it has no free variables. But the distinction
//! usually does not matter, so we prefer to use the word "complete".
//!
//! Type inference is done progressively during construction of Simplicity
//! expressions. It is completed by the [`Type::finalize`] method, which
//! recursively completes types by setting any remaining free variables to unit.
//! If any type constraints are incompatible with each other (e.g. a type is
//! bound to be both a product and a sum type) then inference fails at that point
//! by returning an error.
//!
//! In addition to completing types [`Type::finalize`], does one additional
//! check, the "occurs check", to ensures that there are no infinitely-sized
//! types. Such types occur when a type has itself as a child, are illegal in
//! Simplicity, and could not be represented by our data structures.
//!
//! There are three main types in this module:
//!   * [`Type`] is the main type representing a Simplicity type, whether it is
//!     complete or not. Its main methods are [`Type::bound`] which returns the
//!     current state of the type and [`Type::bind`] which adds a new constraint
//!     to the type.
//!   * `Final` is a mutex-free structure that can be obtained from a complete
//!     type. It includes the TMR and the complete bound describing the type.
//!   * `Bound` defines the structure of a type: whether it is free, complete,
//!     or a sum or product of other types.
//!

use self::union_bound::{PointerLike, UbElement};
use crate::dag::{Dag, DagLike, NoSharing};
use crate::Tmr;

use std::collections::HashSet;
use std::fmt;
use std::sync::Arc;

pub mod arrow;
mod context;
mod final_data;
mod precomputed;
mod union_bound;
mod variable;

pub use context::{BoundRef, Context};
pub use final_data::{CompleteBound, Final};

/// Error type for simplicity
#[non_exhaustive]
#[derive(Clone, Debug)]
pub enum Error {
    /// An attempt to bind a type conflicted with an existing bound on the type
    Bind {
        existing_bound: Bound,
        new_bound: Bound,
        hint: &'static str,
    },
    /// Two unequal complete types were attempted to be unified
    CompleteTypeMismatch {
        type1: Arc<Final>,
        type2: Arc<Final>,
        hint: &'static str,
    },
    /// A type is recursive (i.e., occurs within itself), violating the "occurs check"
    OccursCheck { infinite_bound: Bound },
    /// Attempted to combine two nodes which had different type inference
    /// contexts. This is probably a programming error.
    InferenceContextMismatch,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::Bind {
                ref existing_bound,
                ref new_bound,
                hint,
            } => {
                write!(
                    f,
                    "failed to apply bound `{}` to existing bound `{}`: {}",
                    new_bound, existing_bound, hint,
                )
            }
            Error::CompleteTypeMismatch {
                ref type1,
                ref type2,
                hint,
            } => {
                write!(
                    f,
                    "attempted to unify unequal types `{}` and `{}`: {}",
                    type1, type2, hint,
                )
            }
            Error::OccursCheck { infinite_bound } => {
                write!(f, "infinitely-sized type {}", infinite_bound,)
            }
            Error::InferenceContextMismatch => {
                f.write_str("attempted to combine two nodes with different type inference contexts")
            }
        }
    }
}

impl std::error::Error for Error {}

mod bound_mutex {
    use super::{Bound, CompleteBound, Error, Final};
    use std::fmt;
    use std::sync::{Arc, Mutex};

    /// Source or target type of a Simplicity expression
    pub struct BoundMutex {
        /// The type's status according to the union-bound algorithm.
        inner: Mutex<Bound>,
    }

    impl fmt::Debug for BoundMutex {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            self.get().fmt(f)
        }
    }

    impl BoundMutex {
        pub fn new(bound: Bound) -> Self {
            BoundMutex {
                inner: Mutex::new(bound),
            }
        }

        pub fn get(&self) -> Bound {
            self.inner.lock().unwrap().shallow_clone()
        }

        pub fn set(&self, new: Bound) {
            let mut lock = self.inner.lock().unwrap();
            assert!(
                !matches!(*lock, Bound::Complete(..)),
                "tried to modify finalized type",
            );
            *lock = new;
        }

        pub fn bind(&self, bound: Bound, hint: &'static str) -> Result<(), Error> {
            let existing_bound = self.get();
            let bind_error = || Error::Bind {
                existing_bound: existing_bound.shallow_clone(),
                new_bound: bound.shallow_clone(),
                hint,
            };

            match (&existing_bound, &bound) {
                // Binding a free type to anything is a no-op
                (_, Bound::Free(_)) => Ok(()),
                // Free types are simply dropped and replaced by the new bound
                (Bound::Free(_), _) => {
                    // Free means non-finalized, so set() is ok.
                    self.set(bound);
                    Ok(())
                }
                // Binding complete->complete shouldn't ever happen, but if so, we just
                // compare the two types and return a pass/fail
                (Bound::Complete(ref existing_final), Bound::Complete(ref new_final)) => {
                    if existing_final == new_final {
                        Ok(())
                    } else {
                        Err(bind_error())
                    }
                }
                // Binding an incomplete to a complete type requires recursion.
                (Bound::Complete(complete), incomplete)
                | (incomplete, Bound::Complete(complete)) => {
                    match (complete.bound(), incomplete) {
                        // A unit might match a Bound::Free(..) or a Bound::Complete(..),
                        // and both cases were handled above. So this is an error.
                        (CompleteBound::Unit, _) => Err(bind_error()),
                        (
                            CompleteBound::Product(ref comp1, ref comp2),
                            Bound::Product(ref ty1, ref ty2),
                        )
                        | (
                            CompleteBound::Sum(ref comp1, ref comp2),
                            Bound::Sum(ref ty1, ref ty2),
                        ) => {
                            ty1.bind(Bound::Complete(Arc::clone(comp1)), hint)?;
                            ty2.bind(Bound::Complete(Arc::clone(comp2)), hint)
                        }
                        _ => Err(bind_error()),
                    }
                }
                (Bound::Sum(ref x1, ref x2), Bound::Sum(ref y1, ref y2))
                | (Bound::Product(ref x1, ref x2), Bound::Product(ref y1, ref y2)) => {
                    x1.unify(y1, hint)?;
                    x2.unify(y2, hint)?;
                    // This type was not complete, but it may be after unification, giving us
                    // an opportunity to finaliize it. We do this eagerly to make sure that
                    // "complete" (no free children) is always equivalent to "finalized" (the
                    // bound field having variant Bound::Complete(..)), even during inference.
                    //
                    // It also gives the user access to more information about the type,
                    // prior to finalization.
                    if let (Some(data1), Some(data2)) = (y1.final_data(), y2.final_data()) {
                        self.set(Bound::Complete(if let Bound::Sum(..) = bound {
                            Final::sum(data1, data2)
                        } else {
                            Final::product(data1, data2)
                        }));
                    }
                    Ok(())
                }
                (x, y) => Err(Error::Bind {
                    existing_bound: x.shallow_clone(),
                    new_bound: y.shallow_clone(),
                    hint,
                }),
            }
        }
    }
}

/// The state of a [`Type`] based on all constraints currently imposed on it.
#[derive(Clone, Debug)]
pub enum Bound {
    /// Fully-unconstrained type
    Free(String),
    /// Fully-constrained (i.e. complete) type, which has no free variables.
    Complete(Arc<Final>),
    /// A sum of two other types
    Sum(Type, Type),
    /// A product of two other types
    Product(Type, Type),
}

impl fmt::Display for Bound {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Bound::Free(s) => f.write_str(s),
            Bound::Complete(comp) => comp.fmt(f),
            Bound::Sum(ty1, ty2) => {
                ty1.fmt(f)?;
                f.write_str(" + ")?;
                ty2.fmt(f)
            }
            Bound::Product(ty1, ty2) => {
                ty1.fmt(f)?;
                f.write_str(" × ")?;
                ty2.fmt(f)
            }
        }
    }
}

impl Bound {
    /// Clones the `Bound`.
    ///
    /// This is the same as just calling `.clone()` but has a different name to
    /// emphasize that what's being cloned is (at most) a pair of ref-counted
    /// pointers.
    pub fn shallow_clone(&self) -> Bound {
        self.clone()
    }

    fn sum(a: Type, b: Type) -> Self {
        if let (Some(adata), Some(bdata)) = (a.final_data(), b.final_data()) {
            Bound::Complete(Final::sum(adata, bdata))
        } else {
            Bound::Sum(a, b)
        }
    }

    fn product(a: Type, b: Type) -> Self {
        if let (Some(adata), Some(bdata)) = (a.final_data(), b.final_data()) {
            Bound::Complete(Final::product(adata, bdata))
        } else {
            Bound::Product(a, b)
        }
    }
}

/// Source or target type of a Simplicity expression.
///
/// Internally this type is essentially just a refcounted pointer; it is
/// therefore quite cheap to clone, but be aware that cloning will not
/// actually create a new independent type, just a second pointer to the
/// first one.
#[derive(Clone)]
pub struct Type {
    ctx: Context,
    /// A set of constraints, which maintained by the union-bound algorithm and
    /// is progressively tightened as type inference proceeds.
    bound: UbElement<BoundRef>,
}

impl Type {
    /// Return an unbound type with the given name
    pub fn free(ctx: &Context, name: String) -> Self {
        Type {
            ctx: ctx.shallow_clone(),
            bound: UbElement::new(ctx.alloc_free(name)),
        }
    }

    /// Create the unit type.
    pub fn unit(ctx: &Context) -> Self {
        Type {
            ctx: ctx.shallow_clone(),
            bound: UbElement::new(ctx.alloc_unit()),
        }
    }

    /// Create the type `2^(2^n)` for the given `n`.
    ///
    /// The type is precomputed and fast to access.
    pub fn two_two_n(ctx: &Context, n: usize) -> Self {
        Self::complete(ctx, precomputed::nth_power_of_2(n))
    }

    /// Create the sum of the given `left` and `right` types.
    pub fn sum(ctx: &Context, left: Self, right: Self) -> Self {
        Type {
            ctx: ctx.shallow_clone(),
            bound: UbElement::new(ctx.alloc_sum(left, right)),
        }
    }

    /// Create the product of the given `left` and `right` types.
    pub fn product(ctx: &Context, left: Self, right: Self) -> Self {
        Type {
            ctx: ctx.shallow_clone(),
            bound: UbElement::new(ctx.alloc_product(left, right)),
        }
    }

    /// Create a complete type.
    pub fn complete(ctx: &Context, final_data: Arc<Final>) -> Self {
        Type {
            ctx: ctx.shallow_clone(),
            bound: UbElement::new(ctx.alloc_complete(final_data)),
        }
    }

    /// Clones the `Type`.
    ///
    /// This is the same as just calling `.clone()` but has a different name to
    /// emphasize that what's being cloned is merely a ref-counted pointer.
    pub fn shallow_clone(&self) -> Type {
        self.clone()
    }

    /// Binds the type to a given bound. If this fails, attach the provided
    /// hint to the error.
    ///
    /// Fails if the type has an existing incompatible bound.
    fn bind(&self, bound: Bound, hint: &'static str) -> Result<(), Error> {
        let root = self.bound.root();
        root.bind(bound, hint)
    }

    /// Unify the type with another one.
    ///
    /// Fails if the bounds on the two types are incompatible
    fn unify(&self, other: &Self, hint: &'static str) -> Result<(), Error> {
        self.bound.unify(&other.bound, |x_bound, y_bound| {
            x_bound.bind(self.ctx.get(y_bound), hint)
        })
    }

    /// Accessor for this type's bound
    pub fn bound(&self) -> Bound {
        self.ctx.get(&self.bound.root())
    }

    /// Accessor for the TMR of this type, if it is final
    pub fn tmr(&self) -> Option<Tmr> {
        self.final_data().map(|data| data.tmr())
    }

    /// Accessor for the data of this type, if it is complete
    pub fn final_data(&self) -> Option<Arc<Final>> {
        if let Bound::Complete(ref data) = self.bound() {
            Some(Arc::clone(data))
        } else {
            None
        }
    }

    /// Whether this type is known to be final
    ///
    /// During type inference this may be false even though the type is, in fact,
    /// complete, since its children may have been unified to a complete type. To
    /// ensure a type is complete, call [`Type::finalize`].
    pub fn is_final(&self) -> bool {
        self.final_data().is_some()
    }

    /// Attempts to finalize the type. Returns its TMR on success.
    pub fn finalize(&self) -> Result<Arc<Final>, Error> {
        use context::OccursCheckId;

        /// Helper type for the occurs-check.
        enum OccursCheckStack {
            Iterate(BoundRef),
            Complete(OccursCheckId),
        }

        // Done with sharing tracker. Actual algorithm follows.
        let root = self.bound.root();
        let bound = self.ctx.get(&root);
        if let Bound::Complete(ref data) = bound {
            return Ok(Arc::clone(data));
        }

        // First, do occurs-check to ensure that we have no infinitely sized types.
        let mut stack = vec![OccursCheckStack::Iterate(root)];
        let mut in_progress = HashSet::new();
        let mut completed = HashSet::new();
        while let Some(top) = stack.pop() {
            let bound = match top {
                OccursCheckStack::Complete(id) => {
                    in_progress.remove(&id);
                    completed.insert(id);
                    continue;
                }
                OccursCheckStack::Iterate(b) => b,
            };

            let id = bound.occurs_check_id();
            if completed.contains(&id) {
                // Once we have iterated through a type, we don't need to check it again.
                // Without this shortcut the occurs-check would take exponential time.
                continue;
            }
            if !in_progress.insert(id) {
                return Err(Error::OccursCheck {
                    infinite_bound: self.ctx.get(&bound),
                });
            }

            stack.push(OccursCheckStack::Complete(id));
            if let Some((_, child)) = (&self.ctx, bound.shallow_clone()).right_child() {
                stack.push(OccursCheckStack::Iterate(child));
            }
            if let Some((_, child)) = (&self.ctx, bound).left_child() {
                stack.push(OccursCheckStack::Iterate(child));
            }
        }

        // Now that we know our types have finite size, we can safely use a
        // post-order iterator to finalize them.
        let mut finalized = vec![];
        for data in self.shallow_clone().post_order_iter::<NoSharing>() {
            let bound = data.node.bound.root();
            let bound_get = self.ctx.get(&bound);
            let final_data = match bound_get {
                Bound::Free(_) => Final::unit(),
                Bound::Complete(ref arc) => Arc::clone(arc),
                Bound::Sum(..) => Final::sum(
                    Arc::clone(&finalized[data.left_index.unwrap()]),
                    Arc::clone(&finalized[data.right_index.unwrap()]),
                ),
                Bound::Product(..) => Final::product(
                    Arc::clone(&finalized[data.left_index.unwrap()]),
                    Arc::clone(&finalized[data.right_index.unwrap()]),
                ),
            };

            if !matches!(bound_get, Bound::Complete(..)) {
                self.ctx
                    .reassign_non_complete(bound, Bound::Complete(Arc::clone(&final_data)));
            }
            finalized.push(final_data);
        }
        Ok(finalized.pop().unwrap())
    }

    /// Return a vector containing the types 2^(2^i) for i from 0 to n-1.
    pub fn powers_of_two(ctx: &Context, n: usize) -> Vec<Self> {
        let mut ret = Vec::with_capacity(n);

        let unit = Type::unit(ctx);
        let mut two = Type::sum(ctx, unit.shallow_clone(), unit);
        for _ in 0..n {
            ret.push(two.shallow_clone());
            two = Type::product(ctx, two.shallow_clone(), two);
        }
        ret
    }
}

const MAX_DISPLAY_DEPTH: usize = 64;

impl fmt::Debug for Type {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for data in (&self.ctx, self.bound.root())
            .verbose_pre_order_iter::<NoSharing>(Some(MAX_DISPLAY_DEPTH))
        {
            if data.depth == MAX_DISPLAY_DEPTH {
                if data.n_children_yielded == 0 {
                    f.write_str("...")?;
                }
                continue;
            }
            let bound = data.node.0.get(&data.node.1);
            match (bound, data.n_children_yielded) {
                (Bound::Free(ref s), _) => f.write_str(s)?,
                (Bound::Complete(ref comp), _) => fmt::Debug::fmt(comp, f)?,
                (Bound::Sum(..), 0) | (Bound::Product(..), 0) => {
                    if data.index > 0 {
                        f.write_str("(")?;
                    }
                }
                (Bound::Sum(..), 2) | (Bound::Product(..), 2) => {
                    if data.index > 0 {
                        f.write_str(")")?
                    }
                }
                (Bound::Sum(..), _) => f.write_str(" + ")?,
                (Bound::Product(..), _) => f.write_str(" × ")?,
            }
        }
        Ok(())
    }
}

impl fmt::Display for Type {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for data in (&self.ctx, self.bound.root())
            .verbose_pre_order_iter::<NoSharing>(Some(MAX_DISPLAY_DEPTH))
        {
            if data.depth == MAX_DISPLAY_DEPTH {
                if data.n_children_yielded == 0 {
                    f.write_str("...")?;
                }
                continue;
            }
            let bound = data.node.0.get(&data.node.1);
            match (bound, data.n_children_yielded) {
                (Bound::Free(ref s), _) => f.write_str(s)?,
                (Bound::Complete(ref comp), _) => fmt::Display::fmt(comp, f)?,
                (Bound::Sum(..), 0) | (Bound::Product(..), 0) => {
                    if data.index > 0 {
                        f.write_str("(")?;
                    }
                }
                (Bound::Sum(..), 2) | (Bound::Product(..), 2) => {
                    if data.index > 0 {
                        f.write_str(")")?
                    }
                }
                (Bound::Sum(..), _) => f.write_str(" + ")?,
                (Bound::Product(..), _) => f.write_str(" × ")?,
            }
        }
        Ok(())
    }
}

impl DagLike for Type {
    type Node = Type;
    fn data(&self) -> &Type {
        self
    }

    fn as_dag_node(&self) -> Dag<Self> {
        match self.bound() {
            Bound::Free(..) | Bound::Complete(..) => Dag::Nullary,
            Bound::Sum(ref ty1, ref ty2) | Bound::Product(ref ty1, ref ty2) => {
                Dag::Binary(ty1.shallow_clone(), ty2.shallow_clone())
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::jet::Core;
    use crate::node::{ConstructNode, CoreConstructible, WitnessNode};

    #[test]
    fn inference_failure() {
        let ctx = Context::new();

        // unit: A -> 1
        let unit = Arc::<ConstructNode<Core>>::unit(&ctx); // 1 -> 1

        // Force unit to be 1->1
        Arc::<ConstructNode<Core>>::comp(&unit, &unit).unwrap();

        // take unit: 1 * B -> 1
        let take_unit = Arc::<ConstructNode<Core>>::take(&unit); // 1*1 -> 1

        // Pair will try to unify 1 and 1*B
        Arc::<ConstructNode<Core>>::pair(&unit, &take_unit).unwrap_err();
        // Trying to do it again should not work.
        Arc::<ConstructNode<Core>>::pair(&unit, &take_unit).unwrap_err();
    }

    #[test]
    fn memory_leak() {
        let ctx = Context::new();
        let iden = Arc::<WitnessNode<Core>>::iden(&ctx);
        let drop = Arc::<WitnessNode<Core>>::drop_(&iden);
        let case = Arc::<WitnessNode<Core>>::case(&iden, &drop).unwrap();

        format!("{:?}", case.arrow().source);
    }
}
