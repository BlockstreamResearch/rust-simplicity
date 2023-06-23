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

use crate::dag::{InternalSharing, PostOrderIterItem};
use crate::jet::Jet;
use crate::types::{self, arrow::Arrow};
use crate::{Cmr, Error, FailEntropy, Value};

use std::marker::PhantomData;
use std::sync::Arc;

use super::{Converter, Hide, Inner, NoWitness, Node, NodeData, Redeem, RedeemData, RedeemNode};
use super::{Constructible, CoreConstructible, JetConstructible, WitnessConstructible};

/// ID used to share [`WitnessNode`]s.
///
/// This is impossible to construct, which is a promise that it is impossible
/// to share [`WitnessNode`]s.
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Debug, Hash)]
pub enum WitnessId {}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Debug, Hash)]
pub enum Witness {}

impl<J: Jet> NodeData<J> for Witness {
    type CachedData = WitnessData<J>;
    type Witness = PopulatedWitness;
    type SharingId = WitnessId;

    fn compute_sharing_id(_: Cmr, _: &WitnessData<J>) -> Option<WitnessId> {
        None
    }
}

pub type WitnessNode<J> = Node<Witness, J>;

impl<J: Jet> WitnessNode<J> {
    /// Accessor for the node's arrow
    pub fn arrow(&self) -> &Arrow {
        &self.data.arrow
    }

    pub fn prune_and_retype(&self) -> Arc<Self> {
        struct Retyper<J: Jet>(PhantomData<J>);

        impl<J: Jet> Converter<Witness, Witness, J> for Retyper<J> {
            type Error = types::Error;
            fn convert_witness(
                &mut self,
                _: &PostOrderIterItem<&WitnessNode<J>>,
                wit: &PopulatedWitness,
            ) -> Result<PopulatedWitness, Self::Error> {
                Ok(wit.clone())
            }

            fn prune_case(
                &mut self,
                _: &PostOrderIterItem<&WitnessNode<J>>,
                left: &Arc<WitnessNode<J>>,
                right: &Arc<WitnessNode<J>>,
            ) -> Result<Hide, Self::Error> {
                // If either child is marked as pruned, we hide it, which will cause this
                // case node to become an assertl or assertr, potentially reducing the size
                // of types since there will be fewer constraints to unify.
                //
                // If both children are marked pruned, this function will only hide the left
                // one. This doesn't matter; in this case the node itself will be marked as
                // pruned and eventually get dropped.
                if left.cached_data().populated == Populated::Pruned {
                    Ok(Hide::Left)
                } else if right.cached_data().populated == Populated::Pruned {
                    Ok(Hide::Right)
                } else {
                    Ok(Hide::Neither)
                }
            }

            fn convert_data(
                &mut self,
                _: &PostOrderIterItem<&WitnessNode<J>>,
                inner: Inner<&Arc<WitnessNode<J>>, J, &PopulatedWitness>,
            ) -> Result<WitnessData<J>, Self::Error> {
                let converted_inner = inner
                    .map(|node| node.cached_data())
                    .map_witness(PopulatedWitness::clone);
                // This next line does the actual retyping.
                WitnessData::from_inner(converted_inner)
            }
        }

        self.convert::<InternalSharing, _, _>(&mut Retyper(PhantomData))
            .expect("type inference won't fail if it succeeded before")
    }

    pub fn finalize(&self) -> Result<Arc<RedeemNode<J>>, Error> {
        // 0. Setup some structure for the WitnessNode->RedeemNode conversion
        struct Finalizer<J>(PhantomData<J>);

        impl<J: Jet> Converter<Witness, Redeem, J> for Finalizer<J> {
            type Error = Error;
            fn convert_witness(
                &mut self,
                _: &PostOrderIterItem<&WitnessNode<J>>,
                wit: &PopulatedWitness,
            ) -> Result<Arc<Value>, Self::Error> {
                if let PopulatedWitness::Yes(ref wit) = wit {
                    Ok(Arc::clone(wit))
                } else {
                    Err(Error::IncompleteFinalization)
                }
            }

            fn convert_data(
                &mut self,
                data: &PostOrderIterItem<&WitnessNode<J>>,
                inner: Inner<&Arc<RedeemNode<J>>, J, &Arc<Value>>,
            ) -> Result<Arc<RedeemData<J>>, Self::Error> {
                let converted_data = inner.map(|node| node.cached_data()).map_witness(Arc::clone);
                Ok(Arc::new(RedeemData::new(
                    data.node.arrow().finalize()?,
                    converted_data,
                )))
            }
        }

        // 1. First, prune everything that we can
        let pruned_self = self.prune_and_retype();
        // 2. Then, set the root arrow to 1->1
        let unit_ty = types::Type::unit();
        pruned_self
            .arrow()
            .source
            .unify(&unit_ty, "setting root source to unit")?;
        pruned_self
            .arrow()
            .target
            .unify(&unit_ty, "setting root source to unit")?;

        // 3. Then attempt to convert the whole program to a RedeemNode.
        //    Despite all of the above this can still fail due to the
        //    occurs check, which checks for infinitely-sized types.
        pruned_self.convert::<InternalSharing, _, _>(&mut Finalizer(PhantomData))

        // FIXME Finally we should prune the program using the bit machine
    }
}

/// Whether or not witness data has been populated for this node and its descendents
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Debug, Hash)]
pub enum Populated {
    /// Witness data has been filled in
    Yes,
    /// Witness data has not been filled in
    No,
    /// Witness data has not been filled in, and never will be
    Pruned,
}

/// Like [`Populated`] but actually holds a value in the `Yes` case
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Debug, Hash)]
pub enum PopulatedWitness {
    /// Witness data has been filled in
    Yes(Arc<Value>),
    /// Witness data has not been filled in
    No,
    /// Witness data has not been filled in, and never will be
    Pruned,
}

impl Populated {
    /// Given two populated values, figures out whether a node with those children will
    /// be populated.
    ///
    /// The propagation rules are: if either child is pruned, this node is pruned. Then
    /// if either child is unpopulated, the node is unpopulated. Otherwise, it is populated.
    pub fn and(self, other: Self) -> Self {
        match (self, other) {
            (Populated::Pruned, _) => Populated::Pruned,
            (_, Populated::Pruned) => Populated::Pruned,
            (Populated::No, _) => Populated::No,
            (_, Populated::No) => Populated::No,
            (Populated::Yes, Populated::Yes) => Populated::Yes,
        }
    }
}

#[derive(Clone, Debug)]
pub struct WitnessData<J: Jet> {
    arrow: Arrow,
    populated: Populated,
    /// This isn't really necessary, but it helps type inference if every
    /// struct has a <J> parameter, since it forces the choice of jets to
    /// be consistent without the user needing to specify it too many times.
    phantom: PhantomData<J>,
}

impl<J: Jet> CoreConstructible for WitnessData<J> {
    fn iden() -> Self {
        WitnessData {
            arrow: Arrow::iden(),
            populated: Populated::Yes,
            phantom: PhantomData,
        }
    }

    fn unit() -> Self {
        WitnessData {
            arrow: Arrow::unit(),
            populated: Populated::Yes,
            phantom: PhantomData,
        }
    }

    fn injl(child: &Self) -> Self {
        WitnessData {
            arrow: Arrow::injl(&child.arrow),
            populated: child.populated,
            phantom: PhantomData,
        }
    }

    fn injr(child: &Self) -> Self {
        WitnessData {
            arrow: Arrow::injr(&child.arrow),
            populated: child.populated,
            phantom: PhantomData,
        }
    }

    fn take(child: &Self) -> Self {
        WitnessData {
            arrow: Arrow::take(&child.arrow),
            populated: child.populated,
            phantom: PhantomData,
        }
    }

    fn drop_(child: &Self) -> Self {
        WitnessData {
            arrow: Arrow::drop_(&child.arrow),
            populated: child.populated,
            phantom: PhantomData,
        }
    }

    fn comp(left: &Self, right: &Self) -> Result<Self, types::Error> {
        Ok(WitnessData {
            arrow: Arrow::comp(&left.arrow, &right.arrow)?,
            populated: left.populated.and(right.populated),
            phantom: PhantomData,
        })
    }

    fn case(left: &Self, right: &Self) -> Result<Self, types::Error> {
        // Specifically for case, rules for propagating prunedness are weird,
        // since we assume we can hide pruned nodes, so only if _both_ are
        // pruned is the case node itself pruned.
        Ok(WitnessData {
            arrow: Arrow::case(&left.arrow, &right.arrow)?,
            populated: match (left.populated, right.populated) {
                (Populated::Pruned, x) => x,
                (y, Populated::Pruned) => y,
                (Populated::No, _) => Populated::No,
                (_, Populated::No) => Populated::No,
                (Populated::Yes, Populated::Yes) => Populated::Yes,
            },
            phantom: PhantomData,
        })
    }

    fn assertl(left: &Self, right: Cmr) -> Result<Self, types::Error> {
        Ok(WitnessData {
            arrow: Arrow::assertl(&left.arrow, right)?,
            populated: left.populated,
            phantom: PhantomData,
        })
    }

    fn assertr(left: Cmr, right: &Self) -> Result<Self, types::Error> {
        Ok(WitnessData {
            arrow: Arrow::assertr(left, &right.arrow)?,
            populated: right.populated,
            phantom: PhantomData,
        })
    }

    fn pair(left: &Self, right: &Self) -> Result<Self, types::Error> {
        Ok(WitnessData {
            arrow: Arrow::pair(&left.arrow, &right.arrow)?,
            populated: left.populated.and(right.populated),
            phantom: PhantomData,
        })
    }

    fn disconnect(left: &Self, right: &Self) -> Result<Self, types::Error> {
        Ok(WitnessData {
            arrow: Arrow::disconnect(&left.arrow, &right.arrow)?,
            populated: left.populated.and(right.populated),
            phantom: PhantomData,
        })
    }

    fn fail(entropy: FailEntropy) -> Self {
        WitnessData {
            arrow: Arrow::fail(entropy),
            populated: Populated::Yes,
            phantom: PhantomData,
        }
    }

    fn const_word(word: Arc<Value>) -> Self {
        WitnessData {
            arrow: Arrow::const_word(word),
            populated: Populated::Yes,
            phantom: PhantomData,
        }
    }
}

impl<'a, J: Jet> WitnessConstructible<PopulatedWitness> for WitnessData<J> {
    fn witness(witness: PopulatedWitness) -> Self {
        WitnessData {
            arrow: Arrow::witness(NoWitness),
            populated: match witness {
                PopulatedWitness::Yes(..) => Populated::Yes,
                PopulatedWitness::No => Populated::No,
                PopulatedWitness::Pruned => Populated::Pruned,
            },
            phantom: PhantomData,
        }
    }
}

impl<J: Jet> JetConstructible<J> for WitnessData<J> {
    fn jet(jet: J) -> Self {
        WitnessData {
            arrow: Arrow::jet(jet),
            populated: Populated::Yes,
            phantom: PhantomData,
        }
    }
}
