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

use std::error;
use std::{fmt, io};

use super::{ExtError, TypeName};
use crate::bititer::BitIter;
use crate::encode::BitWriter;
use crate::exec;
use crate::extension;
use crate::merkle::cmr::Cmr;
use crate::Error;

/// Dummy transaction environment
pub struct TxEnv;

/// Dummy extension provides no combinators and cannot be constructed
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Debug, Hash)]
pub enum DummyNode {}

impl extension::Jet for DummyNode {
    type TxEnv = TxEnv;
    type JetErr = DummyJetErr;

    fn decode<I: Iterator<Item = u8>>(_: &mut BitIter<I>) -> Result<DummyNode, Error> {
        Err(Error::ParseError("[unavailable extension]"))
    }

    fn encode<W: io::Write>(&self, _: &mut BitWriter<W>) -> io::Result<usize> {
        match *self {} // lol rust
    }

    fn exec(&self, _: &mut exec::BitMachine, _: &Self::TxEnv) -> Result<(), Self::JetErr> {
        match *self {}
    }

    fn cmr(&self) -> Cmr {
        match *self {}
    }

    fn source_type(&self) -> TypeName {
        match *self {}
    }

    fn target_type(&self) -> TypeName {
        match *self {}
    }
}

impl fmt::Display for DummyNode {
    fn fmt(&self, _: &mut fmt::Formatter) -> fmt::Result {
        match *self {}
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum DummyJetErr {}

impl fmt::Display for DummyJetErr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "TODO in a later commit")
    }
}

impl error::Error for DummyJetErr {}
impl ExtError for DummyJetErr {}
