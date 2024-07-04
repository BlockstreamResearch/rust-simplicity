// SPDX-License-Identifier: CC0-1.0

//! The Simplicity Bit Encoding
//!
//! This module provides functionality to encode and decode Simplicity programs
//! and expresisons at rest and on the blockchain. For [`core::redeem::RedeemNode`]s,
//! this encoding matches the consensus encoding defined by the Simplicity Tech
//! Report.
//!

mod bititer;
mod bitwriter;
pub mod decode;
pub mod encode;

pub use bititer::{u2, BitIter, CloseError, EarlyEndOfStreamError};
pub use bitwriter::{write_to_vec, BitWriter};
