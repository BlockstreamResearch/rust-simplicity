// Rust Simplicity Library
// Written in 2022 by
//   Christian Lewe <clewe@blockstream.com>
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

use crate::core::iter::PostOrderIter;
use crate::core::node::{NodeInner, RefWrapper};
use crate::jet::Application;
use std::collections::HashSet;

/// Check whether the given program has maximal sharing.
///
/// This imposes the following conditions:
/// 1. For hidden nodes, their hash must be unique in the program.
/// 2. For non-hidden nodes, the triple of their IMR, source type TMR and target type TMR
///    must be unique in the program.
pub(crate) fn check_maximal_sharing<Witness, App: Application>(
    program: PostOrderIter<RefWrapper<Witness, App>>,
) -> bool {
    let mut seen_hashes = HashSet::new();
    let mut seen_keys = HashSet::new();

    for node in program {
        if let NodeInner::Hidden(h) = node.0.inner {
            if seen_hashes.contains(&h) {
                return false;
            } else {
                seen_hashes.insert(h);
            }
        } else {
            let primary_key = (node.0.imr, node.0.ty.source.tmr, node.0.ty.target.tmr);

            if seen_keys.contains(&primary_key) {
                return false;
            } else {
                seen_keys.insert(primary_key);
            }
        }
    }

    true
}
