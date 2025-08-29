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

use super::{Bound, BoundRef, Context};

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
        for data in self.verbose_pre_order_iter::<NoSharing>(None) {
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
    pub(super) fn occurs_check(ctx: &Context, bound_ref: BoundRef) -> Option<Arc<Self>> {
        use std::collections::HashSet;

        use super::context::OccursCheckId;
        use super::BoundRef;

        /// Helper type for the occurs-check.
        enum OccursCheckStack {
            Iterate(BoundRef),
            Complete(OccursCheckId),
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

    pub(super) fn from_bound_ref(ctx: &Context, bound_ref: BoundRef) -> Arc<Self> {
        if let Some(err) = Self::occurs_check(ctx, bound_ref.shallow_clone()) {
            return err;
        }

        // Now that we know our bound has finite size, we can safely use a
        // post-order iterator on it.
        let mut finalized = vec![];
        for data in (ctx, bound_ref).post_order_iter::<NoSharing>() {
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
