// SPDX-License-Identifier: CC0-1.0

use crate::jet::Jet;
use crate::Value;
use std::{cmp, fmt};

#[cfg(feature = "elements")]
use elements::encode::Encodable;
#[cfg(feature = "elements")]
use std::{convert::TryFrom, io};

/// Copy of [`bitcoin::Weight`] that uses [`u32`] instead of [`u64`].
///
/// This struct is useful for conversions between [`bitcoin::Weight`]
/// (which uses [`u64`]) and [`Cost`] (which uses [`u32`]).
#[cfg(feature = "bitcoin")]
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct U32Weight(u32);

#[cfg(feature = "bitcoin")]
impl std::ops::Sub for U32Weight {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self(self.0.saturating_sub(rhs.0))
    }
}

#[cfg(feature = "bitcoin")]
impl From<bitcoin::Weight> for U32Weight {
    fn from(value: bitcoin::Weight) -> Self {
        Self(u32::try_from(value.to_wu()).unwrap_or(u32::MAX))
    }
}

#[cfg(feature = "bitcoin")]
impl From<U32Weight> for bitcoin::Weight {
    fn from(value: U32Weight) -> Self {
        bitcoin::Weight::from_wu(u64::from(value.0))
    }
}

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
    pub fn is_consensus_valid(self) -> bool {
        self <= Self::CONSENSUS_MAX
    }

    /// Return the budget of the given script witness of a transaction output.
    ///
    /// The script witness is passed as `&Vec<Vec<u8>>` in order to use
    /// the consensus encoding implemented for this type.
    #[cfg(feature = "elements")]
    fn get_budget(script_witness: &Vec<Vec<u8>>) -> U32Weight {
        let mut sink = io::sink();
        let witness_stack_serialized_len = script_witness
            .consensus_encode(&mut sink)
            .expect("writing to sink never fails");
        let budget = u32::try_from(witness_stack_serialized_len)
            .expect("Serialized witness stack must be shorter than 2^32 elements")
            .saturating_add(50);
        U32Weight(budget)
    }

    /// Return whether the cost is within the budget of
    /// the given script witness of a transaction input.
    ///
    /// The script witness is passed as `&Vec<Vec<u8>>` in order to use
    /// the consensus encoding implemented for this type.
    #[cfg(feature = "elements")]
    pub fn is_budget_valid(self, script_witness: &Vec<Vec<u8>>) -> bool {
        let budget = Self::get_budget(script_witness);
        self.0 <= budget.0.saturating_mul(1000)
    }

    /// Return the annex bytes that are required as padding
    /// so the transaction input has enough budget to cover the cost.
    ///
    /// The first annex byte is 0x50, as defined in BIP 341.
    /// The following padding bytes are 0x00.
    #[cfg(feature = "elements")]
    pub fn get_padding(self, script_witness: &Vec<Vec<u8>>) -> Option<Vec<u8>> {
        let weight = U32Weight::from(self);
        let budget = Self::get_budget(script_witness);
        if weight <= budget {
            return None;
        }

        // Two bytes are automatically added to the encoded witness stack by adding the annex:
        //
        // 1. The encoded annex starts with the annex byte length
        // 2. The first annex byte is always 0x50
        //
        // The remaining padding is done by adding (zero) bytes to the annex.
        let required_padding = weight - budget - U32Weight(2);
        let padding_len = required_padding.0 as usize; // cast safety: 32-bit machine or higher
        let annex_bytes: Vec<u8> = std::iter::once(0x50)
            .chain(std::iter::repeat(0x00).take(padding_len))
            .collect();

        Some(annex_bytes)
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

#[cfg(feature = "bitcoin")]
impl From<U32Weight> for Cost {
    fn from(value: U32Weight) -> Self {
        Self(value.0.saturating_mul(1000))
    }
}

#[cfg(feature = "bitcoin")]
impl From<Cost> for U32Weight {
    fn from(value: Cost) -> Self {
        // Saturating addition to avoid panic at numeric bounds
        // This results in a slightly different rounding for cost values close to u32::MAX.
        // These values are strictly larger than CONSENSUS_MAX and are of no significance.
        Self(value.0.saturating_add(999) / 1000)
    }
}

#[cfg(feature = "bitcoin")]
impl From<bitcoin::Weight> for Cost {
    fn from(value: bitcoin::Weight) -> Self {
        Self(U32Weight::from(value).0.saturating_mul(1000))
    }
}

#[cfg(feature = "bitcoin")]
impl From<Cost> for bitcoin::Weight {
    fn from(value: Cost) -> Self {
        bitcoin::Weight::from_wu(u64::from(U32Weight::from(value).0))
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
            cost: Cost::OVERHEAD + jet.cost(),
        }
    }

    /// Node bounds for an arbitrary constant word node
    pub fn const_word(value: &Value) -> NodeBounds {
        NodeBounds {
            extra_cells: 0,
            extra_frames: 0,
            cost: Cost::OVERHEAD + Cost::of_type(value.padded_len()),
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
    use super::*;
    use simplicity_sys::ffi::bounded::cost_overhead;

    #[test]
    fn test_overhead() {
        // Check that C overhead is same OVERHEAD
        assert_eq!(Cost::OVERHEAD.0, cost_overhead());
    }

    #[test]
    #[cfg(feature = "bitcoin")]
    fn cost_to_weight() {
        let test_vectors = vec![
            (Cost::NEVER_EXECUTED, 0),
            (Cost::from_milliweight(1), 1),
            (Cost::from_milliweight(999), 1),
            (Cost::from_milliweight(1_000), 1),
            (Cost::from_milliweight(1_001), 2),
            (Cost::from_milliweight(1_999), 2),
            (Cost::from_milliweight(2_000), 2),
            (Cost::CONSENSUS_MAX, 4_000_050),
        ];

        for (cost, expected_weight) in test_vectors {
            let converted_cost = U32Weight::from(cost);
            let expected_weight = U32Weight(expected_weight);
            assert_eq!(converted_cost, expected_weight);
        }
    }

    #[test]
    #[cfg(feature = "elements")]
    fn test_get_padding() {
        // The budget of the empty witness stack is 51 WU:
        //
        // 1. 50 WU of free signature operations
        // 2. 1 WU for the length byte of the witness stack
        let empty = 51_000;

        // The encoded annex starts with a length byte, so remove one padding byte from the annex
        let test_vectors = vec![
            (Cost::from_milliweight(0), vec![], None),
            (Cost::from_milliweight(empty), vec![], None),
            (Cost::from_milliweight(empty + 1), vec![], Some(1)),
            (Cost::from_milliweight(empty + 2_000), vec![], Some(1)),
            (Cost::from_milliweight(empty + 2_001), vec![], Some(2)),
            (Cost::from_milliweight(empty + 3_000), vec![], Some(2)),
            (Cost::from_milliweight(empty + 3_001), vec![], Some(3)),
            (Cost::from_milliweight(empty + 4_000), vec![], Some(3)),
            (Cost::from_milliweight(empty + 4_001), vec![], Some(4)),
            (Cost::from_milliweight(empty + 50_000), vec![], Some(49)),
        ];

        for (cost, mut witness, maybe_padding) in test_vectors {
            match maybe_padding {
                None => {
                    assert!(cost.is_budget_valid(&witness));
                    assert!(cost.get_padding(&witness).is_none());
                }
                Some(expected_annex_len) => {
                    assert!(!cost.is_budget_valid(&witness));

                    let annex_bytes = cost.get_padding(&witness).expect("not enough budget");
                    assert_eq!(expected_annex_len, annex_bytes.len());
                    witness.extend(std::iter::once(annex_bytes));
                    assert!(cost.is_budget_valid(&witness));

                    witness.pop();
                    assert!(!cost.is_budget_valid(&witness), "Padding must be minimal");
                }
            }
        }
    }
}
