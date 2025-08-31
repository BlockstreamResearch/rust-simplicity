// SPDX-License-Identifier: CC0-1.0

//! The Union-Bound Algorithm
//!
//! Type inference proceeds using the union-bound algorithm, which maintains a
//! set of equivalence classes of types. Initially every type is free and lives
//! in its own equivalence class. As inference proceeds, types are constrained
//! to be equal to each other, and their equivalence classes are merged.
//!
//! The union-bound algorithm provides a very efficient way of tracking these
//! equivalence classes. Each type is stored in an [`UbElement`], an opaque data
//! structure which only provides access to the underlying data via the
//! [`UbElement::root`] method, which returns the representative of the element's
//! class.
//!
//! Merging classes is done using the [`UbElement::unify`] method. Since when two
//! classes are merged, there is no longer any way to distinguish elements of
//! either, it is essential that they actually be equal. This is accomplished by
//! the `bind_fn` parameter to `unify`, which takes the representatives of the
//! original classes, checks that they are equal (or uses interior mutability to
//! manipulate them to be equal), and returns either success or failure.
//!
//! If `bind_fn` returns an error, this error is returned by `unify`, and the
//! classes are not merged.
//!
//! This is a classic CS algorithm. We use the "path-halving" variant of it, which
//! was inspired by the same algorithm in the C implementation of the code, which
//! cites ``Worst-Case Analysis of Set Union Algorithms'' by Robert E. Tarjan and
//! Jan van Leeuwen (1984) as giving amortized time complexity of O(InvAck(n)).
//!

use std::sync::Arc;
use std::{cmp, mem, ops};

use ghost_cell::{GhostCell, GhostToken};

/// Trait describing objects that can be stored and manipulated by the union-bound
/// algorithm.
///
/// Because the algorithm depends on identity equality (i.e. two objects being
/// exactly the same in memory) such objects need to have such a notion of
/// equality. In general this differs from the `Eq` trait which implements
/// "semantic equality".
pub trait PointerLike {
    /// Whether two objects are the same.
    fn ptr_eq(&self, other: &Self) -> bool;

    /// A "shallow copy" of the object.
    fn shallow_clone(&self) -> Self;
}

impl<T> PointerLike for Arc<T> {
    fn ptr_eq(&self, other: &Self) -> bool {
        Arc::ptr_eq(self, other)
    }
    fn shallow_clone(&self) -> Self {
        Arc::clone(self)
    }
}

/// Container which associates a ghost token with some arbitrary other data.
///
/// This allows the union-bound algorithm to work with a token (which is used
/// by the algorithm) while passing data (which is -not- used by the algorithm)
/// everywhere that needs it, without offending the borrow checker or forcing
/// callers to manually destructure their types to separate out tokens.
///
/// If Rust had view types, this structure wouldn't be necessary.
pub struct WithGhostToken<'brand, T> {
    pub token: GhostToken<'brand>,
    pub inner: T,
}

impl<'brand, T> ops::Deref for WithGhostToken<'brand, T> {
    type Target = T;
    fn deref(&self) -> &T {
        &self.inner
    }
}

impl<'brand, T> ops::DerefMut for WithGhostToken<'brand, T> {
    fn deref_mut(&mut self) -> &mut T {
        &mut self.inner
    }
}

pub struct UbElement<'brand, T> {
    inner: Arc<GhostCell<'brand, UbInner<'brand, T>>>,
}

impl<T> Clone for UbElement<'_, T> {
    fn clone(&self) -> Self {
        UbElement {
            inner: Arc::clone(&self.inner),
        }
    }
}

struct UbInner<'brand, T> {
    data: UbData<'brand, T>,
    rank: usize,
}

enum UbData<'brand, T> {
    Root(T),
    EqualTo(UbElement<'brand, T>),
}

impl<T> UbData<'_, T> {
    fn unwrap_root(&self) -> &T {
        match self {
            UbData::Root(ref x) => x,
            UbData::EqualTo(..) => unreachable!(),
        }
    }
}

impl<T> UbElement<'_, T> {
    /// Turns an existing piece of data into a singleton union-bound set.
    pub fn new(data: T) -> Self {
        UbElement {
            inner: Arc::new(GhostCell::new(UbInner {
                data: UbData::Root(data),
                rank: 0,
            })),
        }
    }

    /// Clones the `UbElement`.
    ///
    /// This is the same as just calling `.clone()` but has a different name to
    /// emphasize that what's being cloned is internally just an Arc.
    pub fn shallow_clone(&self) -> Self {
        Self {
            inner: Arc::clone(&self.inner),
        }
    }
}

impl<'brand, T: PointerLike> UbElement<'brand, T> {
    /// Find the representative of this object in its disjoint set.
    pub fn root(&self, token: &mut GhostToken<'brand>) -> T {
        let root = self.root_element(token);
        let inner_lock = root.inner.borrow(token);
        inner_lock.data.unwrap_root().shallow_clone()
    }

    /// Find the representative of this object in its disjoint set.
    fn root_element(&self, token: &mut GhostToken<'brand>) -> Self {
        let mut x: Self = self.shallow_clone();
        loop {
            // Find the parent of the current target, if there is one. If not, we
            // are done, so return.
            let parent: &Self = match x.inner.borrow(token).data {
                UbData::Root(..) => return x.shallow_clone(),
                UbData::EqualTo(ref parent) => parent,
            };

            // Find the grandparent of the current target, if there is one. If not, we
            // are done, so return.
            let grandparent: &Self = match parent.inner.borrow(token).data {
                UbData::Root(..) => return parent.shallow_clone(),
                UbData::EqualTo(ref grandparent) => grandparent,
            };

            // If the parent has a parent, remove the intermediate link. (This is
            // the "halving" variant of union-bound.)
            let grandparent = grandparent.shallow_clone();
            x.inner.borrow_mut(token).data = UbData::EqualTo(grandparent.shallow_clone());
            x = grandparent;
        }
    }

    /// Combine two sets.
    ///
    /// When the two sets are combined, the representative of one will be replaced
    /// by the representative of the other. The user must force these two elements
    /// to actually be equal. This is accomplished with the `bind_fn` function,
    /// which takes two arguments: the **new representative that will be kept**
    /// followed by the **old representative that will be dropped**.
    pub fn unify<D, E, Bind: FnOnce(&mut WithGhostToken<'brand, D>, T, &T) -> Result<(), E>>(
        &self,
        with_token: &mut WithGhostToken<'brand, D>,
        other: &Self,
        bind_fn: Bind,
    ) -> Result<(), E> {
        let token = &mut with_token.token;
        let mut x_root = self.root_element(token);
        let mut y_root = other.root_element(token);

        let x_elem = x_root.inner.borrow(token);
        let y_elem = y_root.inner.borrow(token);

        // In the case that we are unifying a variable with itself, there is
        // nothing to do. So exit early.
        if x_elem.data.unwrap_root().ptr_eq(y_elem.data.unwrap_root()) {
            return Ok(());
        }

        // Swap so that x always has rank >= y. Then x will become the parent of
        // y, meaning that y's root will be replaced by x's root.
        let rank_ord = x_elem.rank.cmp(&y_elem.rank);
        match rank_ord {
            cmp::Ordering::Less => {
                // Swap x_root and y_root. This invalidates x_elem and y_elem, but
                // the Rust borrowck ensures that we can't reuse them :-).
                mem::swap(&mut x_root, &mut y_root);
            }
            cmp::Ordering::Equal => {
                let x_elem = x_root.inner.borrow_mut(token);
                assert_ne!(
                    x_elem.rank,
                    usize::MAX,
                    "Attempted to unify two frozen variables",
                );
                x_elem.rank += 1;
            }
            _ => {}
        }

        let x_elem = x_root.inner.borrow(token);
        let x_data = match x_elem.data {
            UbData::Root(ref data) => data.shallow_clone(),
            UbData::EqualTo(..) => unreachable!(),
        };

        let y_elem = y_root.inner.borrow_mut(token);
        let old_y_data = mem::replace(&mut y_elem.data, UbData::EqualTo(x_root.shallow_clone()));

        match old_y_data {
            UbData::Root(ref y_data) => {
                match bind_fn(with_token, x_data, y_data) {
                    Ok(()) => Ok(()),
                    Err(e) => {
                        // In case of error, put the old data back.
                        y_root.inner.borrow_mut(&mut with_token.token).data = old_y_data;
                        Err(e)
                    }
                }
            }
            UbData::EqualTo(..) => unreachable!(),
        }
    }
}
