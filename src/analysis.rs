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

use std::{cmp, fmt};

/// CPU cost of a Simplicity expression.
///
/// The cost is measured in milli weight units
/// and can be converted into weight units using the appropriate method.
///
/// Roughly speaking, the operational semantics of a combinator
/// on the Bit Machine determine its cost.
///
/// First, every combinator has a fixed overhead cost.
/// Frame allocations, copy and write operations cost proportional
/// to the number of allocated or written bits.
/// Frame moves / drops or cursor moves are one-step operations
/// that are covered by the overhead.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct Cost(usize);

impl Cost {
    /// Overhead constant.
    ///
    /// Every combinator that is executed has this overhead added to its cost.
    const OVERHEAD: Self = Cost(10);

    /// Cost of combinators that are never executed.
    ///
    /// **This should only be used for `fail` nodes!**
    const NEVER_EXECUTED: Self = Cost(0);

    /// Convert the cost to weight units.
    pub const fn to_weight(self) -> usize {
        (self.0 + 999) / 1000
    }

    /// Return the cost of a type with the given bit width.
    pub const fn of_type(bit_width: usize) -> Self {
        Cost(bit_width)
    }

    /// Convert the given milli weight units into cost.
    pub const fn from_milliweight(milliweight: usize) -> Self {
        Cost(milliweight)
    }
}

impl fmt::Display for Cost {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(&self.0, f)
    }
}

impl std::ops::Add for Cost {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Cost(self.0.saturating_add(rhs.0))
    }
}

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
