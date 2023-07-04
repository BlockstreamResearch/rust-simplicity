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

use crate::dag::{DagLike, MaxSharing, NoSharing, PostOrderIterItem};
use crate::jet::Jet;
use crate::types::arrow::{Arrow, FinalArrow};
use crate::{encode, types};
use crate::{Amr, BitIter, BitWriter, Cmr, Error, FirstPassImr, Imr};

use super::{
    Construct, ConstructData, ConstructNode, Constructible, Converter, Inner, Marker, NoWitness,
    Node, Redeem, RedeemNode,
};

use std::io;
use std::marker::PhantomData;
use std::sync::Arc;

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Debug, Hash)]
pub struct Commit<J: Jet> {
    /// Makes the type non-constructible.
    never: std::convert::Infallible,
    /// Required by Rust.
    phantom: std::marker::PhantomData<J>,
}

impl<J: Jet> Marker for Commit<J> {
    type CachedData = Arc<CommitData<J>>;
    type Witness = NoWitness;
    type SharingId = Imr;
    type Jet = J;

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
    /// The AMR of the node if it exists, meaning, if it is not (an ancestor of)
    /// a witness or disconnect node.
    amr: Option<Amr>,
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

    /// Helper function to compute a cached AMR
    fn incomplete_amr(inner: Inner<&Arc<Self>, J, &NoWitness>, arrow: &FinalArrow) -> Option<Amr> {
        match inner {
            Inner::Iden => Some(Amr::iden(arrow)),
            Inner::Unit => Some(Amr::unit(arrow)),
            Inner::InjL(child) => child.amr.map(|amr| Amr::injl(arrow, amr)),
            Inner::InjR(child) => child.amr.map(|amr| Amr::injr(arrow, amr)),
            Inner::Take(child) => child.amr.map(|amr| Amr::take(arrow, amr)),
            Inner::Drop(child) => child.amr.map(|amr| Amr::drop(arrow, amr)),
            Inner::Comp(left, right) => left
                .amr
                .zip(right.amr)
                .map(|(a, b)| Amr::comp(arrow, &left.arrow, a, b)),
            Inner::Case(left, right) => {
                left.amr.zip(right.amr).map(|(a, b)| Amr::case(arrow, a, b))
            }
            Inner::AssertL(left, r_cmr) => left
                .amr
                .map(|l_amr| Amr::assertl(arrow, l_amr, r_cmr.into())),
            Inner::AssertR(l_cmr, right) => right
                .amr
                .map(|r_amr| Amr::assertr(arrow, l_cmr.into(), r_amr)),
            Inner::Pair(left, right) => left
                .amr
                .zip(right.amr)
                .map(|(a, b)| Amr::pair(arrow, &left.arrow, &right.arrow, a, b)),
            Inner::Disconnect(..) => None,
            Inner::Witness(..) => None,
            Inner::Fail(entropy) => Some(Amr::fail(entropy)),
            Inner::Jet(jet) => Some(Amr::jet(jet)),
            Inner::Word(ref val) => Some(Amr::const_word(val)),
        }
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
        let first_pass_imr = Self::first_pass_imr(inner.clone());
        let amr = Self::incomplete_amr(inner, &final_arrow);
        Ok(CommitData {
            first_pass_imr,
            amr,
            imr: first_pass_imr.map(|imr| Imr::compute_pass2(imr, &final_arrow)),
            arrow: final_arrow,
            phantom: PhantomData,
        })
    }

    pub fn from_final(arrow: FinalArrow, inner: Inner<&Arc<Self>, J, &NoWitness>) -> Self {
        let first_pass_imr = Self::first_pass_imr(inner.clone());
        let amr = Self::incomplete_amr(inner, &arrow);
        CommitData {
            first_pass_imr,
            amr,
            imr: first_pass_imr.map(|imr| Imr::compute_pass2(imr, &arrow)),
            arrow,
            phantom: PhantomData,
        }
    }
}

pub type CommitNode<J> = Node<Commit<J>>;

impl<J: Jet> CommitNode<J> {
    /// Accessor for the node's arrow
    pub fn arrow(&self) -> &FinalArrow {
        &self.data.arrow
    }

    /// Accessor for the node's AMR, if known
    pub fn amr(&self) -> Option<Amr> {
        self.data.amr
    }

    /// Accessor for the node's IMR, if known
    pub fn imr(&self) -> Option<Imr> {
        self.data.imr
    }

    /// Finalizes a DAG, by iterating through through it without sharing, attaching
    /// witnesses, and hiding branches.
    ///
    /// This is a thin wrapper around [`Node::convert`] which fixes a few types to make
    /// it easier to use.
    pub fn finalize<C: Converter<Commit<J>, Redeem<J>>>(
        &self,
        converter: &mut C,
    ) -> Result<Arc<RedeemNode<J>>, C::Error> {
        self.convert::<NoSharing, Redeem<J>, _>(converter)
    }

    /// Convert a [`CommitNode`] back to a [`ConstructNode`] by redoing type inference
    pub fn unfinalize_types(&self) -> Result<Arc<ConstructNode<J>>, types::Error> {
        struct UnfinalizeTypes<J: Jet>(PhantomData<J>);

        impl<J: Jet> Converter<Commit<J>, Construct<J>> for UnfinalizeTypes<J> {
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

        self.convert::<MaxSharing<Commit<J>>, _, _>(&mut UnfinalizeTypes(PhantomData))
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
        if program.as_ref().is_shared_as::<MaxSharing<Commit<J>>>() {
            Ok(program)
        } else {
            Err(Error::SharingNotMaximal)
        }
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
