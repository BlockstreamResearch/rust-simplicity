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
use crate::core::Term;
use crate::core::{types, Value};
use crate::core::{LinearProgram, UntypedProgram};
use crate::jet::Application;
use crate::{decode, Error};
use std::sync::Arc;

/// Simplicity node with source and target type.
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Debug, Hash)]
pub struct TypedNode<Witness, App: Application> {
    /// Underlying term
    pub term: Term<Witness, App>,
    /// Source type of the node
    pub source_ty: Arc<FinalType>,
    /// Target type of the node
    pub target_ty: Arc<FinalType>,
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
    pub fn iter(&self) -> impl Iterator<Item = &TypedNode<Witness, App>> {
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
