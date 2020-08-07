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

use std::{fmt, io};

use super::TypeName;
use crate::bititer::BitIter;
use crate::cmr;
use crate::encode;
use crate::exec;
use crate::extension;
use crate::Error;

/// Dummy transaction environment
pub struct TxEnv;

/// Dummy extension provides no combinators and cannot be constructed
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum DummyNode {}

impl extension::Jet for DummyNode {
    type TxEnv = TxEnv;

    fn decode<I: Iterator<Item = u8>>(_: &mut BitIter<I>) -> Result<DummyNode, Error> {
        Err(Error::ParseError("[unavailable extension]"))
    }

    fn source_type(&self) -> TypeName {
        match *self {} // lol rust
    }

    fn target_type(&self) -> TypeName {
        match *self {}
    }

    fn cmr(&self) -> cmr::Cmr {
        match *self {}
    }

    fn encode<W: encode::BitWrite>(&self, _: &mut W) -> io::Result<usize> {
        match *self {}
    }

    fn exec(&self, _: &mut exec::BitMachine, _: &Self::TxEnv) {
        match *self {}
    }
}

impl fmt::Display for DummyNode {
    fn fmt(&self, _: &mut fmt::Formatter) -> fmt::Result {
        match *self {}
    }
}
