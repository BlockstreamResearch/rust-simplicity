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

use crate::jet::Jet;
use crate::types::{self, arrow::Arrow};
use crate::{Cmr, Context, FailEntropy, Value};

use std::marker::PhantomData;
use std::sync::Arc;

use super::{Constructible, NoWitness, Node, NodeData};

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

impl<'a, J: Jet> Constructible<&'a NoWitness, J> for ConstructData<J> {
    fn iden(ctx: &mut Context<J>) -> Self {
        ConstructData {
            arrow: Arrow::iden(ctx),
            phantom: PhantomData,
        }
    }

    fn unit(ctx: &mut Context<J>) -> Self {
        ConstructData {
            arrow: Arrow::unit(ctx),
            phantom: PhantomData,
        }
    }

    fn injl(ctx: &mut Context<J>, child: &Self) -> Self {
        ConstructData {
            arrow: Arrow::injl(ctx, &child.arrow),
            phantom: PhantomData,
        }
    }

    fn injr(ctx: &mut Context<J>, child: &Self) -> Self {
        ConstructData {
            arrow: Arrow::injr(ctx, &child.arrow),
            phantom: PhantomData,
        }
    }

    fn take(ctx: &mut Context<J>, child: &Self) -> Self {
        ConstructData {
            arrow: Arrow::take(ctx, &child.arrow),
            phantom: PhantomData,
        }
    }

    fn drop_(ctx: &mut Context<J>, child: &Self) -> Self {
        ConstructData {
            arrow: Arrow::drop_(ctx, &child.arrow),
            phantom: PhantomData,
        }
    }

    fn comp(ctx: &mut Context<J>, left: &Self, right: &Self) -> Result<Self, types::Error> {
        Ok(ConstructData {
            arrow: Arrow::comp(ctx, &left.arrow, &right.arrow)?,
            phantom: PhantomData,
        })
    }

    fn case(ctx: &mut Context<J>, left: &Self, right: &Self) -> Result<Self, types::Error> {
        Ok(ConstructData {
            arrow: Arrow::case(ctx, &left.arrow, &right.arrow)?,
            phantom: PhantomData,
        })
    }

    fn assertl(ctx: &mut Context<J>, left: &Self, right: Cmr) -> Result<Self, types::Error> {
        Ok(ConstructData {
            arrow: Arrow::assertl(ctx, &left.arrow, right)?,
            phantom: PhantomData,
        })
    }

    fn assertr(ctx: &mut Context<J>, left: Cmr, right: &Self) -> Result<Self, types::Error> {
        Ok(ConstructData {
            arrow: Arrow::assertr(ctx, left, &right.arrow)?,
            phantom: PhantomData,
        })
    }

    fn pair(ctx: &mut Context<J>, left: &Self, right: &Self) -> Result<Self, types::Error> {
        Ok(ConstructData {
            arrow: Arrow::pair(ctx, &left.arrow, &right.arrow)?,
            phantom: PhantomData,
        })
    }

    fn disconnect(ctx: &mut Context<J>, left: &Self, right: &Self) -> Result<Self, types::Error> {
        Ok(ConstructData {
            arrow: Arrow::disconnect(ctx, &left.arrow, &right.arrow)?,
            phantom: PhantomData,
        })
    }

    fn witness(ctx: &mut Context<J>, witness: &NoWitness) -> Self {
        ConstructData {
            arrow: Arrow::witness(ctx, *witness),
            phantom: PhantomData,
        }
    }

    fn fail(ctx: &mut Context<J>, entropy: FailEntropy) -> Self {
        ConstructData {
            arrow: Arrow::fail(ctx, entropy),
            phantom: PhantomData,
        }
    }

    fn jet(ctx: &mut Context<J>, jet: J) -> Self {
        ConstructData {
            arrow: Arrow::jet(ctx, jet),
            phantom: PhantomData,
        }
    }

    fn const_word(ctx: &mut Context<J>, word: Arc<Value>) -> Self {
        ConstructData {
            arrow: Arrow::const_word(ctx, word),
            phantom: PhantomData,
        }
    }
}
