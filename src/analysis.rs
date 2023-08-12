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

use crate::jet::Jet;
use crate::Value;
use std::{cmp, fmt};

#[cfg(feature = "elements")]
use std::io;

#[cfg(feature = "elements")]
use elements::encode::Encodable;

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
///
/// The cost of a program is compared to its _budget_.
/// A program is valid if it does not exceed its budget.
///
/// The budget is the size of the witness stack
/// of the transaction input that includes the program.
/// Users pay for their Simplicity programs in terms of fees
/// which are based on transaction size, like normal Tapscript.
///
/// Programs that are CPU-heavy need to be padded
/// so that the witness stack provides a large-enough budget.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct Cost(u32);

impl Cost {
    /// Overhead constant.
    ///
    /// Every combinator that is executed has this overhead added to its cost.
    const OVERHEAD: Self = Cost(100);

    /// Cost of combinators that are never executed.
    ///
    /// **This should only be used for `fail` nodes!**
    const NEVER_EXECUTED: Self = Cost(0);

    /// Maximum cost allowed by consensus.
    ///
    /// This is equal to the maximum budget that any program
    /// can have inside a Taproot transaction:
    /// 4 million weight units plus 50 free weight units for validation.
    ///
    /// This assumes a block that consists of a single transaction
    /// which in turn consists of nothing but its witness stack.
    ///
    /// Transactions include other data besides the witness stack.
    /// Also, transaction may have multiple inputs and
    /// blocks usually include multiple transactions.
    /// This means that the maximum budget is an unreachable upper bound.
    pub const CONSENSUS_MAX: Self = Cost(4_000_050_000);

    /// Return the cost of a type with the given bit width.
    pub const fn of_type(bit_width: usize) -> Self {
        // Cast safety: bit width cannot be more than 2^32 - 1
        Cost(bit_width as u32)
    }

    /// Convert the given milli weight units into cost.
    pub const fn from_milliweight(milliweight: u32) -> Self {
        Cost(milliweight)
    }

    /// Return whether the cost is allowed by consensus.
    ///
    /// This means the cost is within the maximum budget
    /// that any program inside a Taproot transaction can have.
    pub fn is_consensus_valid(&self) -> bool {
        self <= &Self::CONSENSUS_MAX
    }

    /// Return whether the cost is less or equal the given weight.
    pub fn less_equal_weight(&self, weight: u32) -> bool {
        self.0 <= weight.saturating_mul(1000)
    }

    /// Return whether the cost is within the budget of
    /// the given transaction input.
    #[cfg(feature = "elements")]
    pub fn is_budget_valid(&self, txin: &elements::TxIn) -> bool {
        let mut sink = io::sink();
        let witness_stack_serialized_len = txin
            .witness
            .script_witness
            .consensus_encode(&mut sink)
            .expect("writing to sink never fails");
        let budget = witness_stack_serialized_len + 50;
        // Cast safety: witness stack serialized length cannot be more than 2^32 - 51
        self.less_equal_weight(budget as u32)
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
    /// CPU cost
    pub cost: Cost,
}

impl NodeBounds {
    const NOP: Self = NodeBounds {
        extra_cells: 0,
        extra_frames: 0,
        cost: Cost::OVERHEAD,
    };
    const NEVER_EXECUTED: Self = NodeBounds {
        extra_cells: 0,
        extra_frames: 0,
        cost: Cost::NEVER_EXECUTED,
    };

    fn from_child(child: Self) -> Self {
        NodeBounds {
            extra_cells: child.extra_cells,
            extra_frames: child.extra_frames,
            cost: Cost::OVERHEAD + child.cost,
        }
    }

    /// Node bounds for an `iden` node
    pub fn iden(target_type: usize) -> NodeBounds {
        NodeBounds {
            extra_cells: 0,
            extra_frames: 0,
            cost: Cost::OVERHEAD + Cost::of_type(target_type),
        }
    }

    /// Node bounds for a `unit` node
    pub const fn unit() -> NodeBounds {
        NodeBounds::NOP
    }

    /// Node bounds for an `injl` node
    pub fn injl(child: Self) -> NodeBounds {
        Self::from_child(child)
    }

    /// Node bounds for an `injr` node
    pub fn injr(child: Self) -> NodeBounds {
        Self::from_child(child)
    }

    /// Node bounds for a `take` node
    pub fn take(child: Self) -> NodeBounds {
        Self::from_child(child)
    }

    /// Node bounds for a `drop` node
    pub fn drop(child: Self) -> NodeBounds {
        Self::from_child(child)
    }

    /// Node bounds for a `comp` node
    pub fn comp(left: Self, right: Self, mid_ty_bit_width: usize) -> NodeBounds {
        NodeBounds {
            extra_cells: mid_ty_bit_width + cmp::max(left.extra_cells, right.extra_cells),
            extra_frames: 1 + cmp::max(left.extra_frames, right.extra_frames),
            cost: Cost::OVERHEAD + Cost::of_type(mid_ty_bit_width) + left.cost + right.cost,
        }
    }

    /// Node bounds for a `case` node
    pub fn case(left: Self, right: Self) -> NodeBounds {
        NodeBounds {
            extra_cells: cmp::max(left.extra_cells, right.extra_cells),
            extra_frames: cmp::max(left.extra_frames, right.extra_frames),
            cost: Cost::OVERHEAD + cmp::max(left.cost, right.cost),
        }
    }

    /// Node bounds for a `assertl` node
    pub fn assertl(child: Self) -> NodeBounds {
        Self::from_child(child)
    }

    /// Node bounds for a `assertr` node
    pub fn assertr(child: Self) -> NodeBounds {
        Self::from_child(child)
    }

    /// Node bounds for a `pair` node
    pub fn pair(left: Self, right: Self) -> NodeBounds {
        NodeBounds {
            extra_cells: cmp::max(left.extra_cells, right.extra_cells),
            extra_frames: cmp::max(left.extra_frames, right.extra_frames),
            cost: Cost::OVERHEAD + left.cost + right.cost,
        }
    }

    // disconnect, jet, witness, word
    /// Node bounds for a `disconnect` node
    pub fn disconnect(
        left: Self,
        right: Self,
        left_target_b_bit_width: usize, // bit width of B in (b x C) target type
        left_source_bit_width: usize,
        left_target_bit_width: usize,
    ) -> NodeBounds {
        NodeBounds {
            extra_cells: left_source_bit_width
                + left_target_bit_width
                + cmp::max(left.extra_cells, right.extra_cells),
            extra_frames: 2 + cmp::max(left.extra_frames, right.extra_frames),
            cost: Cost::OVERHEAD
                + Cost::of_type(left_source_bit_width)
                + Cost::of_type(left_source_bit_width)
                + Cost::of_type(left_target_bit_width)
                + Cost::of_type(left_target_b_bit_width)
                + left.cost
                + right.cost,
        }
    }

    /// Node bounds for an arbitrary jet node
    pub fn witness(target_ty_bit_width: usize) -> NodeBounds {
        NodeBounds {
            extra_cells: target_ty_bit_width,
            extra_frames: 0,
            cost: Cost::OVERHEAD + Cost::of_type(target_ty_bit_width),
        }
    }

    /// Node bounds for an arbitrary jet node
    pub fn jet<J: Jet>(jet: J) -> NodeBounds {
        NodeBounds {
            extra_cells: 0,
            extra_frames: 0,
            cost: jet.cost(),
        }
    }

    /// Node bounds for an arbitrary constant word node
    pub fn const_word(value: &Value) -> NodeBounds {
        NodeBounds {
            extra_cells: 0,
            extra_frames: 0,
            cost: Cost::OVERHEAD + Cost::of_type(value.len()),
        }
    }

    /// Node bounds for a `fail` node.
    ///
    /// This is a bit of a silly constructor because if a `fail` node is actually
    /// executed in the bit machine, it will fail instantly, while if it *isn't*
    /// executed, it will fail the "no unexecuted nodes" check. But to analyze
    /// arbitrary programs, we need it.
    pub const fn fail() -> NodeBounds {
        NodeBounds::NEVER_EXECUTED
    }
}

/// Number of frames required for the input and output of a Simplicity expression
pub(crate) const IO_EXTRA_FRAMES: usize = 2;

#[cfg(test)]
mod tests {
    use simplicity_sys::ffi::bounded::cost_overhead;

    #[test]
    fn test_overhead() {
        // Check that C overhead is same OVERHEAD
        assert_eq!(super::Cost::OVERHEAD.0, cost_overhead());
    }
}
