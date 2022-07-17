// Rust Simplicity Library
// Written in 2020 by
//   Andrew Poelstra <apoelstra@blockstream.com>
//   Sanket Kanjalkar <sanket1729@gmail.com>
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

//! # Simplicity terms
//!
//! Terms are the atomic building blocks of linear Simplicity programs.
//! A program is a sequence of terms, and
//! the meaning of a term depends on its program.
//! We speak of _nodes_ to refer to terms in context of their program.
//!
//! This linear representation is meant for (de)serialization and execution of programs.
//! To compose programs by hand, use the DAG representation of [`super::term_dag`].

use crate::core::iter::DagIterable;
use crate::core::LinearProgram;
use crate::jet::{Application, JetNode};
use crate::merkle::cmr::Cmr;
use std::collections::HashMap;
use std::hash::Hash;

/// Simplicity term for some witness type and application.
/// Terms in context of programs are called _nodes_,
/// and terms have no meaning outside a program.
///
/// Nodes refer to other nodes via indices that are relative to the node's own index.
/// This encodes a DAG _(directed acyclic graph)_.
/// For example, node `InjL(2)` at index 7 in its program
/// represents node `InjL` whose child is `x`,
/// where `x` is the node at index 7 - 2 = 5.
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub enum Term<Witness, App: Application> {
    /// Identity
    Iden,
    /// Unit constant
    Unit,
    /// Left injection of some term
    InjL(usize),
    /// Right injection of some term
    InjR(usize),
    /// Take of some term
    Take(usize),
    /// Drop of some term
    Drop(usize),
    /// Composition of two terms
    Comp(usize, usize),
    /// Case of two terms
    Case(usize, usize),
    /// Left assertion of two terms.
    ///
    /// Right child must be [`Self::Hidden`]
    AssertL(usize, usize),
    /// Right assertion of two terms.
    ///
    /// Left child must be [`Self::Hidden`]
    AssertR(usize, usize),
    /// Pair of two terms
    Pair(usize, usize),
    /// Disconnect of two terms
    Disconnect(usize, usize),
    /// Witness data
    Witness(Witness),
    /// Fail
    Fail(Cmr, Cmr),
    /// Hidden
    Hidden(Cmr),
    /// Application jet
    Jet(&'static JetNode<App>),
}

impl<Witness, App: Application> Term<Witness, App> {
    /// Converts the term from one witness to another,
    /// using a function that translates witness values.
    pub(crate) fn translate_witness<AltWitness, F>(self, mut translate: F) -> Term<AltWitness, App>
    where
        F: FnMut(Witness) -> AltWitness,
    {
        match self {
            Term::Iden => Term::Iden,
            Term::Unit => Term::Unit,
            Term::InjL(i) => Term::InjL(i),
            Term::InjR(i) => Term::InjR(i),
            Term::Take(i) => Term::Take(i),
            Term::Drop(i) => Term::Drop(i),
            Term::Comp(i, j) => Term::Comp(i, j),
            Term::Case(i, j) => Term::Case(i, j),
            Term::AssertL(i, j) => Term::AssertL(i, j),
            Term::AssertR(i, j) => Term::AssertR(i, j),
            Term::Pair(i, j) => Term::Pair(i, j),
            Term::Disconnect(i, j) => Term::Disconnect(i, j),
            Term::Witness(w) => Term::Witness(translate(w)),
            Term::Fail(x, y) => Term::Fail(x, y),
            Term::Hidden(x) => Term::Hidden(x),
            Term::Jet(j) => Term::Jet(j),
        }
    }

    /// Return the relative index of the left child of the given term, if there is such a child.
    pub(crate) fn get_left(&self) -> Option<usize> {
        match self {
            Term::InjL(i) | Term::InjR(i) | Term::Take(i) | Term::Drop(i) => Some(*i),
            Term::Comp(i, _)
            | Term::Case(i, _)
            | Term::AssertL(i, _)
            | Term::AssertR(i, _)
            | Term::Pair(i, _)
            | Term::Disconnect(i, _) => Some(*i),
            _ => None,
        }
    }

    /// Return the relative index of the right child of the given term, if there is such a child.
    pub(crate) fn get_right(&self) -> Option<usize> {
        match self {
            Term::Comp(_, j)
            | Term::Case(_, j)
            | Term::AssertL(_, j)
            | Term::AssertR(_, j)
            | Term::Pair(_, j)
            | Term::Disconnect(_, j) => Some(*j),
            _ => None,
        }
    }

    /// Converts the term to a term that uses sharing.
    ///
    /// The relative indices are updated such that
    /// the IMRs of terms pointed to in the unshared program equal
    /// the IMRs of terms pointed to in the shared program.
    pub(crate) fn into_shared(
        self,
        unshared_idx: usize,
        shared_idx: usize,
        unshared_to_shared_idx: &HashMap<usize, usize>,
    ) -> Self {
        let get_relative_shared_ptr_idx = |relative_unshared_ptr_idx: usize| {
            let unshared_ptr_idx = unshared_idx - relative_unshared_ptr_idx;
            let shared_ptr_idx = unshared_to_shared_idx[&unshared_ptr_idx];
            shared_idx - shared_ptr_idx
        };

        match self {
            Term::InjL(i) => Term::InjL(get_relative_shared_ptr_idx(i)),
            Term::InjR(i) => Term::InjR(get_relative_shared_ptr_idx(i)),
            Term::Take(i) => Term::Take(get_relative_shared_ptr_idx(i)),
            Term::Drop(i) => Term::Drop(get_relative_shared_ptr_idx(i)),
            Term::Comp(i, j) => Term::Comp(
                get_relative_shared_ptr_idx(i),
                get_relative_shared_ptr_idx(j),
            ),
            Term::Case(i, j) => Term::Case(
                get_relative_shared_ptr_idx(i),
                get_relative_shared_ptr_idx(j),
            ),
            Term::AssertL(i, j) => Term::AssertL(
                get_relative_shared_ptr_idx(i),
                get_relative_shared_ptr_idx(j),
            ),
            Term::AssertR(i, j) => Term::AssertR(
                get_relative_shared_ptr_idx(i),
                get_relative_shared_ptr_idx(j),
            ),
            Term::Pair(i, j) => Term::Pair(
                get_relative_shared_ptr_idx(i),
                get_relative_shared_ptr_idx(j),
            ),
            Term::Disconnect(i, j) => Term::Disconnect(
                get_relative_shared_ptr_idx(i),
                get_relative_shared_ptr_idx(j),
            ),
            _ => self,
        }
    }
}

/// Untyped Simplicity program (see [`Term`]).
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct UntypedProgram<Witness, App: Application>(pub(crate) Vec<Term<Witness, App>>);

impl<Witness, App: Application> UntypedProgram<Witness, App> {
    /// Return an iterator over the nodes of the program.
    pub fn iter(&self) -> impl Iterator<Item = &Term<Witness, App>> {
        self.0.iter()
    }

    /// Return whether the program is in _canonical order_.
    /// This means that node indices appear in _post order_,
    /// i.e., left children appear before right ones,
    /// children appear before their parent,
    /// and every node can be reached from the root.
    pub fn has_canonical_order(&self) -> bool {
        let mut bottom = 0;

        for index in self.iter_post_order() {
            if index == bottom {
                bottom += 1;
            } else {
                return false;
            }
        }

        bottom == self.len()
    }
}

impl<Witness, App: Application> LinearProgram for UntypedProgram<Witness, App> {
    type Node = Term<Witness, App>;

    fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    fn len(&self) -> usize {
        self.0.len()
    }

    fn root(&self) -> &Self::Node {
        &self.0[self.0.len() - 1]
    }
}

impl<Witness, App: Application> IntoIterator for UntypedProgram<Witness, App> {
    type Item = Term<Witness, App>;
    type IntoIter = std::vec::IntoIter<Term<Witness, App>>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl<Witness, App: Application> DagIterable for UntypedProgram<Witness, App> {
    type Node = usize;

    fn root(&self) -> Option<Self::Node> {
        if self.0.is_empty() {
            None
        } else {
            Some(self.0.len() - 1)
        }
    }

    fn left_of(&self, index: Self::Node) -> Option<Self::Node> {
        self.0[index].get_left().map(|l| index - l)
    }

    fn right_of(&self, index: Self::Node) -> Option<Self::Node> {
        self.0[index].get_right().map(|r| index - r)
    }
}
