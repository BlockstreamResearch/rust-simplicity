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
use std::sync::{Arc, Mutex};

use super::bound_mutex::BoundMutex;
use super::{Bound, Error, Final, Type};

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
        BoundRef {
            context: Arc::as_ptr(&self.slab),
            index: Arc::new(BoundMutex::new(bound)),
        }
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

        self.alloc_bound(Bound::sum(left, right))
    }

    /// Allocate a new product-type bound, and return a reference to it.
    ///
    /// # Panics
    ///
    /// Panics if either of the child types are from a different inference context.
    pub fn alloc_product(&self, left: Type, right: Type) -> BoundRef {
        left.bound.root().assert_matches_context(self);
        right.bound.root().assert_matches_context(self);

        self.alloc_bound(Bound::product(left, right))
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

    /// Binds the type to a given bound. If this fails, attach the provided
    /// hint to the error.
    ///
    /// Fails if the type has an existing incompatible bound.
    pub fn bind(&self, existing: &Type, new: Arc<Bound>, hint: &'static str) -> Result<(), Error> {
        existing.bind(new, hint)
    }

    /// Unify the type with another one.
    ///
    /// Fails if the bounds on the two types are incompatible
    pub fn unify(&self, ty1: &Type, ty2: &Type, hint: &'static str) -> Result<(), Error> {
        ty1.unify(ty2, hint)
    }
}

#[derive(Debug)]
pub struct BoundRef {
    context: *const Mutex<Vec<Bound>>,
    // Will become an index into the context in a latter commit, but for
    // now we set it to an Arc<BoundMutex> to preserve semantics.
    index: Arc<BoundMutex>,
}

impl BoundRef {
    pub fn assert_matches_context(&self, ctx: &Context) {
        assert_eq!(
            self.context,
            Arc::as_ptr(&ctx.slab),
            "bound was accessed from a type inference context that did not create it",
        );
    }

    pub fn get(&self) -> Arc<Bound> {
        self.index.get()
    }

    pub fn set(&self, new: Arc<Bound>) {
        self.index.set(new)
    }

    pub fn bind(&self, bound: Arc<Bound>, hint: &'static str) -> Result<(), Error> {
        self.index.bind(bound, hint)
    }
}

impl super::PointerLike for BoundRef {
    fn ptr_eq(&self, other: &Self) -> bool {
        debug_assert_eq!(
            self.context, other.context,
            "tried to compare two bounds from different inference contexts"
        );
        Arc::ptr_eq(&self.index, &other.index)
    }

    fn shallow_clone(&self) -> Self {
        BoundRef {
            context: self.context,
            index: Arc::clone(&self.index),
        }
    }
}
