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
use crate::core::iter::DagIterable;
use crate::core::types::Type;
use crate::core::{iter, Value};
use crate::decode::WitnessDecoder;
use crate::jet::Jet;
use crate::merkle::cmr::Cmr;
use crate::merkle::imr::Imr;
use crate::{decode, encode, impl_ref_wrapper, sharing, Error};
use std::rc::Rc;
use std::sync::Arc;
use std::{fmt, io};

/// Underlying combinator of a [`RedeemNode`].
///
/// # See
/// [`crate::core::commit::CommitNodeInner`]
#[derive(Debug)]
pub enum RedeemNodeInner<App: Jet> {
    /// Identity
    Iden,
    /// Unit constant
    Unit,
    /// Left injection of some child
    InjL(Rc<RedeemNode<App>>),
    /// Right injection of some child
    InjR(Rc<RedeemNode<App>>),
    /// Take of some child
    Take(Rc<RedeemNode<App>>),
    /// Drop of some child
    Drop(Rc<RedeemNode<App>>),
    /// Composition of a left and right child
    Comp(Rc<RedeemNode<App>>, Rc<RedeemNode<App>>),
    /// Case of a left and right child
    Case(Rc<RedeemNode<App>>, Rc<RedeemNode<App>>),
    /// Left assertion of a left and right child.
    AssertL(Rc<RedeemNode<App>>, Rc<RedeemNode<App>>),
    /// Right assertion of a left and right child.
    AssertR(Rc<RedeemNode<App>>, Rc<RedeemNode<App>>),
    /// Pair of a left and right child
    Pair(Rc<RedeemNode<App>>, Rc<RedeemNode<App>>),
    /// Disconnect of a left and right child
    Disconnect(Rc<RedeemNode<App>>, Rc<RedeemNode<App>>),
    /// Witness data
    Witness(Value),
    /// Universal fail
    Fail(Cmr, Cmr),
    /// Hidden CMR
    Hidden(Cmr),
    /// Application jet
    Jet(App),
}

impl<App: Jet> RedeemNodeInner<App> {
    /// Return the left child of the node, if there is such a child.
    pub fn get_left(&self) -> Option<&RedeemNode<App>> {
        match self {
            RedeemNodeInner::Iden
            | RedeemNodeInner::Unit
            | RedeemNodeInner::Witness(..)
            | RedeemNodeInner::Fail(..)
            | RedeemNodeInner::Hidden(..)
            | RedeemNodeInner::Jet(..) => None,
            RedeemNodeInner::InjL(l)
            | RedeemNodeInner::InjR(l)
            | RedeemNodeInner::Take(l)
            | RedeemNodeInner::Drop(l)
            | RedeemNodeInner::Comp(l, _)
            | RedeemNodeInner::Case(l, _)
            | RedeemNodeInner::AssertL(l, _)
            | RedeemNodeInner::AssertR(l, _)
            | RedeemNodeInner::Pair(l, _)
            | RedeemNodeInner::Disconnect(l, _) => Some(l),
        }
    }

    /// Return the right child of the node, if there is such a child.
    pub fn get_right(&self) -> Option<&RedeemNode<App>> {
        match self {
            RedeemNodeInner::Iden
            | RedeemNodeInner::Unit
            | RedeemNodeInner::Witness(..)
            | RedeemNodeInner::Fail(..)
            | RedeemNodeInner::Hidden(..)
            | RedeemNodeInner::Jet(..)
            | RedeemNodeInner::InjL(_)
            | RedeemNodeInner::InjR(_)
            | RedeemNodeInner::Take(_)
            | RedeemNodeInner::Drop(_) => None,
            RedeemNodeInner::Comp(_, r)
            | RedeemNodeInner::Case(_, r)
            | RedeemNodeInner::AssertL(_, r)
            | RedeemNodeInner::AssertR(_, r)
            | RedeemNodeInner::Pair(_, r)
            | RedeemNodeInner::Disconnect(_, r) => Some(r),
        }
    }
}

impl<App: Jet> fmt::Display for RedeemNodeInner<App> {
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
        }
    }
}

/// Source and target type of a node
#[derive(Debug)]
pub struct NodeType {
    /// Source type of the node
    pub(crate) source: Arc<Type>,
    /// Target type of the node
    pub(crate) target: Arc<Type>,
}

impl fmt::Display for NodeType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} → {}", self.source, self.target)
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
pub struct RedeemNode<App: Jet> {
    /// Underlying combinator of the node
    pub inner: RedeemNodeInner<App>,
    /// Commitment Merkle root of the node
    pub cmr: Cmr,
    /// Identity Merkle root of the node
    pub imr: Imr,
    /// Type of the node
    pub ty: NodeType,
    /// Bounds for the node during execution on the Bit Machine
    pub bounds: NodeBounds,
}

impl<App: Jet> RedeemNode<App> {
    /// Return the left child of the node, if there is such a child.
    pub fn get_left(&self) -> Option<&Self> {
        self.inner.get_left()
    }

    /// Return the right child of the node, if there is such a child.
    pub fn get_right(&self) -> Option<&Self> {
        self.inner.get_right()
    }

    /// Return an iterator over the types of values that make up a valid witness for the program.
    pub fn get_witness_types(&self) -> impl Iterator<Item = &Type> {
        RefWrapper(self).iter_post_order().filter_map(|node| {
            if let RedeemNodeInner::Witness(_) = &node.0.inner {
                Some(node.0.ty.target.as_ref())
            } else {
                None
            }
        })
    }

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

impl<App: Jet> fmt::Display for RedeemNode<App> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        RefWrapper(self).display(
            f,
            |node, f| fmt::Display::fmt(&node.0.inner, f),
            |node, f| write!(f, ": {}", node.0.ty),
        )
    }
}

/// Wrapper of references to [`RedeemNode`].
/// Zero-cost implementation of `Copy`, `Eq` and `Hash` via pointer equality.
#[derive(Debug)]
pub struct RefWrapper<'a, App: Jet>(pub &'a RedeemNode<App>);

impl_ref_wrapper!(RefWrapper);
