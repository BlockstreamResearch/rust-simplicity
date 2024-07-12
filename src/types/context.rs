// SPDX-License-Identifier: CC0-1.0

//! Type Inference Context
//!
//! When constructing a Simplicity program, you must first create a type inference
//! context, in which type inference occurs incrementally during construction. Each
//! leaf node (e.g. `unit` and `iden`) must explicitly refer to the type inference
//! context, while combinator nodes (e.g. `comp`) infer the context from their
//! children, raising an error if there are multiple children whose contexts don't
//! match.
//!
//! This helps to prevent situations in which users attempt to construct multiple
//! independent programs, but types in one program accidentally refer to types in
//! the other.
//!

use std::fmt;
use std::sync::{Arc, Mutex, MutexGuard};

use crate::dag::{Dag, DagLike};

use super::{Bound, CompleteBound, Error, Final, Type};

/// Type inference context, or handle to a context.
///
/// Can be cheaply cloned with [`Context::shallow_clone`]. These clones will
/// refer to the same underlying type inference context, and can be used as
/// handles to each other. The derived [`Context::clone`] has the same effect.
///
/// There is currently no way to create an independent context with the same
/// type inference variables (i.e. a deep clone). If you need this functionality,
/// please file an issue.
#[derive(Clone, Default)]
pub struct Context {
    slab: Arc<Mutex<Vec<Bound>>>,
}

impl fmt::Debug for Context {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let id = Arc::as_ptr(&self.slab) as usize;
        write!(f, "inference_ctx_{:08x}", id)
    }
}

impl PartialEq for Context {
    fn eq(&self, other: &Self) -> bool {
        Arc::ptr_eq(&self.slab, &other.slab)
    }
}
impl Eq for Context {}

impl Context {
    /// Creates a new empty type inference context.
    pub fn new() -> Self {
        Context {
            slab: Arc::new(Mutex::new(vec![])),
        }
    }

    /// Helper function to allocate a bound and return a reference to it.
    fn alloc_bound(&self, bound: Bound) -> BoundRef {
        let mut lock = self.lock();
        lock.alloc_bound(bound)
    }

    /// Allocate a new free type bound, and return a reference to it.
    pub fn alloc_free(&self, name: String) -> BoundRef {
        self.alloc_bound(Bound::Free(name))
    }

    /// Allocate a new unit type bound, and return a reference to it.
    pub fn alloc_unit(&self) -> BoundRef {
        self.alloc_bound(Bound::Complete(Final::unit()))
    }

    /// Allocate a new unit type bound, and return a reference to it.
    pub fn alloc_complete(&self, data: Arc<Final>) -> BoundRef {
        self.alloc_bound(Bound::Complete(data))
    }

    /// Allocate a new sum-type bound, and return a reference to it.
    ///
    /// # Panics
    ///
    /// Panics if either of the child types are from a different inference context.
    pub fn alloc_sum(&self, left: Type, right: Type) -> BoundRef {
        left.bound.root().assert_matches_context(self);
        right.bound.root().assert_matches_context(self);

        let mut lock = self.lock();
        if let Some((data1, data2)) = lock.complete_pair_data(&left, &right) {
            lock.alloc_bound(Bound::Complete(Final::sum(data1, data2)))
        } else {
            lock.alloc_bound(Bound::Sum(left, right))
        }
    }

    /// Allocate a new product-type bound, and return a reference to it.
    ///
    /// # Panics
    ///
    /// Panics if either of the child types are from a different inference context.
    pub fn alloc_product(&self, left: Type, right: Type) -> BoundRef {
        left.bound.root().assert_matches_context(self);
        right.bound.root().assert_matches_context(self);

        let mut lock = self.lock();
        if let Some((data1, data2)) = lock.complete_pair_data(&left, &right) {
            lock.alloc_bound(Bound::Complete(Final::product(data1, data2)))
        } else {
            lock.alloc_bound(Bound::Product(left, right))
        }
    }

    /// Creates a new handle to the context.
    ///
    /// This handle holds a reference to the underlying context and will keep
    /// it alive. The context will only be dropped once all handles, including
    /// the original context object, are dropped.
    pub fn shallow_clone(&self) -> Self {
        Self {
            slab: Arc::clone(&self.slab),
        }
    }

    /// Checks whether two inference contexts are equal, and returns an error if not.
    pub fn check_eq(&self, other: &Self) -> Result<(), super::Error> {
        if self == other {
            Ok(())
        } else {
            Err(super::Error::InferenceContextMismatch)
        }
    }

    /// Accesses a bound.
    ///
    /// # Panics
    ///
    /// Panics if passed a `BoundRef` that was not allocated by this context.
    pub fn get(&self, bound: &BoundRef) -> Bound {
        bound.assert_matches_context(self);
        let lock = self.lock();
        lock.slab[bound.index].shallow_clone()
    }

    /// Reassigns a bound to a different bound.
    ///
    /// # Panics
    ///
    /// Panics if called on a complete type. This is a sanity-check to avoid
    /// replacing already-completed types, which can cause inefficiencies in
    /// the union-bound algorithm (and if our replacement changes the type,
    /// this is probably a bug.
    ///
    /// Also panics if passed a `BoundRef` that was not allocated by this context.
    pub fn reassign_non_complete(&self, bound: BoundRef, new: Bound) {
        let mut lock = self.lock();
        lock.reassign_non_complete(bound, new);
    }

    /// Binds the type to a product bound formed by the two inner types. If this
    /// fails, attach the provided hint to the error.
    ///
    /// Fails if the type has an existing incompatible bound.
    ///
    /// # Panics
    ///
    /// Panics if any of the three types passed in were allocated from a different
    /// context than this one.
    pub fn bind_product(
        &self,
        existing: &Type,
        prod_l: &Type,
        prod_r: &Type,
        hint: &'static str,
    ) -> Result<(), Error> {
        assert_eq!(
            existing.ctx, *self,
            "attempted to bind existing type with wrong context",
        );
        assert_eq!(
            prod_l.ctx, *self,
            "attempted to bind product whose left type had wrong context",
        );
        assert_eq!(
            prod_r.ctx, *self,
            "attempted to bind product whose right type had wrong context",
        );

        let existing_root = existing.bound.root();
        let new_bound = Bound::Product(prod_l.shallow_clone(), prod_r.shallow_clone());

        let mut lock = self.lock();
        lock.bind(existing_root, new_bound).map_err(|e| {
            let new_bound = lock.alloc_bound(e.new);
            Error::Bind {
                existing_bound: Type::wrap_bound(self, e.existing),
                new_bound: Type::wrap_bound(self, new_bound),
                hint,
            }
        })
    }

    /// Unify the type with another one.
    ///
    /// Fails if the bounds on the two types are incompatible
    pub fn unify(&self, ty1: &Type, ty2: &Type, hint: &'static str) -> Result<(), Error> {
        let mut lock = self.lock();
        lock.unify(ty1, ty2).map_err(|e| {
            let new_bound = lock.alloc_bound(e.new);
            Error::Bind {
                existing_bound: Type::wrap_bound(self, e.existing),
                new_bound: Type::wrap_bound(self, new_bound),
                hint,
            }
        })
    }

    /// Locks the underlying slab mutex.
    fn lock(&self) -> LockedContext {
        LockedContext {
            context: Arc::as_ptr(&self.slab),
            slab: self.slab.lock().unwrap(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct BoundRef {
    context: *const Mutex<Vec<Bound>>,
    index: usize,
}

impl BoundRef {
    pub fn assert_matches_context(&self, ctx: &Context) {
        assert_eq!(
            self.context,
            Arc::as_ptr(&ctx.slab),
            "bound was accessed from a type inference context that did not create it",
        );
    }

    /// Creates an "occurs-check ID" which is just a copy of the [`BoundRef`]
    /// with `PartialEq` and `Eq` implemented in terms of underlying pointer
    /// equality.
    pub fn occurs_check_id(&self) -> OccursCheckId {
        OccursCheckId {
            context: self.context,
            index: self.index,
        }
    }
}

impl super::PointerLike for BoundRef {
    fn ptr_eq(&self, other: &Self) -> bool {
        debug_assert_eq!(
            self.context, other.context,
            "tried to compare two bounds from different inference contexts"
        );
        self.index == other.index
    }

    fn shallow_clone(&self) -> Self {
        BoundRef {
            context: self.context,
            index: self.index,
        }
    }
}

impl<'ctx> DagLike for (&'ctx Context, BoundRef) {
    type Node = BoundRef;
    fn data(&self) -> &BoundRef {
        &self.1
    }

    fn as_dag_node(&self) -> Dag<Self> {
        match self.0.get(&self.1) {
            Bound::Free(..) | Bound::Complete(..) => Dag::Nullary,
            Bound::Sum(ref ty1, ref ty2) | Bound::Product(ref ty1, ref ty2) => {
                Dag::Binary((self.0, ty1.bound.root()), (self.0, ty2.bound.root()))
            }
        }
    }
}

#[derive(Copy, Clone, PartialEq, Eq, Debug, Hash)]
pub struct OccursCheckId {
    context: *const Mutex<Vec<Bound>>,
    index: usize,
}

struct BindError {
    existing: BoundRef,
    new: Bound,
}

/// Structure representing an inference context with its slab allocator mutex locked.
///
/// This type is never exposed outside of this module and should only exist
/// ephemerally within function calls into this module.
struct LockedContext<'ctx> {
    context: *const Mutex<Vec<Bound>>,
    slab: MutexGuard<'ctx, Vec<Bound>>,
}

impl<'ctx> LockedContext<'ctx> {
    fn alloc_bound(&mut self, bound: Bound) -> BoundRef {
        self.slab.push(bound);
        let index = self.slab.len() - 1;

        BoundRef {
            context: self.context,
            index,
        }
    }

    fn reassign_non_complete(&mut self, bound: BoundRef, new: Bound) {
        assert!(
            !matches!(self.slab[bound.index], Bound::Complete(..)),
            "tried to modify finalized type",
        );
        self.slab[bound.index] = new;
    }

    /// It is a common situation that we are pairing two types, and in the
    /// case that they are both complete, we want to pair the complete types.
    ///
    /// This method deals with all the annoying/complicated member variable
    /// paths to get the actual complete data out.
    fn complete_pair_data(
        &self,
        inn1: &Type,
        inn2: &Type,
    ) -> Option<(Arc<Final>, Arc<Final>)> {
        let bound1 = &self.slab[inn1.bound.root().index];
        let bound2 = &self.slab[inn2.bound.root().index];
        if let (Bound::Complete(ref data1), Bound::Complete(ref data2)) = (bound1, bound2) {
            Some((Arc::clone(data1), Arc::clone(data2)))
        } else {
            None
        }
    }

    /// Unify the type with another one.
    ///
    /// Fails if the bounds on the two types are incompatible
    fn unify(&mut self, existing: &Type, other: &Type) -> Result<(), BindError> {
        existing.bound.unify(&other.bound, |x_bound, y_bound| {
            self.bind(x_bound, self.slab[y_bound.index].shallow_clone())
        })
    }

    fn bind(&mut self, existing: BoundRef, new: Bound) -> Result<(), BindError> {
        let existing_bound = self.slab[existing.index].shallow_clone();
        let bind_error = || BindError {
            existing: existing.clone(),
            new: new.shallow_clone(),
        };

        match (&existing_bound, &new) {
            // Binding a free type to anything is a no-op
            (_, Bound::Free(_)) => Ok(()),
            // Free types are simply dropped and replaced by the new bound
            (Bound::Free(_), _) => {
                // Free means non-finalized, so set() is ok.
                self.reassign_non_complete(existing, new);
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
            (Bound::Complete(complete), incomplete) | (incomplete, Bound::Complete(complete)) => {
                match (complete.bound(), incomplete) {
                    // A unit might match a Bound::Free(..) or a Bound::Complete(..),
                    // and both cases were handled above. So this is an error.
                    (CompleteBound::Unit, _) => Err(bind_error()),
                    (
                        CompleteBound::Product(ref comp1, ref comp2),
                        Bound::Product(ref ty1, ref ty2),
                    )
                    | (CompleteBound::Sum(ref comp1, ref comp2), Bound::Sum(ref ty1, ref ty2)) => {
                        let bound1 = ty1.bound.root();
                        let bound2 = ty2.bound.root();
                        self.bind(bound1, Bound::Complete(Arc::clone(comp1)))?;
                        self.bind(bound2, Bound::Complete(Arc::clone(comp2)))
                    }
                    _ => Err(bind_error()),
                }
            }
            (Bound::Sum(ref x1, ref x2), Bound::Sum(ref y1, ref y2))
            | (Bound::Product(ref x1, ref x2), Bound::Product(ref y1, ref y2)) => {
                self.unify(x1, y1)?;
                self.unify(x2, y2)?;
                // This type was not complete, but it may be after unification, giving us
                // an opportunity to finaliize it. We do this eagerly to make sure that
                // "complete" (no free children) is always equivalent to "finalized" (the
                // bound field having variant Bound::Complete(..)), even during inference.
                //
                // It also gives the user access to more information about the type,
                // prior to finalization.
                if let Some((data1, data2)) = self.complete_pair_data(y1, y2) {
                    self.reassign_non_complete(
                        existing,
                        Bound::Complete(if let Bound::Sum(..) = existing_bound {
                            Final::sum(data1, data2)
                        } else {
                            Final::product(data1, data2)
                        }),
                    );
                }
                Ok(())
            }
            (_, _) => Err(bind_error()),
        }
    }
}
