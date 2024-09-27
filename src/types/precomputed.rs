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
use std::convert::TryInto;
use std::sync::Arc;

// Directly use the size of the precomputed TMR table to make sure they're in sync.
const N_POWERS: usize = Tmr::POWERS_OF_TWO.len();

thread_local! {
    static POWERS_OF_TWO: RefCell<Option<[Arc<Final>; N_POWERS]>> = const { RefCell::new(None) };
}

fn initialize(write: &mut Option<[Arc<Final>; N_POWERS]>) {
    let one = Final::unit();
    let mut powers = Vec::with_capacity(N_POWERS);

    // Two^(2^0) = Two = (One + One)
    let mut power = Final::sum(Arc::clone(&one), one);
    powers.push(Arc::clone(&power));

    // Two^(2^(i + 1)) = (Two^(2^i) * Two^(2^i))
    for _ in 1..N_POWERS {
        power = Final::product(Arc::clone(&power), power);
        powers.push(Arc::clone(&power));
    }

    let powers: [Arc<Final>; N_POWERS] = powers.try_into().unwrap();
    *write = Some(powers);
}

/// Obtain a precomputed copy of the nth power of two
///
/// # Panics
///
/// Panics if you request a number `n` greater than or equal to [`Tmr::POWERS_OF_TWO`].
pub fn nth_power_of_2(n: usize) -> Arc<Final> {
    POWERS_OF_TWO.with(|arr| {
        if arr.borrow().is_none() {
            initialize(&mut arr.borrow_mut());
        }
        debug_assert!(arr.borrow().is_some());
        Arc::clone(&arr.borrow().as_ref().unwrap()[n])
    })
}
