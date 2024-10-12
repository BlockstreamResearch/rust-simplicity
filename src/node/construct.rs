// SPDX-License-Identifier: CC0-1.0

use crate::dag::{InternalSharing, PostOrderIterItem};
use crate::encode;
use crate::jet::Jet;
use crate::types::{self, arrow::Arrow};
use crate::{BitIter, BitWriter, Cmr, FailEntropy};

use crate::value::Word;
use std::io;
use std::marker::PhantomData;
use std::sync::Arc;

use super::{
    Commit, CommitData, CommitNode, Converter, Inner, Marker, NoDisconnect, NoWitness, Node,
};
use super::{CoreConstructible, DisconnectConstructible, JetConstructible, WitnessConstructible};

/// ID used to share [`ConstructNode`]s.
///
/// This is impossible to construct, which is a promise that it is impossible
/// to share [`ConstructNode`]s.
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Debug, Hash)]
pub enum ConstructId {}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Debug, Hash)]
pub struct Construct<J> {
    /// Makes the type non-constructible.
    never: std::convert::Infallible,
    /// Required by Rust.
    phantom: std::marker::PhantomData<J>,
}

impl<J: Jet> Marker for Construct<J> {
    type CachedData = ConstructData<J>;
    type Witness = NoWitness;
    type Disconnect = Option<Arc<ConstructNode<J>>>;
    type SharingId = ConstructId;
    type Jet = J;

    fn compute_sharing_id(_: Cmr, _: &ConstructData<J>) -> Option<ConstructId> {
        None
    }
}

pub type ConstructNode<J> = Node<Construct<J>>;

impl<J: Jet> ConstructNode<J> {
    /// Accessor for the node's arrow
    pub fn arrow(&self) -> &Arrow {
        self.data.arrow()
    }

    /// Sets the source and target type of the node to unit
    pub fn set_arrow_to_program(&self) -> Result<(), types::Error> {
        let ctx = self.data.inference_context();
        let unit_ty = types::Type::unit(ctx);
        ctx.unify(
            &self.arrow().source,
            &unit_ty,
            "setting root source to unit",
        )?;
        ctx.unify(
            &self.arrow().target,
            &unit_ty,
            "setting root target to unit",
        )?;
        Ok(())
    }

    /// Convert a [`ConstructNode`] to a [`CommitNode`] by finalizing all of the types.
    ///
    /// Also sets the source and target type of this node to unit. This is almost
    /// certainly what you want, since the resulting `CommitNode` cannot be further
    /// composed, and needs to be 1->1 to go on-chain. But if you don't, call
    /// [`Self::finalize_types_non_program`] instead.
    pub fn finalize_types(&self) -> Result<Arc<CommitNode<J>>, crate::Error> {
        self.set_arrow_to_program()?;
        self.finalize_types_non_program()
    }

    /// Convert a [`ConstructNode`] to a [`CommitNode`] by finalizing all of the types.
    ///
    /// Does *not* sets the source and target type of this node to unit.
    pub fn finalize_types_non_program(&self) -> Result<Arc<CommitNode<J>>, crate::Error> {
        struct FinalizeTypes<J: Jet>(PhantomData<J>);

        impl<J: Jet> Converter<Construct<J>, Commit<J>> for FinalizeTypes<J> {
            type Error = crate::Error;
            fn convert_witness(
                &mut self,
                _: &PostOrderIterItem<&ConstructNode<J>>,
                _: &NoWitness,
            ) -> Result<NoWitness, Self::Error> {
                Ok(NoWitness)
            }

            fn convert_disconnect(
                &mut self,
                _: &PostOrderIterItem<&ConstructNode<J>>,
                maybe_converted: Option<&Arc<CommitNode<J>>>,
                _: &Option<Arc<ConstructNode<J>>>,
            ) -> Result<NoDisconnect, Self::Error> {
                if maybe_converted.is_some() {
                    Err(crate::Error::DisconnectCommitTime)
                } else {
                    Ok(NoDisconnect)
                }
            }

            fn convert_data(
                &mut self,
                data: &PostOrderIterItem<&ConstructNode<J>>,
                inner: Inner<&Arc<CommitNode<J>>, J, &NoDisconnect, &NoWitness>,
            ) -> Result<Arc<CommitData<J>>, Self::Error> {
                let converted_data = inner.map(|node| node.cached_data());
                CommitData::new(&data.node.data.arrow, converted_data)
                    .map(Arc::new)
                    .map_err(crate::Error::from)
            }
        }

        self.convert::<InternalSharing, _, _>(&mut FinalizeTypes(PhantomData))
    }

    /// Decode a Simplicity expression from bits, without witness data.
    ///
    /// # Usage
    ///
    /// Use this method only if the serialization **does not** include the witness data.
    /// This means, the program simply has no witness during commitment,
    /// or the witness is provided by other means.
    ///
    /// If the serialization contains the witness data, then use [`crate::RedeemNode::decode()`].
    pub fn decode<I: Iterator<Item = u8>>(
        mut bits: BitIter<I>,
    ) -> Result<Arc<Self>, crate::decode::Error> {
        let res = crate::decode::decode_expression(&mut bits)?;
        bits.close()?;
        Ok(res)
    }

    /// Encode a Simplicity expression to bits, with no witness data
    pub fn encode<W: io::Write>(&self, w: &mut BitWriter<W>) -> io::Result<usize> {
        let program_bits = encode::encode_program(self, w)?;
        w.flush_all()?;
        Ok(program_bits)
    }
}

#[derive(Clone, Debug)]
pub struct ConstructData<J> {
    arrow: Arrow,
    /// This isn't really necessary, but it helps type inference if every
    /// struct has a \<J\> parameter, since it forces the choice of jets to
    /// be consistent without the user needing to specify it too many times.
    phantom: PhantomData<J>,
}

impl<J: Jet> ConstructData<J> {
    /// Constructs a new [`ConstructData`] from an (unfinalized) type arrow
    pub fn new(arrow: Arrow) -> Self {
        ConstructData {
            arrow,
            phantom: PhantomData,
        }
    }

    /// Accessor for the node's arrow
    pub fn arrow(&self) -> &Arrow {
        &self.arrow
    }
}

impl<J> CoreConstructible for ConstructData<J> {
    fn iden(inference_context: &types::Context) -> Self {
        ConstructData {
            arrow: Arrow::iden(inference_context),
            phantom: PhantomData,
        }
    }

    fn unit(inference_context: &types::Context) -> Self {
        ConstructData {
            arrow: Arrow::unit(inference_context),
            phantom: PhantomData,
        }
    }

    fn injl(child: &Self) -> Self {
        ConstructData {
            arrow: Arrow::injl(&child.arrow),
            phantom: PhantomData,
        }
    }

    fn injr(child: &Self) -> Self {
        ConstructData {
            arrow: Arrow::injr(&child.arrow),
            phantom: PhantomData,
        }
    }

    fn take(child: &Self) -> Self {
        ConstructData {
            arrow: Arrow::take(&child.arrow),
            phantom: PhantomData,
        }
    }

    fn drop_(child: &Self) -> Self {
        ConstructData {
            arrow: Arrow::drop_(&child.arrow),
            phantom: PhantomData,
        }
    }

    fn comp(left: &Self, right: &Self) -> Result<Self, types::Error> {
        Ok(ConstructData {
            arrow: Arrow::comp(&left.arrow, &right.arrow)?,
            phantom: PhantomData,
        })
    }

    fn case(left: &Self, right: &Self) -> Result<Self, types::Error> {
        Ok(ConstructData {
            arrow: Arrow::case(&left.arrow, &right.arrow)?,
            phantom: PhantomData,
        })
    }

    fn assertl(left: &Self, right: Cmr) -> Result<Self, types::Error> {
        Ok(ConstructData {
            arrow: Arrow::assertl(&left.arrow, right)?,
            phantom: PhantomData,
        })
    }

    fn assertr(left: Cmr, right: &Self) -> Result<Self, types::Error> {
        Ok(ConstructData {
            arrow: Arrow::assertr(left, &right.arrow)?,
            phantom: PhantomData,
        })
    }

    fn pair(left: &Self, right: &Self) -> Result<Self, types::Error> {
        Ok(ConstructData {
            arrow: Arrow::pair(&left.arrow, &right.arrow)?,
            phantom: PhantomData,
        })
    }

    fn fail(inference_context: &types::Context, entropy: FailEntropy) -> Self {
        ConstructData {
            arrow: Arrow::fail(inference_context, entropy),
            phantom: PhantomData,
        }
    }

    fn const_word(inference_context: &types::Context, word: Word) -> Self {
        ConstructData {
            arrow: Arrow::const_word(inference_context, word),
            phantom: PhantomData,
        }
    }

    fn inference_context(&self) -> &types::Context {
        self.arrow.inference_context()
    }
}

impl<J: Jet> DisconnectConstructible<Option<Arc<ConstructNode<J>>>> for ConstructData<J> {
    fn disconnect(
        left: &Self,
        right: &Option<Arc<ConstructNode<J>>>,
    ) -> Result<Self, types::Error> {
        let right = right.as_ref();
        Ok(ConstructData {
            arrow: Arrow::disconnect(&left.arrow, &right.map(|n| n.arrow()))?,
            phantom: PhantomData,
        })
    }
}

impl<J> WitnessConstructible<NoWitness> for ConstructData<J> {
    fn witness(inference_context: &types::Context, witness: NoWitness) -> Self {
        ConstructData {
            arrow: Arrow::witness(inference_context, witness),
            phantom: PhantomData,
        }
    }
}

impl<J: Jet> JetConstructible<J> for ConstructData<J> {
    fn jet(inference_context: &types::Context, jet: J) -> Self {
        ConstructData {
            arrow: Arrow::jet(inference_context, jet),
            phantom: PhantomData,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::jet::Core;
    use crate::types::Final;
    use crate::Value;

    #[test]
    fn occurs_check_error() {
        let ctx = types::Context::new();
        let iden = Arc::<ConstructNode<Core>>::iden(&ctx);
        let node = Arc::<ConstructNode<Core>>::disconnect(&iden, &Some(Arc::clone(&iden))).unwrap();

        assert!(matches!(
            node.finalize_types_non_program(),
            Err(crate::Error::Type(types::Error::OccursCheck { .. })),
        ));
    }

    #[test]
    fn occurs_check_2() {
        let ctx = types::Context::new();
        // A more complicated occurs-check test that caused a deadlock in the past.
        let iden = Arc::<ConstructNode<Core>>::iden(&ctx);
        let injr = Arc::<ConstructNode<Core>>::injr(&iden);
        let pair = Arc::<ConstructNode<Core>>::pair(&injr, &iden).unwrap();
        let drop = Arc::<ConstructNode<Core>>::drop_(&pair);

        let case1 = Arc::<ConstructNode<Core>>::case(&drop, &drop).unwrap();
        let case2 = Arc::<ConstructNode<Core>>::case(&case1, &case1).unwrap();

        let comp1 = Arc::<ConstructNode<Core>>::comp(&case2, &case2).unwrap();
        let comp2 = Arc::<ConstructNode<Core>>::comp(&comp1, &case1).unwrap();

        assert!(matches!(
            comp2.finalize_types_non_program(),
            Err(crate::Error::Type(types::Error::OccursCheck { .. })),
        ));
    }

    #[test]
    fn occurs_check_3() {
        let ctx = types::Context::new();
        // A similar example that caused a slightly different deadlock in the past.
        let wit = Arc::<ConstructNode<Core>>::witness(&ctx, NoWitness);
        let drop = Arc::<ConstructNode<Core>>::drop_(&wit);

        let comp1 = Arc::<ConstructNode<Core>>::comp(&drop, &drop).unwrap();
        let comp2 = Arc::<ConstructNode<Core>>::comp(&comp1, &comp1).unwrap();
        let comp3 = Arc::<ConstructNode<Core>>::comp(&comp2, &comp2).unwrap();
        let comp4 = Arc::<ConstructNode<Core>>::comp(&comp3, &comp3).unwrap();
        let comp5 = Arc::<ConstructNode<Core>>::comp(&comp4, &comp4).unwrap();

        let case = Arc::<ConstructNode<Core>>::case(&comp5, &comp4).unwrap();
        let drop2 = Arc::<ConstructNode<Core>>::drop_(&case);
        let case2 = Arc::<ConstructNode<Core>>::case(&drop2, &case).unwrap();
        let comp6 = Arc::<ConstructNode<Core>>::comp(&case2, &case2).unwrap();
        let case3 = Arc::<ConstructNode<Core>>::case(&comp6, &comp6).unwrap();

        let comp7 = Arc::<ConstructNode<Core>>::comp(&case3, &case3).unwrap();
        let comp8 = Arc::<ConstructNode<Core>>::comp(&comp7, &comp7).unwrap();

        assert!(matches!(
            comp8.finalize_types_non_program(),
            Err(crate::Error::Type(types::Error::OccursCheck { .. })),
        ));
    }

    #[test]
    fn type_check_error() {
        let ctx = types::Context::new();
        let unit = Arc::<ConstructNode<Core>>::unit(&ctx);
        let case = Arc::<ConstructNode<Core>>::case(&unit, &unit).unwrap();

        assert!(matches!(
            Arc::<ConstructNode<Core>>::disconnect(&case, &Some(unit)),
            Err(types::Error::Bind { .. }),
        ));
    }

    #[test]
    fn scribe() {
        // Ok to use same type inference context for all the below tests,
        // since everything has concrete types and anyway we only care
        // about CMRs, for which type inference is irrelevant.
        let ctx = types::Context::new();
        let unit = Arc::<ConstructNode<Core>>::unit(&ctx);
        let bit0 = Arc::<ConstructNode<Core>>::const_word(&ctx, Word::u1(0));
        let bit1 = Arc::<ConstructNode<Core>>::const_word(&ctx, Word::u1(1));

        assert_eq!(
            unit.cmr(),
            Arc::<ConstructNode<Core>>::scribe(&ctx, &Value::unit()).cmr()
        );
        assert_eq!(
            bit0.cmr(),
            Arc::<ConstructNode<Core>>::scribe(&ctx, &Value::u1(0)).cmr()
        );
        assert_eq!(
            bit1.cmr(),
            Arc::<ConstructNode<Core>>::scribe(&ctx, &Value::u1(1)).cmr()
        );
        assert_eq!(
            Arc::<ConstructNode<Core>>::const_word(&ctx, Word::u2(1)).cmr(),
            Arc::<ConstructNode<Core>>::scribe(&ctx, &Value::u2(1)).cmr()
        );
        assert_eq!(
            Arc::<ConstructNode<Core>>::injl(&bit0).cmr(),
            Arc::<ConstructNode<Core>>::scribe(&ctx, &Value::left(Value::u1(0), Final::unit()))
                .cmr()
        );
        assert_eq!(
            Arc::<ConstructNode<Core>>::injr(&bit1).cmr(),
            Arc::<ConstructNode<Core>>::scribe(&ctx, &Value::right(Final::unit(), Value::u1(1)))
                .cmr()
        );
        assert_eq!(
            Arc::<ConstructNode<Core>>::pair(&unit, &unit)
                .unwrap()
                .cmr(),
            Arc::<ConstructNode<Core>>::scribe(&ctx, &Value::product(Value::unit(), Value::unit()))
                .cmr()
        );
    }
}
