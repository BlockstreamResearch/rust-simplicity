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
