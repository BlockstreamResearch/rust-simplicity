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

use crate::dag::InternalSharing;
use crate::jet::Jet;
use crate::types::{self, arrow::Arrow};
use crate::{Cmr, Context, FailEntropy, Value};

use std::marker::PhantomData;
use std::sync::Arc;

use super::{CommitData, CommitNode, NoWitness, Node, NodeData};
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
        self.convert::<InternalSharing, _, _, _, _>(
            |data, converted| {
                let converted_data = converted.map(|node| node.cached_data());
                CommitData::new(&data.node.data.arrow, converted_data).map(Arc::new)
            },
            |_, _| Ok(NoWitness),
        )
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
    fn iden(ctx: &mut Context) -> Self {
        ConstructData {
            arrow: Arrow::iden(ctx),
            phantom: PhantomData,
        }
    }

    fn unit(ctx: &mut Context) -> Self {
        ConstructData {
            arrow: Arrow::unit(ctx),
            phantom: PhantomData,
        }
    }

    fn injl(ctx: &mut Context, child: &Self) -> Self {
        ConstructData {
            arrow: Arrow::injl(ctx, &child.arrow),
            phantom: PhantomData,
        }
    }

    fn injr(ctx: &mut Context, child: &Self) -> Self {
        ConstructData {
            arrow: Arrow::injr(ctx, &child.arrow),
            phantom: PhantomData,
        }
    }

    fn take(ctx: &mut Context, child: &Self) -> Self {
        ConstructData {
            arrow: Arrow::take(ctx, &child.arrow),
            phantom: PhantomData,
        }
    }

    fn drop_(ctx: &mut Context, child: &Self) -> Self {
        ConstructData {
            arrow: Arrow::drop_(ctx, &child.arrow),
            phantom: PhantomData,
        }
    }

    fn comp(ctx: &mut Context, left: &Self, right: &Self) -> Result<Self, types::Error> {
        Ok(ConstructData {
            arrow: Arrow::comp(ctx, &left.arrow, &right.arrow)?,
            phantom: PhantomData,
        })
    }

    fn case(ctx: &mut Context, left: &Self, right: &Self) -> Result<Self, types::Error> {
        Ok(ConstructData {
            arrow: Arrow::case(ctx, &left.arrow, &right.arrow)?,
            phantom: PhantomData,
        })
    }

    fn assertl(ctx: &mut Context, left: &Self, right: Cmr) -> Result<Self, types::Error> {
        Ok(ConstructData {
            arrow: Arrow::assertl(ctx, &left.arrow, right)?,
            phantom: PhantomData,
        })
    }

    fn assertr(ctx: &mut Context, left: Cmr, right: &Self) -> Result<Self, types::Error> {
        Ok(ConstructData {
            arrow: Arrow::assertr(ctx, left, &right.arrow)?,
            phantom: PhantomData,
        })
    }

    fn pair(ctx: &mut Context, left: &Self, right: &Self) -> Result<Self, types::Error> {
        Ok(ConstructData {
            arrow: Arrow::pair(ctx, &left.arrow, &right.arrow)?,
            phantom: PhantomData,
        })
    }

    fn disconnect(ctx: &mut Context, left: &Self, right: &Self) -> Result<Self, types::Error> {
        Ok(ConstructData {
            arrow: Arrow::disconnect(ctx, &left.arrow, &right.arrow)?,
            phantom: PhantomData,
        })
    }

    fn fail(ctx: &mut Context, entropy: FailEntropy) -> Self {
        ConstructData {
            arrow: Arrow::fail(ctx, entropy),
            phantom: PhantomData,
        }
    }

    fn const_word(ctx: &mut Context, word: Arc<Value>) -> Self {
        ConstructData {
            arrow: Arrow::const_word(ctx, word),
            phantom: PhantomData,
        }
    }
}

impl<'a, J: Jet> WitnessConstructible<&'a NoWitness> for ConstructData<J> {
    fn witness(ctx: &mut Context, witness: &NoWitness) -> Self {
        ConstructData {
            arrow: Arrow::witness(ctx, *witness),
            phantom: PhantomData,
        }
    }
}

impl<J: Jet> JetConstructible<J> for ConstructData<J> {
    fn jet(ctx: &mut Context, jet: J) -> Self {
        ConstructData {
            arrow: Arrow::jet(ctx, jet),
            phantom: PhantomData,
        }
    }
}
