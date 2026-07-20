// SPDX-License-Identifier: CC0-1.0

//! Precomputed Types
//!
//! There are several types (the unit type and 2^(2^n) for various powers of two)
//! that we frequently need complete versions of, and therefore want to precompute.
//! They are not very large or even expensive to create, but we don't want to be
//! computing them repeatedly in the middle of a decoding loop.
//!
//! However because [`Type`] is recursive it will not be easy to use constfns for
//! this, for a long time (even with Rust 1.70 it's not obvious how). So instead
//! we store them in thread-local-storage and and out copies of those. Because
//! `Type` internally contains an `Arc` these copies are very cheap.

use crate::Tmr;

use super::Final;

use std::cell::RefCell;
use std::sync::Arc;

// Directly use the size of the precomputed TMR table to make sure they're in sync.
const N_POWERS: usize = Tmr::TWO_TWO_N.len();

// Directly use the size of the precomputed TMR table to make sure they're in sync.
const N_BUFFERS: usize = Tmr::BUFFER8_TWO_N_PLUS_ONE.len();

thread_local! {
    /// Types of the form 2^(2^n) for several small `n`.
    static TWO_TWO_N: RefCell<Option<[Arc<Final>; N_POWERS]>> = const { RefCell::new(None) };

    /// The "variable-length buffer" typed used by sha256 and other jets.
    static BUFFER8_TWO_N_PLUS_ONE: RefCell<Option<[Arc<Final>; N_BUFFERS]>> = const { RefCell::new(None) };

    /// The sha256 Ctx8 type.
    static CTX8: RefCell<Option<Arc<Final>>> = const { RefCell::new(None) };
}

fn initialize(write: &mut Option<[Arc<Final>; N_POWERS]>) {
    let one = Final::unit();

    // Two^(2^0) = Two = (One + One)
    let mut power = Final::sum(Arc::clone(&one), one);
    *write = Some(core::array::from_fn(|i| {
        if i > 0 {
            power = Final::product(Arc::clone(&power), Arc::clone(&power));
        }
        Arc::clone(&power)
    }));
}

/// Obtain a precomputed copy of the nth power of two
///
/// # Panics
///
/// Panics if you request a number `n` greater than or equal to the length
/// of [`Tmr::TWO_TWO_N`].
pub fn nth_power_of_2(n: usize) -> Arc<Final> {
    TWO_TWO_N.with(|arr| {
        if arr.borrow().is_none() {
            initialize(&mut arr.borrow_mut());
        }
        debug_assert!(arr.borrow().is_some());
        Arc::clone(&arr.borrow().as_ref().unwrap()[n])
    })
}

fn initialize_buffers(write: &mut Option<[Arc<Final>; N_BUFFERS]>) {
    // (TWO^8)^<2 = S(TWO^8)
    let mut buf = nth_power_of_2(3).successor();
    *write = Some(core::array::from_fn(|i| {
        if i > 0 {
            buf = Final::product(
                Final::two_two_n(i + 3).unwrap().successor(),
                Arc::clone(&buf),
            );
        }
        Arc::clone(&buf)
    }));
}

/// Obtain a precomputed copy of a "variable-length buffer" type.
///
/// Precisely, this type is `(TWO^8)^<2^(n+1)`, where
///
/// * The notation X^<2 is notation for the type (S X)
/// * The notation X^<(2*n) is notation for the type S (X^n) * X^<n
///
/// And `S X` is the successor of `X`, i.e. `Option<X>`
///
/// # Panics
///
/// Panics if you request a number `n` greater than or equal to the length
/// of [`Tmr::BUFFER8_TWO_N_PLUS_ONE`].
pub fn buffer8_two_n_plus_one(n: usize) -> Arc<Final> {
    BUFFER8_TWO_N_PLUS_ONE.with(|arr| {
        if arr.borrow().is_none() {
            initialize_buffers(&mut arr.borrow_mut());
        }
        debug_assert!(arr.borrow().is_some());
        Arc::clone(&arr.borrow().as_ref().unwrap()[n])
    })
}

/// Obtain a precomputed copy of the `SHA256` `Ctx8` type.
pub fn ctx8() -> Arc<Final> {
    CTX8.with(|opt| {
        if opt.borrow().is_none() {
            *opt.borrow_mut() = Some(Final::product(
                buffer8_two_n_plus_one(5),
                Final::product(
                    Final::two_two_n_fixed::<6>(), // 2^64
                    Final::two_two_n_fixed::<8>(), // 2^256
                ),
            ));
        }
        debug_assert!(opt.borrow().is_some());
        Arc::clone(opt.borrow().as_ref().unwrap())
    })
}
