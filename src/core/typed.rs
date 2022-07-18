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
use crate::core::types::FinalType;
use crate::core::{types, LinearProgram, Program, ProgramNode, Term, UntypedProgram, Value};
use crate::jet::Application;
use crate::merkle::cmr::Cmr;
use crate::merkle::imr;
use crate::{analysis, decode, Error};
use std::fmt;
use std::sync::Arc;

/// Simplicity node with metadata for the time of commitment.
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Debug, Hash)]
pub struct TypedNode<Witness, App: Application> {
    /// Underlying term
    pub term: Term<Witness, App>,
    /// Source type of the node
    pub source_ty: Arc<FinalType>,
    /// Target type of the node
    pub target_ty: Arc<FinalType>,
    /// Index of the node inside the surrounding program
    pub index: usize,
    /// Commitment Merkle root of the node
    pub cmr: Cmr,
}

impl<Witness, App: Application> fmt::Display for TypedNode<Witness, App> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}] ", self.index)?;
        match self.term {
            Term::Iden => f.write_str("iden")?,
            Term::Unit => f.write_str("unit")?,
            Term::InjL(i) => write!(f, "injl({})", self.index - i)?,
            Term::InjR(i) => write!(f, "injr({})", self.index - i)?,
            Term::Take(i) => write!(f, "take({})", self.index - i)?,
            Term::Drop(i) => write!(f, "drop({})", self.index - i)?,
            Term::Comp(i, j) => write!(f, "comp({}, {})", self.index - i, self.index - j)?,
            Term::Case(i, j) => write!(f, "case({}, {})", self.index - i, self.index - j)?,
            Term::AssertL(i, j) => write!(f, "assertL({}, {})", self.index - i, self.index - j)?,
            Term::AssertR(i, j) => write!(f, "assertR({}, {})", self.index - i, self.index - j)?,
            Term::Pair(i, j) => write!(f, "pair({}, {})", self.index - i, self.index - j)?,
            Term::Disconnect(i, j) => {
                write!(f, "disconnect({}, {})", self.index - i, self.index - j)?
            }
            Term::Witness(..) => f.write_str("witness")?,
            Term::Hidden(..) => f.write_str("hidden")?,
            Term::Fail(..) => f.write_str("fail")?,
            Term::Jet(j) => write!(f, "[jet]{}", j)?,
        }
        write!(f, ": {} → {}", self.source_ty, self.target_ty,)
    }
}

/// Typed Simplicity program (see [`TypedNode`]).
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct TypedProgram<Witness, App: Application>(pub(crate) Vec<TypedNode<Witness, App>>);

impl<App: Application> TypedProgram<(), App> {
    /// Decode a typed program from bits.
    pub fn decode<I: Iterator<Item = u8>>(iter: &mut BitIter<I>) -> Result<Self, Error> {
        let program = UntypedProgram::<(), App>::decode(iter)?;
        types::type_check(program)
    }
}

impl<Witness, App: Application> TypedProgram<Witness, App> {
    /// Return an iterator over the nodes of the program.
    pub fn iter(&self) -> impl ExactSizeIterator<Item = &TypedNode<Witness, App>> + Clone {
        self.0.iter()
    }

    // TODO: Return error instead of panicking upon witness that is too short or of wrong type
    /// Add the given witness to the program.
    ///
    /// Panics if the witness is too short.
    pub fn add_witness(self, witness: Vec<Value>) -> Result<TypedProgram<Value, App>, Error> {
        let mut it = witness.into_iter();
        let mut translate = |_old_witness: Witness| it.next().expect("witness too short!");
        let ret = self
            .0
            .into_iter()
            .map(|node| TypedNode {
                term: node.term.translate_witness(&mut translate),
                source_ty: node.source_ty,
                target_ty: node.target_ty,
                index: node.index,
                cmr: node.cmr,
            })
            .collect();

        Ok(TypedProgram(ret))
    }

    /// Decode a witness from bits and add it to the program.
    pub fn decode_witness<I: Iterator<Item = u8>>(
        self,
        iter: &mut BitIter<I>,
    ) -> Result<TypedProgram<Value, App>, Error> {
        let witness = decode::decode_witness(&self, iter)?;
        self.add_witness(witness)
    }

    /// Return a vector of the types of values that make up a valid witness for the program.
    pub fn get_witness_types(&self) -> Vec<&FinalType> {
        let mut witness_types = Vec::new();

        for node in &self.0 {
            if let Term::Witness(_) = &node.term {
                witness_types.push(node.target_ty.as_ref());
            }
        }

        witness_types
    }
}

impl<App: Application> TypedProgram<Value, App> {
    /// Decode a typed program from bits.
    pub fn decode<I: Iterator<Item = u8>>(iter: &mut BitIter<I>) -> Result<Self, Error> {
        let program = TypedProgram::<(), App>::decode(iter)?;
        program.decode_witness(iter)
    }

    /// Add metadata for the time of redemption and return a finalized program.
    ///
    /// The typed program must already include the witness _(enforced through type system)_.
    pub fn finalize(self) -> Result<Program<App>, Error> {
        let program = self.finalize_insane();

        if program.has_maximal_sharing() {
            Ok(program)
        } else {
            Err(Error::SharingNotMaximal)
        }
    }

    /// Add metadata for the time of redemption and return a finalized program.
    ///
    /// **Does not check if the resulting program has maximal sharing.**
    ///
    /// The typed program must already include the witness _(enforced through type system)_.
    pub fn finalize_insane(self) -> Program<App> {
        let mut finalized_program = Vec::new();

        for node in self.0 {
            let finalized_node = ProgramNode {
                imr: imr::compute_imr(&finalized_program, &node, node.index),
                extra_cells_bound: analysis::compute_extra_cells_bound(&finalized_program, &node),
                frame_count_bound: analysis::compute_frame_count_bound(&finalized_program, &node),
                typed: node,
            };
            finalized_program.push(finalized_node)
        }

        Program {
            nodes: finalized_program,
        }
    }
}

impl<Witness, App: Application> LinearProgram for TypedProgram<Witness, App> {
    type Node = TypedNode<Witness, App>;

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

impl<Witness, App: Application> IntoIterator for TypedProgram<Witness, App> {
    type Item = TypedNode<Witness, App>;
    type IntoIter = std::vec::IntoIter<TypedNode<Witness, App>>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}
