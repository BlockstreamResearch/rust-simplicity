use crate::extension::jets::JetsNode;
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
pub enum Term<Witness, Extension> {
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
    Ext(Extension),
    Jet(JetsNode),
}

impl<Witness, Extension> Term<Witness, Extension> {
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
pub struct UnTypedProg<Witness, Extension>(pub Vec<Term<Witness, Extension>>);

impl<Witness, Extension> UnTypedProg<Witness, Extension> {
    /// Whether this is the null program
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    /// The number of (shared) terms in the program
    pub fn len(&self) -> usize {
        self.0.len()
    }

    /// Returns an iterator over the (shared) terms in the program
    pub fn iter(&self) -> impl Iterator<Item = &Term<Witness, Extension>> {
        self.0.iter()
    }
}
