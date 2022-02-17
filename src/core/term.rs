use crate::extension::jets::JetsNode;
use crate::merkle::cmr::Cmr;
use std::hash::Hash;

/// Simplicity expression node, including Bitcoin/Elements extensions
///
/// If Bitcoin/Elements support is not compiled (see `bitcoin` and
/// `elements` features) programs using these extensions will fail to
/// parse.
///
/// All references being relative indices in the context of a program.
/// For ex: InjL(2) at index 7, represents InjL(x) where x is a node
/// at index 5.
/// This is used for representing a final constructed simplicity program.
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
