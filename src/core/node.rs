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
use crate::core::iter::DagIterable;
use crate::core::types::Type;
use crate::core::{iter, Value};
use crate::decode::WitnessDecoder;
use crate::encode::BitWriter;
use crate::jet::{Application, JetNode};
use crate::merkle::cmr::Cmr;
use crate::merkle::imr::Imr;
use crate::{decode, encode, impl_ref_wrapper, sharing, Error};
use std::io;
use std::rc::Rc;
use std::sync::Arc;

/// Underlying combinator of a [`Node`].
///
/// # See
/// - [`crate::core::commit::CommitNodeInner`]
#[derive(Debug)]
pub enum NodeInner<Witness, App: Application> {
    /// Identity
    Iden,
    /// Unit constant
    Unit,
    /// Left injection of some child
    InjL(Rc<Node<Witness, App>>),
    /// Right injection of some child
    InjR(Rc<Node<Witness, App>>),
    /// Take of some child
    Take(Rc<Node<Witness, App>>),
    /// Drop of some child
    Drop(Rc<Node<Witness, App>>),
    /// Composition of a left and right child
    Comp(Rc<Node<Witness, App>>, Rc<Node<Witness, App>>),
    /// Case of a left and right child
    Case(Rc<Node<Witness, App>>, Rc<Node<Witness, App>>),
    /// Left assertion of a left and right child.
    AssertL(Rc<Node<Witness, App>>, Rc<Node<Witness, App>>),
    /// Right assertion of a left and right child.
    AssertR(Rc<Node<Witness, App>>, Rc<Node<Witness, App>>),
    /// Pair of a left and right child
    Pair(Rc<Node<Witness, App>>, Rc<Node<Witness, App>>),
    /// Disconnect of a left and right child
    Disconnect(Rc<Node<Witness, App>>, Rc<Node<Witness, App>>),
    /// Witness data
    Witness(Witness),
    /// Universal fail
    Fail(Cmr, Cmr),
    /// Hidden CMR
    Hidden(Cmr),
    /// Application jet
    Jet(&'static JetNode<App>),
}

/// Source and target type of a node
#[derive(Debug)]
pub struct NodeType {
    /// Source type of the node
    pub(crate) source: Arc<Type>,
    /// Target type of the node
    pub(crate) target: Arc<Type>,
}

/// Bounds on the resources required by a node during execution on the Bit Machine
#[derive(Debug)]
pub struct NodeBounds {
    /// Upper bound on the required number of cells
    pub extra_cells: usize,
    /// Upper bound on the required number of frames
    pub frame_count: usize,
}

/// Root node of a Simplicity DAG for some application.
/// The DAG contains full metadata, including the witness, for redeeming it.
///
/// # See
/// - [`crate::core::CommitNode`]
#[derive(Debug)]
pub struct Node<Witness, App: Application> {
    /// Underlying combinator of the node
    pub inner: NodeInner<Witness, App>,
    /// Commitment Merkle root of the node
    pub cmr: Cmr,
    /// Identity Merkle root of the node
    pub imr: Imr,
    /// Type of the node
    pub ty: NodeType,
    /// Bounds for the node during execution on the Bit Machine
    pub bounds: NodeBounds,
}

impl<Witness, App: Application> Node<Witness, App> {
    /// Return the left child of the node, if there is such a child.
    pub fn get_left(&self) -> Option<&Self> {
        match &self.inner {
            NodeInner::Iden
            | NodeInner::Unit
            | NodeInner::Witness(_)
            | NodeInner::Fail(_, _)
            | NodeInner::Hidden(_)
            | NodeInner::Jet(_) => None,
            NodeInner::InjL(l)
            | NodeInner::InjR(l)
            | NodeInner::Take(l)
            | NodeInner::Drop(l)
            | NodeInner::Comp(l, _)
            | NodeInner::Case(l, _)
            | NodeInner::AssertL(l, _)
            | NodeInner::AssertR(l, _)
            | NodeInner::Pair(l, _)
            | NodeInner::Disconnect(l, _) => Some(l),
        }
    }

    /// Return the right child of the node, if there is such a child.
    pub fn get_right(&self) -> Option<&Self> {
        match &self.inner {
            NodeInner::Iden
            | NodeInner::Unit
            | NodeInner::Witness(_)
            | NodeInner::Fail(_, _)
            | NodeInner::Hidden(_)
            | NodeInner::Jet(_)
            | NodeInner::InjL(_)
            | NodeInner::InjR(_)
            | NodeInner::Take(_)
            | NodeInner::Drop(_) => None,
            NodeInner::Comp(_, r)
            | NodeInner::Case(_, r)
            | NodeInner::AssertL(_, r)
            | NodeInner::AssertR(_, r)
            | NodeInner::Pair(_, r)
            | NodeInner::Disconnect(_, r) => Some(r),
        }
    }

    /// Return an iterator over the types of values that make up a valid witness for the program.
    pub fn get_witness_types(&self) -> impl Iterator<Item = &Type> {
        RefWrapper(self).iter_post_order().filter_map(|node| {
            if let NodeInner::Witness(_) = &node.0.inner {
                Some(node.0.ty.target.as_ref())
            } else {
                None
            }
        })
    }
}

impl<App: Application> Node<Value, App> {
    /// Decode a Simplicity program from bits, including the witness data.
    pub fn decode<I: Iterator<Item = u8>>(bits: &mut BitIter<I>) -> Result<Rc<Self>, Error> {
        let commit = decode::decode_program_exact_witness(bits)?;
        let witness = WitnessDecoder::new(bits)?;
        let program = commit.finalize(witness)?;

        if sharing::check_maximal_sharing(RefWrapper(&program).iter_post_order()) {
            Ok(program)
        } else {
            Err(Error::SharingNotMaximal)
        }
    }

    /// Encode a Simplicity program to bits, including the witness data.
    pub fn encode<W: io::Write>(&self, w: &mut BitWriter<W>) -> io::Result<usize> {
        let program = RefWrapper(self).iter_post_order();
        let program_bits = encode::encode_program(program.clone(), w)?;
        let witness_bits = encode::encode_witness(iter::into_witness(program), w)?;
        Ok(program_bits + witness_bits)
    }
}

/// Wrapper of references to [`Node`].
#[derive(Debug)]
pub struct RefWrapper<'a, Witness, App: Application>(pub &'a Node<Witness, App>);

impl_ref_wrapper!(RefWrapper);
