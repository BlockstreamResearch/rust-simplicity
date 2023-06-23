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
use crate::dag::{DagLike, InternalSharing};
use crate::jet::Jet;
use crate::merkle::amr::Amr;
use crate::merkle::cmr::Cmr;
use crate::merkle::imr::Imr;
use crate::node;
use crate::types::arrow::FinalArrow;
use crate::{FailEntropy, Value};
use std::fmt;
use std::rc::Rc;
use std::sync::Arc;

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
}

impl<J: Jet> fmt::Display for RedeemNode<J> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(&self.to_node(), f)
    }
}
