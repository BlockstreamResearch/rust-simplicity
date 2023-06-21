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
//! If a type has no free children we refer to it as "complete". The inference
//! code, when it notices that a type has become complete, populates its cached
//! `Final` structure, which can then be accessed using the `final_data` method.
//! If this structure is populated we refer to the type as "finalized".
//!
//! Generally speaking, "complete" and "finalized" can be considered synonyms,
//! though this is not strictly true because types may become complete indirectly,
//! by their children becoming complete, without the inference algorithm updating
//! the `Final` structure.
//!
//! It may be the case that some types are still free even after all constraints
//! are considered. The `finalize` function completes all types by setting any
//! free types to the unit type.
//!
//! In addition to requiring that type inference succeeds (i.e. no constraints
//! are in conflict with each other), there is one additional check, the "occurs
//! check", which ensures that there are no infinitely-sized types. Such types
//! occur when a type has itself as a child, and are illegal in Simplicity. This
//! check is explicitly done by `finalize`, to avoid infinitely looping, but any
//! final type is guaranteed to pass the occurs-check, whether it was finalized
//! explicitly or during unification.
//!
//! There are four main types in this module:
//!   * `Type` is the main type representing a Simplicity type, whether it is
//!     complete or not. Internally it contains a mutex holding a `TypeInner`
//!     which in turn contains a `Constraint`, but these are not public types
//!     and not exposed to users.
//!   * `Final` is a mutex-free structure that can be obtained from a complete
//!     type. It includes the TMR and the complete bound describing the type.
//!   * `Constraint` defines whether a given type is free, bound to a particular
//!     `Bound`, or equal to another type.
//!   * `Bound` defines the structure of a type: whether it is a unit, or the
//!     sum or product of two other types.
//!

use crate::merkle::tmr::Tmr;

use std::borrow::Cow;
use std::sync::{Arc, Mutex, MutexGuard};
use std::{cmp, fmt, mem, ops};

pub mod arrow;
mod precomputed;
mod variable;

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
    OccursCheck { incomplete_bound: Bound },
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
                ref incomplete_bound,
            } => {
                write!(
                    f,
                    "occurs check failed (incomplete bound: {})",
                    incomplete_bound,
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
    /// The rank of the type which provides an ordering within disjoint sets
    /// during the union-bound type inference algorithm.
    rank: usize,
}

impl TypeInner {
    fn final_data(&self) -> Option<&Arc<Final>> {
        if let Constraint::Bound { ref final_data, .. } = self.constraint {
            final_data.as_ref()
        } else {
            None
        }
    }
}

/// Data related to a finalized type, which can be extracted from a `Type`
/// if (and only if) it is finalized.
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Debug, Hash)]
pub struct Final {
    /// Underlying type
    bound: CompleteBound,
    /// Width of the type, in bits, in the bit machine
    bit_width: usize,
    /// TMR of the type
    tmr: Tmr,
    /// Cached string representation of the type
    display: Cow<'static, str>,
}

impl Final {
    /// (Non-public) constructor for the final data of the unit type
    const fn unit() -> Self {
        Final {
            bound: CompleteBound::Unit,
            bit_width: 0,
            tmr: Tmr::unit(),
            display: Cow::Borrowed("1"),
        }
    }

    /// Return a precomputed copy of 2^(2^n), for given n.
    pub fn two_two_n(n: usize) -> Arc<Self> {
        precomputed::nth_power_of_2(n).final_data().unwrap()
    }

    /// (Non-public) constructor for the final data of a sum type
    fn sum(left: Arc<Self>, right: Arc<Self>) -> Self {
        Final {
            tmr: Tmr::sum(left.tmr, right.tmr),
            bit_width: 1 + cmp::max(left.bit_width, right.bit_width),
            display: if left.bound == CompleteBound::Unit && right.bound == CompleteBound::Unit {
                "2".into()
            } else {
                format!("({} + {})", left.display, right.display).into()
            },
            bound: CompleteBound::Sum(left, right),
        }
    }

    /// (Non-public) constructor for the final data of a product type
    fn product(left: Arc<Self>, right: Arc<Self>) -> Self {
        Final {
            tmr: Tmr::product(left.tmr, right.tmr),
            bit_width: left.bit_width + right.bit_width,
            display: if left.display == right.display {
                match left.display.as_ref() {
                    "2" => "2^2".into(),
                    "2^2" => "2^4".into(),
                    "2^4" => "2^8".into(),
                    "2^8" => "2^16".into(),
                    "2^16" => "2^32".into(),
                    "2^32" => "2^64".into(),
                    "2^64" => "2^128".into(),
                    "2^128" => "2^256".into(),
                    "2^256" => "2^512".into(),
                    _ => format!("({} × {})", left.display, right.display).into(),
                }
            } else {
                format!("({} × {})", left.display, right.display).into()
            },
            bound: CompleteBound::Product(left, right),
        }
    }

    /// Accessor for the TMR
    pub fn tmr(&self) -> Tmr {
        self.tmr
    }

    /// Accessor for the Bit Machine bit-width of the type
    pub fn bit_width(&self) -> usize {
        self.bit_width
    }

    /// Accessor for the type bound
    pub fn bound(&self) -> &CompleteBound {
        &self.bound
    }

    /// Accessor for both children of the type, if they exist.
    pub fn split(&self) -> Option<(Arc<Self>, Arc<Self>)> {
        match &self.bound {
            CompleteBound::Unit => None,
            CompleteBound::Sum(left, right) | CompleteBound::Product(left, right) => {
                Some((Arc::clone(left), Arc::clone(right)))
            }
        }
    }
}

impl fmt::Display for Final {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(&self.display, f)
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
    /// Unconstrained type
    Free(String),
    /// Type bound to a particular `Bound`. If the type is complete, this bound
    /// fully specifies the type.
    Bound {
        /// The bound
        bound: Bound,
        /// If this bound defines a complete type, its cached type data
        final_data: Option<Arc<Final>>,
        /// Used during finalization; whether this type has been seen before while
        /// traversing the type tree. If it has, it means we have a type containing
        /// itself, which is illegal. This is referred to as the "occurs check".
        occurs_check: bool,
    },
    /// Type is equal to another type, which is referred to as the "parent" of
    /// the type.
    EqualTo { parent: Type },
}

/// The structure that a `Type` is bound to
#[derive(Clone, Debug)]
pub enum Bound {
    /// The unit type which has one value
    Unit,
    /// A sum of two other types
    Sum(Type, Type),
    /// A product of two other types
    Product(Type, Type),
}

/// A finalized type bound, whose tree is accessible without any mutex locking
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Debug, Hash)]
pub enum CompleteBound {
    /// The unit type
    Unit,
    /// A sum of two other types
    Sum(Arc<Final>, Arc<Final>),
    /// A product of two other types
    Product(Arc<Final>, Arc<Final>),
}

impl Constraint {
    fn free(name: String) -> Self {
        Constraint::Free(name)
    }

    fn unit() -> Self {
        Constraint::Bound {
            occurs_check: false,
            bound: Bound::Unit,
            final_data: Some(Arc::new(Final::unit())),
        }
    }

    fn sum(a: Type, b: Type) -> Self {
        Constraint::Bound {
            occurs_check: false,
            final_data: if let (Some(adata), Some(bdata)) = (a.final_data(), b.final_data()) {
                Some(Arc::new(Final::sum(adata, bdata)))
            } else {
                None
            },
            bound: Bound::Sum(a, b),
        }
    }

    fn product(a: Type, b: Type) -> Self {
        Constraint::Bound {
            occurs_check: false,
            final_data: if let (Some(adata), Some(bdata)) = (a.final_data(), b.final_data()) {
                Some(Arc::new(Final::product(adata, bdata)))
            } else {
                None
            },
            bound: Bound::Product(a, b),
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
    pub fn bind(
        &self,
        bound: Bound,
        final_data: &Option<Arc<Final>>,
        hint: &'static str,
    ) -> Result<(), Error> {
        let root = self.find_root();
        let lock = root.inner_lock();
        match lock.constraint {
            // Free types are simply dropped and replaced by the new bound
            Constraint::Free(_) => {
                let mut lock = lock.into_mutable(); // free means non-finalized
                lock.constraint = Constraint::Bound {
                    bound,
                    final_data: final_data.as_ref().map(Arc::clone),
                    occurs_check: false,
                };
                Ok(())
            }
            // Types with bounds are more interesting: we have to recursively unify
            // the existing bound with the new one.
            Constraint::Bound {
                bound: ref existing_bound,
                ref final_data,
                ..
            } => {
                match (&existing_bound, &bound) {
                    (Bound::Unit, Bound::Unit) => Ok(()),
                    (Bound::Sum(ref x1, ref x2), Bound::Sum(ref y1, ref y2))
                    | (Bound::Product(ref x1, ref x2), Bound::Product(ref y1, ref y2)) => {
                        // Before recursing we need to drop the lock, and before
                        // that we need to clone some types from inside it.
                        let x1 = x1.shallow_clone();
                        let x2 = x2.shallow_clone();
                        let is_not_finalized = final_data.is_none();
                        drop(lock); // drop lock before recursing
                        x1.unify(y1, hint)?;
                        x2.unify(y2, hint)?;
                        if is_not_finalized {
                            // If this type is not complete, it may be after unification, giving us
                            // an opportunity to finaliize it. We do this eagerly to make sure that
                            // "complete" (no free children) is always equivalent to "finalized" (the
                            // `final_data` field being populated), even during inference.
                            //
                            // It also gives the user access to more information about the type,
                            // prior to finalization.
                            if let (Some(data1), Some(data2)) = (y1.final_data(), y2.final_data()) {
                                let mut lock = root.inner_lock().into_mutable(); // ok, we are in a `if is_not_finalized` block
                                if let Constraint::Bound {
                                    ref mut final_data, ..
                                } = lock.constraint
                                {
                                    if let Bound::Sum(..) = bound {
                                        *final_data = Some(Arc::new(Final::sum(data1, data2)));
                                    } else {
                                        *final_data = Some(Arc::new(Final::product(data1, data2)));
                                    }
                                }
                            }
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
                if xdata.tmr == ydata.tmr {
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
            Constraint::Free(_) => Ok(()),
            // If y was already bound to a type, then x must be bound, too
            Constraint::Bound {
                bound: y_bound,
                final_data,
                ..
            } => x_root.bind(y_bound, &final_data, hint),
            Constraint::EqualTo { .. } => unreachable!("y_lock is a root node"),
        }
    }

    /// Accessor for this type's bound
    pub fn bound(&self) -> Option<Bound> {
        let root = self.find_root();
        let root_lock = root.inner_lock();
        if let Constraint::Bound { ref bound, .. } = root_lock.constraint {
            Some(bound.shallow_clone())
        } else {
            None
        }
    }

    /// Accessor for the TMR of this type, if it is final
    pub fn tmr(&self) -> Option<Tmr> {
        let root = self.find_root(); // Final data is only actually set on root nodes
        let root_lock = root.inner_lock();
        root_lock.final_data().map(|data| data.tmr)
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
    /// finalized, since its children may have been unified to a complete type.
    /// To ensure this value is consistent, first call `attempt_finalize`.
    pub fn is_final(&self) -> bool {
        self.tmr().is_some()
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
        let data = match root_lock.constraint {
            Constraint::Free(_) => {
                root_lock.constraint = Constraint::Bound {
                    occurs_check: false,
                    bound: Bound::Unit,
                    final_data: None, // will be overwritten below
                };
                Arc::new(Final::unit())
            }
            Constraint::Bound {
                ref bound,
                ref mut occurs_check,
                ..
            } => {
                if *occurs_check {
                    return Err(Error::OccursCheck {
                        incomplete_bound: bound.shallow_clone(),
                    });
                }
                *occurs_check = true;
                Arc::new(match bound {
                    Bound::Unit => Final::unit(),
                    Bound::Sum(ref ty1, ref ty2) => {
                        // Drop the lock before recursing
                        let ty1 = ty1.clone();
                        let ty2 = ty2.clone();
                        drop(root_lock);
                        let data1 = ty1.finalize()?;
                        let data2 = ty2.finalize()?;
                        // Then re-lock to update own final data
                        root_lock = root.inner_lock().into_mutable();
                        Final::sum(data1, data2)
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
                        Final::product(data1, data2)
                    }
                })
            }
            Constraint::EqualTo { .. } => unreachable!("lock is a root node"),
        };

        // After the above code, we definitely have a `Constraint::Bound` with
        // a populated `final_data` field.
        if let Constraint::Bound {
            ref mut final_data, ..
        } = root_lock.constraint
        {
            *final_data = Some(Arc::clone(&data));
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
            Constraint::Free(ref id) => fmt::Display::fmt(id, f),
            Constraint::Bound {
                final_data: Some(ref data),
                ..
            } => fmt::Display::fmt(data, f),
            Constraint::Bound { ref bound, .. } => fmt::Display::fmt(bound, f),
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
            Bound::Unit => f.write_str("1"),
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
                constraint,
            })),
        }
    }
}
