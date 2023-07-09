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

use crate::dag::{InternalSharing, MaxSharing, PostOrderIterItem};
use crate::encode;
use crate::human_encoding::{ErrorSet, Position};
use crate::jet::Jet;
use crate::node::{self, Commit, CommitData, CommitNode, Converter, NoDisconnect, NoWitness, Node};
use crate::node::{Construct, ConstructData, Constructible};
use crate::types;
use crate::types::arrow::{Arrow, FinalArrow};
use crate::{BitWriter, Cmr};

use std::io;
use std::marker::PhantomData;
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
    /// Name assigned to the node.
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

pub type NamedConstructNode<J> = Node<Named<Construct<J>>>;

impl<J: Jet> node::Marker for Named<Construct<J>> {
    type CachedData = NamedConstructData<J>;
    type Witness = <Construct<J> as node::Marker>::Witness;
    type Disconnect = NoDisconnect;
    type SharingId = Arc<str>;
    type Jet = J;

    fn compute_sharing_id(_: Cmr, cached_data: &Self::CachedData) -> Option<Arc<str>> {
        Some(Arc::clone(&cached_data.name))
    }
}

#[derive(Clone, Debug)]
pub struct NamedConstructData<J> {
    /// Data related to the node itself
    internal: ConstructData<J>,
    /// Name assigned to the node
    name: Arc<str>,
    /// Position of the node, if it comes from source code.
    position: Position,
    /// User-provided type bounds on the source (will be checked for consistency
    /// but only after the type checking has completed.)
    user_source_types: Arc<[types::Type]>,
    /// User-provided type bounds on the target (will be checked for consistency
    /// but only after the type checking has completed.)
    user_target_types: Arc<[types::Type]>,
}

impl<J: Jet> NamedConstructNode<J> {
    /// Construct a named construct node from parts.
    pub fn new(
        name: Arc<str>,
        position: Position,
        user_source_types: Arc<[types::Type]>,
        user_target_types: Arc<[types::Type]>,
        inner: node::Inner<Arc<Self>, J, NoDisconnect, NoWitness>,
    ) -> Result<Self, types::Error> {
        let construct_data = ConstructData::from_inner(
            inner
                .as_ref()
                .map(|data| &data.cached_data().internal)
                .map_disconnect(|_| &None)
                .copy_witness(),
        )?;
        let named_data = NamedConstructData {
            internal: construct_data,
            name,
            position,
            user_source_types,
            user_target_types,
        };
        Ok(Node::from_parts(inner, named_data))
    }

    /// Accessor for the node's name
    pub fn name(&self) -> &Arc<str> {
        &self.cached_data().name
    }

    /// Accessor for the node's position
    pub fn position(&self) -> Position {
        self.cached_data().position
    }

    /// Accessor for the node's arrow
    pub fn arrow(&self) -> &Arrow {
        self.cached_data().internal.arrow()
    }

    /// Finalizes the types of the underlying [`ConstructNode`].
    pub fn finalize_types_main(&self) -> Result<Arc<NamedCommitNode<J>>, ErrorSet> {
        self.finalize_types_inner(true)
    }

    /// Finalizes the types of the underlying [`ConstructNode`], without setting
    /// the root node's arrow to 1->1.
    pub fn finalize_types_non_main(&self) -> Result<Arc<NamedCommitNode<J>>, ErrorSet> {
        self.finalize_types_inner(false)
    }

    pub fn finalize_types_inner(
        &self,
        for_main: bool,
    ) -> Result<Arc<NamedCommitNode<J>>, ErrorSet> {
        struct FinalizeTypes<J: Jet> {
            for_main: bool,
            errors: ErrorSet,
            phantom: PhantomData<J>,
        }

        impl<J: Jet> Converter<Named<Construct<J>>, Named<Commit<J>>> for FinalizeTypes<J> {
            type Error = ErrorSet;
            fn convert_witness(
                &mut self,
                _: &PostOrderIterItem<&NamedConstructNode<J>>,
                _: &NoWitness,
            ) -> Result<NoWitness, Self::Error> {
                Ok(NoWitness)
            }

            fn convert_disconnect(
                &mut self,
                _: &PostOrderIterItem<&NamedConstructNode<J>>,
                _: Option<&Arc<NamedCommitNode<J>>>,
                _: &NoDisconnect,
            ) -> Result<NoDisconnect, Self::Error> {
                Ok(NoDisconnect)
            }

            fn convert_data(
                &mut self,
                data: &PostOrderIterItem<&NamedConstructNode<J>>,
                inner: node::Inner<&Arc<NamedCommitNode<J>>, J, &NoDisconnect, &NoWitness>,
            ) -> Result<NamedCommitData<J>, Self::Error> {
                let converted_data = inner.map(|node| &node.cached_data().internal);

                if !self.for_main {
                    // For non-`main` fragments, treat the ascriptions as normative, and apply them
                    // before finalizing the type.
                    let arrow = data.node.arrow();
                    for ty in data.node.cached_data().user_source_types.as_ref() {
                        if let Err(e) = arrow.source.unify(ty, "binding source type annotation") {
                            self.errors.add(data.node.position(), e);
                        }
                    }
                    for ty in data.node.cached_data().user_target_types.as_ref() {
                        if let Err(e) = arrow.target.unify(ty, "binding target type annotation") {
                            self.errors.add(data.node.position(), e);
                        }
                    }
                }

                let commit_data = match CommitData::new(data.node.arrow(), converted_data) {
                    Ok(commit_data) => Arc::new(commit_data),
                    Err(e) => {
                        self.errors.add(data.node.position(), e);
                        return Err(self.errors.clone());
                    }
                };

                if self.for_main {
                    // For `main`, only apply type ascriptions *after* inference has completely
                    // determined the type.
                    let source_bound =
                        types::Bound::Complete(Arc::clone(&commit_data.arrow().source));
                    let source_ty = types::Type::from(source_bound);
                    for ty in data.node.cached_data().user_source_types.as_ref() {
                        if let Err(e) = source_ty.unify(ty, "binding source type annotation") {
                            self.errors.add(data.node.position(), e);
                        }
                    }
                    let target_bound =
                        types::Bound::Complete(Arc::clone(&commit_data.arrow().target));
                    let target_ty = types::Type::from(target_bound);
                    for ty in data.node.cached_data().user_target_types.as_ref() {
                        if let Err(e) = target_ty.unify(ty, "binding target type annotation") {
                            self.errors.add(data.node.position(), e);
                        }
                    }
                }

                Ok(NamedCommitData {
                    name: Arc::clone(&data.node.cached_data().name),
                    internal: commit_data,
                })
            }
        }

        let mut finalizer = FinalizeTypes {
            for_main,
            errors: ErrorSet::default(),
            phantom: PhantomData,
        };

        if for_main {
            let unit_ty = types::Type::unit();
            if self.cached_data().user_source_types.is_empty() {
                if let Err(e) = self
                    .arrow()
                    .source
                    .unify(&unit_ty, "setting root source to unit")
                {
                    finalizer.errors.add(self.position(), e);
                }
            }
            if self.cached_data().user_target_types.is_empty() {
                if let Err(e) = self
                    .arrow()
                    .target
                    .unify(&unit_ty, "setting root source to unit")
                {
                    finalizer.errors.add(self.position(), e);
                }
            }
        }

        let root = self.convert::<InternalSharing, _, _>(&mut finalizer)?;
        finalizer.errors.into_result(root)
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
