// Rust Simplicity Library
// Written in 2023 by
//   Andrew Poelstra <rust-simplicity@wpsoftware.net>
//   Christian Lewe <clewe@blockstream.com>
//   Sanket Kanjalkar <sanket1729@gmail.com>
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

pub use bititer::{BitIter, EarlyEndOfStreamError};
pub use bitwriter::BitWriter;
