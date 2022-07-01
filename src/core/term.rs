use crate::core::iter::DagIterable;
use crate::jet::{Application, JetNode};
use crate::merkle::cmr::Cmr;
use std::collections::HashMap;
use std::hash::Hash;

/// Single, untyped Simplicity node.
/// May include Bitcoin/Elements extensions.
///
/// If Bitcoin/Elements support is not compiled (see `bitcoin` and
/// `elements` features), then programs using these extensions will
/// fail to parse.
///
/// A node consists of a combinator and its payload
/// _(references to other nodes, witness data, etc.)_.
/// A list of nodes forms an untyped Simplicity program,
/// which represents an untyped Simplicity DAG.
///
/// References to other nodes are relative indices in the context of a program.
/// For example, node `InjL(2)` at index 7 represents DAG `InjL → x`,
/// where `x` is the DAG that the node at index 5 represents.
/// A node has no meaning without a program.
///
/// The node representation is later used for executing Simplicity programs.
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub enum Term<Witness, App: Application> {
    Iden,
    Unit,
    InjL(usize),
    InjR(usize),
    Take(usize),
    Drop(usize),
    Comp(usize, usize),
    Case(usize, usize),
    AssertL(usize, usize), // Right Node must be hidden
    AssertR(usize, usize), // Left Node must be hidden
    Pair(usize, usize),
    Disconnect(usize, usize),
    Witness(Witness),
    Fail(Cmr, Cmr),
    Hidden(Cmr),
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

// TODO: move to trait that is common to all program types
impl<Witness, App: Application> UntypedProgram<Witness, App> {
    /// Whether this program is empty
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    /// Returns the number of terms in the program
    pub fn len(&self) -> usize {
        self.0.len()
    }

    /// Returns an iterator over the terms in the program
    pub fn iter(&self) -> impl Iterator<Item = &Term<Witness, App>> {
        self.0.iter()
    }

    /// Return whether the program is in _canonical order_.
    /// This means that node indices appear in post order,
    /// i.e., _left child < right child < parent_, for each node in the DAG.
    /// It also means that if there is a node that cannot be reached from the root,
    /// then the program is not in canonical order.
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
