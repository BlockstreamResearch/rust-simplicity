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

use super::{Constructible, CoreConstructible, JetConstructible, WitnessConstructible};
use super::{Converter, Hide, Inner, Marker, NoWitness, Node, Redeem, RedeemData, RedeemNode};

/// ID used to share [`WitnessNode`]s.
///
/// This is impossible to construct, which is a promise that it is impossible
/// to share [`WitnessNode`]s.
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Debug, Hash)]
pub enum WitnessId {}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Debug, Hash)]
pub struct Witness<J: Jet> {
    /// Makes the type non-constructible.
    never: std::convert::Infallible,
    /// Required by Rust.
    phantom: std::marker::PhantomData<J>,
}

impl<J: Jet> Marker for Witness<J> {
    type CachedData = WitnessData<J>;
    type Witness = Option<Arc<Value>>;
    type SharingId = WitnessId;
    type Jet = J;

    fn compute_sharing_id(_: Cmr, _: &WitnessData<J>) -> Option<WitnessId> {
        None
    }
}

pub type WitnessNode<J> = Node<Witness<J>>;

impl<J: Jet> WitnessNode<J> {
    /// Creates a copy of the node (and its entire DAG with the prune bit set)
    #[must_use]
    pub fn pruned(&self) -> Arc<Self> {
        let new_data = WitnessData {
            must_prune: true,
            ..self.data.clone()
        };
        Arc::new(WitnessNode {
            data: new_data,
            cmr: self.cmr,
            inner: self
                .inner
                .as_ref()
                .map(Arc::clone)
                .map_witness(|wit| wit.as_ref().map(Arc::clone)),
        })
    }

    /// Accessor for the node's arrow
    pub fn arrow(&self) -> &Arrow {
        &self.data.arrow
    }

    /// Whether the "must prune" bit is set on this node
    pub fn must_prune(&self) -> bool {
        self.data.must_prune
    }

    pub fn prune_and_retype(&self) -> Arc<Self> {
        struct Retyper<J: Jet>(PhantomData<J>);

        impl<J: Jet> Converter<Witness<J>, Witness<J>> for Retyper<J> {
            type Error = types::Error;
            fn convert_witness(
                &mut self,
                _: &PostOrderIterItem<&WitnessNode<J>>,
                wit: &Option<Arc<Value>>,
            ) -> Result<Option<Arc<Value>>, Self::Error> {
                Ok(wit.as_ref().map(Arc::clone))
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
                if left.cached_data().must_prune {
                    Ok(Hide::Left)
                } else if right.cached_data().must_prune {
                    Ok(Hide::Right)
                } else {
                    Ok(Hide::Neither)
                }
            }

            fn convert_data(
                &mut self,
                data: &PostOrderIterItem<&WitnessNode<J>>,
                inner: Inner<&Arc<WitnessNode<J>>, J, &Option<Arc<Value>>>,
            ) -> Result<WitnessData<J>, Self::Error> {
                let converted_inner = inner
                    .map(|node| node.cached_data())
                    .map_witness(|wit| wit.as_ref().map(Arc::clone));
                // This next line does the actual retyping.
                let mut retyped = WitnessData::from_inner(converted_inner)?;
                // Sometimes we set the prune bit on nodes without setting that
                // of their children; in this case the prune bit inferred from
                // `converted_inner` will be incorrect.
                if data.node.data.must_prune {
                    retyped.must_prune = true;
                }
                Ok(retyped)
            }
        }

        // FIXME after running the `ReTyper` we should run a `WitnessShrinker` which
        // shrinks the witness data in case the ReTyper shrank its types.
        self.convert::<InternalSharing, _, _>(&mut Retyper(PhantomData))
            .expect("type inference won't fail if it succeeded before")
    }

    pub fn finalize(&self) -> Result<Arc<RedeemNode<J>>, Error> {
        // 0. Setup some structure for the WitnessNode->RedeemNode conversion
        struct Finalizer<J>(PhantomData<J>);

        impl<J: Jet> Converter<Witness<J>, Redeem<J>> for Finalizer<J> {
            type Error = Error;
            fn convert_witness(
                &mut self,
                _: &PostOrderIterItem<&WitnessNode<J>>,
                wit: &Option<Arc<Value>>,
            ) -> Result<Arc<Value>, Self::Error> {
                if let Some(ref wit) = wit {
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

#[derive(Clone, Debug)]
pub struct WitnessData<J: Jet> {
    arrow: Arrow,
    must_prune: bool,
    /// This isn't really necessary, but it helps type inference if every
    /// struct has a <J> parameter, since it forces the choice of jets to
    /// be consistent without the user needing to specify it too many times.
    phantom: PhantomData<J>,
}

impl<J: Jet> CoreConstructible for WitnessData<J> {
    fn iden() -> Self {
        WitnessData {
            arrow: Arrow::iden(),
            must_prune: false,
            phantom: PhantomData,
        }
    }

    fn unit() -> Self {
        WitnessData {
            arrow: Arrow::unit(),
            must_prune: false,
            phantom: PhantomData,
        }
    }

    fn injl(child: &Self) -> Self {
        WitnessData {
            arrow: Arrow::injl(&child.arrow),
            must_prune: child.must_prune,
            phantom: PhantomData,
        }
    }

    fn injr(child: &Self) -> Self {
        WitnessData {
            arrow: Arrow::injr(&child.arrow),
            must_prune: child.must_prune,
            phantom: PhantomData,
        }
    }

    fn take(child: &Self) -> Self {
        WitnessData {
            arrow: Arrow::take(&child.arrow),
            must_prune: child.must_prune,
            phantom: PhantomData,
        }
    }

    fn drop_(child: &Self) -> Self {
        WitnessData {
            arrow: Arrow::drop_(&child.arrow),
            must_prune: child.must_prune,
            phantom: PhantomData,
        }
    }

    fn comp(left: &Self, right: &Self) -> Result<Self, types::Error> {
        Ok(WitnessData {
            arrow: Arrow::comp(&left.arrow, &right.arrow)?,
            must_prune: left.must_prune || right.must_prune,
            phantom: PhantomData,
        })
    }

    fn case(left: &Self, right: &Self) -> Result<Self, types::Error> {
        // Specifically for case, rules for propagating prunedness are weird,
        // since we assume we can hide pruned nodes, so only if _both_ are
        // pruned is the case node itself pruned.
        Ok(WitnessData {
            arrow: Arrow::case(&left.arrow, &right.arrow)?,
            must_prune: left.must_prune && right.must_prune,
            phantom: PhantomData,
        })
    }

    fn assertl(left: &Self, right: Cmr) -> Result<Self, types::Error> {
        Ok(WitnessData {
            arrow: Arrow::assertl(&left.arrow, right)?,
            must_prune: left.must_prune,
            phantom: PhantomData,
        })
    }

    fn assertr(left: Cmr, right: &Self) -> Result<Self, types::Error> {
        Ok(WitnessData {
            arrow: Arrow::assertr(left, &right.arrow)?,
            must_prune: right.must_prune,
            phantom: PhantomData,
        })
    }

    fn pair(left: &Self, right: &Self) -> Result<Self, types::Error> {
        Ok(WitnessData {
            arrow: Arrow::pair(&left.arrow, &right.arrow)?,
            must_prune: left.must_prune || right.must_prune,
            phantom: PhantomData,
        })
    }

    fn disconnect(left: &Self, right: &Self) -> Result<Self, types::Error> {
        Ok(WitnessData {
            arrow: Arrow::disconnect(&left.arrow, &right.arrow)?,
            must_prune: left.must_prune || right.must_prune,
            phantom: PhantomData,
        })
    }

    fn fail(entropy: FailEntropy) -> Self {
        // Fail nodes always get pruned.
        WitnessData {
            arrow: Arrow::fail(entropy),
            must_prune: true,
            phantom: PhantomData,
        }
    }

    fn const_word(word: Arc<Value>) -> Self {
        WitnessData {
            arrow: Arrow::const_word(word),
            must_prune: false,
            phantom: PhantomData,
        }
    }
}

impl<J: Jet> WitnessConstructible<Option<Arc<Value>>> for WitnessData<J> {
    fn witness(witness: Option<Arc<Value>>) -> Self {
        WitnessData {
            arrow: Arrow::witness(NoWitness),
            must_prune: witness.is_none(),
            phantom: PhantomData,
        }
    }
}

impl<J: Jet> JetConstructible<J> for WitnessData<J> {
    fn jet(jet: J) -> Self {
        WitnessData {
            arrow: Arrow::jet(jet),
            must_prune: false,
            phantom: PhantomData,
        }
    }
}
