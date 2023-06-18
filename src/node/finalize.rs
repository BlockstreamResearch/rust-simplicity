// Rust Simplicity Library
// Written in 2023 by
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

//! Node Finalization
//!
//! This module defines a trait [`Finalizer`] which is called by the
//! [`Node::rtl_convert_finalize`] method. The `Finalizer` trait is used
//! convert between one node type and another, controlling several nuanced
//! aspects of the conversion. Specifically:
//!
//! 1. Like [`Node::convert`], witness data can be attached to witnesses
//!    and cached data can be translated.
//! 2. Unlike [`Node::convert`], witness data can be optionally *not*
//!    provided, in which case the witness node (and its ancestors) will
//!    not be converted.
//! 3. Similarly, cached data can be *not* provided, with the same effect
//!    (an arbitrary node and its ancestors are not converted).
//! 4. For `case` nodes, the decision can be made to hide zero, one, or
//!    both children. (If both are hidden, the `case` node and all its
//!    ancestors are not converted.) This allows unconverted branches to
//!    be harmlessly dropped rather than causing the entire conversion
//!    to fail.
//!
//! If the root node is not converted, because it or one of its descendents
//! was not converted (and there was no intervening `case` node to prune
//! the dead branch away), the method returns an error.
//!

use crate::dag::PostOrderIterItem;
use crate::jet::Jet;

use super::{Inner, Node, NodeData};

use std::sync::Arc;

/// A decision about which, if any, child branches of a `case` combinator to hide
/// during a [`Node::convert_hiding`] conversion
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Debug, Hash)]
pub enum Hide {
    Neither,
    Left,
    Right,
    Both,
}

/// The primary trait controlling how a node conversion is done.
///
/// When a node is converted using [`Node::rtl_convert_finalize`], the original
/// DAG rooted at that node is traversed **in right-to-left post order**. For
/// each node, the following steps are taken:
///
/// 1. First, [`Finalizer::visit_node`] is called, before any other checks are
///    done. This happens regardless of the node's type or whether it is going
///    to be pruned.
///
///    This method is a convenience to the finalizer and does not affect the
///    general algorithm.
///
/// 2. Then, if the node is a witness node, `Finalizer::convert_witness` is
///    called to obtain witness data. If `None` is returned, the node is marked
///    as pruned and the loop skips to the next one.
///
/// 3. If the node is a case node, [`Finalizer::prune_case`] is called to decide
///    whether to prune either child of the node (turning the `case` into an
///    `assertl` or `assertr`, or pruning the `case` itself). If either child
///    has pruned (e.g. because it was missing witness data) it must be hidden.
///    If not, the iterator will panic.
///
///    If `prune_case` returns [`Hide::Both`], the case node itself is marked as
///    pruned and the loop skips to the next node.
///
/// 4. For non-witness non-case nodes, if the node has any pruned children, it
///    is marked as pruned and the loop skips to the next node.
///
/// 5. Similarly, if the node is a fail node, it is marked as pruned and the
///    loop skips to the next node.
///
/// 6. Finally, if all of the above steps succeed, the node's data is passed to
///    [`Finalizer::convert_data`], whose job it is to compute the cached data
///    for the new node. This method may return `None`, in which case the node
///    will be pruned even if it was not a `witness` or a `case` node.
///
/// If any [`Finalizer`] method returns an error, then iteration is aborted
/// immediately and the error returned to the caller. If the caller would like
/// to attempt recovering from errors and/or accumulate multiple errors, it
/// needs to do this by tracking the errors internally.
///
/// The finalization method will not return any errors except those returned by
/// methods on [`Finalizer`].
///
/// After iteration is complete, it may be the case that the root node is
/// pruned. In this case, the entire method returns `None` and it is the
/// caller's responsibility to convert this to a proper error type.
pub trait Finalizer<N: NodeData<J>, M: NodeData<J>, J: Jet> {
    /// The error type returned by the methods on this trait.
    type Error;

    /// This method is called on every node, to inform the Finalizer about the
    /// state of the iterator.
    ///
    /// No action needs to be taken. The default implementation does nothing.
    fn visit_node(&mut self, _data: &PostOrderIterItem<&Node<N, J>>) {}

    /// For witness nodes, this method is called first to attach witness data to
    /// the node.
    ///
    /// It takes the iteration data of the current node, as well as the current
    /// witness (which in a typical scenario will be an empty structure, but
    /// with custom node types may be a placeholder or other useful information)
    ///
    /// If this returns a value, that value will be attached to the witness node.
    /// No typechecking or other sanity-checking is done.
    ///
    /// If it does *not* return a value, the witness node is marked as pruned and
    /// no further processing will be done on it.
    fn convert_witness(
        &mut self,
        data: &PostOrderIterItem<&Node<N, J>>,
        witness: &N::Witness,
    ) -> Result<Option<M::Witness>, Self::Error>;

    /// For case nodes, this method is called first to decide which, if any, children
    /// to prune.
    ///
    /// It takes the iteration data of the current node, as well as both of its already
    /// converted children, if they were converted. (If not, they will be passed as
    /// `None`.) This method returns a hiding decision. **It is required that any `None`
    /// children be pruned. If not, the iterator will panic.**
    ///
    /// In the case that [`Hide::Both`] is returned, the `case` node is marked as
    /// pruned and no further processing will be done on it.
    ///
    /// The default implementation simply checks which children have been pruned, and
    /// hides those.
    fn prune_case(
        &mut self,
        _data: &PostOrderIterItem<&Node<N, J>>,
        left: Option<&Arc<Node<M, J>>>,
        right: Option<&Arc<Node<M, J>>>,
    ) -> Result<Hide, Self::Error> {
        Ok(match (left.is_some(), right.is_some()) {
            (true, true) => Hide::Neither,
            (false, true) => Hide::Left,
            (true, false) => Hide::Right,
            (false, false) => Hide::Both,
        })
    }

    /// This method is called for every node whose children (if any) have been converted,
    /// except for witness nodes for which [`Self::convert_witness`] returned `None` and
    /// case nodes for which [`Self::prune_case`] returned [`Hide::Both`].
    ///
    /// For case nodes for which [`Self::prune_case`] returned [`Hide::Left`] or
    /// [`Hide::Right`], `inner` will be an [`Inner::AssertR`] or [`Inner::AssertL`]
    /// respectively; the pruned child will then appear only as a CMR.
    ///
    /// It accepts the iteration data of the current node, from which the existing
    /// cached data can be obtained by calling `data.node.cached_data()`, as well
    /// as an `Inner` structure containing its already-converted children.
    ///
    /// If it returns new cached data, then this data is attached to a newly converted
    /// node. If it returns `None`, the node is pruned.
    fn convert_data(
        &mut self,
        data: &PostOrderIterItem<&Node<N, J>>,
        inner: Inner<&Arc<Node<M, J>>, J, M::Witness>,
    ) -> Result<Option<M::CachedData>, Self::Error>;
}
