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
use std::marker::PhantomData;
use std::sync::{Arc, Mutex, MutexGuard};

use ghost_cell::GhostToken;

use crate::dag::{Dag, DagLike};

use super::{Bound, CompleteBound, Error, Final, Incomplete, Type, TypeInner, WithGhostToken};

// Copied from ghost_cell source. See
//     https://arhan.sh/blog/the-generativity-pattern-in-rust/
// in particular the box labeled "Throughout the years lifetime invariance has
// been achieved in several other ways." for some detail about this.
type InvariantLifetime<'brand> = PhantomData<fn(&'brand ()) -> &'brand ()>;

/// Type inference context, or handle to a context.
///
/// Can be cheaply cloned with [`Context::shallow_clone`]. These clones will
/// refer to the same underlying type inference context, and can be used as
/// handles to each other. The derived [`Context::clone`] has the same effect.
///
/// There is currently no way to create an independent context with the same
/// type inference variables (i.e. a deep clone). If you need this functionality,
/// please file an issue.
#[derive(Clone)]
pub struct Context<'brand> {
    inner: Arc<Mutex<WithGhostToken<'brand, ContextInner<'brand>>>>,
}

struct ContextInner<'brand> {
    slab: Vec<Bound<'brand>>,
}

impl fmt::Debug for Context<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let id = Arc::as_ptr(&self.inner) as usize;
        write!(f, "inference_ctx_{:08x}", id)
    }
}

impl PartialEq for Context<'_> {
    fn eq(&self, other: &Self) -> bool {
        Arc::ptr_eq(&self.inner, &other.inner)
    }
}
impl Eq for Context<'_> {}

impl<'brand> Context<'brand> {
    /// Creates a scope with a new empty type inference context.
    pub fn with_context<R, F>(fun: F) -> R
    where
        F: for<'new_brand> FnOnce(Context<'new_brand>) -> R,
    {
        GhostToken::new(|token| {
            let ctx = Context::new(token);
            fun(ctx)
        })
    }

    /// Creates a new empty type inference context.
    pub fn new(token: GhostToken<'brand>) -> Self {
        Context {
            inner: Arc::new(Mutex::new(WithGhostToken {
                _token: token,
                inner: ContextInner { slab: vec![] },
            })),
        }
    }

    /// Helper function to allocate a bound and return a reference to it.
    fn alloc_bound(&self, bound: Bound<'brand>) -> BoundRef<'brand> {
        let mut lock = self.lock();
        lock.alloc_bound(bound)
    }

    /// Allocate a new free type bound, and return a reference to it.
    pub fn alloc_free(&self, name: String) -> BoundRef<'brand> {
        self.alloc_bound(Bound::Free(name))
    }

    /// Allocate a new unit type bound, and return a reference to it.
    pub fn alloc_unit(&self) -> BoundRef<'brand> {
        self.alloc_bound(Bound::Complete(Final::unit()))
    }

    /// Allocate a new unit type bound, and return a reference to it.
    pub fn alloc_complete(&self, data: Arc<Final>) -> BoundRef<'brand> {
        self.alloc_bound(Bound::Complete(data))
    }

    /// Allocate a new sum-type bound, and return a reference to it.
    pub fn alloc_sum(&self, left: Type<'brand>, right: Type<'brand>) -> BoundRef<'brand> {
        let mut lock = self.lock();
        if let Some((data1, data2)) = lock.complete_pair_data(&left.inner, &right.inner) {
            lock.alloc_bound(Bound::Complete(Final::sum(data1, data2)))
        } else {
            lock.alloc_bound(Bound::Sum(left.inner, right.inner))
        }
    }

    /// Allocate a new product-type bound, and return a reference to it.
    pub fn alloc_product(&self, left: Type<'brand>, right: Type<'brand>) -> BoundRef<'brand> {
        let mut lock = self.lock();
        if let Some((data1, data2)) = lock.complete_pair_data(&left.inner, &right.inner) {
            lock.alloc_bound(Bound::Complete(Final::product(data1, data2)))
        } else {
            lock.alloc_bound(Bound::Product(left.inner, right.inner))
        }
    }

    /// Creates a new handle to the context.
    ///
    /// This handle holds a reference to the underlying context and will keep
    /// it alive. The context will only be dropped once all handles, including
    /// the original context object, are dropped.
    pub fn shallow_clone(&self) -> Self {
        Self {
            inner: Arc::clone(&self.inner),
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
    pub(super) fn get(&self, bound: &BoundRef<'brand>) -> Bound<'brand> {
        let lock = self.lock();
        lock.inner.slab[bound.index].shallow_clone()
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
    pub(super) fn reassign_non_complete(&self, bound: BoundRef<'brand>, new: Bound<'brand>) {
        let mut lock = self.lock();
        lock.reassign_non_complete(bound, new);
    }

    /// Binds the type to a product bound formed by the two inner types. If this
    /// fails, attach the provided hint to the error.
    ///
    /// Fails if the type has an existing incompatible bound.
    pub fn bind_product(
        &self,
        existing: &Type<'brand>,
        prod_l: &Type<'brand>,
        prod_r: &Type<'brand>,
        hint: &'static str,
    ) -> Result<(), Error> {
        let existing_root = existing.inner.bound.root();
        let new_bound = Bound::Product(prod_l.inner.shallow_clone(), prod_r.inner.shallow_clone());

        let mut lock = self.lock();
        lock.bind(existing_root, new_bound).map_err(|e| {
            let new_bound = lock.alloc_bound(e.new);
            drop(lock);
            Error::Bind {
                existing_bound: Incomplete::from_bound_ref(self, e.existing),
                new_bound: Incomplete::from_bound_ref(self, new_bound),
                hint,
            }
        })
    }

    /// Unify the type with another one.
    ///
    /// Fails if the bounds on the two types are incompatible
    pub fn unify(
        &self,
        ty1: &Type<'brand>,
        ty2: &Type<'brand>,
        hint: &'static str,
    ) -> Result<(), Error> {
        let mut lock = self.lock();
        lock.unify(&ty1.inner, &ty2.inner).map_err(|e| {
            let new_bound = lock.alloc_bound(e.new);
            drop(lock);
            Error::Bind {
                existing_bound: Incomplete::from_bound_ref(self, e.existing),
                new_bound: Incomplete::from_bound_ref(self, new_bound),
                hint,
            }
        })
    }

    /// Locks the underlying slab mutex.
    fn lock(&self) -> LockedContext<'_, 'brand> {
        LockedContext {
            inner: self.inner.lock().unwrap(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct BoundRef<'brand> {
    phantom: InvariantLifetime<'brand>,
    index: usize,
}

impl<'brand> BoundRef<'brand> {
    /// Creates an "occurs-check ID" which is just a copy of the [`BoundRef`]
    /// with `PartialEq` and `Eq` implemented in terms of underlying pointer
    /// equality.
    pub fn occurs_check_id(&self) -> OccursCheckId<'brand> {
        OccursCheckId {
            phantom: InvariantLifetime::default(),
            index: self.index,
        }
    }
}

impl super::PointerLike for BoundRef<'_> {
    fn ptr_eq(&self, other: &Self) -> bool {
        self.index == other.index
    }

    fn shallow_clone(&self) -> Self {
        BoundRef {
            phantom: InvariantLifetime::default(),
            index: self.index,
        }
    }
}

impl<'brand> DagLike for (&'_ Context<'brand>, BoundRef<'brand>) {
    type Node = BoundRef<'brand>;
    fn data(&self) -> &BoundRef<'brand> {
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
pub struct OccursCheckId<'brand> {
    phantom: InvariantLifetime<'brand>,
    index: usize,
}

struct BindError<'brand> {
    existing: BoundRef<'brand>,
    new: Bound<'brand>,
}

/// Structure representing an inference context with its slab allocator mutex locked.
///
/// This type is never exposed outside of this module and should only exist
/// ephemerally within function calls into this module.
struct LockedContext<'ctx, 'brand> {
    inner: MutexGuard<'ctx, WithGhostToken<'brand, ContextInner<'brand>>>,
}

impl<'brand> LockedContext<'_, 'brand> {
    fn alloc_bound(&mut self, bound: Bound<'brand>) -> BoundRef<'brand> {
        self.inner.slab.push(bound);
        let index = self.inner.slab.len() - 1;

        BoundRef {
            phantom: InvariantLifetime::default(),
            index,
        }
    }

    fn reassign_non_complete(&mut self, bound: BoundRef<'brand>, new: Bound<'brand>) {
        assert!(
            !matches!(self.inner.slab[bound.index], Bound::Complete(..)),
            "tried to modify finalized type",
        );
        self.inner.slab[bound.index] = new;
    }

    /// It is a common situation that we are pairing two types, and in the
    /// case that they are both complete, we want to pair the complete types.
    ///
    /// This method deals with all the annoying/complicated member variable
    /// paths to get the actual complete data out.
    fn complete_pair_data(
        &self,
        inn1: &TypeInner<'brand>,
        inn2: &TypeInner<'brand>,
    ) -> Option<(Arc<Final>, Arc<Final>)> {
        let bound1 = &self.inner.slab[inn1.bound.root().index];
        let bound2 = &self.inner.slab[inn2.bound.root().index];
        if let (Bound::Complete(ref data1), Bound::Complete(ref data2)) = (bound1, bound2) {
            Some((Arc::clone(data1), Arc::clone(data2)))
        } else {
            None
        }
    }

    /// Unify the type with another one.
    ///
    /// Fails if the bounds on the two types are incompatible
    fn unify(
        &mut self,
        existing: &TypeInner<'brand>,
        other: &TypeInner<'brand>,
    ) -> Result<(), BindError<'brand>> {
        existing.bound.unify(&other.bound, |x_bound, y_bound| {
            self.bind(x_bound, self.inner.slab[y_bound.index].shallow_clone())
        })
    }

    fn bind(
        &mut self,
        existing: BoundRef<'brand>,
        new: Bound<'brand>,
    ) -> Result<(), BindError<'brand>> {
        let existing_bound = self.inner.slab[existing.index].shallow_clone();
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
