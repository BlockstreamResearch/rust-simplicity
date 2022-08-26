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

use crate::core::commit::CommitNodeInner;
use crate::core::node::{NodeBounds, NodeType};
use crate::core::Value;
use crate::core::{CommitNode, Node};
use crate::jet::Application;
use std::cmp;
use std::rc::Rc;

/// Return the bounds for the given node, once finalized.
///
/// Nodes with left children require their finalized left child,
/// while nodes with right children require their finalized right child.
/// Witness nodes require their node type.
pub(crate) fn compute_bounds<Witness, App: Application>(
    untyped_node: &CommitNode<Witness, App>,
    left: Option<Rc<Node<Value, App>>>,
    right: Option<Rc<Node<Value, App>>>,
    ty: &NodeType,
) -> NodeBounds {
    NodeBounds {
        extra_cells: compute_extra_cells_bound(untyped_node, left.clone(), right.clone(), ty),
        frame_count: compute_frame_count_bound(untyped_node, left, right),
    }
}

/// Return an upper bound on the number of cells required
/// by the given node during execution on the Bit Machine.
fn compute_extra_cells_bound<Witness, App: Application>(
    untyped_node: &CommitNode<Witness, App>,
    left: Option<Rc<Node<Value, App>>>,
    right: Option<Rc<Node<Value, App>>>,
    ty: &NodeType,
) -> usize {
    match untyped_node.inner {
        CommitNodeInner::Iden
        | CommitNodeInner::Unit
        | CommitNodeInner::Fail(_, _)
        | CommitNodeInner::Hidden(_)
        | CommitNodeInner::Jet(_) => 0,
        CommitNodeInner::InjL(_)
        | CommitNodeInner::InjR(_)
        | CommitNodeInner::Take(_)
        | CommitNodeInner::Drop(_) => left.unwrap().bounds.extra_cells,
        CommitNodeInner::Comp(_, _) => {
            let left = left.unwrap();
            left.ty.target.bit_width
                + cmp::max(left.bounds.extra_cells, right.unwrap().bounds.extra_cells)
        }
        CommitNodeInner::Case(_, _)
        | CommitNodeInner::AssertL(_, _)
        | CommitNodeInner::AssertR(_, _)
        | CommitNodeInner::Pair(_, _) => cmp::max(
            left.unwrap().bounds.extra_cells,
            right.unwrap().bounds.extra_cells,
        ),
        CommitNodeInner::Disconnect(_, _) => {
            let left = left.unwrap();
            left.ty.source.bit_width
                + left.ty.target.bit_width
                + cmp::max(left.bounds.extra_cells, right.unwrap().bounds.extra_cells)
        }
        CommitNodeInner::Witness(_) => ty.target.bit_width,
    }
}

/// Return an upper bound on the number of frames required
/// by the given node during execution on the Bit Machine.
fn compute_frame_count_bound<Witness, App: Application>(
    untyped_node: &CommitNode<Witness, App>,
    left: Option<Rc<Node<Value, App>>>,
    right: Option<Rc<Node<Value, App>>>,
) -> usize {
    match untyped_node.inner {
        CommitNodeInner::Iden
        | CommitNodeInner::Unit
        | CommitNodeInner::Witness(_)
        | CommitNodeInner::Fail(_, _)
        | CommitNodeInner::Hidden(_)
        | CommitNodeInner::Jet(_) => 0,
        CommitNodeInner::InjL(_)
        | CommitNodeInner::InjR(_)
        | CommitNodeInner::Take(_)
        | CommitNodeInner::Drop(_) => left.unwrap().bounds.frame_count,
        CommitNodeInner::Comp(_, _) => {
            1 + cmp::max(
                left.unwrap().bounds.frame_count,
                right.unwrap().bounds.frame_count,
            )
        }
        CommitNodeInner::Case(_, _)
        | CommitNodeInner::AssertL(_, _)
        | CommitNodeInner::AssertR(_, _)
        | CommitNodeInner::Pair(_, _) => cmp::max(
            left.unwrap().bounds.frame_count,
            right.unwrap().bounds.frame_count,
        ),
        CommitNodeInner::Disconnect(_, _) => {
            2 + cmp::max(
                left.unwrap().bounds.frame_count,
                right.unwrap().bounds.frame_count,
            )
        }
    }
}
