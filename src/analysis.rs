// SPDX-License-Identifier: CC0-1.0

use crate::jet::Jet;
use std::{cmp, fmt};

use crate::value::Word;
#[cfg(feature = "bitcoin")]
use bitcoin::VarInt;
#[cfg(feature = "serde")]
use serde::Serialize;

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
#[cfg_attr(feature = "serde", derive(Serialize))]
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

    /// Return the budget of the given script witness of a transaction input.
    ///
    /// The budget is the serialized size of the witness stack, in weight units, plus 50 weight units.
    /// Bitcoin and Elements serialize a witness stack the same way.
    /// A CompactSize item count, then each item with a CompactSize length prefix.
    #[cfg(feature = "bitcoin")]
    fn get_budget(script_witness: &[Vec<u8>]) -> U32Weight {
        let mut serialized_len = VarInt::from(script_witness.len()).size();
        for item in script_witness {
            serialized_len += VarInt::from(item.len()).size() + item.len();
        }

        // A witness longer than u32::MAX bytes covers any `Cost`, so saturate
        let budget = u32::try_from(serialized_len)
            .unwrap_or(u32::MAX)
            .saturating_add(50);

        U32Weight(budget)
    }

    /// Return whether the cost is within the budget of
    /// the given script witness of a transaction input.
    #[cfg(feature = "bitcoin")]
    pub fn is_budget_valid(self, script_witness: &[Vec<u8>]) -> bool {
        let budget = Self::get_budget(script_witness);
        self.0 <= budget.0.saturating_mul(1000)
    }

    /// Return the length of the shortest annex that, appended to the given script witness,
    /// gives the transaction input enough budget for the cost.
    ///
    /// Returns `None` if the cost already fits.
    /// Otherwise the length is at least 1, because an annex starts with the `0x50` tag (BIP 341).
    #[cfg(feature = "bitcoin")]
    pub fn get_annex_padding_size(self, script_witness: &[Vec<u8>]) -> Option<usize> {
        let weight = U32Weight::from(self);
        let budget = Self::get_budget(script_witness);
        if weight <= budget {
            return None;
        }

        // Adding the annex to the witness stack increases the serialized size by:
        //
        // 1. CompactSize(annex_len): the length prefix of the annex item
        // 2. annex_len: the annex bytes themselves (0x50 tag + zero padding)
        //
        // CompactSize uses 1 byte for values <= 252, 3 bytes for <= 65535,
        // and 5 bytes for larger values. The overhead subtracted must account
        // for the actual CompactSize encoding length of the resulting annex.
        let deficit = (weight - budget).0 as usize; // cast safety: 32-bit machine or higher

        // The annex is one more stack item.
        // If that takes the item count past 252 or 65 535, its CompactSize grows by 2 bytes,
        // which also count towards the budget, so take them off the deficit first.
        let count_prefix_growth = VarInt::from(script_witness.len() + 1).size()
            - VarInt::from(script_witness.len()).size();
        let deficit = deficit.saturating_sub(count_prefix_growth);

        // `padding_len` counts the zero bytes after the 0x50 tag, so
        // overhead = the CompactSize length of annex_len + 1 (for the tag).
        let padding_len = match deficit {
            // annex_len <= 252, compact_size uses 1 byte, overhead = 2
            0..=253 => deficit.saturating_sub(2),
            // Boundary region: annex must be >= 253 bytes (3-byte compact_size),
            // but deficit - 4 < 252. Use minimum padding for 3-byte encoding.
            254..=255 => 252,
            // annex_len in 253..=65535, compact_size uses 3 bytes, overhead = 4
            256..=65538 => deficit - 4,
            // Boundary region for 5-byte compact_size encoding.
            65539..=65540 => 65535,
            // annex_len >= 65536, compact_size uses 5 bytes, overhead = 6
            _ => deficit - 6,
            // Note: the 9-byte compact_size boundary (deficit > 4_294_967_300)
            // is unreachable because Cost uses u32 milliweight, limiting the
            // maximum deficit to ~4_294_968 weight units.
        };

        Some(padding_len + 1)
    }

    /// Return the annex bytes that are required as padding so the transaction input
    /// has enough budget to cover the cost.
    ///
    /// The first annex byte is 0x50, as defined in BIP 341.
    /// The following padding bytes are 0x00.
    /// The length is [`Self::get_annex_padding_size`].
    #[cfg(feature = "elements")]
    pub fn get_padding(self, script_witness: &[Vec<u8>]) -> Option<Vec<u8>> {
        self.get_annex_padding_size(script_witness).map(|len| {
            let mut annex = vec![0x00; len];
            annex[0] = 0x50;
            annex
        })
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
        // Round up, because part of a weight unit still needs a whole unit of budget
        Self(value.0.div_ceil(1000))
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
#[cfg_attr(feature = "serde", derive(Serialize))]
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
    pub fn jet(jet: &dyn Jet) -> NodeBounds {
        NodeBounds {
            extra_cells: 0,
            extra_frames: 0,
            cost: Cost::OVERHEAD + jet.cost(),
        }
    }

    /// Node bounds for an arbitrary constant word node
    pub fn const_word(word: &Word) -> NodeBounds {
        NodeBounds {
            extra_cells: 0,
            extra_frames: 0,
            cost: Cost::OVERHEAD + Cost::of_type(word.len()),
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
            (Cost::from_milliweight(4_294_967_000), 4_294_967),
            (Cost::from_milliweight(4_294_967_001), 4_294_968),
            (Cost::from_milliweight(u32::MAX), 4_294_968),
        ];

        for (cost, expected_weight) in test_vectors {
            let converted_cost = U32Weight::from(cost);
            let expected_weight = U32Weight(expected_weight);
            assert_eq!(converted_cost, expected_weight);
        }
    }

    #[cfg(feature = "bitcoin")]
    fn padding_vectors() -> Vec<(Cost, Vec<Vec<u8>>, Option<usize>)> {
        // An empty witness has a budget of 51 WU: 50 WU plus its 1-byte item count.
        let empty = 51_000;

        // 252 empty items have a budget of 1 + 252 + 50 WU.
        // A 253rd item grows the item count from 1 byte to 3.
        let items_252 = (1 + 252 + 50) * 1000;

        vec![
            (Cost::from_milliweight(0), vec![], None),
            (Cost::from_milliweight(empty), vec![], None),
            // 1 WU short: the 1-byte annex [0x50] adds 2 bytes.
            (Cost::from_milliweight(empty + 1), vec![], Some(1)),
            (Cost::from_milliweight(empty + 50_000), vec![], Some(49)),
            // Annex lengths around the 1 -> 3 byte CompactSize boundary.
            (Cost::from_milliweight(empty + 253_000), vec![], Some(252)),
            (Cost::from_milliweight(empty + 254_000), vec![], Some(253)),
            // Annex lengths around the 3 -> 5 byte CompactSize boundary.
            (
                Cost::from_milliweight(empty + 65_538_000),
                vec![],
                Some(65_535),
            ),
            (
                Cost::from_milliweight(empty + 65_539_000),
                vec![],
                Some(65_536),
            ),
            // The growth of the item count counts towards the budget.
            (
                Cost::from_milliweight(items_252 + 10_000),
                vec![vec![]; 252],
                Some(7),
            ),
            // Hash loop example
            (
                Cost::from_milliweight(8_045_103),
                vec![vec![], vec![0; 497], vec![0; 32], vec![0; 33]],
                Some(7_424),
            ),
            // Max
            (Cost::CONSENSUS_MAX, vec![], Some(3_999_994)),
        ]
    }

    #[test]
    #[cfg(feature = "bitcoin")]
    fn test_get_annex_padding_size() {
        for (cost, mut witness, expected) in padding_vectors() {
            let size = cost.get_annex_padding_size(&witness);
            assert_eq!(size, expected, "cost {cost}");

            let Some(size) = size else {
                assert!(cost.is_budget_valid(&witness));
                continue;
            };
            assert!(!cost.is_budget_valid(&witness));

            // Only the length of the annex affects the budget.
            witness.push(vec![0x00; size]);
            assert!(
                cost.is_budget_valid(&witness),
                "{size} bytes do not cover cost {cost}"
            );

            // The annex must be minimal. One byte shorter must not suffice.
            // A 1-byte annex is only the tag and cannot be shorter.
            if size > 1 {
                witness.last_mut().expect("non-empty").pop();
                assert!(
                    !cost.is_budget_valid(&witness),
                    "{size} bytes are not minimal for cost {cost}"
                );
            }
        }
    }

    #[test]
    #[cfg(feature = "elements")]
    fn test_get_padding() {
        for (cost, witness, expected) in padding_vectors() {
            let annex = cost.get_padding(&witness);
            assert_eq!(annex.as_ref().map(Vec::len), expected);

            if let Some(annex) = annex {
                assert_eq!(annex[0], 0x50);
                assert!(annex[1..].iter().all(|&b| b == 0x00));
            }
        }
    }
}
