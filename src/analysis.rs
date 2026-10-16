// SPDX-License-Identifier: CC0-1.0

use crate::jet::Jet;
use std::{cmp, fmt};

use crate::value::Word;
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
    /// The budget is the serialized size of the witness stack (in weight
    /// units) plus the 50 free weight units of validation weight.
    ///
    /// The witness stack is passed as `&[Vec<u8>]`, where each inner `Vec<u8>`
    /// is one stack item. Both Bitcoin and Elements serialize the witness stack
    /// identically: a CompactSize item count, followed by each item as a
    /// CompactSize length prefix and its bytes. The math is therefore
    /// chain-agnostic, and this method is available under the `bitcoin` feature
    /// (which `elements` implies) so it can be shared by both.
    #[cfg(feature = "bitcoin")]
    fn get_budget(script_witness: &[Vec<u8>]) -> U32Weight {
        // Serialized witness stack size, in bytes.
        let mut serialized_len = compact_size_len(script_witness.len());
        for item in script_witness {
            serialized_len += compact_size_len(item.len()) + item.len();
        }
        let budget = serialized_len.saturating_add(50);
        let budget =
            u32::try_from(budget).expect("Serialized witness stack must be shorter than 2^32");
        U32Weight(budget)
    }

    /// Return whether the cost is within the budget of
    /// the given script witness of a transaction input.
    ///
    /// The script witness is passed as `&[Vec<u8>]`, where each inner `Vec<u8>`
    /// is one witness stack item.
    #[cfg(feature = "bitcoin")]
    pub fn is_budget_valid(self, script_witness: &[Vec<u8>]) -> bool {
        let budget = Self::get_budget(script_witness);
        self.0 <= budget.0.saturating_mul(1000)
    }

    /// Return the length, in bytes, of the padding witness stack item required
    /// so the transaction input has enough budget to cover the cost.
    ///
    /// The returned length is chain-agnostic: adding a stack item of length `L`
    /// increases the serialized witness-stack budget by `CompactSize(L) + L`,
    /// which depends only on the item's length, not its content. Bitcoin and
    /// Elements therefore require the same padding *length*; only the bytes
    /// that fill it differ (see [`Self::get_padding_bytes`]).
    ///
    /// The script witness is passed as `&[Vec<u8>]`, where each inner `Vec<u8>`
    /// is one witness stack item (the padding item is *not* yet included).
    ///
    /// Returns `None` if no padding is required, i.e. the cost already fits
    /// within the budget of the given witness stack.
    #[cfg(feature = "bitcoin")]
    pub fn get_padding_size(self, script_witness: &[Vec<u8>]) -> Option<usize> {
        let weight = U32Weight::from(self);
        let budget = Self::get_budget(script_witness);
        if weight <= budget {
            return None;
        }

        // Adding the padding item to the witness stack increases the serialized
        // size by:
        //
        // 1. CompactSize(item_len): the length prefix of the padding item
        // 2. item_len: the padding item bytes themselves
        //
        // CompactSize uses 1 byte for values <= 252, 3 bytes for <= 65535,
        // and 5 bytes for larger values. The overhead subtracted must account
        // for the actual CompactSize encoding length of the resulting item.
        let deficit = (weight - budget).0 as usize; // cast safety: 32-bit machine or higher

        // overhead = compact_size_len + 1 (for the 0x50 annex tag, when an
        // Elements annex is used; for a plain Bitcoin zero item there is no
        // tag, but the deficit accounting is identical because what matters is
        // only the total serialized item length).
        let padding_len = match deficit {
            // item_len <= 252, compact_size uses 1 byte, overhead = 2
            0..=253 => deficit.saturating_sub(2),
            // Boundary region: item must be >= 253 bytes (3-byte compact_size),
            // but deficit - 4 < 252. Use minimum padding for 3-byte encoding.
            254..=255 => 252,
            // item_len in 253..=65535, compact_size uses 3 bytes, overhead = 4
            256..=65538 => deficit - 4,
            // Boundary region for 5-byte compact_size encoding.
            65539..=65540 => 65535,
            // item_len >= 65536, compact_size uses 5 bytes, overhead = 6
            _ => deficit - 6,
            // Note: the 9-byte compact_size boundary (deficit > 4_294_967_300)
            // is unreachable because Cost uses u32 milliweight, limiting the
            // maximum deficit to ~4_294_968 weight units.
        };

        Some(padding_len + 1)
    }

    /// Return the bytes of the padding witness stack item required so the
    /// transaction input has enough budget to cover the cost.
    ///
    /// This is the Bitcoin-specific form: the padding is a single all-zero
    /// witness stack item. (Bitcoin has no annex, so the padding must be a
    /// regular stack item; a leading `0x50` byte would be misread as an annex.)
    #[cfg(all(feature = "bitcoin", not(feature = "elements")))]
    pub fn get_padding_bytes(self, script_witness: &[Vec<u8>]) -> Option<Vec<u8>> {
        self.get_padding_size(script_witness)
            .map(|len| vec![0x00; len])
    }

    /// Return the bytes of the padding witness stack item required so the
    /// transaction input has enough budget to cover the cost.
    ///
    /// This is the Elements-specific form: on Elements, padding is an annex,
    /// which is a single witness stack item whose first byte is the `0x50`
    /// annex tag (BIP 341) followed by zero bytes.
    ///
    /// To build an annex of total length `L` (as returned by
    /// [`Self::get_padding_size`]) this returns `[0x50]` followed by `L - 1`
    /// zero bytes. Callers targeting non-Liquid Elements (where annexes are
    /// unavailable) should instead use the length and build their own all-zero
    /// item, as Bitcoin does.
    #[cfg(feature = "elements")]
    pub fn get_padding_bytes(self, script_witness: &[Vec<u8>]) -> Option<Vec<u8>> {
        self.get_padding_size(script_witness).map(|len| {
            let mut annex = Vec::with_capacity(len);
            annex.push(0x50);
            annex.extend(std::iter::repeat(0x00).take(len.saturating_sub(1)));
            annex
        })
    }

    /// Return the annex bytes that are required as padding so the transaction
    /// input has enough budget to cover the cost.
    ///
    /// The first annex byte is `0x50`, as defined in BIP 341.
    /// The following padding bytes are `0x00`.
    #[cfg(feature = "elements")]
    #[deprecated(
        since = "0.7.1",
        note = "use `get_padding_size` (chain-agnostic item length) or `get_padding_bytes` (chain-specific bytes) instead"
    )]
    pub fn get_padding(self, script_witness: &[Vec<u8>]) -> Option<Vec<u8>> {
        self.get_padding_bytes(script_witness)
    }
}

/// The length, in bytes, of the CompactSize (VarInt) encoding of `value`.
///
/// CompactSize uses 1 byte for values <= 252, 3 bytes for <= 65535, 5 bytes
/// for <= 2^32-1, and 9 bytes otherwise. `Cost` uses `u32` milliweight, so
/// serialized lengths can never exceed the 5-byte encoding range.
#[cfg(feature = "bitcoin")]
fn compact_size_len(value: usize) -> usize {
    match value {
        0..=252 => 1,
        253..=65_535 => 3,
        65_536..=4_294_967_295 => 5,
        _ => 9,
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
        ];

        for (cost, expected_weight) in test_vectors {
            let converted_cost = U32Weight::from(cost);
            let expected_weight = U32Weight(expected_weight);
            assert_eq!(converted_cost, expected_weight);
        }
    }

    #[test]
    #[cfg(feature = "elements")]
    #[allow(deprecated)]
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
            // Test around CompactSize boundary (annex_len crossing 252 -> 253)
            // deficit = 253: annex_len = 252, compact_size = 1 byte, overhead = 2
            (Cost::from_milliweight(empty + 253_000), vec![], Some(252)),
            // deficit = 254: annex_len must be 253 (3-byte compact_size), overhead = 4
            (Cost::from_milliweight(empty + 254_000), vec![], Some(253)),
            // deficit = 255: same boundary case
            (Cost::from_milliweight(empty + 255_000), vec![], Some(253)),
            // deficit = 256: annex_len = 253, compact_size = 3, exact fit
            (Cost::from_milliweight(empty + 256_000), vec![], Some(253)),
            // deficit = 257: annex_len = 254
            (Cost::from_milliweight(empty + 257_000), vec![], Some(254)),
            // Large annex (exercises the 3-byte compact_size path)
            (
                Cost::from_milliweight(empty + 7_424_000),
                vec![],
                Some(7_421),
            ),
            // Hash loop example
            (
                Cost::from_milliweight(8_045_103),
                vec![vec![], vec![0; 497], vec![0; 32], vec![0; 33]],
                Some(7_424),
            ),
            // Max
            (Cost::CONSENSUS_MAX, vec![], Some(3_999_994)),
        ];

        for (cost, mut witness, maybe_padding) in test_vectors {
            let size = cost.get_padding_size(&witness);
            match maybe_padding {
                None => {
                    assert!(cost.is_budget_valid(&witness));
                    assert_eq!(size, None);
                    assert!(cost.get_padding_bytes(&witness).is_none());
                    // deprecated alias still works and agrees
                    assert!(cost.get_padding(&witness).is_none());
                }
                Some(expected_annex_len) => {
                    assert!(!cost.is_budget_valid(&witness));

                    // The chain-agnostic size is the total item length, which for
                    // the Elements annex equals 0x50 tag + zero padding bytes.
                    let size = size.expect("not enough budget");
                    assert_eq!(expected_annex_len, size);

                    let annex_bytes = cost.get_padding_bytes(&witness).expect("not enough budget");
                    assert_eq!(size, annex_bytes.len());
                    assert_eq!(annex_bytes[0], 0x50);
                    assert!(annex_bytes[1..].iter().all(|&b| b == 0x00));

                    // The deprecated alias agrees with get_padding_bytes.
                    assert_eq!(cost.get_padding(&witness).unwrap(), annex_bytes);

                    witness.extend(std::iter::once(annex_bytes));
                    assert!(cost.is_budget_valid(&witness));

                    witness.pop();
                    assert!(!cost.is_budget_valid(&witness), "Padding must be minimal");
                }
            }
        }
    }

    #[test]
    #[cfg(all(feature = "bitcoin", not(feature = "elements")))]
    fn test_get_padding_bitcoin() {
        // Empty witness stack budget is 51 WU: 50 free weight + 1 length byte.
        let empty = 51_000;

        let test_vectors = vec![
            (Cost::from_milliweight(empty + 2_001), vec![], 2usize),
            (
                Cost::from_milliweight(8_045_103),
                vec![vec![], vec![0; 497], vec![0; 32], vec![0; 33]],
                7_424usize,
            ),
        ];

        for (cost, mut witness, expected_len) in test_vectors {
            assert!(!cost.is_budget_valid(&witness));

            let size = cost.get_padding_size(&witness).expect("not enough budget");
            assert_eq!(expected_len, size);

            // Bitcoin padding is a plain all-zero witness stack item: no annex
            // tag, and no 0x50-leading byte (which would be misread as an annex).
            let padding = cost.get_padding_bytes(&witness).expect("not enough budget");
            assert_eq!(size, padding.len());
            assert!(padding.iter().all(|&b| b == 0x00));

            witness.push(padding);
            assert!(cost.is_budget_valid(&witness));

            witness.pop();
            assert!(!cost.is_budget_valid(&witness), "Padding must be minimal");
        }
    }
}
