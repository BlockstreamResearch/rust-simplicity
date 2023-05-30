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

//! Type Variables
//!

use std::collections::VecDeque;
use std::iter::FromIterator;

/// Factory for creating free variables with fresh names.
/// Identifiers are assigned sequentially as follows:
/// `A`, `B`, `C`, ... `Z`, `AA`, `AB`, `AC`, ...
pub(crate) struct Factory {
    next_id: usize,
}

impl Factory {
    /// Create a new factory.
    pub fn new() -> Self {
        Self { next_id: 1 }
    }

    /// Return a free variable with a fresh name.
    pub fn new_name(&mut self) -> String {
        let mut n = self.next_id;
        self.next_id += 1;
        let mut ascii_bytes = VecDeque::new();

        while n / 26 > 0 {
            ascii_bytes.push_front((n % 26) as u8 + 65);
            n /= 26;
        }

        ascii_bytes.push_front((n % 26) as u8 + 64);
        String::from_utf8(Vec::from_iter(ascii_bytes.into_iter())).unwrap()
    }

    /// Return a free variable with a fresh name.
    pub fn free_variable(&mut self) -> crate::core::types::RcVar {
        crate::core::types::Variable::free(self.new_name())
    }
}
