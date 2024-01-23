// SPDX-License-Identifier: CC0-1.0

//! Node Conversion
//!
//! This module defines a trait [`Conversion`] which is called by the
//! [`Node::convert`] method. The `Convert` trait is used to convert between
//! one node type and another, controlling several nuanced aspects of the
//! conversion. Specifically:
//!
//! 1. Cached data can be translated from the old type to the new one.
//! 2. Witness data can be provided (or translated from the old witness type,
//!    if it was nontrivial) to attach to witness nodes.
//! 3. For `case` nodes, the decision can be made to hide one of the children.
//!    In this case the `case` node is converted to an `AssertL` or `AssertR`
//!    node, depending which child was hidden.
//!

use crate::dag::PostOrderIterItem;
use crate::jet::Jet;
use crate::Value;

use super::{
    Commit, CommitNode, Inner, Marker, NoDisconnect, NoWitness, Node, Redeem, RedeemData,
    RedeemNode,
};

use std::sync::Arc;

/// A decision about which, if any, child branches of a `case` combinator to hide
/// during a [`Node::convert_hiding`] conversion
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Debug, Hash)]
pub enum Hide {
    Neither,
    Left,
    Right,
}

/// The primary trait controlling how a node conversion is done.
///
/// When a node is converted using [`Node::convert`], the original DAG rooted at
/// that node is traversed in post order. For each node, the following steps are
/// taken:
///
/// 1. First, [`Self::visit_node`] is called, before any other checks are
///    done. This happens regardless of the node's type or whether it is going
///    to be pruned.
///
///    This method is provided for convenience and does not affect the course
///    of the algorithm. It has a default implementation which does nothing.
///
/// 2. Then, if the node is a witness node, `Self::convert_witness` is called
///    to obtain witness data.
///
/// 3. If the node is a case node, [`Self::prune_case`] is called to decide
///    whether to prune either child of the node (turning the `case` into an
///    `assertl` or `assertr`). The default implementation hides neither.
///
/// 4. Finally, the node's data is passed to [`Self::convert_data`], whose job
///    it is to compute the cached data for the new node. For `case` combinators
///    where one child was pruned, `convert_data` will receive an `assertl` or
///    `assertl`, as appropriate, rather than a `case`.
///
/// If any method returns an error, then iteration is aborted immediately and
/// the error returned to the caller. If the converter would like to recover
/// from errors and/or accumulate multiple errors, it needs to do this by
/// tracking errors internally.
///
/// The finalization method will not return any errors except those returned by
/// methods on [`Converter`].
pub trait Converter<N: Marker, M: Marker> {
    /// The error type returned by the methods on this trait.
    type Error;

    /// This method is called on every node, to inform the `Converter` about the
    /// state of the iterator.
    ///
    /// No action needs to be taken. The default implementation does nothing.
    fn visit_node(&mut self, _data: &PostOrderIterItem<&Node<N>>) {}

    /// For witness nodes, this method is called first to attach witness data to
    /// the node.
    ///
    /// It takes the iteration data of the current node, as well as the current
    /// witness (which in a typical scenario will be an empty structure, but
    /// with custom node types may be a placeholder or other useful information)
    ///
    /// No typechecking or other sanity-checking is done on the returned value.
    /// It is the caller's responsibility to make sure that the provided witness
    /// actually matches the type of the combinator that it is being attached to.
    fn convert_witness(
        &mut self,
        data: &PostOrderIterItem<&Node<N>>,
        witness: &N::Witness,
    ) -> Result<M::Witness, Self::Error>;

    /// For disconnect nodes, this method is called first to attach a disconnected
    /// expression to the node.
    ///
    /// It takes the iteration data of the current node, as well as the current
    /// disconnected expression (which in a typical scenario will be an empty
    /// structure, but with custom node types may be a placeholder or other
    /// useful information)
    ///
    /// No typechecking or other sanity-checking is done on the returned expression.
    /// It is the caller's responsibility to make sure that the provided expression
    /// actually matches the type of the combinator that it is being attached to.
    fn convert_disconnect(
        &mut self,
        data: &PostOrderIterItem<&Node<N>>,
        maybe_converted: Option<&Arc<Node<M>>>,
        disconnect: &N::Disconnect,
    ) -> Result<M::Disconnect, Self::Error>;

    /// For case nodes, this method is called first to decide which, if any, children
    /// to prune.
    ///
    /// It takes the iteration data of the current node, as well as both of its already
    /// converted children. This method returns a hiding decision.
    ///
    /// The default implementation doesn't do any hiding.
    fn prune_case(
        &mut self,
        _data: &PostOrderIterItem<&Node<N>>,
        _left: &Arc<Node<M>>,
        _right: &Arc<Node<M>>,
    ) -> Result<Hide, Self::Error> {
        Ok(Hide::Neither)
    }

    /// This method is called for every node, after [`Self::convert_witness`] or
    /// [`Self::prune_case`], if either is applicable.
    ///
    /// For case nodes for which [`Self::prune_case`] returned [`Hide::Left`] or
    /// [`Hide::Right`], `inner` will be an [`Inner::AssertR`] or [`Inner::AssertL`]
    /// respectively; the pruned child will then appear only as a CMR.
    ///
    /// It accepts the iteration data of the current node, from which the existing
    /// cached data can be obtained by calling `data.node.cached_data()`, as well
    /// as an `Inner` structure containing its already-converted children.
    ///
    /// Returns new cached data which will be attached to the newly-converted node.
    fn convert_data(
        &mut self,
        data: &PostOrderIterItem<&Node<N>>,
        inner: Inner<&Arc<Node<M>>, M::Jet, &M::Disconnect, &M::Witness>,
    ) -> Result<M::CachedData, Self::Error>;
}

/// Basic finalizer which converts a [`super::CommitNode`] to a [`super::RedeemNode`]
/// by attaching witness data from an iterator.
///
/// Does not do any type-checking and may attach an invalid witness to a combinator.
///
/// If it encounters a disconnect node, it simply returns an error.
// FIXME we should do type checking, but this would require a method to check
// type compatibility between a Value and a type::Final.
pub struct SimpleFinalizer<W: Iterator<Item = Arc<Value>>> {
    iter: W,
}

impl<W: Iterator<Item = Arc<Value>>> SimpleFinalizer<W> {
    pub fn new(iter: W) -> Self {
        SimpleFinalizer { iter }
    }
}

impl<W: Iterator<Item = Arc<Value>>, J: Jet> Converter<Commit<J>, Redeem<J>>
    for SimpleFinalizer<W>
{
    type Error = crate::Error;

    fn convert_witness(
        &mut self,
        _: &PostOrderIterItem<&CommitNode<J>>,
        _: &NoWitness,
    ) -> Result<Arc<Value>, Self::Error> {
        self.iter.next().ok_or(crate::Error::NoMoreWitnesses)
    }

    fn convert_disconnect(
        &mut self,
        _: &PostOrderIterItem<&CommitNode<J>>,
        _: Option<&Arc<RedeemNode<J>>>,
        _: &NoDisconnect,
    ) -> Result<Arc<RedeemNode<J>>, Self::Error> {
        Err(crate::Error::IncompleteFinalization)
    }

    fn convert_data(
        &mut self,
        data: &PostOrderIterItem<&CommitNode<J>>,
        inner: Inner<&Arc<RedeemNode<J>>, J, &Arc<RedeemNode<J>>, &Arc<Value>>,
    ) -> Result<Arc<RedeemData<J>>, Self::Error> {
        let converted_data = inner
            .map(|node| node.cached_data())
            .map_disconnect(|node| node.cached_data())
            .map_witness(Arc::clone);
        Ok(Arc::new(RedeemData::new(
            data.node.arrow().shallow_clone(),
            converted_data,
        )))
    }
}
