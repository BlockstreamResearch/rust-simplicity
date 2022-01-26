use crate::extension::Jet;
use crate::{cmr, extension};
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
    Fail([u8; 32], [u8; 32]),
    Hidden(cmr::Cmr),
    Ext(Extension),
    Jet(extension::jets::JetsNode),
}

impl<Witness, Extension: extension::Jet> Term<Witness, Extension> {
    /// Compute the cmr_iv of the term.
    /// Jet's don't technically have an IV, but this function
    /// returns the CMR
    pub(crate) fn cmr_iv(&self) -> cmr::Cmr {
        // This helps in avoiding repeated code by allowing to merge
        // patterns in cmr calculation code.
        match self {
            Term::Iden => cmr::tag::iden_cmr(),
            Term::Unit => cmr::tag::unit_cmr(),
            Term::InjL(_i) => cmr::tag::injl_cmr(),
            Term::InjR(_i) => cmr::tag::injr_cmr(),
            Term::Take(_i) => cmr::tag::take_cmr(),
            Term::Drop(_i) => cmr::tag::drop_cmr(),
            Term::Comp(_i, _j) => cmr::tag::comp_cmr(),
            Term::Case(_i, _j) | Term::AssertL(_i, _j) | Term::AssertR(_i, _j) => {
                cmr::tag::case_cmr()
            }
            Term::Pair(_i, _j) => cmr::tag::pair_cmr(),
            Term::Disconnect(_i, _) => cmr::tag::disconnect_cmr(),
            Term::Witness(..) => cmr::tag::witness_cmr(),
            Term::Hidden(cmr) => *cmr,
            Term::Ext(j) => j.cmr(),
            Term::Jet(j) => j.cmr(),
            Term::Fail(..) => unimplemented!(),
        }
    }

    /// Compute the cmr_iv of the term.
    /// Jet's don't technically have an IV, but this function
    /// returns the CMR
    pub(crate) fn amr_iv(&self) -> cmr::Amr {
        // This helps in avoiding repeated code by allowing to merge
        // patterns in cmr calculation code.
        match self {
            Term::Iden => cmr::tag::iden_amr(),
            Term::Unit => cmr::tag::unit_amr(),
            Term::InjL(_i) => cmr::tag::injl_amr(),
            Term::InjR(_i) => cmr::tag::injr_amr(),
            Term::Take(_i) => cmr::tag::take_amr(),
            Term::Drop(_i) => cmr::tag::drop_amr(),
            Term::Comp(_i, _j) => cmr::tag::comp_amr(),
            Term::Case(_i, _j) => cmr::tag::case_amr(),
            Term::AssertL(_i, _j) => cmr::tag::assertl_amr(),
            Term::AssertR(_i, _j) => cmr::tag::assertr_amr(),
            Term::Pair(_i, _j) => cmr::tag::pair_amr(),
            Term::Disconnect(_i, _) => cmr::tag::disconnect_amr(),
            Term::Witness(..) => cmr::tag::witness_amr(),
            Term::Hidden(cmr) => cmr::Amr::from(<[u8; 32]>::from(*cmr)),
            Term::Ext(j) => cmr::Amr::from(<[u8; 32]>::from(j.cmr())),
            Term::Jet(j) => cmr::Amr::from(<[u8; 32]>::from(j.cmr())),
            Term::Fail(..) => unimplemented!(),
        }
    }
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
