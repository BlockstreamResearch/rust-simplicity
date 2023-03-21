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

//! # Merkle roots
//!
//! Tools for creating Merkle roots.
//! There exist different Merkle roots for commitment and for redemption.

pub mod amr;
pub mod cmr;
pub mod common;
pub mod imr;
pub mod tmr;
