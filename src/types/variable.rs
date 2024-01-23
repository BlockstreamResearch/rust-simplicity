// SPDX-License-Identifier: CC0-1.0

//! Names for type variables

use std::sync::atomic::{AtomicUsize, Ordering};

/// Global counter used to give type variables unique names.
static NEXT_ID: AtomicUsize = AtomicUsize::new(1);

/// Obtain a fresh variable name, with the given `prefix` and a unique incrementing
/// numeric suffix.
pub fn new_name(prefix: &str) -> String {
    let id = NEXT_ID.fetch_add(1, Ordering::SeqCst);
    format!("{}{:02}", prefix, id)
}
