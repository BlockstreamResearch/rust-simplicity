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

//! # Jet Extensions
//!
//! Six jets hacked in to make the program from
//! https://medium.com/blockstream/simplicity-jets-release-803db10fd589
//! work. We have exhausted the set of prefix codes starting with `11` here,
//! so when we do a real jet implementation probably this whole thing will
//! need to be scrapped
//!

use bititer::BitIter;
use encode::Error;

/// Dummy extension provides no combinators and cannot be constructed
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum Node {
    Version,
    LockTime,
    InputsHash,
    OutputsHash,
    NumInputs,
    TotalInputValue,
    CurrentPrevOutpoint,
    CurrentValue,
    CurrentSequence,
    CurrentIndex,
    InputPrevOutpoint,
    InputValue,
    InputSequence,
    NumOutputs,
    TotalOutputValue,
    OutputValue,
    OutputScriptHash,
    ScriptCMR,
}

/// Decode a natural number according to section 7.2.1
/// of the Simplicity whitepaper.
pub fn decode_node_no_witness<I: Iterator<Item = u8>>(
    iter: &mut BitIter<I>,
) -> Result<Node, Error> {
    let code = match iter.read_bits_be(4) {
        Some(code) => code,
        None => return Err(Error::EndOfStream),
    };
    match code {
        0 => match iter.next() {
            Some(false) => Ok(Node::Version),
            Some(true) => Ok(Node::LockTime),
            None => Err(Error::EndOfStream),
        },
        1 => Ok(Node::InputsHash),
        2 => Ok(Node::OutputsHash),
        3 => Ok(Node::NumInputs),
        4 => Ok(Node::TotalInputValue),
        5 => Ok(Node::CurrentPrevOutpoint),
        6 => Ok(Node::CurrentValue),
        7 => Ok(Node::CurrentSequence),
        8 => match iter.next() {
            Some(false) => Ok(Node::CurrentIndex),
            Some(true) => Ok(Node::InputPrevOutpoint),
            None => Err(Error::EndOfStream),
        },
        9 => Ok(Node::InputValue),
        10 => Ok(Node::InputSequence),
        11 => Ok(Node::NumOutputs),
        12 => Ok(Node::TotalOutputValue),
        13 => Ok(Node::OutputValue),
        14 => Ok(Node::OutputScriptHash),
        15 => Ok(Node::ScriptCMR),
        _ => unreachable!(),
    }
}

