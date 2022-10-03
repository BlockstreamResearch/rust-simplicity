// Rust Simplicity Library
// Written in 2020 by
//   Andrew Poelstra <apoelstra@blockstream.com>
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

//! # Core Simplicity structures
//!
//! Defines Simplicity terms, values, types, DAGs and programs.

pub mod commit;
mod context;
pub mod iter;
pub mod redeem;
pub mod types;
mod value;

pub use commit::CommitNode;
pub use context::Context;
pub use redeem::RedeemNode;
pub use value::Value;
