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

use std::sync::{Arc, Mutex};
use std::{cmp, fmt, mem, ops};

use ghost_cell::GhostToken;

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
    pub _token: GhostToken<'brand>,
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

pub struct UbElement<T> {
    inner: Arc<Mutex<UbInner<T>>>,
}

impl<T> Clone for UbElement<T> {
    fn clone(&self) -> Self {
        UbElement {
            inner: Arc::clone(&self.inner),
        }
    }
}

impl<T: PointerLike + fmt::Debug> fmt::Debug for UbElement<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Debug::fmt(&self.root(), f)
    }
}

struct UbInner<T> {
    data: UbData<T>,
    rank: usize,
}

enum UbData<T> {
    Root(T),
    EqualTo(UbElement<T>),
}

impl<T> UbData<T> {
    fn unwrap_root(&self) -> &T {
        match self {
            UbData::Root(ref x) => x,
            UbData::EqualTo(..) => unreachable!(),
        }
    }
}

impl<T> UbElement<T> {
    /// Turns an existing piece of data into a singleton union-bound set.
    pub fn new(data: T) -> Self {
        UbElement {
            inner: Arc::new(Mutex::new(UbInner {
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

impl<T: PointerLike> UbElement<T> {
    /// Find the representative of this object in its disjoint set.
    pub fn root(&self) -> T {
        let root = self.root_element();
        let inner_lock = root.inner.lock().unwrap();
        inner_lock.data.unwrap_root().shallow_clone()
    }

    /// Find the representative of this object in its disjoint set.
    fn root_element(&self) -> Self {
        let mut x: Self = self.shallow_clone();
        loop {
            let x_inner_lock = x.inner.lock().unwrap();
            // Find the parent of the current target, if there is one. If not, we
            // are done, so return.
            let parent: Self = match x_inner_lock.data {
                UbData::Root(..) => return x.shallow_clone(),
                UbData::EqualTo(ref parent) => {
                    let parent = parent.shallow_clone();
                    drop(x_inner_lock);
                    parent
                }
            };

            let parent_inner_lock = parent.inner.lock().unwrap();
            // If the parent has a parent, remove the intermediate link. (This is
            // the "halving" variant of union-bound.)
            match parent_inner_lock.data {
                // If there is no grandparent, return the parent.
                UbData::Root(..) => return parent.shallow_clone(),
                UbData::EqualTo(ref grandparent) => {
                    let grandparent = grandparent.shallow_clone();
                    // The previous `x_inner_lock` was dropped above, so this will not deadlock.
                    let mut x_inner_lock = x.inner.lock().unwrap();
                    x_inner_lock.data = UbData::EqualTo(grandparent.shallow_clone());
                    drop(x_inner_lock);
                    x = grandparent;
                }
            }
        }
    }

    /// Combine two sets.
    ///
    /// When the two sets are combined, the representative of one will be replaced
    /// by the representative of the other. The user must force these two elements
    /// to actually be equal. This is accomplished with the `bind_fn` function,
    /// which takes two arguments: the **new representative that will be kept**
    /// followed by the **old representative that will be dropped**.
    pub fn unify<E, Bind: FnOnce(T, &T) -> Result<(), E>>(
        &self,
        other: &Self,
        bind_fn: Bind,
    ) -> Result<(), E> {
        let x_root = self.root_element();
        let y_root = other.root_element();

        // In the case that we are unifying literally the same variable with
        // itself, there is nothing to do. We need to do this early check now
        // so that we don't deadlock in the next lines when we lock the
        // mutexes for both x and y.
        if Arc::ptr_eq(&x_root.inner, &y_root.inner) {
            return Ok(());
        }

        // Lock both x and y.
        let mut x_lock = x_root.inner.lock().unwrap();
        let mut y_lock = y_root.inner.lock().unwrap();

        // If our two variables are not literally the same, but through
        // unification have become the same, we detect _this_ and exit early.
        if x_lock.data.unwrap_root().ptr_eq(y_lock.data.unwrap_root()) {
            return Ok(());
        }

        // We need to swap x_root and y_root below, and the only way to make the
        // borrowck okay with that is to replace both types by references
        // to themselves.
        let mut x_root = &x_root;
        let mut y_root = &y_root;

        // Swap so that x always has rank >= y. Then x will become the parent of
        // y, meaning that y's root will be replaced by x's root.
        let rank_ord = x_lock.rank.cmp(&y_lock.rank);
        match rank_ord {
            cmp::Ordering::Less => {
                mem::swap(&mut x_root, &mut y_root);
                mem::swap(&mut x_lock, &mut y_lock)
            }
            cmp::Ordering::Equal => {
                assert_ne!(
                    x_lock.rank,
                    usize::MAX,
                    "Attempted to unify two frozen variables",
                );
                x_lock.rank += 1;
            }
            _ => {}
        }

        let x_data = match x_lock.data {
            UbData::Root(ref data) => data.shallow_clone(),
            UbData::EqualTo(..) => unreachable!(),
        };
        drop(x_lock);
        let old_y_data = mem::replace(&mut y_lock.data, UbData::EqualTo(x_root.shallow_clone()));
        drop(y_lock);

        match old_y_data {
            UbData::Root(ref y_data) => {
                match bind_fn(x_data, y_data) {
                    Ok(()) => Ok(()),
                    Err(e) => {
                        // In case of error, put the old data back.
                        y_root.inner.lock().unwrap().data = old_y_data;
                        Err(e)
                    }
                }
            }
            UbData::EqualTo(..) => unreachable!(),
        }
    }
}
