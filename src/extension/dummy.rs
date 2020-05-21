// Rust Simplicity Library
// Written in 2020 by
//   Andrew Poelstra <apoelstra@blockstream.com>
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

//! # Dummy Extension
//!
//! If some extension is compiled out, it is replaced with this
//!

use bititer::BitIter;
use super::TypeName;
use Error;
use cmr;

/// Dummy extension provides no combinators and cannot be constructed
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum Node { }

/// Decode a natural number according to section 7.2.1
/// of the Simplicity whitepaper.
pub fn decode_node_no_witness<I: Iterator<Item = u8>>(
    _: &mut BitIter<I>,
) -> Result<Node, Error> {
    Err(Error::ParseError("[unavailable extension]"))
}

impl Node {
    /// Name of the source type for this node
    pub fn source_type(&self) -> TypeName {
        match *self {}  // lol rust
    }

    /// Name of the target type for this node
    pub fn target_type(&self) -> TypeName {
        match *self {}
    }

    /// CMR for this node
    pub fn cmr(&self) -> cmr::Cmr {
        match *self {}
    }
}


