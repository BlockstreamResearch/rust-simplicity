// SPDX-License-Identifier: CC0-1.0

//! # Decoding
//!
//! Functionality to decode Simplicity programs.
//! Refer to [`crate::encode`] for information on the encoding.

use crate::dag::{Dag, DagLike, InternalSharing};
use crate::jet::Jet;
use crate::merkle::cmr::Cmr;
use crate::node::{
    ConstructNode, CoreConstructible, DisconnectConstructible, JetConstructible, NoWitness,
    WitnessConstructible,
};
use crate::types;
use crate::{BitIter, FailEntropy, Value};
use std::collections::HashSet;
use std::sync::Arc;
use std::{error, fmt};

use super::bititer::u2;

type ArcNode<J> = Arc<ConstructNode<J>>;

/// Decoding error
#[non_exhaustive]
#[derive(Debug)]
pub enum Error {
    /// Node made a back-reference past the beginning of the program
    BadIndex,
    /// Both children of a node are hidden
    BothChildrenHidden,
    /// Program must not be empty
    EmptyProgram,
    /// Bitstream ended early
    EndOfStream,
    /// Hidden node occurred outside of a case combinator
    HiddenNode,
    /// Tried to parse a jet but the name wasn't recognized
    InvalidJet,
    /// Number exceeded 32 bits
    NaturalOverflow,
    /// Program is not encoded in canonical order
    NotInCanonicalOrder,
    /// Program does not have maximal sharing
    SharingNotMaximal,
    /// Tried to allocate too many nodes in a program
    TooManyNodes(usize),
    /// Type-checking error
    Type(crate::types::Error),
}

impl From<super::bititer::EarlyEndOfStreamError> for Error {
    fn from(_: super::bititer::EarlyEndOfStreamError) -> Error {
        Error::EndOfStream
    }
}

impl From<crate::types::Error> for Error {
    fn from(e: crate::types::Error) -> Error {
        Error::Type(e)
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::BadIndex => {
                f.write_str("node made a back-reference past the beginning of the program")
            }
            Error::BothChildrenHidden => f.write_str("both children of a case node are hidden"),
            Error::EmptyProgram => f.write_str("empty program"),
            Error::EndOfStream => f.write_str("bitstream ended early"),
            Error::HiddenNode => write!(f, "hidden node occurred outside of a case combinator"),
            Error::InvalidJet => write!(f, "unrecognized jet"),
            Error::NaturalOverflow => f.write_str("encoded number exceeded 32 bits"),
            Error::NotInCanonicalOrder => f.write_str("program not in canonical order"),
            Error::SharingNotMaximal => f.write_str("Decoded programs must have maximal sharing"),
            Error::TooManyNodes(k) => {
                write!(f, "program has too many nodes ({})", k)
            }
            Error::Type(ref e) => fmt::Display::fmt(e, f),
        }
    }
}

impl error::Error for Error {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match *self {
            Error::BadIndex => None,
            Error::BothChildrenHidden => None,
            Error::EmptyProgram => None,
            Error::EndOfStream => None,
            Error::HiddenNode => None,
            Error::InvalidJet => None,
            Error::NaturalOverflow => None,
            Error::NotInCanonicalOrder => None,
            Error::SharingNotMaximal => None,
            Error::TooManyNodes(..) => None,
            Error::Type(ref e) => Some(e),
        }
    }
}

#[derive(Debug)]
enum DecodeNode<J: Jet> {
    Iden,
    Unit,
    InjL(usize),
    InjR(usize),
    Take(usize),
    Drop(usize),
    Comp(usize, usize),
    Case(usize, usize),
    Pair(usize, usize),
    Disconnect1(usize),
    Disconnect(usize, usize),
    Witness,
    Fail(FailEntropy),
    Hidden(Cmr),
    Jet(J),
    Word(Arc<Value>),
}

impl<'d, J: Jet> DagLike for (usize, &'d [DecodeNode<J>]) {
    type Node = DecodeNode<J>;

    fn data(&self) -> &DecodeNode<J> {
        &self.1[self.0]
    }

    fn as_dag_node(&self) -> Dag<Self> {
        let nodes = &self.1;
        match self.1[self.0] {
            DecodeNode::Iden
            | DecodeNode::Unit
            | DecodeNode::Fail(..)
            | DecodeNode::Hidden(..)
            | DecodeNode::Jet(..)
            | DecodeNode::Word(..) => Dag::Nullary,
            DecodeNode::InjL(i)
            | DecodeNode::InjR(i)
            | DecodeNode::Take(i)
            | DecodeNode::Drop(i)
            | DecodeNode::Disconnect1(i) => Dag::Unary((i, nodes)),
            DecodeNode::Comp(li, ri)
            | DecodeNode::Case(li, ri)
            | DecodeNode::Pair(li, ri)
            | DecodeNode::Disconnect(li, ri) => Dag::Binary((li, nodes), (ri, nodes)),
            DecodeNode::Witness => Dag::Nullary,
        }
    }
}

pub fn decode_expression<I: Iterator<Item = u8>, J: Jet>(
    bits: &mut BitIter<I>,
) -> Result<ArcNode<J>, Error> {
    enum Converted<J: Jet> {
        Node(ArcNode<J>),
        Hidden(Cmr),
    }
    use Converted::{Hidden, Node};
    impl<J: Jet> Converted<J> {
        fn get(&self) -> Result<&ArcNode<J>, Error> {
            match self {
                Node(arc) => Ok(arc),
                Hidden(_) => Err(Error::HiddenNode),
            }
        }
    }

    let len = bits.read_natural(None)?;

    if len == 0 {
        return Err(Error::EmptyProgram);
    }
    // FIXME: check maximum length of DAG that is allowed by consensus
    if len > 1_000_000 {
        return Err(Error::TooManyNodes(len));
    }

    let inference_context = types::Context::new();
    let mut nodes = Vec::with_capacity(len);
    for _ in 0..len {
        let new_node = decode_node(bits, nodes.len())?;
        nodes.push(new_node);
    }

    // It is a sharing violation for any hidden node to be repeated. Track them in this set.
    let mut hidden_set = HashSet::<Cmr>::new();
    // Convert the DecodeNode structure into a CommitNode structure
    let mut converted = Vec::<Converted<J>>::with_capacity(len);
    for data in (nodes.len() - 1, &nodes[..]).post_order_iter::<InternalSharing>() {
        // Check canonical order as we go
        if data.index != data.node.0 {
            return Err(Error::NotInCanonicalOrder);
        }

        let new = match nodes[data.node.0] {
            DecodeNode::Unit => Node(ArcNode::unit(&inference_context)),
            DecodeNode::Iden => Node(ArcNode::iden(&inference_context)),
            DecodeNode::InjL(i) => Node(ArcNode::injl(converted[i].get()?)),
            DecodeNode::InjR(i) => Node(ArcNode::injr(converted[i].get()?)),
            DecodeNode::Take(i) => Node(ArcNode::take(converted[i].get()?)),
            DecodeNode::Drop(i) => Node(ArcNode::drop_(converted[i].get()?)),
            DecodeNode::Comp(i, j) => {
                Node(ArcNode::comp(converted[i].get()?, converted[j].get()?)?)
            }
            DecodeNode::Case(i, j) => {
                // Case is a special case, since it uniquely is allowed to have hidden
                // children (but only one!) in which case it becomes an assertion.
                match (&converted[i], &converted[j]) {
                    (Node(left), Node(right)) => Node(ArcNode::case(left, right)?),
                    (Node(left), Hidden(cmr)) => Node(ArcNode::assertl(left, *cmr)?),
                    (Hidden(cmr), Node(right)) => Node(ArcNode::assertr(*cmr, right)?),
                    (Hidden(_), Hidden(_)) => return Err(Error::BothChildrenHidden),
                }
            }
            DecodeNode::Pair(i, j) => {
                Node(ArcNode::pair(converted[i].get()?, converted[j].get()?)?)
            }
            DecodeNode::Disconnect1(i) => Node(ArcNode::disconnect(converted[i].get()?, &None)?),
            DecodeNode::Disconnect(i, j) => Node(ArcNode::disconnect(
                converted[i].get()?,
                &Some(Arc::clone(converted[j].get()?)),
            )?),
            DecodeNode::Witness => Node(ArcNode::witness(&inference_context, NoWitness)),
            DecodeNode::Fail(entropy) => Node(ArcNode::fail(&inference_context, entropy)),
            DecodeNode::Hidden(cmr) => {
                if !hidden_set.insert(cmr) {
                    return Err(Error::SharingNotMaximal);
                }
                Hidden(cmr)
            }
            DecodeNode::Jet(j) => Node(ArcNode::jet(&inference_context, j)),
            DecodeNode::Word(ref w) => Node(ArcNode::const_word(&inference_context, Arc::clone(w))),
        };
        converted.push(new);
    }

    converted[len - 1].get().map(Arc::clone)
}

/// Decode a single Simplicity node from bits and
/// insert it into a hash map at its index for future reference by ancestor nodes.
fn decode_node<I: Iterator<Item = u8>, J: Jet>(
    bits: &mut BitIter<I>,
    index: usize,
) -> Result<DecodeNode<J>, Error> {
    // First bit: 1 for jets/words, 0 for normal combinators
    if bits.read_bit()? {
        // Second bit: 1 for jets, 0 for words
        if bits.read_bit()? {
            J::decode(bits).map(|jet| DecodeNode::Jet(jet))
        } else {
            let depth = bits.read_natural(Some(32))?;
            let word = decode_power_of_2(bits, 1 << (depth - 1))?;
            Ok(DecodeNode::Word(word))
        }
    } else {
        // Bits 2 and 3: code
        match bits.read_u2()? {
            u2::_0 => {
                let subcode = bits.read_u2()?;
                let i_abs = index - bits.read_natural(Some(index))?;
                let j_abs = index - bits.read_natural(Some(index))?;

                // Bits 4 and 5: subcode
                match subcode {
                    u2::_0 => Ok(DecodeNode::Comp(i_abs, j_abs)),
                    u2::_1 => Ok(DecodeNode::Case(i_abs, j_abs)),
                    u2::_2 => Ok(DecodeNode::Pair(i_abs, j_abs)),
                    u2::_3 => Ok(DecodeNode::Disconnect(i_abs, j_abs)),
                }
            }
            u2::_1 => {
                let subcode = bits.read_u2()?;
                let i_abs = index - bits.read_natural(Some(index))?;
                // Bits 4 and 5: subcode
                match subcode {
                    u2::_0 => Ok(DecodeNode::InjL(i_abs)),
                    u2::_1 => Ok(DecodeNode::InjR(i_abs)),
                    u2::_2 => Ok(DecodeNode::Take(i_abs)),
                    u2::_3 => Ok(DecodeNode::Drop(i_abs)),
                }
            }
            u2::_2 => {
                // Bits 4 and 5: subcode
                match bits.read_u2()? {
                    u2::_0 => Ok(DecodeNode::Iden),
                    u2::_1 => Ok(DecodeNode::Unit),
                    u2::_2 => Ok(DecodeNode::Fail(bits.read_fail_entropy()?)),
                    u2::_3 => {
                        let i_abs = index - bits.read_natural(Some(index))?;
                        Ok(DecodeNode::Disconnect1(i_abs))
                    }
                }
            }
            u2::_3 => {
                // Bit 4: subcode
                if bits.read_bit()? {
                    Ok(DecodeNode::Witness)
                } else {
                    Ok(DecodeNode::Hidden(bits.read_cmr()?))
                }
            }
        }
    }
}

/// Decode a value from bits, of the form 2^exp
///
/// # Panics
///
/// Panics if `exp` itself is not a power of 2
pub fn decode_power_of_2<I: Iterator<Item = bool>>(
    iter: &mut I,
    exp: usize,
) -> Result<Arc<Value>, Error> {
    struct StackElem {
        value: Arc<Value>,
        width: usize,
    }

    assert_eq!(exp.count_ones(), 1, "exp must be a power of 2");

    let mut stack = Vec::with_capacity(32);
    for _ in 0..exp {
        // Read next bit
        let bit = Value::u1(u8::from(iter.next().ok_or(Error::EndOfStream)?));
        stack.push(StackElem {
            value: bit,
            width: 1,
        });

        while stack.len() >= 2 && stack[stack.len() - 1].width == stack[stack.len() - 2].width {
            let right = stack.pop().unwrap();
            let left = stack.pop().unwrap();
            stack.push(StackElem {
                value: Value::prod(left.value, right.value),
                width: left.width * 2,
            });
        }
    }
    Ok(stack.pop().unwrap().value)
}

/// Decode a natural number from bits.
/// If a bound is specified, then the decoding terminates before trying to decode a larger number.
pub fn decode_natural<I: Iterator<Item = bool>>(
    iter: &mut I,
    bound: Option<usize>,
) -> Result<usize, Error> {
    let mut recurse_depth = 0;
    loop {
        match iter.next() {
            Some(true) => recurse_depth += 1,
            Some(false) => break,
            None => return Err(Error::EndOfStream),
        }
    }

    let mut len = 0;
    loop {
        let mut n = 1;
        for _ in 0..len {
            let bit = match iter.next() {
                Some(false) => 0,
                Some(true) => 1,
                None => return Err(Error::EndOfStream),
            };
            n = 2 * n + bit;
        }

        if recurse_depth == 0 {
            if let Some(bound) = bound {
                if n > bound {
                    return Err(Error::BadIndex);
                }
            }
            return Ok(n);
        } else {
            len = n;
            if len > 31 {
                return Err(Error::NaturalOverflow);
            }
            recurse_depth -= 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::encode;
    use crate::jet::Core;
    use crate::node::{CommitNode, RedeemNode};
    use crate::BitWriter;

    #[test]
    fn root_unit_to_unit() {
        // main = jet_eq_32 :: 2^64 -> 2 # 7387d279
        let justjet = [0x6d, 0xb8, 0x80];
        // Should be able to decode this as an expression...
        let mut iter = BitIter::from(&justjet[..]);
        decode_expression::<_, Core>(&mut iter).unwrap();
        // ...but NOT as a CommitNode
        let mut iter = BitIter::from(&justjet[..]);
        CommitNode::<Core>::decode::<_>(&mut iter).unwrap_err();
        // ...or as a RedeemNode
        let mut iter = BitIter::from(&justjet[..]);
        RedeemNode::<Core>::decode::<_>(&mut iter).unwrap_err();
    }

    #[test]
    fn decode_fixed_natural() {
        let tries = vec![
            (1, vec![false]),
            (2, vec![true, false, false]),
            (3, vec![true, false, true]),
            (4, vec![true, true, false, false, false, false]),
            (5, vec![true, true, false, false, false, true]),
            (6, vec![true, true, false, false, true, false]),
            (7, vec![true, true, false, false, true, true]),
            (8, vec![true, true, false, true, false, false, false]),
            (15, vec![true, true, false, true, true, true, true]),
            (
                16,
                vec![
                    true, true, true, false, // len: 1
                    false, // len: 2
                    false, false, // len: 4
                    false, false, false, false,
                ],
            ),
            // 31
            (
                31,
                vec![
                    true, true, true, false, // len: 1
                    false, // len: 2
                    false, false, // len: 4
                    true, true, true, true,
                ],
            ),
            // 32
            (
                32,
                vec![
                    true, true, true, false, // len: 1
                    false, // len: 2
                    false, true, // len: 5
                    false, false, false, false, false,
                ],
            ),
            // 2^15
            (
                32768,
                vec![
                    true, true, true, false, // len: 1
                    true,  // len: 3
                    true, true, true, // len: 15
                    false, false, false, false, false, false, false, false, false, false, false,
                    false, false, false, false,
                ],
            ),
            (
                65535,
                vec![
                    true, true, true, false, // len: 1
                    true,  // len: 3
                    true, true, true, // len: 15
                    true, true, true, true, true, true, true, true, true, true, true, true, true,
                    true, true,
                ],
            ),
            (
                65536,
                vec![
                    true, true, true, true, false, // len: 1
                    false, // len: 2
                    false, false, // len: 4
                    false, false, false, false, // len: 16
                    false, false, false, false, false, false, false, false, false, false, false,
                    false, false, false, false, false,
                ],
            ),
        ];

        for (natural, bitvec) in tries {
            let truncated = bitvec[0..bitvec.len() - 1].to_vec();
            assert!(matches!(
                decode_natural(&mut truncated.into_iter(), None),
                Err(Error::EndOfStream)
            ));

            let mut sink = Vec::<u8>::new();

            let mut w = BitWriter::from(&mut sink);
            encode::encode_natural(natural, &mut w).expect("encoding to vector");
            w.flush_all().expect("flushing");
            assert_eq!(w.n_total_written(), bitvec.len());

            let decoded_natural = decode_natural(&mut BitIter::from(sink.into_iter()), None)
                .expect("decoding from vector");
            assert_eq!(natural, decoded_natural);
        }
    }
}
