// SPDX-License-Identifier: CC0-1.0

use crate::dag::{InternalSharing, PostOrderIterItem};
use crate::jet::Jet;
use crate::types::{self, arrow::Arrow};
use crate::{Cmr, Error, FailEntropy, Value};

use crate::value::Word;
use std::marker::PhantomData;
use std::sync::Arc;

use super::{
    Converter, CoreConstructible, DisconnectConstructible, Inner, JetConstructible, Marker,
    NoWitness, Node, Redeem, RedeemData, RedeemNode, WitnessConstructible,
};

/// ID used to share [`WitnessNode`]s.
///
/// This is impossible to construct, which is a promise that it is impossible
/// to share [`WitnessNode`]s.
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Debug, Hash)]
pub enum WitnessId {}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Debug, Hash)]
pub struct Witness<J> {
    /// Makes the type non-constructible.
    never: std::convert::Infallible,
    /// Required by Rust.
    phantom: std::marker::PhantomData<J>,
}

impl<J: Jet> Marker for Witness<J> {
    type CachedData = WitnessData<J>;
    type Witness = Option<Value>;
    type Disconnect = Option<Arc<WitnessNode<J>>>;
    type SharingId = WitnessId;
    type Jet = J;

    fn compute_sharing_id(_: Cmr, _: &WitnessData<J>) -> Option<WitnessId> {
        None
    }
}

pub type WitnessNode<J> = Node<Witness<J>>;

impl<J: Jet> WitnessNode<J> {
    /// Accessor for the node's arrow
    pub fn arrow(&self) -> &Arrow {
        &self.data.arrow
    }

    /// Finalize the witness program as an unpruned redeem program.
    ///
    /// Witness nodes must be populated with sufficient data,
    /// to ensure that the resulting redeem program successfully runs on the Bit Machine.
    /// Furthermore, **all** disconnected branches must be populated,
    /// even those that are not executed.
    ///
    /// The resulting redeem program is **not pruned**.
    ///
    /// ## See
    ///
    /// [`RedeemNode::prune`]
    pub fn finalize_unpruned(&self) -> Result<Arc<RedeemNode<J>>, Error> {
        struct Finalizer<J>(PhantomData<J>);

        impl<J: Jet> Converter<Witness<J>, Redeem<J>> for Finalizer<J> {
            type Error = Error;

            fn convert_witness(
                &mut self,
                data: &PostOrderIterItem<&WitnessNode<J>>,
                wit: &Option<Value>,
            ) -> Result<Value, Self::Error> {
                if let Some(ref wit) = wit {
                    Ok(wit.shallow_clone())
                } else {
                    // We insert a zero value into unpopulated witness nodes,
                    // assuming that this node will later be pruned out of the program.
                    //
                    // Pruning requires running a program on the Bit Machine,
                    // which in turn requires a program with fully populated witness nodes.
                    // It would be horrible UX to force the caller to provide witness data
                    // even for unexecuted branches, so we insert zero values here.
                    //
                    // If this node is executed after all, then the caller can fix the witness
                    // data based on the returned execution error.
                    //
                    // Zero values may "accidentally" satisfy a program even if the caller
                    // didn't provide any witness data. However, this is only the case for the
                    // most trivial programs. The only place where we must be careful is our
                    // unit tests, which tend to include these kinds of trivial programs.
                    let ty = data.node.arrow().target.finalize()?;
                    Ok(Value::zero(&ty))
                }
            }

            fn convert_disconnect(
                &mut self,
                _: &PostOrderIterItem<&WitnessNode<J>>,
                maybe_converted: Option<&Arc<RedeemNode<J>>>,
                _: &Option<Arc<WitnessNode<J>>>,
            ) -> Result<Arc<RedeemNode<J>>, Self::Error> {
                if let Some(child) = maybe_converted {
                    Ok(Arc::clone(child))
                } else {
                    Err(Error::DisconnectRedeemTime)
                }
            }

            fn convert_data(
                &mut self,
                data: &PostOrderIterItem<&WitnessNode<J>>,
                inner: Inner<&Arc<RedeemNode<J>>, J, &Arc<RedeemNode<J>>, &Value>,
            ) -> Result<Arc<RedeemData<J>>, Self::Error> {
                let converted_data = inner
                    .map(|node| node.cached_data())
                    .map_disconnect(|node| node.cached_data())
                    .map_witness(Value::shallow_clone);
                Ok(Arc::new(RedeemData::new(
                    data.node.arrow().finalize()?,
                    converted_data,
                )))
            }
        }

        self.convert::<InternalSharing, _, _>(&mut Finalizer(PhantomData))
    }

    /// Finalize the witness program as a pruned redeem program.
    ///
    /// Witness nodes must be populated with sufficient data,
    /// to ensure that the resulting redeem program successfully runs on the Bit Machine.
    /// Furthermore, **all** disconnected branches must be populated,
    /// even those that are not executed.
    ///
    /// The resulting redeem program is **pruned** based on the given transaction environment.
    ///
    /// ## See
    ///
    /// [`RedeemNode::prune`]
    pub fn finalize_pruned(&self, env: &J::Environment) -> Result<Arc<RedeemNode<J>>, Error> {
        let unpruned = self.finalize_unpruned()?;
        unpruned.prune(env).map_err(Error::Execution)
    }
}

#[derive(Clone, Debug)]
pub struct WitnessData<J> {
    arrow: Arrow,
    /// This isn't really necessary, but it helps type inference if every
    /// struct has a \<J\> parameter, since it forces the choice of jets to
    /// be consistent without the user needing to specify it too many times.
    phantom: PhantomData<J>,
}

impl<J> CoreConstructible for WitnessData<J> {
    fn iden(inference_context: &types::Context) -> Self {
        WitnessData {
            arrow: Arrow::iden(inference_context),
            phantom: PhantomData,
        }
    }

    fn unit(inference_context: &types::Context) -> Self {
        WitnessData {
            arrow: Arrow::unit(inference_context),
            phantom: PhantomData,
        }
    }

    fn injl(child: &Self) -> Self {
        WitnessData {
            arrow: Arrow::injl(&child.arrow),
            phantom: PhantomData,
        }
    }

    fn injr(child: &Self) -> Self {
        WitnessData {
            arrow: Arrow::injr(&child.arrow),
            phantom: PhantomData,
        }
    }

    fn take(child: &Self) -> Self {
        WitnessData {
            arrow: Arrow::take(&child.arrow),
            phantom: PhantomData,
        }
    }

    fn drop_(child: &Self) -> Self {
        WitnessData {
            arrow: Arrow::drop_(&child.arrow),
            phantom: PhantomData,
        }
    }

    fn comp(left: &Self, right: &Self) -> Result<Self, types::Error> {
        Ok(WitnessData {
            arrow: Arrow::comp(&left.arrow, &right.arrow)?,
            phantom: PhantomData,
        })
    }

    fn case(left: &Self, right: &Self) -> Result<Self, types::Error> {
        // Specifically for case, rules for propagating prunedness are weird,
        // since we assume we can hide pruned nodes, so only if _both_ are
        // pruned is the case node itself pruned.
        Ok(WitnessData {
            arrow: Arrow::case(&left.arrow, &right.arrow)?,
            phantom: PhantomData,
        })
    }

    fn assertl(left: &Self, right: Cmr) -> Result<Self, types::Error> {
        Ok(WitnessData {
            arrow: Arrow::assertl(&left.arrow, right)?,
            phantom: PhantomData,
        })
    }

    fn assertr(left: Cmr, right: &Self) -> Result<Self, types::Error> {
        Ok(WitnessData {
            arrow: Arrow::assertr(left, &right.arrow)?,
            phantom: PhantomData,
        })
    }

    fn pair(left: &Self, right: &Self) -> Result<Self, types::Error> {
        Ok(WitnessData {
            arrow: Arrow::pair(&left.arrow, &right.arrow)?,
            phantom: PhantomData,
        })
    }

    fn fail(inference_context: &types::Context, entropy: FailEntropy) -> Self {
        // Fail nodes always get pruned.
        WitnessData {
            arrow: Arrow::fail(inference_context, entropy),
            phantom: PhantomData,
        }
    }

    fn const_word(inference_context: &types::Context, word: Word) -> Self {
        WitnessData {
            arrow: Arrow::const_word(inference_context, word),
            phantom: PhantomData,
        }
    }

    fn inference_context(&self) -> &types::Context {
        self.arrow.inference_context()
    }
}

impl<J: Jet> DisconnectConstructible<Option<Arc<WitnessNode<J>>>> for WitnessData<J> {
    fn disconnect(left: &Self, right: &Option<Arc<WitnessNode<J>>>) -> Result<Self, types::Error> {
        let right = right.as_ref();
        Ok(WitnessData {
            arrow: Arrow::disconnect(&left.arrow, &right.map(|n| n.arrow()))?,
            phantom: PhantomData,
        })
    }
}

impl<J> WitnessConstructible<Option<Value>> for WitnessData<J> {
    fn witness(inference_context: &types::Context, _witness: Option<Value>) -> Self {
        WitnessData {
            arrow: Arrow::witness(inference_context, NoWitness),
            phantom: PhantomData,
        }
    }
}

impl<J: Jet> JetConstructible<J> for WitnessData<J> {
    fn jet(inference_context: &types::Context, jet: J) -> Self {
        WitnessData {
            arrow: Arrow::jet(inference_context, jet),
            phantom: PhantomData,
        }
    }
}
