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

use self::union_bound::UbElement;
use crate::Tmr;

use std::fmt;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

pub mod arrow;
mod final_data;
mod precomputed;
mod union_bound;
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
    OccursCheck,
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
            Error::OccursCheck => f.write_str("detected infinitely-sized type"),
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
    /// A set of constraints, which maintained by the union-bound algorithm and
    /// is progressively tightened as type inference proceeds.
    bound: UbElement<bound_mutex::BoundMutex>,
    /// Used during finalization to detect infinitely-sized types.
    occurs_check: Arc<AtomicBool>,
}

mod bound_mutex {
    use super::{Bound, CompleteBound, Error, Final};
    use std::sync::{Arc, Mutex};

    /// Source or target type of a Simplicity expression
    #[derive(Debug)]
    pub struct BoundMutex {
        /// The type's status according to the union-bound algorithm.
        inner: Mutex<Arc<Bound>>,
    }

    impl BoundMutex {
        pub fn new(bound: Bound) -> Self {
            BoundMutex {
                inner: Mutex::new(Arc::new(bound)),
            }
        }

        pub fn get(&self) -> Arc<Bound> {
            Arc::clone(&self.inner.lock().unwrap())
        }

        pub fn set(&self, new: Arc<Bound>) {
            let mut lock = self.inner.lock().unwrap();
            assert!(
                !matches!(**lock, Bound::Complete(..)),
                "tried to modify finalized type",
            );
            *lock = new;
        }

        pub fn bind(&self, bound: Arc<Bound>, hint: &'static str) -> Result<(), Error> {
            let existing_bound = self.get();
            let bind_error = || Error::Bind {
                existing_bound: existing_bound.shallow_clone(),
                new_bound: bound.shallow_clone(),
                hint,
            };

            match (existing_bound.as_ref(), bound.as_ref()) {
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
                            ty1.bind(Arc::new(Bound::Complete(Arc::clone(comp1))), hint)?;
                            ty2.bind(Arc::new(Bound::Complete(Arc::clone(comp2))), hint)
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
                        self.set(Arc::new(Bound::Complete(Arc::new(
                            if let Bound::Sum(..) = *bound {
                                Final::sum(data1, data2)
                            } else {
                                Final::product(data1, data2)
                            },
                        ))));
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

impl Bound {
    /// Clones the `Bound`.
    ///
    /// This is the same as just calling `.clone()` but has a different name to
    /// emphasize that what's being cloned is (at most) a pair of ref-counted
    /// pointers.
    pub fn shallow_clone(&self) -> Bound {
        self.clone()
    }

    fn free(name: String) -> Self {
        Bound::Free(name)
    }

    fn unit() -> Self {
        Bound::Complete(Arc::new(Final::unit()))
    }

    fn sum(a: Type, b: Type) -> Self {
        if let (Some(adata), Some(bdata)) = (a.final_data(), b.final_data()) {
            Bound::Complete(Arc::new(Final::sum(adata, bdata)))
        } else {
            Bound::Sum(a, b)
        }
    }

    fn product(a: Type, b: Type) -> Self {
        if let (Some(adata), Some(bdata)) = (a.final_data(), b.final_data()) {
            Bound::Complete(Arc::new(Final::product(adata, bdata)))
        } else {
            Bound::Product(a, b)
        }
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

impl Type {
    /// Return an unbound type with the given name
    pub fn free(name: String) -> Self {
        Type::from(Bound::free(name))
    }

    /// Return a unit type.
    pub fn unit() -> Self {
        Type::from(Bound::unit())
    }

    /// Return a precomputed copy of 2^(2^n), for given n.
    pub fn two_two_n(n: usize) -> Self {
        precomputed::nth_power_of_2(n)
    }

    /// Return the sum of the given two types.
    pub fn sum(a: Self, b: Self) -> Self {
        Type::from(Bound::sum(a, b))
    }

    /// Return the product of the given two types.
    pub fn product(a: Self, b: Self) -> Self {
        Type::from(Bound::product(a, b))
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
    pub fn bind(&self, bound: Arc<Bound>, hint: &'static str) -> Result<(), Error> {
        let root = self.bound.root();
        root.bind(bound, hint)
    }

    /// Unify the type with another one.
    ///
    /// Fails if the bounds on the two types are incompatible
    pub fn unify(&self, other: &Self, hint: &'static str) -> Result<(), Error> {
        self.bound.unify(&other.bound, |x_bound, y_bound| {
            x_bound.bind(y_bound.get(), hint)
        })
    }

    /// Accessor for this type's bound
    pub fn bound(&self) -> Arc<Bound> {
        self.bound.root().get()
    }

    /// Accessor for the TMR of this type, if it is final
    pub fn tmr(&self) -> Option<Tmr> {
        self.final_data().map(|data| data.tmr())
    }

    /// Accessor for the data of this type, if it is complete
    pub fn final_data(&self) -> Option<Arc<Final>> {
        if let Bound::Complete(ref data) = *self.bound.root().get() {
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
        matches!(*self.bound.root().get(), Bound::Complete(..))
    }

    /// Attempts to finalize the type. Returns its TMR on success.
    pub fn finalize(&self) -> Result<Arc<Final>, Error> {
        let root = self.bound.root();
        let bound = root.get();
        if let Bound::Complete(ref data) = *bound {
            return Ok(Arc::clone(data));
        }

        // FIXME there is a race condition (which actually predates this version
        // of the logic) in case the same type is being finalized from multiple
        // threads at once.
        if self.occurs_check.fetch_or(true, Ordering::SeqCst) {
            return Err(Error::OccursCheck);
        }

        let data = match *bound {
            Bound::Free(_) => Arc::new(Final::unit()),
            Bound::Complete(ref arc) => Arc::clone(arc),
            Bound::Sum(ref ty1, ref ty2) => {
                let data1 = ty1.finalize()?;
                let data2 = ty2.finalize()?;
                Arc::new(Final::sum(data1, data2))
            }
            Bound::Product(ref ty1, ref ty2) => {
                let data1 = ty1.finalize()?;
                let data2 = ty2.finalize()?;
                Arc::new(Final::product(data1, data2))
            }
        };
        // We checked at the start of the function that this type was not final,
        // so set() is okay here.
        root.set(Arc::new(Bound::Complete(Arc::clone(&data))));
        Ok(data)
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
        fmt::Display::fmt(&self.bound.root().get(), f)
    }
}

impl From<Bound> for Type {
    /// Promotes a `Bound` to a type defined by that constraint
    fn from(bound: Bound) -> Type {
        Type {
            bound: UbElement::new(Arc::new(bound_mutex::BoundMutex::new(bound))),
            occurs_check: Arc::new(AtomicBool::new(false)),
        }
    }
}
