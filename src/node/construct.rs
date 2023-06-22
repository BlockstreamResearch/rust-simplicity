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
use crate::encode;
use crate::jet::Jet;
use crate::types::{self, arrow::Arrow};
use crate::{BitIter, BitWriter, Cmr, FailEntropy, Value};

use std::io;
use std::marker::PhantomData;
use std::sync::Arc;

use super::{Commit, CommitData, CommitNode, Converter, Inner, NoWitness, Node, NodeData};
use super::{CoreConstructible, JetConstructible, WitnessConstructible};

/// ID used to share [`ConstructNode`]s.
///
/// This is impossible to construct, which is a promise that it is impossible
/// to share [`ConstructNode`]s.
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Debug, Hash)]
pub enum ConstructId {}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Debug, Hash)]
pub enum Construct {}

impl<J: Jet> NodeData<J> for Construct {
    type CachedData = ConstructData<J>;
    type Witness = NoWitness;
    type SharingId = ConstructId;

    fn compute_sharing_id(_: Cmr, _: &ConstructData<J>) -> Option<ConstructId> {
        None
    }
}

pub type ConstructNode<J> = Node<Construct, J>;

impl<J: Jet> ConstructNode<J> {
    /// Accessor for the node's arrow
    pub fn arrow(&self) -> &Arrow {
        &self.data.arrow
    }

    /// Sets the source and target type of the node to unit
    pub fn set_arrow_to_program(&self) -> Result<(), types::Error> {
        let unit_ty = types::Type::unit();
        self.arrow()
            .source
            .unify(&unit_ty, "setting root source to unit")?;
        self.arrow()
            .target
            .unify(&unit_ty, "setting root source to unit")?;
        Ok(())
    }

    /// Convert a [`ConstructNode`] to a [`CommitNode`] by finalizing all of the types.
    ///
    /// Also sets the source and target type of this node to unit. This is almost
    /// certainly what you want, since the resulting `CommitNode` cannot be further
    /// composed, and needs to be 1->1 to go on-chain. But if you don't, call
    /// [`Self::finalize_types_without_fixing`] instead.
    pub fn finalize_types(&self) -> Result<Arc<CommitNode<J>>, types::Error> {
        self.set_arrow_to_program()?;
        self.finalize_types_non_program()
    }

    /// Convert a [`ConstructNode`] to a [`CommitNode`] by finalizing all of the types.
    ///
    /// Does *not* sets the source and target type of this node to unit.
    pub fn finalize_types_non_program(&self) -> Result<Arc<CommitNode<J>>, types::Error> {
        struct FinalizeTypes<J: Jet>(PhantomData<J>);

        impl<J: Jet> Converter<Construct, Commit, J> for FinalizeTypes<J> {
            type Error = types::Error;
            fn convert_witness(
                &mut self,
                _: &PostOrderIterItem<&ConstructNode<J>>,
                _: &NoWitness,
            ) -> Result<NoWitness, Self::Error> {
                Ok(NoWitness)
            }

            fn convert_data(
                &mut self,
                data: &PostOrderIterItem<&ConstructNode<J>>,
                inner: Inner<&Arc<CommitNode<J>>, J, &NoWitness>,
            ) -> Result<Arc<CommitData<J>>, Self::Error> {
                let converted_data = inner.map(|node| node.cached_data());
                CommitData::new(&data.node.data.arrow, converted_data).map(Arc::new)
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
    /// If the serialization contains the witness data, then use [`RedeemNode::decode()`].
    pub fn decode<I: Iterator<Item = u8>>(
        bits: &mut BitIter<I>,
    ) -> Result<Arc<Self>, crate::decode::Error> {
        crate::decode::decode_expression(bits)
    }

    /// Encode a Simplicity expression to bits, with no witness data
    pub fn encode<W: io::Write>(&self, w: &mut BitWriter<W>) -> io::Result<usize> {
        let program_bits = encode::encode_program(self, w)?;
        w.flush_all()?;
        Ok(program_bits)
    }
}

#[derive(Clone, Debug)]
pub struct ConstructData<J: Jet> {
    arrow: Arrow,
    /// This isn't really necessary, but it helps type inference if every
    /// struct has a <J> parameter, since it forces the choice of jets to
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
}

impl<J: Jet> CoreConstructible for ConstructData<J> {
    fn iden() -> Self {
        ConstructData {
            arrow: Arrow::iden(),
            phantom: PhantomData,
        }
    }

    fn unit() -> Self {
        ConstructData {
            arrow: Arrow::unit(),
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

    fn disconnect(left: &Self, right: &Self) -> Result<Self, types::Error> {
        Ok(ConstructData {
            arrow: Arrow::disconnect(&left.arrow, &right.arrow)?,
            phantom: PhantomData,
        })
    }

    fn fail(entropy: FailEntropy) -> Self {
        ConstructData {
            arrow: Arrow::fail(entropy),
            phantom: PhantomData,
        }
    }

    fn const_word(word: Arc<Value>) -> Self {
        ConstructData {
            arrow: Arrow::const_word(word),
            phantom: PhantomData,
        }
    }
}

impl<'a, J: Jet> WitnessConstructible<&'a NoWitness> for ConstructData<J> {
    fn witness(witness: &NoWitness) -> Self {
        ConstructData {
            arrow: Arrow::witness(*witness),
            phantom: PhantomData,
        }
    }
}

impl<J: Jet> JetConstructible<J> for ConstructData<J> {
    fn jet(jet: J) -> Self {
        ConstructData {
            arrow: Arrow::jet(jet),
            phantom: PhantomData,
        }
    }
}
