// SPDX-License-Identifier: CC0-1.0

//! "Finalized" Incomplete Type Data
//!
//! This structure is essentially the same as [types::Final](super::Final) except
//! that it has free variables (represented by strings) and supports self-reference.
//! The purpose of this structure is to provide a useful representation of a type
//! in error messages.
//!

use crate::dag::{Dag, DagLike, NoSharing};
use crate::types::union_bound::PointerLike;

use super::context::BoundRefSharing;
use super::{Bound, BoundRef, Context};
use super::{MAX_DISPLAY_DEPTH, MAX_DISPLAY_LENGTH};

use std::fmt;
use std::sync::Arc;

/// An incomplete type bound for use in error messages.
#[derive(Clone)]
pub enum Incomplete {
    /// A free variable.
    Free(String),
    /// A type containing this type.
    Cycle,
    /// A sum of two other types
    Sum(Arc<Incomplete>, Arc<Incomplete>),
    /// A product of two other types
    Product(Arc<Incomplete>, Arc<Incomplete>),
    /// A complete type (including unit)
    Final(Arc<super::Final>),
}

impl Incomplete {
    /// Private helper function for `Incomplete::drop`. See `node::Inner::into_dag` which is
    /// similar but doesn't require unsafe code.
    fn into_dag(mut self) -> Dag<Arc<Self>> {
        use core::{mem, ptr};
        let ret = match &mut self {
            Incomplete::Sum(ref mut left, ref mut right)
            | Incomplete::Product(ref mut left, ref mut right) => {
                // Because Rust is stupid, we cannot just move 'left' and 'right' out of 'self'.
                // We get the error "cannot move out of type that implements Drop". This message
                // dates to before Rust 1.0, before mem::forget was marked safe, and has no
                // justification that stands up to any scrutiny. There have been mulitple RFCs to
                // remove it but for some reason they have never gone anywhere. See for example
                //
                // https://internals.rust-lang.org/t/destructuring-droppable-structs/20993/41
                //
                // Anyway, instead we have to use unsafe code here to do this obviously-safe
                // operation in an overcomplicated and hard-to-review way.
                unsafe {
                    // SAFETY we are calling `ptr::read` on valid pointers (they come directly
                    // from references, which are always valid), and we will mem::forget their old
                    // locations before any early returns or panics.

                    let left = ptr::read(left);
                    let right = ptr::read(right);
                    Dag::Binary(left, right)
                }
            }
            Incomplete::Cycle => Dag::Nullary,
            Incomplete::Free(s) => {
                // SAFETY: see above. We are manually dropping `s` here, which we have to do
                // by reading it out of &mut self, because Rust is stupid.
                unsafe {
                    ptr::read(s);
                }
                Dag::Nullary
            }
            Incomplete::Final(fin) => {
                // SAFETY: see above
                unsafe {
                    ptr::read(fin);
                }
                Dag::Nullary
            }
        };
        // Because `Incomplete::drop` calls this method, we cannot allow `self` to be dropped
        // under any circumstances, or else we will infinitely recurse and stack-overflow. (This
        // is why we had to do ptr::read in every branch above).
        mem::forget(self);
        ret
    }
}

impl DagLike for &'_ Incomplete {
    type Node = Incomplete;
    fn data(&self) -> &Incomplete {
        self
    }
    fn as_dag_node(&self) -> Dag<Self> {
        match *self {
            Incomplete::Free(_) | Incomplete::Cycle | Incomplete::Final(_) => Dag::Nullary,
            Incomplete::Sum(ref left, ref right) | Incomplete::Product(ref left, ref right) => {
                Dag::Binary(left, right)
            }
        }
    }
}

impl fmt::Debug for Incomplete {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(self, f)
    }
}

impl fmt::Display for Incomplete {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut skip_next = false;
        for data in self.verbose_pre_order_iter::<NoSharing>(Some(MAX_DISPLAY_DEPTH)) {
            if data.index > MAX_DISPLAY_LENGTH {
                write!(f, "... [truncated type after {} nodes]", MAX_DISPLAY_LENGTH)?;
                return Ok(());
            }
            if data.depth == MAX_DISPLAY_DEPTH {
                if data.n_children_yielded == 0 {
                    f.write_str("...")?;
                }
                continue;
            }

            if skip_next {
                skip_next = false;
                continue;
            }

            match (data.node, data.n_children_yielded) {
                (Incomplete::Free(ref s), _) => f.write_str(s)?,
                (Incomplete::Cycle, _) => f.write_str("<self-reference>")?,
                // special-case 1 + A as A?
                (Incomplete::Sum(ref left, _), 0) if left.is_unit() => {
                    skip_next = true;
                }
                (Incomplete::Sum(ref left, _), 1) if left.is_unit() => {}
                (Incomplete::Sum(ref left, _), 2) if left.is_unit() => {
                    f.write_str("?")?;
                }
                // other sums and products
                (Incomplete::Sum(..), 0) | (Incomplete::Product(..), 0) => {
                    if data.index > 0 {
                        f.write_str("(")?;
                    }
                }
                (Incomplete::Sum(..), 2) | (Incomplete::Product(..), 2) => {
                    if data.index > 0 {
                        f.write_str(")")?;
                    }
                }
                (Incomplete::Sum(..), _) => f.write_str(" + ")?,
                (Incomplete::Product(..), _) => f.write_str(" × ")?,
                (Incomplete::Final(ref fnl), _) => fnl.fmt(f)?,
            }
        }
        Ok(())
    }
}

impl Drop for Incomplete {
    fn drop(&mut self) {
        // Note: this is basically identical to the drop impl for node::Node.
        fn push_children(stack: &mut Vec<Arc<Incomplete>>, inner: Incomplete) {
            use crate::dag::Dag;
            match inner.into_dag() {
                Dag::Nullary => {}
                Dag::Unary(child) => stack.push(child),
                Dag::Binary(left, right) => {
                    stack.push(left);
                    stack.push(right);
                }
            }
        }

        let mut stack = Vec::new();
        push_children(&mut stack, std::mem::replace(self, Incomplete::Cycle));
        while let Some(child) = stack.pop() {
            if let Some(mut child) = Arc::into_inner(child) {
                push_children(&mut stack, std::mem::replace(&mut child, Incomplete::Cycle));
            }
        }
    }
}

impl Incomplete {
    /// Whether this "incomplete bound" is the unit type.
    pub fn is_unit(&self) -> bool {
        if let Incomplete::Final(ref fnl) = self {
            fnl.is_unit()
        } else {
            false
        }
    }

    /// Does the occurs-check on a type bound.
    ///
    /// Returns None on success, and a Some(Incomplete) indicating the occurs-check
    /// failure if there is a cyclic reference.
    pub(super) fn occurs_check<'brand>(
        ctx: &Context<'brand>,
        bound_ref: BoundRef<'brand>,
    ) -> Option<Arc<Self>> {
        use std::collections::HashSet;

        use super::context::OccursCheckId;

        /// Helper type for the occurs-check.
        enum OccursCheckStack<'brand> {
            Iterate(BoundRef<'brand>),
            Complete(OccursCheckId<'brand>),
        }

        // First, do occurs-check to ensure that we have no infinitely sized types.
        let mut stack = vec![OccursCheckStack::Iterate(bound_ref)];
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
                // FIXME unwind the stack to somehow provide a more useful trace of the occurs-check failure
                return Some(Arc::new(Self::Cycle));
            }

            stack.push(OccursCheckStack::Complete(id));
            if let Some((_, child)) = (ctx, bound.shallow_clone()).right_child() {
                stack.push(OccursCheckStack::Iterate(child));
            }
            if let Some((_, child)) = (ctx, bound).left_child() {
                stack.push(OccursCheckStack::Iterate(child));
            }
        }

        None
    }

    pub(super) fn from_bound_ref<'brand>(
        ctx: &Context<'brand>,
        bound_ref: BoundRef<'brand>,
    ) -> Arc<Self> {
        if let Some(err) = Self::occurs_check(ctx, bound_ref.shallow_clone()) {
            return err;
        }

        // Now that we know our bound has finite size, we can safely use a
        // post-order iterator on it.
        let mut finalized = vec![];
        for data in (ctx, bound_ref).post_order_iter::<BoundRefSharing<'_>>() {
            let bound_get = data.node.0.get(&data.node.1);
            let final_data = match bound_get {
                Bound::Free(s) => Incomplete::Free(s),
                Bound::Complete(ref arc) => Incomplete::Final(Arc::clone(arc)),
                Bound::Sum(..) => Incomplete::Sum(
                    Arc::clone(&finalized[data.left_index.unwrap()]),
                    Arc::clone(&finalized[data.right_index.unwrap()]),
                ),
                Bound::Product(..) => Incomplete::Product(
                    Arc::clone(&finalized[data.left_index.unwrap()]),
                    Arc::clone(&finalized[data.right_index.unwrap()]),
                ),
            };

            finalized.push(Arc::new(final_data));
        }
        finalized.pop().unwrap()
    }
}
