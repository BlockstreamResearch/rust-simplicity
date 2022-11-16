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
use crate::core::redeem::{RedeemNodeInner, RefWrapper};
use crate::jet::Jet;
use std::collections::{HashMap, HashSet};

/// Check whether the given program has maximal sharing.
///
/// This imposes the following conditions:
/// 1. For hidden nodes, their hash must be unique in the program.
/// 2. For non-hidden nodes, the triple of their IMR, source type TMR and target type TMR
///    must be unique in the program.
pub(crate) fn check_maximal_sharing<App: Jet>(program: PostOrderIter<RefWrapper<App>>) -> bool {
    let mut seen_hashes = HashSet::new();
    let mut seen_keys = HashSet::new();

    for node in program {
        if let RedeemNodeInner::Hidden(h) = node.0.inner {
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

/// Compute a maximal sharing for the given program.
///
/// Return a mapping of node references to their shared index,
/// and the total length of the shared program.
///
/// # See
/// [`check_maximal_sharing()`]
pub(crate) fn compute_maximal_sharing<App: Jet>(
    program: PostOrderIter<RefWrapper<App>>,
) -> (HashMap<RefWrapper<App>, usize>, usize) {
    let mut node_to_index = HashMap::new();
    let mut index = 0;
    let mut hash_to_node = HashMap::new();
    let mut primary_key_to_node = HashMap::new();

    for node in program {
        debug_assert!(!node_to_index.contains_key(&node));

        if let RedeemNodeInner::Hidden(h) = node.0.inner {
            if let Some(shared_node) = hash_to_node.get(&h) {
                node_to_index.insert(node, *node_to_index.get(shared_node).unwrap());
            } else {
                hash_to_node.insert(h, node);
                node_to_index.insert(node, index);
                index += 1;
            }
        } else {
            let primary_key = (node.0.imr, node.0.ty.source.tmr, node.0.ty.target.tmr);
            if let Some(shared_node) = primary_key_to_node.get(&primary_key) {
                node_to_index.insert(node, *node_to_index.get(shared_node).unwrap());
            } else {
                primary_key_to_node.insert(primary_key, node);
                node_to_index.insert(node, index);
                index += 1;
            }
        }
    }

    (node_to_index, index)
}
