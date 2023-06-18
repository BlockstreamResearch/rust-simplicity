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

use crate::dag::{DagLike, MaxSharing, PostOrderIterItem};
use crate::jet::Jet;
use crate::types;
use crate::types::arrow::{Arrow, FinalArrow};
use crate::{BitIter, Cmr, Error, FirstPassImr, Imr, Value};

use super::{
    Construct, ConstructData, ConstructNode, Constructible, Converter, Inner, NoWitness, Node,
    NodeData, Redeem, RedeemData, RedeemNode,
};

use std::marker::PhantomData;
use std::sync::Arc;

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Debug, Hash)]
pub enum Commit {}

impl<J: Jet> NodeData<J> for Commit {
    type CachedData = Arc<CommitData<J>>;
    type Witness = NoWitness;
    type SharingId = Imr;

    fn compute_sharing_id(_: Cmr, cached_data: &Arc<CommitData<J>>) -> Option<Imr> {
        cached_data.imr
    }
}

#[derive(Clone, Debug)]
pub struct CommitData<J: Jet> {
    /// The source and target types of the node
    arrow: FinalArrow,
    /// The first-pass IMR of the node if it exists.
    first_pass_imr: Option<FirstPassImr>,
    /// The IMR of the node if it exists, meaning, if it is not (an ancestor of)
    /// a witness or disconnect node.
    imr: Option<Imr>,
    /// This isn't really necessary, but it helps type inference if every
    /// struct has a <J> parameter, since it forces the choice of jets to
    /// be consistent without the user needing to specify it too many times.
    phantom: PhantomData<J>,
}

impl<J: Jet> PartialEq for CommitData<J> {
    // Two nodes are equal if they both have IMRs and those IMRs are equal.
    fn eq(&self, other: &Self) -> bool {
        self.imr
            .zip(other.imr)
            .map(|(a, b)| a == b)
            .unwrap_or(false)
    }
}

impl<J: Jet> CommitData<J> {
    /// Accessor for the node's arrow
    pub fn arrow(&self) -> &FinalArrow {
        &self.arrow
    }

    /// Helper function to compute a cached first-pass IMR
    fn first_pass_imr(inner: Inner<&Arc<Self>, J, &NoWitness>) -> Option<FirstPassImr> {
        match inner {
            Inner::Iden => Some(FirstPassImr::iden()),
            Inner::Unit => Some(FirstPassImr::unit()),
            Inner::InjL(child) => child.first_pass_imr.map(FirstPassImr::injl),
            Inner::InjR(child) => child.first_pass_imr.map(FirstPassImr::injr),
            Inner::Take(child) => child.first_pass_imr.map(FirstPassImr::take),
            Inner::Drop(child) => child.first_pass_imr.map(FirstPassImr::drop),
            Inner::Comp(left, right) => left
                .first_pass_imr
                .zip(right.first_pass_imr)
                .map(|(a, b)| FirstPassImr::comp(a, b)),
            Inner::Case(left, right) => left
                .first_pass_imr
                .zip(right.first_pass_imr)
                .map(|(a, b)| FirstPassImr::case(a, b)),
            Inner::AssertL(left, r_cmr) => left
                .first_pass_imr
                .map(|l_imr| FirstPassImr::case(l_imr, r_cmr.into())),
            Inner::AssertR(l_cmr, right) => right
                .first_pass_imr
                .map(|r_imr| FirstPassImr::case(l_cmr.into(), r_imr)),
            Inner::Pair(left, right) => left
                .first_pass_imr
                .zip(right.first_pass_imr)
                .map(|(a, b)| FirstPassImr::pair(a, b)),
            Inner::Disconnect(..) => None,
            Inner::Witness(..) => None,
            Inner::Fail(entropy) => Some(FirstPassImr::fail(entropy)),
            Inner::Jet(jet) => Some(FirstPassImr::jet(jet)),
            Inner::Word(ref val) => Some(FirstPassImr::const_word(val)),
        }
    }

    pub fn new(
        arrow: &Arrow,
        inner: Inner<&Arc<Self>, J, &NoWitness>,
    ) -> Result<Self, types::Error> {
        let final_arrow = arrow.finalize()?;
        let first_pass_imr = Self::first_pass_imr(inner);
        Ok(CommitData {
            first_pass_imr,
            imr: first_pass_imr.map(|imr| Imr::compute_pass2(imr, &final_arrow)),
            arrow: final_arrow,
            phantom: PhantomData,
        })
    }

    pub fn from_final(arrow: FinalArrow, inner: Inner<&Arc<Self>, J, &NoWitness>) -> Self {
        let first_pass_imr = Self::first_pass_imr(inner);
        CommitData {
            first_pass_imr,
            imr: first_pass_imr.map(|imr| Imr::compute_pass2(imr, &arrow)),
            arrow,
            phantom: PhantomData,
        }
    }
}

pub type CommitNode<J> = Node<Commit, J>;

impl<J: Jet> CommitNode<J> {
    /// Accessor for the node's arrow
    pub fn arrow(&self) -> &FinalArrow {
        &self.data.arrow
    }

    /// Accessor for the node's IMR, if known
    pub fn imr(&self) -> Option<Imr> {
        self.data.imr
    }

    /// Finalize the [`CommitNode`], assuming it has no witnesses. Does not do any
    /// pruning.
    ///
    /// It will populate unit witnesses, to make this method a bit more useful for
    /// testing purposes. But for any other witness nodes, it will error out with
    /// [`Error::NoMoreWitnesses`].
    pub fn finalize_no_witnesses(&self) -> Result<Arc<RedeemNode<J>>, Error> {
        struct SimpleFinalizer<J: Jet>(PhantomData<J>);

        impl<J: Jet> Converter<Commit, Redeem, J> for SimpleFinalizer<J> {
            type Error = Error;
            fn convert_witness(
                &mut self,
                data: &PostOrderIterItem<&CommitNode<J>>,
                _: &NoWitness,
            ) -> Result<Arc<Value>, Self::Error> {
                if data.node.arrow().target.is_unit() {
                    Ok(Arc::new(Value::Unit))
                } else {
                    Err(Error::NoMoreWitnesses)
                }
            }

            fn convert_data(
                &mut self,
                data: &PostOrderIterItem<&CommitNode<J>>,
                inner: Inner<&Arc<RedeemNode<J>>, J, &Arc<Value>>,
            ) -> Result<Arc<RedeemData<J>>, Self::Error> {
                let converted_data = inner.map(|node| node.cached_data()).map_witness(Arc::clone);
                Ok(Arc::new(RedeemData::new(
                    data.node.data.arrow.shallow_clone(),
                    converted_data,
                )))
            }
        }

        self.convert::<MaxSharing<Commit, J>, _, _>(&mut SimpleFinalizer(PhantomData))
    }

    /// Convert a [`CommitNode`] back to a [`ConstructNode`] by redoing type inference
    pub fn unfinalize_types(&self) -> Result<Arc<ConstructNode<J>>, types::Error> {
        struct UnfinalizeTypes<J: Jet>(PhantomData<J>);

        impl<J: Jet> Converter<Commit, Construct, J> for UnfinalizeTypes<J> {
            type Error = types::Error;
            fn convert_witness(
                &mut self,
                _: &PostOrderIterItem<&CommitNode<J>>,
                _: &NoWitness,
            ) -> Result<NoWitness, Self::Error> {
                Ok(NoWitness)
            }

            fn convert_data(
                &mut self,
                _: &PostOrderIterItem<&CommitNode<J>>,
                inner: Inner<&Arc<ConstructNode<J>>, J, &NoWitness>,
            ) -> Result<ConstructData<J>, Self::Error> {
                let inner = inner.map(|node| node.arrow());
                Ok(ConstructData::new(Arrow::from_inner(inner)?))
            }
        }

        self.convert::<MaxSharing<Commit, J>, _, _>(&mut UnfinalizeTypes(PhantomData))
    }

    /// Decode a Simplicity program from bits, without witness data.
    ///
    /// # Usage
    ///
    /// Use this method only if the serialization **does not** include the witness data.
    /// This means, the program simply has no witness during commitment,
    /// or the witness is provided by other means.
    ///
    /// If the serialization contains the witness data, then use [`RedeemNode::decode()`].
    pub fn decode<I: Iterator<Item = u8>>(bits: &mut BitIter<I>) -> Result<Arc<Self>, Error> {
        // 1. Decode program with out witnesses.
        let program = crate::decode::decode_program(bits)?;
        // 2. Do sharing check, using incomplete IMRs
        if program.as_ref().is_shared_as::<MaxSharing<Commit, J>>() {
            Ok(program)
        } else {
            Err(Error::SharingNotMaximal)
        }
    }
}
