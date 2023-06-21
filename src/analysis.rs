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
use crate::jet::Jet;
use crate::types::arrow::FinalArrow;
use crate::{CommitNode, RedeemNode};
use std::cmp;
use std::rc::Rc;

/// Bounds on the resources required by a node during execution on the Bit Machine
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct NodeBounds {
    /// Upper bound on the required number of cells (bits).
    /// The root additionally requires the bit width of its source and target type (input, output)
    pub extra_cells: usize,
    /// Upper bound on the required number of frames (sum of read and write frames).
    /// The root additionally requires two frames (input, output)
    pub extra_frames: usize,
}

impl NodeBounds {
    const ZERO: Self = NodeBounds {
        extra_cells: 0,
        extra_frames: 0,
    };

    /// Node bounds for an `iden` node
    pub const fn iden() -> NodeBounds {
        NodeBounds::ZERO
    }

    /// Node bounds for a `unit` node
    pub const fn unit() -> NodeBounds {
        NodeBounds::ZERO
    }

    /// Node bounds for an `injl` node
    pub const fn injl(child: Self) -> NodeBounds {
        child
    }

    /// Node bounds for an `injr` node
    pub const fn injr(child: Self) -> NodeBounds {
        child
    }

    /// Node bounds for a `take` node
    pub const fn take(child: Self) -> NodeBounds {
        child
    }

    /// Node bounds for a `drop` node
    pub const fn drop(child: Self) -> NodeBounds {
        child
    }

    /// Node bounds for a `comp` node
    pub fn comp(left: Self, right: Self, mid_ty_bit_width: usize) -> NodeBounds {
        NodeBounds {
            extra_cells: mid_ty_bit_width + cmp::max(left.extra_cells, right.extra_cells),
            extra_frames: 1 + cmp::max(left.extra_frames, right.extra_frames),
        }
    }

    /// Node bounds for a `case` node
    pub fn case(left: Self, right: Self) -> NodeBounds {
        NodeBounds {
            extra_cells: cmp::max(left.extra_cells, right.extra_cells),
            extra_frames: cmp::max(left.extra_frames, right.extra_frames),
        }
    }

    /// Node bounds for a `assertl` node
    pub const fn assertl(child: Self) -> NodeBounds {
        child
    }

    /// Node bounds for a `assertr` node
    pub const fn assertr(child: Self) -> NodeBounds {
        child
    }

    /// Node bounds for a `pair` node
    pub fn pair(left: Self, right: Self) -> NodeBounds {
        NodeBounds {
            extra_cells: cmp::max(left.extra_cells, right.extra_cells),
            extra_frames: cmp::max(left.extra_frames, right.extra_frames),
        }
    }

    // disconnect, jet, witness, word
    /// Node bounds for a `disconnect` node
    pub fn disconnect(
        left: Self,
        right: Self,
        left_source_bit_width: usize,
        left_target_bit_width: usize,
    ) -> NodeBounds {
        NodeBounds {
            extra_cells: left_source_bit_width
                + left_target_bit_width
                + cmp::max(left.extra_cells, right.extra_cells),
            extra_frames: 2 + cmp::max(left.extra_frames, right.extra_frames),
        }
    }

    /// Node bounds for an arbitrary jet node
    pub fn witness(target_ty_bit_width: usize) -> NodeBounds {
        NodeBounds {
            extra_cells: target_ty_bit_width,
            extra_frames: 0,
        }
    }

    /// Node bounds for an arbitrary jet node
    pub const fn jet() -> NodeBounds {
        NodeBounds::ZERO
    }

    /// Node bounds for an arbitrary constant word node
    pub const fn const_word() -> NodeBounds {
        NodeBounds::ZERO
    }

    /// Node bounds for a `fail` node.
    ///
    /// This is a bit of a silly constructor because if a `fail` node is actually
    /// executed in the bit machine, it will fail instantly, while if it *isn't*
    /// executed, it will fail the "no unexecuted nodes" check. But to analyze
    /// arbitrary programs, we need it.
    pub const fn fail() -> NodeBounds {
        NodeBounds::ZERO
    }
}

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
    ty: &FinalArrow,
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
    ty: &FinalArrow,
) -> usize {
    match untyped_node.inner() {
        CommitNodeInner::Iden
        | CommitNodeInner::Unit
        | CommitNodeInner::Fail(_)
        | CommitNodeInner::Jet(_)
        | CommitNodeInner::Word(_) => 0,
        CommitNodeInner::InjL(_)
        | CommitNodeInner::InjR(_)
        | CommitNodeInner::Take(_)
        | CommitNodeInner::Drop(_)
        | CommitNodeInner::AssertL(_, _)
        | CommitNodeInner::AssertR(_, _) => left.unwrap().bounds.extra_cells,
        CommitNodeInner::Comp(_, _) => {
            let left = left.unwrap();
            left.ty.target.bit_width()
                + cmp::max(left.bounds.extra_cells, right.unwrap().bounds.extra_cells)
        }
        CommitNodeInner::Case(_, _) | CommitNodeInner::Pair(_, _) => cmp::max(
            left.unwrap().bounds.extra_cells,
            right.unwrap().bounds.extra_cells,
        ),
        CommitNodeInner::Disconnect(_, _) => {
            let left = left.unwrap();
            left.ty.source.bit_width()
                + left.ty.target.bit_width()
                + cmp::max(left.bounds.extra_cells, right.unwrap().bounds.extra_cells)
        }
        CommitNodeInner::Witness => ty.target.bit_width(),
    }
}

/// Return an upper bound on the number of frames required
/// by the given node during execution on the Bit Machine.
fn compute_extra_frames_bound<J: Jet>(
    untyped_node: &CommitNode<J>,
    left: Option<Rc<RedeemNode<J>>>,
    right: Option<Rc<RedeemNode<J>>>,
) -> usize {
    match untyped_node.inner() {
        CommitNodeInner::Iden
        | CommitNodeInner::Unit
        | CommitNodeInner::Witness
        | CommitNodeInner::Fail(_)
        | CommitNodeInner::Jet(_)
        | CommitNodeInner::Word(_) => 0,
        CommitNodeInner::InjL(_)
        | CommitNodeInner::InjR(_)
        | CommitNodeInner::Take(_)
        | CommitNodeInner::Drop(_)
        | CommitNodeInner::AssertL(_, _)
        | CommitNodeInner::AssertR(_, _) => left.unwrap().bounds.extra_frames,
        CommitNodeInner::Comp(_, _) => {
            1 + cmp::max(
                left.unwrap().bounds.extra_frames,
                right.unwrap().bounds.extra_frames,
            )
        }
        CommitNodeInner::Case(_, _) | CommitNodeInner::Pair(_, _) => cmp::max(
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
