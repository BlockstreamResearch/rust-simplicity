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

use crate::bititer::BitIter;
use crate::bitwriter::BitWriter;
use crate::core::Value;
use crate::dag::DagLike;
use crate::decode::WitnessDecoder;
use crate::jet::Jet;
use crate::merkle::amr::Amr;
use crate::merkle::cmr::Cmr;
use crate::merkle::imr::Imr;
use crate::types::{self, arrow::FinalArrow};
use crate::{encode, impl_ref_wrapper, sharing, Error};
use std::rc::Rc;
use std::{fmt, io};

/// Underlying combinator of a [`RedeemNode`].
///
/// # See
/// [`crate::core::commit::CommitNodeInner`]
#[derive(Debug)]
pub enum RedeemNodeInner<J: Jet> {
    /// Identity
    Iden,
    /// Unit constant
    Unit,
    /// Left injection of some child
    InjL(Rc<RedeemNode<J>>),
    /// Right injection of some child
    InjR(Rc<RedeemNode<J>>),
    /// Take of some child
    Take(Rc<RedeemNode<J>>),
    /// Drop of some child
    Drop(Rc<RedeemNode<J>>),
    /// Composition of a left and right child
    Comp(Rc<RedeemNode<J>>, Rc<RedeemNode<J>>),
    /// Case of a left and right child
    Case(Rc<RedeemNode<J>>, Rc<RedeemNode<J>>),
    /// Left assertion of a left and right child.
    AssertL(Rc<RedeemNode<J>>, Rc<RedeemNode<J>>),
    /// Right assertion of a left and right child.
    AssertR(Rc<RedeemNode<J>>, Rc<RedeemNode<J>>),
    /// Pair of a left and right child
    Pair(Rc<RedeemNode<J>>, Rc<RedeemNode<J>>),
    /// Disconnect of a left and right child
    Disconnect(Rc<RedeemNode<J>>, Rc<RedeemNode<J>>),
    /// Witness data
    Witness(Value),
    /// Universal fail
    Fail(Cmr, Cmr),
    /// Hidden CMR
    Hidden(Cmr),
    /// Application jet
    Jet(J),
    /// Constant word
    Word(Value),
}

impl<J: Jet> fmt::Display for RedeemNodeInner<J> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RedeemNodeInner::Iden => f.write_str("iden"),
            RedeemNodeInner::Unit => f.write_str("unit"),
            RedeemNodeInner::InjL(_) => f.write_str("injl"),
            RedeemNodeInner::InjR(_) => f.write_str("injr"),
            RedeemNodeInner::Take(_) => f.write_str("take"),
            RedeemNodeInner::Drop(_) => f.write_str("drop"),
            RedeemNodeInner::Comp(_, _) => f.write_str("comp"),
            RedeemNodeInner::Case(_, _) => f.write_str("case"),
            RedeemNodeInner::AssertL(_, _) => f.write_str("assertl"),
            RedeemNodeInner::AssertR(_, _) => f.write_str("assertr"),
            RedeemNodeInner::Pair(_, _) => f.write_str("pair"),
            RedeemNodeInner::Disconnect(_, _) => f.write_str("disconnect"),
            RedeemNodeInner::Witness(..) => f.write_str("witness"),
            RedeemNodeInner::Fail(..) => f.write_str("fail"),
            RedeemNodeInner::Hidden(..) => f.write_str("hidden"),
            RedeemNodeInner::Jet(jet) => write!(f, "jet({})", jet),
            RedeemNodeInner::Word(w) => write!(f, "word({})", w),
        }
    }
}

/// Bounds on the resources required by a node during execution on the Bit Machine
#[derive(Debug)]
pub struct NodeBounds {
    /// Upper bound on the required number of cells (bits).
    /// The root additionally requires the bit width of its source and target type (input, output)
    pub extra_cells: usize,
    /// Upper bound on the required number of frames (sum of read and write frames).
    /// The root additionally requires two frames (input, output)
    pub extra_frames: usize,
}

/// Root node of a Simplicity DAG for some application.
/// The DAG contains full metadata, including the witness, for redeeming it.
///
/// # See
/// [`crate::core::CommitNode`]
#[derive(Debug)]
pub struct RedeemNode<J: Jet> {
    /// Underlying combinator of the node
    pub inner: RedeemNodeInner<J>,
    /// Commitment Merkle root of the node
    pub cmr: Cmr,
    /// Identity Merkle root of the node
    pub imr: Imr,
    /// Annotated Merkle root of the node
    pub amr: Amr,
    /// Type of the node
    pub ty: FinalArrow,
    /// Bounds for the node during execution on the Bit Machine
    pub bounds: NodeBounds,
}

impl<J: Jet> RedeemNode<J> {
    /// Return the left child of the node, if there is such a child.
    pub fn get_left(&self) -> Option<&Self> {
        <&Self as DagLike>::left_child(&self)
    }

    /// Return the right child of the node, if there is such a child.
    pub fn get_right(&self) -> Option<&Self> {
        <&Self as DagLike>::right_child(&self)
    }

    /// Return an iterator over the unshared nodes of the program
    pub fn iter(&self) -> crate::dag::PostOrderIter<&Self> {
        <&Self as DagLike>::post_order_iter(self)
    }

    // FIXME: Compute length without iterating over entire DAG?
    /// Return the number of unshared nodes in the program
    #[allow(clippy::len_without_is_empty)]
    pub fn len(&self) -> usize {
        self.iter().count()
    }

    /// Return an iterator over the types of values that make up a valid witness for the program.
    pub fn get_witness_types(&self) -> impl Iterator<Item = &types::Final> {
        self.iter().filter_map(|node| {
            if let RedeemNodeInner::Witness(_) = &node.inner {
                Some(node.ty.target.as_ref())
            } else {
                None
            }
        })
    }

    /// Decode a Simplicity program from bits, including the witness data.
    pub fn decode<I: Iterator<Item = u8>>(bits: &mut BitIter<I>) -> Result<Rc<Self>, Error> {
        let commit = crate::decode_program(bits)?;
        let witness = WitnessDecoder::new(bits)?;
        let program = commit.finalize(witness, false)?;

        if sharing::check_maximal_sharing(program.iter()) {
            Ok(program)
        } else {
            Err(Error::SharingNotMaximal)
        }
    }

    /// Encode a Simplicity program to bits, including the witness data.
    pub fn encode<W: io::Write>(&self, w: &mut BitWriter<W>) -> io::Result<usize> {
        let program_bits = encode::encode_program(self.iter(), w)?;
        let witness_bits = encode::encode_witness(self.iter().into_deduped_witnesses(), w)?;
        w.flush_all()?;
        Ok(program_bits + witness_bits)
    }
}

impl<J: Jet> fmt::Display for RedeemNode<J> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.iter().into_display(
            f,
            |node, f| fmt::Display::fmt(&node.inner, f),
            |node, f| write!(f, ": {}", node.ty),
        )
    }
}

/// Wrapper of references to [`RedeemNode`].
/// Zero-cost implementation of `Copy`, `Eq` and `Hash` via pointer equality.
#[derive(Debug)]
pub struct RefWrapper<'a, J: Jet>(pub &'a RedeemNode<J>);

impl_ref_wrapper!(RefWrapper);

#[cfg(test)]
mod tests {
    use std::iter;

    use super::*;

    use crate::bititer::BitIter;
    use crate::bitwriter::BitWriter;
    use crate::core::Value;
    use crate::jet::Core;

    #[test]
    fn encode_shared_witnesses() {
        // # Program code:
        // wit1 = witness :: 1 -> 2^32
        // wit2 = witness :: 1 -> 2^32
        //
        // wits_are_equal = comp (pair wit1 wit2) jet_eq_32 :: 1 -> 2
        // main = comp wits_are_equal jet_verify            :: 1 -> 1
        let eqwits = vec![0xc9, 0xc4, 0x6d, 0xb8, 0x82, 0x30, 0x10];
        let mut iter = BitIter::from(&eqwits[..]);
        let eqwits_prog = crate::decode_program::<_, Core>(&mut iter).unwrap();

        let mut witness_iter = iter::repeat(Value::u32(0xDEADBEEF));
        // Generally when we are manually adding witnesses we want to unshare them so that
        // we have a choice to add distinct witnesses to every spot. But in this case we
        // are providing the same witness for every spot, so it really doesn't matter, so
        // try both cases.
        for unshare_witnesses in vec![false, true] {
            let eqwits_final = eqwits_prog
                .finalize(&mut witness_iter, unshare_witnesses)
                .unwrap();
            let mut output = vec![];
            let mut writer = BitWriter::new(&mut output);
            eqwits_final.encode(&mut writer).unwrap();

            assert_eq!(
                output,
                [0xc9, 0xc4, 0x6d, 0xb8, 0x82, 0x30, 0x11, 0xe2, 0x0d, 0xea, 0xdb, 0xee, 0xf0],
            );
        }
    }

    #[test]
    fn decode_shared_witnesses() {
        // This program is exactly the output from the `encode_shared_witnesses` test.
        // The point of this is to make sure that our witness-unsharing logic doesn't
        // get confused here and try to read two witnesses when there are only one.
        let eqwits = vec![
            0xc9, 0xc4, 0x6d, 0xb8, 0x82, 0x30, 0x11, 0xe2, 0x0d, 0xea, 0xdb, 0xee, 0xf0,
        ];
        let mut iter = BitIter::from(&eqwits[..]);
        RedeemNode::<crate::jet::Core>::decode(&mut iter).unwrap();
    }
}
