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
use crate::core::redeem::{NodeBounds, NodeType};
use crate::core::{CommitNode, RedeemNode};
use crate::jet::Jet;
use std::cmp;
use std::rc::Rc;

/// Number of frames required for the input and output of a Simplicity expression
pub(crate) const IO_EXTRA_FRAMES: usize = 2;

/// Return the bounds for the given node, once finalized.
///
/// Nodes with left children require their finalized left child,
/// while nodes with right children require their finalized right child.
/// Witness nodes require their node type.
pub(crate) fn compute_bounds<J: Jet>(
    untyped_node: &CommitNode<J>,
    left: Option<Rc<RedeemNode<J>>>,
    right: Option<Rc<RedeemNode<J>>>,
    ty: &NodeType,
) -> NodeBounds {
    NodeBounds {
        extra_cells: compute_extra_cells_bound(untyped_node, left.clone(), right.clone(), ty),
        extra_frames: compute_extra_frames_bound(untyped_node, left, right),
    }
}

/// Return an upper bound on the number of cells required
/// by the given node during execution on the Bit Machine.
fn compute_extra_cells_bound<J: Jet>(
    untyped_node: &CommitNode<J>,
    left: Option<Rc<RedeemNode<J>>>,
    right: Option<Rc<RedeemNode<J>>>,
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
        CommitNodeInner::Witness => ty.target.bit_width,
    }
}

/// Return an upper bound on the number of frames required
/// by the given node during execution on the Bit Machine.
fn compute_extra_frames_bound<J: Jet>(
    untyped_node: &CommitNode<J>,
    left: Option<Rc<RedeemNode<J>>>,
    right: Option<Rc<RedeemNode<J>>>,
) -> usize {
    match untyped_node.inner {
        CommitNodeInner::Iden
        | CommitNodeInner::Unit
        | CommitNodeInner::Witness
        | CommitNodeInner::Fail(_, _)
        | CommitNodeInner::Hidden(_)
        | CommitNodeInner::Jet(_) => 0,
        CommitNodeInner::InjL(_)
        | CommitNodeInner::InjR(_)
        | CommitNodeInner::Take(_)
        | CommitNodeInner::Drop(_) => left.unwrap().bounds.extra_frames,
        CommitNodeInner::Comp(_, _) => {
            1 + cmp::max(
                left.unwrap().bounds.extra_frames,
                right.unwrap().bounds.extra_frames,
            )
        }
        CommitNodeInner::Case(_, _)
        | CommitNodeInner::AssertL(_, _)
        | CommitNodeInner::AssertR(_, _)
        | CommitNodeInner::Pair(_, _) => cmp::max(
            left.unwrap().bounds.extra_frames,
            right.unwrap().bounds.extra_frames,
        ),
        CommitNodeInner::Disconnect(_, _) => {
            2 + cmp::max(
                left.unwrap().bounds.extra_frames,
                right.unwrap().bounds.extra_frames,
            )
        }
    }
}
