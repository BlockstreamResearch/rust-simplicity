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

//! Human-readable Nodes

use crate::dag::{MaxSharing, PostOrderIterItem};
use crate::encode;
use crate::jet::Jet;
use crate::node::{self, Commit, CommitData, CommitNode, Converter, NoDisconnect, NoWitness, Node};
use crate::types::arrow::FinalArrow;
use crate::{BitWriter, Cmr};

use std::io;
use std::sync::Arc;

pub type NamedCommitNode<J> = Node<Named<Commit<J>>>;

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Debug, Hash)]
pub struct Named<N> {
    /// Makes the type non-constructible.
    never: std::convert::Infallible,
    /// Required by Rust.
    phantom: std::marker::PhantomData<N>,
}

impl<J: Jet> node::Marker for Named<Commit<J>> {
    type CachedData = NamedCommitData<J>;
    type Witness = <Commit<J> as node::Marker>::Witness;
    type Disconnect = <Commit<J> as node::Marker>::Disconnect;
    type SharingId = Arc<str>;
    type Jet = J;

    fn compute_sharing_id(_: Cmr, cached_data: &Self::CachedData) -> Option<Arc<str>> {
        Some(Arc::clone(&cached_data.name))
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct NamedCommitData<J> {
    /// Data related to the node itself
    internal: Arc<CommitData<J>>,
    /// Name assigned to the node
    name: Arc<str>,
}

impl<J: Jet> NamedCommitNode<J> {
    pub fn from_node(root: &CommitNode<J>) -> Arc<Self> {
        let mut namer = Namer::new_rooted(root.cmr());
        root.convert::<MaxSharing<Commit<J>>, _, _>(&mut namer)
            .unwrap()
    }

    /// Accessor for the node's name
    pub fn name(&self) -> &Arc<str> {
        &self.cached_data().name
    }

    /// Accessor for the node's type arrow
    pub fn arrow(&self) -> &FinalArrow {
        self.cached_data().internal.arrow()
    }

    /// Encode a Simplicity expression to bits without any witness data
    pub fn encode<W: io::Write>(&self, w: &mut BitWriter<W>) -> io::Result<usize> {
        let program_bits = encode::encode_program(self, w)?;
        w.flush_all()?;
        Ok(program_bits)
    }

    /// Encode a Simplicity program to a vector of bytes, without any witness data.
    pub fn encode_to_vec(&self) -> Vec<u8> {
        let mut program_and_witness_bytes = Vec::<u8>::new();
        let mut writer = BitWriter::new(&mut program_and_witness_bytes);
        self.encode(&mut writer)
            .expect("write to vector never fails");
        debug_assert!(!program_and_witness_bytes.is_empty());

        program_and_witness_bytes
    }
}

pub struct Namer {
    const_idx: usize,
    wit_idx: usize,
    other_idx: usize,
    root_cmr: Option<Cmr>,
}

impl Namer {
    /// Costruct a new `Namer`. Will assign the name `main` to the node with
    /// the given CMR.
    pub fn new_rooted(root_cmr: Cmr) -> Self {
        Namer {
            const_idx: 0,
            wit_idx: 0,
            other_idx: 0,
            root_cmr: Some(root_cmr),
        }
    }

    /// Generate a fresh name for the given node.
    pub fn assign_name<C, J, X, W>(&mut self, inner: node::Inner<C, J, X, W>) -> String {
        let prefix = match inner {
            node::Inner::Iden => "id",
            node::Inner::Unit => "ut",
            node::Inner::InjL(..) => "jl",
            node::Inner::InjR(..) => "jr",
            node::Inner::Drop(..) => "dp",
            node::Inner::Take(..) => "tk",
            node::Inner::Comp(..) => "cp",
            node::Inner::Case(..) => "cs",
            node::Inner::AssertL(..) => "asstl",
            node::Inner::AssertR(..) => "asstr",
            node::Inner::Pair(..) => "pr",
            node::Inner::Disconnect(..) => "disc",
            node::Inner::Witness(..) => "wit",
            node::Inner::Fail(..) => "FAIL",
            node::Inner::Jet(..) => "jt",
            node::Inner::Word(..) => "const",
        };
        let index = match inner {
            node::Inner::Word(..) => &mut self.const_idx,
            node::Inner::Witness(..) => &mut self.wit_idx,
            _ => &mut self.other_idx,
        };
        *index += 1;
        format!("{}{}", prefix, index)
    }
}

impl<J: Jet> Converter<Commit<J>, Named<Commit<J>>> for Namer {
    type Error = ();
    fn convert_witness(
        &mut self,
        _: &PostOrderIterItem<&CommitNode<J>>,
        _: &NoWitness,
    ) -> Result<NoWitness, Self::Error> {
        Ok(NoWitness)
    }

    fn convert_disconnect(
        &mut self,
        _: &PostOrderIterItem<&CommitNode<J>>,
        _: Option<&Arc<NamedCommitNode<J>>>,
        _: &NoDisconnect,
    ) -> Result<NoDisconnect, Self::Error> {
        Ok(NoDisconnect)
    }

    fn convert_data(
        &mut self,
        data: &PostOrderIterItem<&CommitNode<J>>,
        inner: node::Inner<&Arc<NamedCommitNode<J>>, J, &NoDisconnect, &NoWitness>,
    ) -> Result<NamedCommitData<J>, Self::Error> {
        // Special-case the root node, which is always called main.
        // The CMR of the root node, conveniently, is guaranteed to be
        // unique, so we can key on the CMR to figure out which node to do.
        if Some(data.node.cmr()) == self.root_cmr {
            return Ok(NamedCommitData {
                internal: Arc::clone(data.node.cached_data()),
                name: Arc::from("main"),
            });
        }

        Ok(NamedCommitData {
            internal: Arc::clone(data.node.cached_data()),
            name: Arc::from(self.assign_name(inner).as_str()),
        })
    }
}
