// SPDX-License-Identifier: CC0-1.0

//! Human-readable Nodes

use crate::dag::{InternalSharing, MaxSharing, PostOrderIterItem};
use crate::human_encoding::{Error, ErrorSet, Position, WitnessOrHole};
use crate::jet::Jet;
use crate::node::{
    self, Commit, CommitData, CommitNode, Construct, ConstructData, Constructible as _, Converter,
    CoreConstructible as _, Inner, NoDisconnect, NoWitness, Node,
};
use crate::types;
use crate::types::arrow::{Arrow, FinalArrow};
use crate::{encode, ConstructNode, Value};
use crate::{BitWriter, Cmr, Ihr};

use std::collections::HashMap;
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
    type Disconnect = Arc<str>;
    type SharingId = <Commit<J> as node::Marker>::SharingId;
    type Jet = J;

    fn compute_sharing_id(cmr: Cmr, cached_data: &Self::CachedData) -> Option<Self::SharingId> {
        Commit::<J>::compute_sharing_id(cmr, &cached_data.internal)
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

    /// Accessor for the node's name
    pub fn ihr(&self) -> Option<Ihr> {
        self.cached_data().internal.ihr()
    }

    /// Accessor for the node's type arrow
    pub fn arrow(&self) -> &FinalArrow {
        self.cached_data().internal.arrow()
    }

    /// Forget the names, yielding an ordinary [`CommitNode`].
    pub fn to_commit_node(&self) -> Arc<CommitNode<J>> {
        struct Forgetter<J>(PhantomData<J>);

        impl<J: Jet> Converter<Named<Commit<J>>, Commit<J>> for Forgetter<J> {
            type Error = ();
            fn convert_witness(
                &mut self,
                _: &PostOrderIterItem<&NamedCommitNode<J>>,
                _: &NoWitness,
            ) -> Result<NoWitness, Self::Error> {
                Ok(NoWitness)
            }

            fn convert_disconnect(
                &mut self,
                _: &PostOrderIterItem<&NamedCommitNode<J>>,
                _: Option<&Arc<CommitNode<J>>>,
                _: &Arc<str>,
            ) -> Result<NoDisconnect, Self::Error> {
                Ok(NoDisconnect)
            }

            fn convert_data(
                &mut self,
                data: &PostOrderIterItem<&NamedCommitNode<J>>,
                _: node::Inner<&Arc<CommitNode<J>>, J, &NoDisconnect, &NoWitness>,
            ) -> Result<Arc<CommitData<J>>, Self::Error> {
                Ok(Arc::clone(&data.node.cached_data().internal))
            }
        }

        self.convert::<InternalSharing, _, _>(&mut Forgetter(PhantomData))
            .unwrap()
    }

    pub fn to_construct_node(
        &self,
        witness: &HashMap<Arc<str>, Value>,
        disconnect: &HashMap<Arc<str>, Arc<NamedCommitNode<J>>>,
    ) -> Arc<ConstructNode<J>> {
        struct Populator<'a, J: Jet> {
            witness_map: &'a HashMap<Arc<str>, Value>,
            disconnect_map: &'a HashMap<Arc<str>, Arc<NamedCommitNode<J>>>,
            inference_context: types::Context,
            phantom: PhantomData<J>,
        }

        impl<J: Jet> Converter<Named<Commit<J>>, Construct<J>> for Populator<'_, J> {
            type Error = ();

            fn convert_witness(
                &mut self,
                data: &PostOrderIterItem<&Node<Named<Commit<J>>>>,
                _: &NoWitness,
            ) -> Result<Option<Value>, Self::Error> {
                let name = &data.node.cached_data().name;
                // We keep the witness nodes without data unpopulated.
                // Some nodes are pruned later so they don't need to be populated.
                // Which nodes are pruned is not known when this code is executed.
                // If an unpruned node is unpopulated, then there will be an error
                // during the finalization.
                Ok(self.witness_map.get(name).cloned())
            }

            fn convert_disconnect(
                &mut self,
                data: &PostOrderIterItem<&Node<Named<Commit<J>>>>,
                maybe_converted: Option<&Arc<Node<Construct<J>>>>,
                _: &Arc<str>,
            ) -> Result<Option<Arc<Node<Construct<J>>>>, Self::Error> {
                if let Some(converted) = maybe_converted {
                    Ok(Some(converted.clone()))
                } else {
                    let hole_name = match data.node.inner() {
                        Inner::Disconnect(_, hole_name) => hole_name,
                        _ => unreachable!(),
                    };
                    // We keep the missing disconnected branches empty.
                    // Like witness nodes (see above), disconnect nodes may be pruned later.
                    // The finalization will detect missing branches and throw an error.
                    let maybe_commit = self.disconnect_map.get(hole_name);
                    // FIXME: recursive call to convert
                    // We cannot introduce a stack because we are implementing the Converter
                    // trait and do not have access to the actual algorithm used for conversion
                    // in order to save its state.
                    //
                    // OTOH, if a user writes a program with so many disconnected expressions
                    // that there is a stack overflow, it's his own fault :)
                    // This may fail in a fuzz test.
                    let witness = maybe_commit
                        .map(|commit| commit.convert::<InternalSharing, _, _>(self).unwrap());
                    Ok(witness)
                }
            }

            fn convert_data(
                &mut self,
                _: &PostOrderIterItem<&Node<Named<Commit<J>>>>,
                inner: Inner<
                    &Arc<Node<Construct<J>>>,
                    J,
                    &Option<Arc<ConstructNode<J>>>,
                    &Option<Value>,
                >,
            ) -> Result<ConstructData<J>, Self::Error> {
                let inner = inner
                    .map(|node| node.cached_data())
                    .map_witness(|maybe_value| maybe_value.clone());
                Ok(ConstructData::from_inner(&self.inference_context, inner)
                    .expect("types are already finalized"))
            }
        }

        self.convert::<InternalSharing, _, _>(&mut Populator {
            witness_map: witness,
            disconnect_map: disconnect,
            inference_context: types::Context::new(),
            phantom: PhantomData,
        })
        .unwrap()
    }

    /// Encode a Simplicity expression to bits without any witness data
    #[deprecated(since = "0.5.0", note = "use Self::encode_without_witness instead")]
    pub fn encode<W: io::Write>(&self, w: &mut BitWriter<W>) -> io::Result<usize> {
        let program_bits = encode::encode_program(self, w)?;
        w.flush_all()?;
        Ok(program_bits)
    }

    /// Encode a Simplicity program to a vector of bytes, without any witness data.
    #[deprecated(since = "0.5.0", note = "use Self::to_vec_without_witness instead")]
    pub fn encode_to_vec(&self) -> Vec<u8> {
        let mut prog = Vec::<u8>::new();
        self.encode_without_witness(&mut prog)
            .expect("write to vector never fails");
        debug_assert!(!prog.is_empty());

        prog
    }
}

pub type NamedConstructNode<J> = Node<Named<Construct<J>>>;

impl<J: Jet> node::Marker for Named<Construct<J>> {
    type CachedData = NamedConstructData<J>;
    type Witness = WitnessOrHole;
    type Disconnect = Arc<NamedConstructNode<J>>;
    type SharingId = <Construct<J> as node::Marker>::SharingId;
    type Jet = J;

    fn compute_sharing_id(cmr: Cmr, cached_data: &Self::CachedData) -> Option<Self::SharingId> {
        Construct::<J>::compute_sharing_id(cmr, &cached_data.internal)
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
        inference_context: &types::Context,
        name: Arc<str>,
        position: Position,
        user_source_types: Arc<[types::Type]>,
        user_target_types: Arc<[types::Type]>,
        inner: node::Inner<Arc<Self>, J, Arc<Self>, WitnessOrHole>,
    ) -> Result<Self, types::Error> {
        let construct_data = ConstructData::from_inner(
            inference_context,
            inner
                .as_ref()
                .map(|data| &data.cached_data().internal)
                .map_disconnect(|_| &None)
                .map_witness(|_| None),
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

    /// Creates a copy of a node with a different name.
    pub fn renamed(&self, new_name: Arc<str>) -> Self {
        let data = NamedConstructData {
            internal: self.cached_data().internal.clone(),
            user_source_types: Arc::clone(&self.cached_data().user_source_types),
            user_target_types: Arc::clone(&self.cached_data().user_target_types),
            name: new_name,
            position: self.position(),
        };
        Self::from_parts(self.inner().clone(), data)
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

    /// Accessor for the node's type inference context.
    pub fn inference_context(&self) -> &types::Context {
        self.cached_data().internal.inference_context()
    }

    /// Finalizes the types of the underlying [`crate::ConstructNode`].
    pub fn finalize_types_main(&self) -> Result<Arc<NamedCommitNode<J>>, ErrorSet> {
        self.finalize_types_inner(true)
    }

    /// Finalizes the types of the underlying [`crate::ConstructNode`], without setting
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
            pending_hole_error: Option<(Position, Error)>,
            phantom: PhantomData<J>,
        }

        impl<J: Jet> Converter<Named<Construct<J>>, Named<Commit<J>>> for FinalizeTypes<J> {
            type Error = ErrorSet;

            fn visit_node(&mut self, data: &PostOrderIterItem<&NamedConstructNode<J>>) {
                // If we encounter a typed hole, this is an error *except* when the typed
                // hole is the right child of a disconnect combinator. Conveniently, this
                // case is very easy to detect: it will always appear as a hole immediately
                // followed by a disconnect.
                //
                // Less conveniently, detecting this from within a post-order-iterator
                // requires a small state machine here: when we encounter a hole, we create
                // an error and put it in `self.pending_hole_error`. If we then encounter a
                // disconnect, we drop the error. Otherwise we move it to `self.errors`.
                let inner = data.node.inner();
                if let node::Inner::Disconnect(..) = inner {
                    self.pending_hole_error = None;
                } else {
                    if let Some((position, error)) = self.pending_hole_error.take() {
                        self.errors.add(position, error);
                    }
                    if let node::Inner::Witness(WitnessOrHole::TypedHole(name)) = inner {
                        self.pending_hole_error = Some((
                            data.node.position(),
                            Error::HoleAtCommitTime {
                                name: Arc::clone(name),
                                arrow: data.node.arrow().shallow_clone(),
                            },
                        ));
                    }
                }
            }

            fn convert_witness(
                &mut self,
                _: &PostOrderIterItem<&NamedConstructNode<J>>,
                _: &WitnessOrHole,
            ) -> Result<NoWitness, Self::Error> {
                Ok(NoWitness)
            }

            fn convert_disconnect(
                &mut self,
                data: &PostOrderIterItem<&NamedConstructNode<J>>,
                _: Option<&Arc<NamedCommitNode<J>>>,
                disc: &Arc<NamedConstructNode<J>>,
            ) -> Result<Arc<str>, Self::Error> {
                match disc.inner() {
                    node::Inner::Witness(WitnessOrHole::TypedHole(hole_name)) => {
                        Ok(hole_name.clone())
                    }
                    _ => {
                        self.errors
                            .add(data.node.position(), Error::HoleFilledAtCommitTime);
                        Ok(Arc::from(""))
                    }
                }
            }

            fn convert_data(
                &mut self,
                data: &PostOrderIterItem<&NamedConstructNode<J>>,
                inner: node::Inner<&Arc<NamedCommitNode<J>>, J, &Arc<str>, &NoWitness>,
            ) -> Result<NamedCommitData<J>, Self::Error> {
                let converted_data = inner
                    .as_ref()
                    .map(|node| &node.cached_data().internal)
                    .map_disconnect(|_| &NoDisconnect)
                    .copy_witness();

                let ctx = data.node.inference_context();

                if !self.for_main {
                    // For non-`main` fragments, treat the ascriptions as normative, and apply them
                    // before finalizing the type.
                    let arrow = data.node.arrow();
                    for ty in data.node.cached_data().user_source_types.as_ref() {
                        if let Err(e) =
                            ctx.unify(&arrow.source, ty, "binding source type annotation")
                        {
                            self.errors.add(data.node.position(), e);
                        }
                    }
                    for ty in data.node.cached_data().user_target_types.as_ref() {
                        if let Err(e) =
                            ctx.unify(&arrow.target, ty, "binding target type annotation")
                        {
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
                    let source_ty =
                        types::Type::complete(ctx, Arc::clone(&commit_data.arrow().source));
                    for ty in data.node.cached_data().user_source_types.as_ref() {
                        if let Err(e) = ctx.unify(&source_ty, ty, "binding source type annotation")
                        {
                            self.errors.add(data.node.position(), e);
                        }
                    }
                    let target_ty =
                        types::Type::complete(ctx, Arc::clone(&commit_data.arrow().target));
                    for ty in data.node.cached_data().user_target_types.as_ref() {
                        if let Err(e) = ctx.unify(&target_ty, ty, "binding target type annotation")
                        {
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
            pending_hole_error: None,
            phantom: PhantomData,
        };

        if for_main {
            let ctx = self.inference_context();
            let unit_ty = types::Type::unit(ctx);
            if self.cached_data().user_source_types.is_empty() {
                if let Err(e) = ctx.unify(
                    &self.arrow().source,
                    &unit_ty,
                    "setting root source to unit",
                ) {
                    finalizer.errors.add(self.position(), e);
                }
            }
            if self.cached_data().user_target_types.is_empty() {
                if let Err(e) = ctx.unify(
                    &self.arrow().target,
                    &unit_ty,
                    "setting root target to unit",
                ) {
                    finalizer.errors.add(self.position(), e);
                }
            }
        }

        let root = self.convert::<InternalSharing, _, _>(&mut finalizer)?;
        finalizer.errors.into_result(root)
    }
}

#[derive(Clone, Debug)]
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

    /// Costruct a new `Namer`.
    pub fn new() -> Self {
        Namer {
            const_idx: 0,
            wit_idx: 0,
            other_idx: 0,
            root_cmr: None,
        }
    }

    /// Generate a fresh name for the given node.
    pub fn assign_name<C, J, X>(&mut self, inner: node::Inner<C, J, X, &WitnessOrHole>) -> String {
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
            node::Inner::Witness(WitnessOrHole::Witness) => "wit",
            node::Inner::Witness(WitnessOrHole::TypedHole(name)) => {
                return name.to_string();
            }
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
    ) -> Result<Arc<str>, Self::Error> {
        let hole_idx = self.other_idx;
        self.other_idx += 1;
        Ok(Arc::from(format!("hole {hole_idx}")))
    }

    fn convert_data(
        &mut self,
        data: &PostOrderIterItem<&CommitNode<J>>,
        inner: node::Inner<&Arc<NamedCommitNode<J>>, J, &Arc<str>, &NoWitness>,
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
            name: Arc::from(
                self.assign_name(inner.map_witness(WitnessOrHole::from).as_ref())
                    .as_str(),
            ),
        })
    }
}
