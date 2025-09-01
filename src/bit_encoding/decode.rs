// SPDX-License-Identifier: CC0-1.0

//! # Decoding
//!
//! Functionality to decode Simplicity programs.
//! Refer to [`crate::encode`] for information on the encoding.

use crate::dag::{Dag, DagLike, InternalSharing};
use crate::jet::Jet;
use crate::merkle::cmr::Cmr;
use crate::node::{
    ConstructNode, CoreConstructible, DisconnectConstructible, JetConstructible,
    WitnessConstructible,
};
use crate::types;
use crate::value::Word;
use crate::{BitIter, FailEntropy};
use std::collections::HashSet;
use std::sync::Arc;
use std::{cmp, error, fmt};

use super::bititer::{u2, DecodeNaturalError};

type ArcNode<'brand, J> = Arc<ConstructNode<'brand, J>>;

/// Decoding error
#[non_exhaustive]
#[derive(Debug)]
pub enum Error {
    /// Error closing the bitstream
    BitIter(crate::BitIterCloseError),
    /// Both children of a node are hidden
    BothChildrenHidden,
    /// Bitstream ended early
    EndOfStream,
    /// Hidden node occurred outside of a case combinator
    HiddenNode,
    /// Tried to parse a jet but the name wasn't recognized
    InvalidJet,
    /// Error decoding a natural number.
    Natural(DecodeNaturalError),
    /// Program is not encoded in canonical order
    NotInCanonicalOrder,
    /// Program does not have maximal sharing
    SharingNotMaximal,
    /// Type-checking error
    Type(crate::types::Error),
}

impl From<crate::EarlyEndOfStreamError> for Error {
    fn from(_: crate::EarlyEndOfStreamError) -> Self {
        Error::EndOfStream
    }
}

impl From<crate::BitIterCloseError> for Error {
    fn from(e: crate::BitIterCloseError) -> Self {
        Error::BitIter(e)
    }
}

impl From<DecodeNaturalError> for Error {
    fn from(e: DecodeNaturalError) -> Error {
        Error::Natural(e)
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
            Error::BitIter(ref e) => fmt::Display::fmt(e, f),
            Error::BothChildrenHidden => f.write_str("both children of a case node are hidden"),
            Error::EndOfStream => f.write_str("bitstream ended early"),
            Error::HiddenNode => write!(f, "hidden node occurred outside of a case combinator"),
            Error::InvalidJet => write!(f, "unrecognized jet"),
            Error::Natural(ref e) => e.fmt(f),
            Error::NotInCanonicalOrder => f.write_str("program not in canonical order"),
            Error::SharingNotMaximal => f.write_str("Decoded programs must have maximal sharing"),
            Error::Type(ref e) => fmt::Display::fmt(e, f),
        }
    }
}

impl error::Error for Error {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match *self {
            Error::BitIter(ref e) => Some(e),
            Error::BothChildrenHidden => None,
            Error::EndOfStream => None,
            Error::HiddenNode => None,
            Error::InvalidJet => None,
            Error::Natural(ref e) => Some(e),
            Error::NotInCanonicalOrder => None,
            Error::SharingNotMaximal => None,
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
    Word(Word),
}

impl<J: Jet> DagLike for (usize, &'_ [DecodeNode<J>]) {
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

pub fn decode_expression<'brand, I: Iterator<Item = u8>, J: Jet>(
    ctx: &types::Context<'brand>,
    bits: &mut BitIter<I>,
) -> Result<ArcNode<'brand, J>, Error> {
    enum Converted<'brand, J: Jet> {
        Node(ArcNode<'brand, J>),
        Hidden(Cmr),
    }
    use Converted::{Hidden, Node};
    impl<'brand, J: Jet> Converted<'brand, J> {
        fn get(&self) -> Result<&ArcNode<'brand, J>, Error> {
            match self {
                Node(arc) => Ok(arc),
                Hidden(_) => Err(Error::HiddenNode),
            }
        }
    }

    let len = bits.read_natural::<usize>(None)?;
    assert_ne!(len, 0, "impossible to encode 0 in Simplicity");

    let mut nodes = Vec::with_capacity(cmp::min(len, 10_000));
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
            DecodeNode::Unit => Node(ArcNode::unit(ctx)),
            DecodeNode::Iden => Node(ArcNode::iden(ctx)),
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
            DecodeNode::Witness => Node(ArcNode::witness(ctx, None)),
            DecodeNode::Fail(entropy) => Node(ArcNode::fail(ctx, entropy)),
            DecodeNode::Hidden(cmr) => {
                if !hidden_set.insert(cmr) {
                    return Err(Error::SharingNotMaximal);
                }
                Hidden(cmr)
            }
            DecodeNode::Jet(j) => Node(ArcNode::jet(ctx, j)),
            DecodeNode::Word(ref w) => Node(ArcNode::const_word(ctx, w.shallow_clone())),
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
            let n = bits.read_natural(Some(32))?;
            let word = Word::from_bits(bits, n - 1)?;
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
        types::Context::with_context(|ctx| {
            decode_expression::<_, Core>(&ctx, &mut iter).unwrap();
        });
        // ...but NOT as a CommitNode
        let iter = BitIter::from(&justjet[..]);
        CommitNode::<Core>::decode(iter).unwrap_err();
        // ...or as a RedeemNode
        let iter = BitIter::from(&justjet[..]);
        RedeemNode::<Core>::decode(iter, BitIter::from(&[][..])).unwrap_err();
    }

    #[test]
    fn decode_fixed_natural() {
        let tries: Vec<(usize, usize, &[u8])> = vec![
            (1, 1, &[0b00000000]),
            (2, 3, &[0b10000000]),
            (3, 3, &[0b10100000]),
            (4, 6, &[0b11_000000]),
            (5, 6, &[0b11_000100]),
            (6, 6, &[0b11_001000]),
            (7, 6, &[0b11_001100]),
            (8, 7, &[0b110_10000]),
            (15, 7, &[0b110_11110]),
            (16, 11, &[0b11100000, 0b00000000]),
            // 31
            (31, 11, &[0b11100001, 0b11100000]),
            // 32
            (32, 12, &[0b11100010, 0b00000000]),
            // 2^15
            (32768, 23, &[0b11101111, 0b00000000, 0b00000000]),
            (65535, 23, &[0b11101111, 0b11111111, 0b11111110]),
            (65536, 28, &[0b11110000, 0b00000000, 0b00000000, 0b00000000]),
            (
                u32::MAX as usize - 1, // cast ok, in unit test
                43,
                &[
                    0b11110000, 0b11111111, 0b11111111, 0b11111111, 0b11111111, 0b11000000,
                ],
            ),
            (
                u32::MAX as usize, // cast ok, in unit test
                43,
                &[
                    0b11110000, 0b11111111, 0b11111111, 0b11111111, 0b11111111, 0b11100000,
                ],
            ),
        ];

        for (natural, len, bitvec) in tries {
            let mut iter = BitIter::new(bitvec.iter().copied());

            // Truncating the iterator causes a clean failure.
            let mut truncated = BitIter::new(bitvec.iter().copied().take(bitvec.len() - 1));
            assert_eq!(
                truncated.read_natural::<usize>(None),
                Err(DecodeNaturalError::EndOfStream(
                    crate::EarlyEndOfStreamError
                )),
            );

            // Test decoding under various bounds
            assert_eq!(iter.clone().read_natural(None), Ok(natural),);
            assert_eq!(iter.clone().read_natural::<u64>(None), Ok(natural as u64),);

            assert_eq!(iter.clone().read_natural(Some(natural)), Ok(natural),);
            assert_eq!(iter.clone().read_natural(Some(natural + 1)), Ok(natural),);
            assert_eq!(
                iter.clone().read_natural(Some(natural - 1)),
                Err(DecodeNaturalError::BadIndex {
                    got: natural,
                    max: natural - 1
                }),
            );
            assert_eq!(
                iter.clone().read_natural(Some(0u64)),
                Err(DecodeNaturalError::BadIndex {
                    got: natural,
                    max: 0
                }),
            );

            // Try decoding as a small type.
            if let Ok(natural) = u16::try_from(natural) {
                assert_eq!(iter.read_natural::<u16>(None), Ok(natural),);
            } else {
                assert_eq!(
                    iter.read_natural::<u16>(None),
                    Err(DecodeNaturalError::Overflow),
                );
            }
            // Attempt to re-encode.
            let mut sink = Vec::<u8>::new();

            let mut w = BitWriter::from(&mut sink);
            encode::encode_natural(natural, &mut w).expect("encoding to vector");
            w.flush_all().expect("flushing");
            assert_eq!(w.n_total_written(), len);

            assert_eq!(sink, bitvec);
        }

        // Test u32::MAX + 1 separately. This should always return an overflow and
        // never succeed or panic. Just hammer it with a bunch of different types
        // and call patterns.
        let iter = BitIter::new(
            [
                0b11110001, 0b00000000, 0b00000000, 0b00000000, 0b00000000, 0b00000000,
            ]
            .into_iter(),
        );
        assert_eq!(
            iter.clone().read_natural::<usize>(None),
            Err(DecodeNaturalError::Overflow),
        );
        assert_eq!(
            iter.clone().read_natural::<u16>(None),
            Err(DecodeNaturalError::Overflow),
        );
        assert_eq!(
            iter.clone().read_natural::<i32>(None),
            Err(DecodeNaturalError::Overflow),
        );
        assert_eq!(
            iter.clone().read_natural::<u32>(None),
            Err(DecodeNaturalError::Overflow),
        );
        assert_eq!(
            iter.clone().read_natural::<u64>(None),
            Err(DecodeNaturalError::Overflow),
        );
        assert_eq!(
            iter.clone().read_natural(Some(0u8)),
            Err(DecodeNaturalError::Overflow),
        );
        assert_eq!(
            iter.clone().read_natural(Some(0i32)),
            Err(DecodeNaturalError::Overflow),
        );
        assert_eq!(
            iter.clone().read_natural(Some(0u32)),
            Err(DecodeNaturalError::Overflow),
        );
    }
}
