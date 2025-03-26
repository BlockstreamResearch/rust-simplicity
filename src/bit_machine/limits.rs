// SPDX-License-Identifier: CC0-1.0

//! Bit Machine Resource Limits
//!
//! Implementation of the Bit Machine, without TCO, as TCO precludes some
//! frame management optimizations which can be used to great benefit.
//!

use core::fmt;
use std::error;

/// The maximum number of cells needed by the bit machine.
///
/// This roughly corresponds to the maximum size of any type used by a
/// program, in bits. In a blockchain context this should be limited by
/// the transaction's weight budget.
///
/// The limit here is an absolute limit enforced by the library to avoid
/// unbounded allocations.
// This value must be less than usize::MAX / 2 to avoid panics in this module.
const MAX_CELLS: usize = 2 * 1024 * 1024 * 1024 - 1;

/// The maximum number of frames needed by the bit machine.
///
/// This roughly corresponds to the maximum depth of nested `comp` and
/// `disconnect` combinators. In a blockchain context this should be
/// limited by the transaction's weight budget.
///
/// The limit here is an absolute limit enforced by the library to avoid
/// unbounded allocations.
// This value must be less than usize::MAX / 2 to avoid panics in this module.
const MAX_FRAMES: usize = 1024 * 1024;

#[non_exhaustive]
#[derive(Clone, Debug)]
pub enum LimitError {
    MaxCellsExceeded {
        /// The number of cells needed by the program.
        got: usize,
        /// The maximum allowed number of cells.
        max: usize,
        /// A description of which cell count exceeded the limit.
        bound: &'static str,
    },
    MaxFramesExceeded {
        /// The number of frames needed by the program.
        got: usize,
        /// The maximum allowed number of frames.
        max: usize,
        /// A description of which frame count exceeded the limit.
        bound: &'static str,
    },
}

impl fmt::Display for LimitError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let (limit, got, max, bound) = match self {
            LimitError::MaxCellsExceeded { got, max, bound } => ("cells", got, max, bound),
            LimitError::MaxFramesExceeded { got, max, bound } => ("frames", got, max, bound),
        };
        write!(
            f,
            "maximum number of {} exceeded (needed {}, maximum {}) (bound: {})",
            limit, got, max, bound,
        )
    }
}
impl error::Error for LimitError {}

impl LimitError {
    fn check_max_cells(got: usize, bound: &'static str) -> Result<(), Self> {
        if got > MAX_CELLS {
            Err(Self::MaxCellsExceeded {
                got,
                max: MAX_CELLS,
                bound,
            })
        } else {
            Ok(())
        }
    }

    fn check_max_frames(got: usize, bound: &'static str) -> Result<(), Self> {
        if got > MAX_FRAMES {
            Err(Self::MaxFramesExceeded {
                got,
                max: MAX_CELLS,
                bound,
            })
        } else {
            Ok(())
        }
    }

    /// Helper function to check every value and sum for being within bounds.
    pub(super) fn check_program<J: crate::jet::Jet>(
        program: &crate::RedeemNode<J>,
    ) -> Result<(), Self> {
        let source_ty_width = program.arrow().source.bit_width();
        let target_ty_width = program.arrow().target.bit_width();
        let bounds = program.bounds();

        Self::check_max_cells(source_ty_width, "source type width")?;
        Self::check_max_cells(target_ty_width, "target type width")?;
        Self::check_max_cells(bounds.extra_cells, "extra cells")?;
        Self::check_max_cells(
            source_ty_width + target_ty_width,
            "source + target type widths",
        )?;
        Self::check_max_cells(
            source_ty_width + target_ty_width + bounds.extra_cells,
            "source + target type widths + extra cells",
        )?;

        Self::check_max_frames(bounds.extra_frames, "extra frames")?;
        Self::check_max_frames(
            bounds.extra_frames + crate::analysis::IO_EXTRA_FRAMES,
            "extra frames + fixed overhead",
        )?;
        Ok(())
    }
}
