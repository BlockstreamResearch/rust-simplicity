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

use crate::analysis::NodeBounds;
use crate::dag::{DagLike, InternalSharing, NoSharing, PostOrderIter};
use crate::jet::Jet;
use crate::merkle::amr::Amr;
use crate::merkle::cmr::Cmr;
use crate::merkle::imr::Imr;
use crate::node;
use crate::types::{self, arrow::FinalArrow};
use crate::{BitIter, BitWriter, Error, FailEntropy, Value};
use std::rc::Rc;
use std::sync::Arc;
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
    AssertL(Rc<RedeemNode<J>>, Cmr),
    /// Right assertion of a left and right child.
    AssertR(Cmr, Rc<RedeemNode<J>>),
    /// Pair of a left and right child
    Pair(Rc<RedeemNode<J>>, Rc<RedeemNode<J>>),
    /// Disconnect of a left and right child
    Disconnect(Rc<RedeemNode<J>>, Rc<RedeemNode<J>>),
    /// Witness data
    Witness(Value),
    /// Universal fail
    Fail(FailEntropy),
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
            RedeemNodeInner::Jet(jet) => write!(f, "jet({})", jet),
            RedeemNodeInner::Word(w) => write!(f, "word({})", w),
        }
    }
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
    /// [will be removed] create a CommitNode from a newfangled `node::ConstructNode`
    pub fn from_node(node: &node::RedeemNode<J>) -> Rc<Self> {
        let mut converted = vec![];
        for data in node.post_order_iter::<InternalSharing>() {
            let left = data.left_index.map(|idx| Rc::clone(&converted[idx]));
            let right = data.right_index.map(|idx| Rc::clone(&converted[idx]));
            let new_node = Rc::new(RedeemNode {
                inner: match data.node.inner() {
                    node::Inner::Iden => RedeemNodeInner::Iden,
                    node::Inner::Unit => RedeemNodeInner::Unit,
                    node::Inner::InjL(..) => RedeemNodeInner::InjL(left.unwrap()),
                    node::Inner::InjR(..) => RedeemNodeInner::InjR(left.unwrap()),
                    node::Inner::Take(..) => RedeemNodeInner::Take(left.unwrap()),
                    node::Inner::Drop(..) => RedeemNodeInner::Drop(left.unwrap()),
                    node::Inner::Comp(..) => RedeemNodeInner::Comp(left.unwrap(), right.unwrap()),
                    node::Inner::Case(..) => RedeemNodeInner::Case(left.unwrap(), right.unwrap()),
                    node::Inner::AssertL(_, cmr) => RedeemNodeInner::AssertL(left.unwrap(), *cmr),
                    node::Inner::AssertR(cmr, _) => RedeemNodeInner::AssertR(*cmr, left.unwrap()),
                    node::Inner::Pair(..) => RedeemNodeInner::Pair(left.unwrap(), right.unwrap()),
                    node::Inner::Disconnect(..) => {
                        RedeemNodeInner::Disconnect(left.unwrap(), right.unwrap())
                    }
                    node::Inner::Witness(val) => RedeemNodeInner::Witness(val.as_ref().clone()),
                    node::Inner::Jet(j) => RedeemNodeInner::Jet(*j),
                    node::Inner::Word(w) => RedeemNodeInner::Word(w.as_ref().clone()),
                    node::Inner::Fail(entropy) => RedeemNodeInner::Fail(*entropy),
                },
                cmr: data.node.cmr(),
                imr: data.node.imr(),
                amr: data.node.amr(),
                ty: data.node.arrow().shallow_clone(),
                bounds: data.node.bounds(),
            });
            converted.push(new_node);
        }
        converted.pop().unwrap()
    }

    /// [will be removed] create a CommitNode from a newfangled `node::ConstructNode`
    pub fn to_node(&self) -> Arc<node::RedeemNode<J>> {
        let mut converted = Vec::<Arc<node::RedeemNode<J>>>::new();
        for data in self.post_order_iter::<InternalSharing>() {
            let left = data.left_index.map(|idx| Arc::clone(&converted[idx]));
            let right = data.right_index.map(|idx| Arc::clone(&converted[idx]));

            let left_data = data.left_index.map(|idx| converted[idx].cached_data());
            let right_data = data.right_index.map(|idx| converted[idx].cached_data());
            let new_node = Arc::new(
                node::RedeemNode::new(
                    data.node.ty.shallow_clone(),
                    match data.node.inner {
                        RedeemNodeInner::Iden => node::Inner::Iden,
                        RedeemNodeInner::Unit => node::Inner::Unit,
                        RedeemNodeInner::InjL(..) => node::Inner::InjL(left.unwrap()),
                        RedeemNodeInner::InjR(..) => node::Inner::InjR(left.unwrap()),
                        RedeemNodeInner::Take(..) => node::Inner::Take(left.unwrap()),
                        RedeemNodeInner::Drop(..) => node::Inner::Drop(left.unwrap()),
                        RedeemNodeInner::Comp(..) => {
                            node::Inner::Comp(left.unwrap(), right.unwrap())
                        }
                        RedeemNodeInner::Case(..) => {
                            node::Inner::Case(left.unwrap(), right.unwrap())
                        }
                        RedeemNodeInner::AssertL(_, cmr) => {
                            node::Inner::AssertL(left.unwrap(), cmr)
                        }
                        RedeemNodeInner::AssertR(cmr, _) => {
                            node::Inner::AssertR(cmr, left.unwrap())
                        }
                        RedeemNodeInner::Pair(..) => {
                            node::Inner::Pair(left.unwrap(), right.unwrap())
                        }
                        RedeemNodeInner::Disconnect(..) => {
                            node::Inner::Disconnect(left.unwrap(), right.unwrap())
                        }
                        RedeemNodeInner::Witness(ref wit) => {
                            node::Inner::Witness(Arc::new(wit.clone()))
                        }
                        RedeemNodeInner::Jet(j) => node::Inner::Jet(j),
                        RedeemNodeInner::Word(ref w) => node::Inner::Word(Arc::new(w.clone())),
                        RedeemNodeInner::Fail(entropy) => node::Inner::Fail(entropy),
                    },
                    match data.node.inner {
                        RedeemNodeInner::Iden => node::Inner::Iden,
                        RedeemNodeInner::Unit => node::Inner::Unit,
                        RedeemNodeInner::InjL(..) => node::Inner::InjL(left_data.unwrap()),
                        RedeemNodeInner::InjR(..) => node::Inner::InjR(left_data.unwrap()),
                        RedeemNodeInner::Take(..) => node::Inner::Take(left_data.unwrap()),
                        RedeemNodeInner::Drop(..) => node::Inner::Drop(left_data.unwrap()),
                        RedeemNodeInner::Comp(..) => {
                            node::Inner::Comp(left_data.unwrap(), right_data.unwrap())
                        }
                        RedeemNodeInner::Case(..) => {
                            node::Inner::Case(left_data.unwrap(), right_data.unwrap())
                        }
                        RedeemNodeInner::AssertL(_, cmr) => {
                            node::Inner::AssertL(left_data.unwrap(), cmr)
                        }
                        RedeemNodeInner::AssertR(cmr, _) => {
                            node::Inner::AssertR(cmr, left_data.unwrap())
                        }
                        RedeemNodeInner::Pair(..) => {
                            node::Inner::Pair(left_data.unwrap(), right_data.unwrap())
                        }
                        RedeemNodeInner::Disconnect(..) => {
                            node::Inner::Disconnect(left_data.unwrap(), right_data.unwrap())
                        }
                        RedeemNodeInner::Witness(ref wit) => {
                            node::Inner::Witness(Arc::new(wit.clone()))
                        }
                        RedeemNodeInner::Jet(j) => node::Inner::Jet(j),
                        RedeemNodeInner::Word(ref w) => node::Inner::Word(Arc::new(w.clone())),
                        RedeemNodeInner::Fail(entropy) => node::Inner::Fail(entropy),
                    },
                )
                .expect("converting between redeemnodes"),
            );
            converted.push(new_node);
        }
        converted.pop().unwrap()
    }

    /// Return the left child of the node, if there is such a child.
    pub fn get_left(&self) -> Option<&Self> {
        <&Self as DagLike>::left_child(&self)
    }

    /// Return the right child of the node, if there is such a child.
    pub fn get_right(&self) -> Option<&Self> {
        <&Self as DagLike>::right_child(&self)
    }

    /// Return an iterator over the unshared nodes of the program
    pub fn iter(&self) -> PostOrderIter<&Self, NoSharing> {
        <&Self as DagLike>::post_order_iter(self)
    }

    /// Return an iterator over the nodes of the program, returning
    /// refcounted pointers to each node. Each refcounted pointer
    /// is returned only once.
    pub fn rc_iter(self: Rc<Self>) -> PostOrderIter<Rc<Self>, InternalSharing> {
        <Rc<Self> as DagLike>::post_order_iter(self)
    }

    // FIXME: Compute length without iterating over entire DAG?
    /// Return the number of unshared nodes in the program
    #[allow(clippy::len_without_is_empty)]
    pub fn len(&self) -> usize {
        self.iter().count()
    }

    /// Return an iterator over the types of values that make up a valid witness for the program.
    pub fn get_witness_types(&self) -> impl Iterator<Item = &types::Final> {
        self.iter().filter_map(|data| {
            if let RedeemNodeInner::Witness(_) = &data.node.inner {
                Some(data.node.ty.target.as_ref())
            } else {
                None
            }
        })
    }

    /// Decode a Simplicity program from bits, including the witness data.
    pub fn decode<I: Iterator<Item = u8>>(bits: &mut BitIter<I>) -> Result<Rc<Self>, Error> {
        Ok(Self::from_node(node::RedeemNode::decode(bits)?.as_ref()))
    }

    /// Encode a Simplicity program to bits, including the witness data.
    pub fn encode<W: io::Write>(&self, w: &mut BitWriter<W>) -> io::Result<usize> {
        self.to_node().encode(w)
    }

    /// Encode a Simplicity program to a vector of bytes, including the witness data.
    pub fn encode_to_vec(&self) -> Vec<u8> {
        let mut program_and_witness_bytes = Vec::<u8>::new();
        let mut writer = BitWriter::new(&mut program_and_witness_bytes);
        self.encode(&mut writer)
            .expect("write to vector never fails");
        writer.flush_all().expect("flushing vector never fails");
        debug_assert!(!program_and_witness_bytes.is_empty());

        program_and_witness_bytes
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

#[cfg(test)]
mod tests {
    use std::iter;

    use super::*;

    use crate::bit_encoding::decode::decode_expression;
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
        let eqwits_prog = decode_expression::<_, Core>(&mut iter).unwrap();
        let eqwits_prog = crate::CommitNode::from_node(&eqwits_prog);

        let mut witness_iter = iter::repeat(Value::u32(0xDEADBEEF));
        // Generally when we are manually adding witnesses we want to unshare them so that
        // we have a choice to add distinct witnesses to every spot. But in this case we
        // are providing the same witness for every spot, so it really doesn't matter, so
        // try both cases.
        for &unshare_witnesses in &[false, true] {
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

    #[test]
    fn unshared_child() {
        // # id1 and id2 should be shared, but are not!
        // id1 = iden          :: A -> A # cmr dbfefcfc...
        // id2 = iden          :: A -> A # cmr dbfefcfc...
        // cp3 = comp id1 id2  :: A -> A # cmr c1ae55b5...
        // main = comp cp3 cp3 :: A -> A # cmr 314e2879...
        let bad = [0xc1, 0x08, 0x04, 0x00, 0x00, 0x74, 0x74, 0x74];
        let mut iter = BitIter::from(&bad[..]);
        let err = RedeemNode::<crate::jet::Core>::decode(&mut iter).unwrap_err();
        assert!(matches!(err, crate::Error::SharingNotMaximal));
    }

    #[test]
    fn witness_consumed() {
        // "main = unit", but with a witness attached. Found by fuzzer.
        let badwit = vec![0x27, 0x00];
        let mut iter = BitIter::from(&badwit[..]);
        if let Err(Error::InconsistentWitnessLength) =
            RedeemNode::<crate::jet::Core>::decode(&mut iter)
        {
            // ok
        } else {
            panic!("accepted program with bad witness length")
        }
    }
}
