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

use crate::core::types::FinalType;
use crate::core::LinearProgram;
use crate::core::Term;
use crate::jet::Application;
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

impl<Witness, App: Application> TypedProgram<Witness, App> {
    /// Return an iterator over the nodes of the program.
    pub fn iter(&self) -> impl Iterator<Item = &TypedNode<Witness, App>> {
        self.0.iter()
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
