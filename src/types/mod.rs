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

use crate::Tmr;

use std::sync::{Arc, Mutex, MutexGuard};
use std::{cmp, fmt, mem, ops};

pub mod arrow;
mod final_data;
mod precomputed;
mod variable;

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
    OccursCheck { incomplete_type: Type },
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
                    "failed to apply bound {} to existing bound {}: {}",
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
                    "attempted to unify unequal types {} and {}: {}",
                    type1, type2, hint,
                )
            }
            Error::OccursCheck {
                ref incomplete_type,
            } => {
                write!(
                    f,
                    "occurs check failed (incomplete type: {})",
                    incomplete_type,
                )
            }
        }
    }
}

impl std::error::Error for Error {}

/// Source or target type of a Simplicity expression
///
/// This is just a mutex holding the actual data, which defined by the
/// `TypeInner` type.
#[derive(Clone, Debug)]
pub struct Type {
    /// A mutex around the actual type data
    inner: Arc<Mutex<TypeInner>>,
}

/// Source or target type of a Simplicity expression
#[derive(Clone, Debug)]
struct TypeInner {
    /// The type's status according to the union-bound algorithm.
    constraint: Constraint,
    /// Used during finalization; whether this type has been seen before while
    /// traversing the type tree. If it has, it means we have a type containing
    /// itself, which is illegal. This is referred to as the "occurs check".
    occurs_check: bool,
    /// The rank of the type which provides an ordering within disjoint sets
    /// during the union-bound type inference algorithm.
    rank: usize,
}

impl TypeInner {
    fn final_data(&self) -> Option<&Arc<Final>> {
        if let Constraint::Bound {
            bound: Bound::Complete(ref final_data),
        } = self.constraint
        {
            Some(final_data)
        } else {
            None
        }
    }
}

/// Internal structure used to add mutability controls to the mutex guarding the
/// internal data of a `Type`.
///
/// An invariant this module maintains is that once a `Type` is final, it will
/// never change again. (We need this to make sure that our precomputed 2^n
/// types don't get modified in the course of type inference.) To do so, we
/// only lock the inner mutex using the `inner_lock()` method on `Type`, which
/// provides read-only access. IT IS A BUG TO DIRECTLY LOCK THE MUTEX.
///
/// If we need to mutate the data, we then call `into_mutable` or `as_mutable`
/// on the returned `ReadOnlyMutexGuard`. Every such call should have a comment
/// justifying why the type in question is not final at the point of the call.
struct ReadOnlyMutexGuard<'g> {
    inner: MutexGuard<'g, TypeInner>,
}

impl<'g> ReadOnlyMutexGuard<'g> {
    /// Enable mutable access to the `TypeInner`.
    ///
    /// # Panics
    /// Will panic if it is called on a finalized typpe.
    fn into_mutable(self) -> MutexGuard<'g, TypeInner> {
        debug_assert!(
            self.inner.final_data().is_none(),
            "tried to get mutable access to a finalized type",
        );
        self.inner
    }

    /// Same as `into_mutable` but does not consume the original guard
    ///
    /// # Panics
    /// Will panic if it is called on a finalized typpe.
    fn as_mutable(&mut self) -> &mut MutexGuard<'g, TypeInner> {
        debug_assert!(
            self.inner.final_data().is_none(),
            "tried to get mutable access to a finalized type",
        );
        &mut self.inner
    }
}

impl<'g> ops::Deref for ReadOnlyMutexGuard<'g> {
    type Target = TypeInner;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

/// Whether a `Type` is free, bound to some `Bound`, or equal to another `Type`
#[derive(Clone, Debug)]
enum Constraint {
    /// Type bound to a particular `Bound`. If the type is complete, this bound
    /// fully specifies the type.
    Bound {
        /// The bound
        bound: Bound,
    },
    /// Type is equal to another type, which is referred to as the "parent" of
    /// the type.
    EqualTo { parent: Type },
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

impl Constraint {
    fn free(name: String) -> Self {
        Constraint::Bound {
            bound: Bound::Free(name),
        }
    }

    fn unit() -> Self {
        Constraint::Bound {
            bound: Bound::Complete(Arc::new(Final::unit())),
        }
    }

    fn sum(a: Type, b: Type) -> Self {
        Constraint::Bound {
            bound: if let (Some(adata), Some(bdata)) = (a.final_data(), b.final_data()) {
                Bound::Complete(Arc::new(Final::sum(adata, bdata)))
            } else {
                Bound::Sum(a, b)
            },
        }
    }

    fn product(a: Type, b: Type) -> Self {
        Constraint::Bound {
            bound: if let (Some(adata), Some(bdata)) = (a.final_data(), b.final_data()) {
                Bound::Complete(Arc::new(Final::product(adata, bdata)))
            } else {
                Bound::Product(a, b)
            },
        }
    }
}

impl Type {
    /// Return an unbound type with the given name
    pub fn free(name: String) -> Self {
        Type::from(Constraint::free(name))
    }

    /// Return a unit type.
    pub fn unit() -> Self {
        Type::from(Constraint::unit())
    }

    /// Return a precomputed copy of 2^(2^n), for given n.
    pub fn two_two_n(n: usize) -> Self {
        precomputed::nth_power_of_2(n)
    }

    /// Return the sum of the given two types.
    pub fn sum(a: Self, b: Self) -> Self {
        Type::from(Constraint::sum(a, b))
    }

    /// Return the product of the given two types.
    pub fn product(a: Self, b: Self) -> Self {
        Type::from(Constraint::product(a, b))
    }

    /// Read-only pointer to data (used internally to make mutability easier to analyse)
    fn inner_lock(&self) -> ReadOnlyMutexGuard {
        ReadOnlyMutexGuard {
            inner: self.inner.lock().unwrap(),
        }
    }

    /// Clones the `Type`.
    ///
    /// This is the same as just calling `.clone()` but has a different name to
    /// emphasize that what's being cloned is merely a ref-counted pointer.
    pub fn shallow_clone(&self) -> Type {
        self.clone()
    }

    /// Find the representative of this type in its disjoint set.
    ///
    /// Used by the union-bound algorithm.
    fn find_root(&self) -> Self {
        let mut x = self.shallow_clone();
        loop {
            let x_lock = x.inner_lock();
            // Find the parent of the current target, if there is one. If not, we
            // are done, so return.
            let parent = match x_lock.constraint {
                Constraint::EqualTo { ref parent } => parent.shallow_clone(),
                _ => {
                    drop(x_lock);
                    return x;
                }
            };
            // If the parent has a parent, remove the intermediate link. (This is
            // the "halving" variant of union-bound.)
            let parent_lock = parent.inner_lock();
            if let Constraint::EqualTo {
                parent: ref grandparent,
            } = parent_lock.constraint
            {
                let mut x_lock = x_lock.into_mutable(); // ok since x is `EqualTo` but only `Bound` constraints are finalized
                x_lock.constraint = Constraint::EqualTo {
                    parent: grandparent.shallow_clone(),
                };
                drop(x_lock); // needed for borrowck to give us access to `x`
                x = grandparent.shallow_clone();
            } else {
                // If there is no grandparent, return the parent.
                drop(parent_lock); // needed for borrowck to give us access to `parent`
                return parent;
            }
        }
    }

    /// Binds the type to a given bound. If this fails, attach the provided
    /// hint to the error.
    ///
    /// Fails if the type has an existing incompatible bound.
    pub fn bind(&self, bound: Bound, hint: &'static str) -> Result<(), Error> {
        let root = self.find_root();
        let lock = root.inner_lock();
        match lock.constraint {
            // Types with bounds are more interesting: we have to recursively unify
            // the existing bound with the new one.
            Constraint::Bound {
                bound: ref existing_bound,
            } => {
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
                        let mut lock = lock.into_mutable(); // free means non-finalized
                        lock.constraint = Constraint::Bound { bound };
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
                    (Bound::Complete(ref complete), incomplete)
                    | (&incomplete, Bound::Complete(ref complete)) => {
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
                        // Before recursing we need to drop the lock, and before
                        // that we need to clone some types from inside it.
                        let x1 = x1.shallow_clone();
                        let x2 = x2.shallow_clone();
                        drop(lock); // drop lock before recursing
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
                            let mut lock = root.inner_lock().into_mutable(); // ok, we are in a `if is_not_finalized` block
                            lock.constraint = Constraint::Bound {
                                bound: Bound::Complete(Arc::new(if let Bound::Sum(..) = bound {
                                    Final::sum(data1, data2)
                                } else {
                                    Final::product(data1, data2)
                                })),
                            };
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
            // Our call to `find_root` above makes this case impossible
            Constraint::EqualTo { .. } => unreachable!("lock is a root node"),
        }
    }

    /// Unify the type with another one.
    ///
    /// Fails if the bounds on the two types are incompatible
    pub fn unify(&self, other: &Self, hint: &'static str) -> Result<(), Error> {
        let x_root = self.find_root();
        let y_root = other.find_root();
        // Already in same set
        if Arc::ptr_eq(&x_root.inner, &y_root.inner) {
            return Ok(());
        }

        let mut x_lock = x_root.inner_lock();
        let mut y_lock = y_root.inner_lock();
        // We need to swap x_root and y_root below, and the only way to make the
        // borrowck okay with that is to replace both types by references
        // to themselves.
        let mut x_root = &x_root;
        let mut y_root = &y_root;
        // Deterine which of x or y should be the parent. This means the finalized
        // one, if only one is finalized, or the higher-ranked one, if neither is.
        match (&x_lock.final_data(), &y_lock.final_data()) {
            // If both are finalized, unification just means checking that
            // they're equal.
            (Some(xdata), Some(ydata)) => {
                if xdata.tmr() == ydata.tmr() {
                    return Ok(());
                } else {
                    return Err(Error::CompleteTypeMismatch {
                        type1: Arc::clone(xdata),
                        type2: Arc::clone(ydata),
                        hint,
                    });
                }
            }
            // If one is finalized but not the other, that one is the parent
            (Some(_), None) => {} // x is the parent
            (None, Some(_)) => {
                mem::swap(&mut x_lock, &mut y_lock);
                mem::swap(&mut x_root, &mut y_root);
            }
            // If neither is finalized, then we use the normal union-bound rank
            // comparison: we ensure that x's rank is strictly greater than y's
            // rank. If they are equal, simply increment x's rank.
            (None, None) => {
                let rank_ord = x_lock.rank.cmp(&y_lock.rank);
                let mut x_lock = x_lock.as_mutable(); // ok since we checked that x is not final
                let mut y_lock = y_lock.as_mutable(); // ok since we checked that y is not final
                match rank_ord {
                    cmp::Ordering::Less => mem::swap(&mut x_lock, &mut y_lock),
                    cmp::Ordering::Equal => x_lock.rank += 1,
                    _ => {}
                }
            }
        }

        // At this point we know that y is not final (if both it and x were final,
        // we did an early return; if it was final but x was not, we swapped them
        // so that the opposite is true). So we can mutate y.
        let mut y_lock = y_lock.into_mutable(); // ok by above swap logic

        // Make x the parent of y
        let old_y_constraint = mem::replace(
            &mut y_lock.constraint,
            Constraint::EqualTo {
                parent: x_root.shallow_clone(),
            },
        );
        // Drop mutexes before recursing.
        drop(x_lock);
        drop(y_lock);
        match old_y_constraint {
            // If y was already bound to a type, then x must be bound, too
            Constraint::Bound { bound: y_bound } => x_root.bind(y_bound, hint),
            Constraint::EqualTo { .. } => unreachable!("y_lock is a root node"),
        }
    }

    /// Accessor for this type's bound
    pub fn bound(&self) -> Bound {
        let root = self.find_root();
        let root_lock = root.inner_lock();
        if let Constraint::Bound { ref bound, .. } = root_lock.constraint {
            bound.shallow_clone()
        } else {
            panic!("Bound is the only variant for Constraint now")
        }
    }

    /// Accessor for the TMR of this type, if it is final
    pub fn tmr(&self) -> Option<Tmr> {
        let root = self.find_root(); // Final data is only actually set on root nodes
        let root_lock = root.inner_lock();
        root_lock.final_data().map(|data| data.tmr())
    }

    /// Accessor for the data of this type, if it is complete
    pub fn final_data(&self) -> Option<Arc<Final>> {
        let root = self.find_root(); // Final data is only actually set on root nodes
        let root_lock = root.inner_lock();
        root_lock.final_data().map(Arc::clone)
    }

    /// Whether this type is known to be final
    ///
    /// During type inference this may be false even though the type is, in fact,
    /// complete, since its children may have been unified to a complete type. To
    /// ensure a type is complete, call [`Type::finalize`].
    pub fn is_final(&self) -> bool {
        self.final_data().is_some()
    }

    /// If a type is finalized, return its final type. Otherwise panic.
    pub fn expect_finalized(&self) -> Arc<Final> {
        self.final_data().unwrap()
    }

    /// Attempts to finalize the type. Returns its TMR on success.
    pub fn finalize(&self) -> Result<Arc<Final>, Error> {
        let root = self.find_root();
        if let Some(data) = root.final_data() {
            return Ok(data);
        }

        // Because we checked above that the type was not finalized, we know
        // at this point that it is not, so we are free to use `.into_mutable()`
        // to edit it.
        let mut root_lock = root.inner_lock().into_mutable();
        if root_lock.occurs_check {
            return Err(Error::OccursCheck {
                incomplete_type: root.shallow_clone(),
            });
        }
        root_lock.occurs_check = true;
        let data = match root_lock.constraint {
            Constraint::Bound { ref bound, .. } => {
                match bound {
                    Bound::Free(_) => Arc::new(Final::unit()),
                    Bound::Complete(ref arc) => Arc::clone(arc),
                    Bound::Sum(ref ty1, ref ty2) => {
                        // Drop the lock before recursing
                        let ty1 = ty1.clone();
                        let ty2 = ty2.clone();
                        drop(root_lock);
                        let data1 = ty1.finalize()?;
                        let data2 = ty2.finalize()?;
                        // Then re-lock to update own final data
                        root_lock = root.inner_lock().into_mutable();
                        Arc::new(Final::sum(data1, data2))
                    }
                    Bound::Product(ref ty1, ref ty2) => {
                        // Drop the lock before recursing
                        let ty1 = ty1.clone();
                        let ty2 = ty2.clone();
                        drop(root_lock);
                        let data1 = ty1.finalize()?;
                        let data2 = ty2.finalize()?;
                        // Then re-lock to update own final data
                        root_lock = root.inner_lock().into_mutable();
                        Arc::new(Final::product(data1, data2))
                    }
                }
            }
            Constraint::EqualTo { .. } => unreachable!("lock is a root node"),
        };

        // After the above code, we definitely have a `Constraint::Bound` with
        // a populated `final_data` field.
        if let Constraint::Bound { ref mut bound, .. } = root_lock.constraint {
            *bound = Bound::Complete(Arc::clone(&data));
            Ok(data)
        } else {
            unreachable!("we just set the constraint to Bound and now it's not Bound")
        }
    }

    /// Return a vector containing the types 2^(2^i) for i from 0 to n-1.
    pub fn powers_of_two(n: usize) -> Vec<Self> {
        let mut ret = Vec::with_capacity(n);

        let unit = Type::unit();
        let mut two = Type::sum(unit.shallow_clone(), unit);
        for _ in 0..n {
            ret.push(two.shallow_clone());
            two = Type::product(two.shallow_clone(), two);
        }
        ret
    }
}

impl fmt::Display for Type {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let root = self.find_root();
        let lock = root.inner_lock();
        match lock.constraint {
            Constraint::Bound { ref bound } => fmt::Display::fmt(bound, f),
            Constraint::EqualTo { ref parent } => fmt::Display::fmt(parent, f),
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
}

impl fmt::Display for Bound {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Bound::Free(ref name) => f.write_str(name),
            Bound::Complete(ref data) => fmt::Display::fmt(data, f),
            Bound::Sum(ref a, ref b) => write!(f, "({} + {})", a, b),
            Bound::Product(ref a, ref b) => write!(f, "({} × {})", a, b),
        }
    }
}

impl From<Constraint> for Type {
    /// Promotes a `Constraint` to a type defined by that constraint
    fn from(constraint: Constraint) -> Type {
        Type {
            inner: Arc::new(Mutex::new(TypeInner {
                rank: 0,
                occurs_check: false,
                constraint,
            })),
        }
    }
}
