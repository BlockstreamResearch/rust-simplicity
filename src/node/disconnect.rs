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

//! Disconnect Nodes
//!
//! This module defines the [`Disconnectable`] trait, which is applied to the
//! right child of `disconnect` Simplicity nodes, allowing them to represent
//! "disconnected expressions" which may or may not be present, and which
//! contribute to a node's IMR but not CMR.

use crate::dag::Dag;

use std::sync::Arc;

/// Null data type used as dummy for [`super::Marker::Disconnect`]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Debug, Hash)]
pub struct NoDisconnect;

/// Trait representing a "disconnected expression".
pub trait Disconnectable<L> {
    /// Given a generic left child, and treating `self` as the disconnected right
    /// child, produce a [`Dag`] node representing a disconnect node.
    ///
    /// If the disconnected expression is present, this should yield a [`Dag::Binary`].
    /// If it is not present, this should yield a [`Dag::Unary`].
    fn disconnect_dag_arc(self, other: Arc<L>) -> Dag<Arc<L>>;

    /// Same as [`Disconnectable::disconnect_dag_ref`] but takes self by reference.
    fn disconnect_dag_ref<'s>(&'s self, other: &'s L) -> Dag<&'s L>;
}

// Rust's coherence rules prevent us from just doing a small set of blanket
// implementations; one for NoDisconnect (always yielding Dag::Unary) and
// one for T: Borrow<L> (always yielding Dag::Binary) and optional variants.
//
// Instead we have this messy and incomplete list.

// `NoDisconnect` works with arbitrary Arcs
impl<L> Disconnectable<L> for NoDisconnect {
    fn disconnect_dag_arc(self, other: Arc<L>) -> Dag<Arc<L>> {
        Dag::Unary(other)
    }

    fn disconnect_dag_ref<'s>(&'s self, other: &'s L) -> Dag<&'s L> {
        Dag::Unary(other)
    }
}

// Arbitrary things (references, Arcs, whatever) work with themselves.
// This blanket impl is why we can't have a blanket impl for NoDisconnect.
impl<L> Disconnectable<L> for Arc<L> {
    fn disconnect_dag_arc(self, other: Arc<L>) -> Dag<Arc<L>> {
        Dag::Binary(other, self)
    }

    fn disconnect_dag_ref<'s>(&'s self, other: &'s L) -> Dag<&'s L> {
        Dag::Binary(other, self)
    }
}

// Then Option<Arc> can work with either Arcs or references.
impl<L> Disconnectable<L> for Option<Arc<L>> {
    fn disconnect_dag_arc(self, other: Arc<L>) -> Dag<Arc<L>> {
        match self {
            Some(right) => Dag::Binary(other, right),
            None => Dag::Unary(other),
        }
    }

    fn disconnect_dag_ref<'s>(&'s self, other: &'s L) -> Dag<&'s L> {
        match self {
            Some(right) => Dag::Binary(other, right),
            None => Dag::Unary(other),
        }
    }
}
