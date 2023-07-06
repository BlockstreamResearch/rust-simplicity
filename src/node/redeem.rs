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

use crate::analysis::NodeBounds;
use crate::dag::{DagLike, InternalSharing, MaxSharing, PostOrderIterItem};
use crate::encode;
use crate::jet::Jet;
use crate::types::{self, arrow::FinalArrow};
use crate::{Amr, BitIter, BitWriter, Cmr, Error, FirstPassImr, Imr, Value};

use super::{Commit, CommitData, CommitNode, Converter, Inner, Marker, NoWitness, Node};

use std::collections::HashSet;
use std::io;
use std::marker::PhantomData;
use std::sync::Arc;

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Debug, Hash)]
pub struct Redeem<J> {
    /// Makes the type non-constructible.
    never: std::convert::Infallible,
    /// Required by Rust.
    phantom: std::marker::PhantomData<J>,
}

impl<J: Jet> Marker for Redeem<J> {
    type CachedData = Arc<RedeemData<J>>;
    type Witness = Arc<Value>;
    type SharingId = Imr;
    type Jet = J;

    fn compute_sharing_id(_: Cmr, cached_data: &Arc<RedeemData<J>>) -> Option<Imr> {
        Some(cached_data.imr)
    }
}

pub type RedeemNode<J> = Node<Redeem<J>>;

#[derive(Clone, Debug)]
pub struct RedeemData<J> {
    amr: Amr,
    first_pass_imr: FirstPassImr,
    imr: Imr,
    arrow: FinalArrow,
    bounds: NodeBounds,
    /// This isn't really necessary, but it helps type inference if every
    /// struct has a <J> parameter, since it forces the choice of jets to
    /// be consistent without the user needing to specify it too many times.
    phantom: PhantomData<J>,
}

impl<J> PartialEq for RedeemData<J> {
    fn eq(&self, other: &Self) -> bool {
        self.imr == other.imr
    }
}
impl<J> Eq for RedeemData<J> {}

impl<J> std::hash::Hash for RedeemData<J> {
    fn hash<H: std::hash::Hasher>(&self, hasher: &mut H) {
        self.imr.hash(hasher)
    }
}

impl<J: Jet> RedeemData<J> {
    pub fn new(arrow: FinalArrow, inner: Inner<&Arc<Self>, J, Arc<Value>>) -> Self {
        let (amr, first_pass_imr, bounds) = match inner {
            Inner::Iden => (Amr::iden(&arrow), FirstPassImr::iden(), NodeBounds::iden()),
            Inner::Unit => (Amr::unit(&arrow), FirstPassImr::unit(), NodeBounds::unit()),
            Inner::InjL(child) => (
                Amr::injl(&arrow, child.amr),
                FirstPassImr::injl(child.first_pass_imr),
                NodeBounds::injl(child.bounds),
            ),
            Inner::InjR(child) => (
                Amr::injr(&arrow, child.amr),
                FirstPassImr::injr(child.first_pass_imr),
                NodeBounds::injr(child.bounds),
            ),
            Inner::Take(child) => (
                Amr::take(&arrow, child.amr),
                FirstPassImr::take(child.first_pass_imr),
                NodeBounds::take(child.bounds),
            ),
            Inner::Drop(child) => (
                Amr::drop(&arrow, child.amr),
                FirstPassImr::drop(child.first_pass_imr),
                NodeBounds::drop(child.bounds),
            ),
            Inner::Comp(left, right) => (
                Amr::comp(&arrow, &left.arrow, left.amr, right.amr),
                FirstPassImr::comp(left.first_pass_imr, right.first_pass_imr),
                NodeBounds::comp(left.bounds, right.bounds, left.arrow.target.bit_width()),
            ),
            Inner::Case(left, right) => (
                Amr::case(&arrow, left.amr, right.amr),
                FirstPassImr::case(left.first_pass_imr, right.first_pass_imr),
                NodeBounds::case(left.bounds, right.bounds),
            ),
            Inner::AssertL(left, r_cmr) => (
                Amr::assertl(&arrow, left.amr, r_cmr.into()),
                FirstPassImr::case(left.first_pass_imr, r_cmr.into()),
                NodeBounds::assertl(left.bounds),
            ),
            Inner::AssertR(l_cmr, right) => (
                Amr::assertr(&arrow, l_cmr.into(), right.amr),
                FirstPassImr::case(l_cmr.into(), right.first_pass_imr),
                NodeBounds::assertr(right.bounds),
            ),
            Inner::Pair(left, right) => (
                Amr::pair(&arrow, &left.arrow, &right.arrow, left.amr, right.amr),
                FirstPassImr::pair(left.first_pass_imr, right.first_pass_imr),
                NodeBounds::pair(left.bounds, right.bounds),
            ),
            Inner::Disconnect(left, right) => (
                Amr::disconnect(&arrow, &right.arrow, left.amr, right.amr),
                FirstPassImr::disconnect(left.first_pass_imr, right.first_pass_imr),
                NodeBounds::disconnect(
                    left.bounds,
                    right.bounds,
                    left.arrow.source.bit_width(),
                    left.arrow.target.bit_width(),
                ),
            ),
            Inner::Witness(ref value) => (
                Amr::witness(&arrow, value),
                FirstPassImr::witness(&arrow, value),
                NodeBounds::witness(arrow.target.bit_width()),
            ),
            Inner::Fail(entropy) => (
                Amr::fail(entropy),
                FirstPassImr::fail(entropy),
                NodeBounds::fail(),
            ),
            Inner::Jet(jet) => (Amr::jet(jet), FirstPassImr::jet(jet), NodeBounds::jet()),
            Inner::Word(ref val) => (
                Amr::const_word(val),
                FirstPassImr::const_word(val),
                NodeBounds::const_word(),
            ),
        };

        RedeemData {
            amr,
            first_pass_imr,
            imr: Imr::compute_pass2(first_pass_imr, &arrow),
            arrow,
            bounds,
            phantom: PhantomData,
        }
    }
}

impl<J: Jet> RedeemNode<J> {
    /// Accessor for the node's AMR
    pub fn amr(&self) -> Amr {
        self.data.amr
    }

    /// Accessor for the node's IMR
    pub fn imr(&self) -> Imr {
        self.data.imr
    }

    /// Accessor for the node's type arrow
    pub fn arrow(&self) -> &FinalArrow {
        &self.data.arrow
    }

    /// Accessor for the node's bit machine bounds
    pub fn bounds(&self) -> NodeBounds {
        self.data.bounds
    }

    /// Convert a [`RedeemNode`] back to a [`CommitNode`] by forgetting witnesses
    /// and cached data.
    pub fn unfinalize(&self) -> Result<Arc<CommitNode<J>>, types::Error> {
        struct Unfinalizer<J>(PhantomData<J>);

        impl<J: Jet> Converter<Redeem<J>, Commit<J>> for Unfinalizer<J> {
            type Error = types::Error;
            fn convert_witness(
                &mut self,
                _: &PostOrderIterItem<&RedeemNode<J>>,
                _: &Arc<Value>,
            ) -> Result<NoWitness, Self::Error> {
                Ok(NoWitness)
            }

            fn convert_data(
                &mut self,
                data: &PostOrderIterItem<&RedeemNode<J>>,
                inner: Inner<&Arc<CommitNode<J>>, J, &NoWitness>,
            ) -> Result<Arc<CommitData<J>>, Self::Error> {
                let converted_data = inner.map(|node| node.cached_data());
                Ok(Arc::new(CommitData::from_final(
                    data.node.data.arrow.shallow_clone(),
                    converted_data,
                )))
            }
        }

        self.convert::<MaxSharing<Redeem<J>>, _, _>(&mut Unfinalizer(PhantomData))
    }

    /// Decode a Simplicity program from bits, including the witness data.
    pub fn decode<I: Iterator<Item = u8>>(bits: &mut BitIter<I>) -> Result<Arc<Self>, Error> {
        // 0. Set up a type to help with the call to `convert` below
        struct DecodeFinalizer<'bits, J: Jet, I: Iterator<Item = u8>> {
            bits: &'bits mut BitIter<I>,
            phantom: PhantomData<J>,
        }

        impl<'bits, J: Jet, I: Iterator<Item = u8>> Converter<Commit<J>, Redeem<J>>
            for DecodeFinalizer<'bits, J, I>
        {
            type Error = Error;
            fn convert_witness(
                &mut self,
                data: &PostOrderIterItem<&CommitNode<J>>,
                _: &NoWitness,
            ) -> Result<Arc<Value>, Self::Error> {
                self.bits
                    .read_value(&data.node.data.arrow().target)
                    .map(Arc::new)
                    .map_err(Error::from)
            }

            fn convert_data(
                &mut self,
                data: &PostOrderIterItem<&CommitNode<J>>,
                inner: Inner<&Arc<RedeemNode<J>>, J, &Arc<Value>>,
            ) -> Result<Arc<RedeemData<J>>, Self::Error> {
                let converted_data = inner.map(|node| node.cached_data()).map_witness(Arc::clone);
                Ok(Arc::new(RedeemData::new(
                    data.node.arrow().shallow_clone(),
                    converted_data,
                )))
            }
        }

        // 1. Decode program without witnesses as CommitNode
        let commit = crate::decode::decode_program(bits)?;

        // 2. Convert to RedeemNode, reading witnesses as we go
        let witness_len = if bits.read_bit()? {
            bits.read_natural(None)?
        } else {
            0
        };
        let witness_start = bits.n_total_read();

        // Importantly, we  use `InternalSharing` here to make sure that we respect
        // the sharing choices that were actually encoded in the bitstream.
        let program: Arc<Self> = commit.convert::<InternalSharing, _, _>(&mut DecodeFinalizer {
            bits,
            phantom: PhantomData,
        })?;

        // 3. Check that we read exactly as much witness data as we expected
        if bits.n_total_read() != witness_start + witness_len {
            return Err(Error::InconsistentWitnessLength);
        }

        // 4. Check sharing
        // This loop is equivalent to using `program.is_shared_as::<MaxSharing>()`
        // but is faster since it only runs a single iterator.
        let mut imrs: HashSet<Imr> = HashSet::new();
        for data in program.as_ref().post_order_iter::<InternalSharing>() {
            if !imrs.insert(data.node.imr()) {
                return Err(Error::Decode(crate::decode::Error::SharingNotMaximal));
            }
        }

        Ok(program)
    }

    /// Encode a Simplicity program to bits, including the witness data.
    pub fn encode<W: io::Write>(&self, w: &mut BitWriter<W>) -> io::Result<usize> {
        let sharing_iter = self.post_order_iter::<MaxSharing<Redeem<J>>>();
        let program_bits = encode::encode_program(self, w)?;
        let witness_bits =
            encode::encode_witness(sharing_iter.into_witnesses().map(Arc::as_ref), w)?;
        w.flush_all()?;
        Ok(program_bits + witness_bits)
    }

    /// Encode a Simplicity program to a vector of bytes, including the witness data.
    pub fn encode_to_vec(&self) -> Vec<u8> {
        let mut program_and_witness_bytes = Vec::<u8>::new();
        let mut writer = BitWriter::new(&mut program_and_witness_bytes);
        self.encode(&mut writer)
            .expect("write to vector never fails");
        debug_assert!(!program_and_witness_bytes.is_empty());

        program_and_witness_bytes
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::jet::Core;
    use crate::node::SimpleFinalizer;

    #[test]
    fn encode_shared_witnesses() {
        // # Program code:
        // wit1 = witness :: 1 -> 2^32
        // wit2 = witness :: 1 -> 2^32
        //
        // wits_are_equal = comp (pair wit1 wit2) jet_eq_32 :: 1 -> 2
        // main = comp wits_are_equal jet_verify            :: 1 -> 1
        let eqwits = vec![0xcd, 0xdc, 0x51, 0xb6, 0xe2, 0x08, 0xc0, 0x40];
        let mut iter = BitIter::from(&eqwits[..]);
        let eqwits_prog = CommitNode::<Core>::decode(&mut iter).unwrap();

        let eqwits_final = eqwits_prog
            .finalize(&mut SimpleFinalizer::new(std::iter::repeat(Arc::new(
                Value::u32(0xDEADBEEF),
            ))))
            .unwrap();
        let output = eqwits_final.encode_to_vec();

        assert_eq!(
            output,
            [0xc9, 0xc4, 0x6d, 0xb8, 0x82, 0x30, 0x11, 0xe2, 0x0d, 0xea, 0xdb, 0xee, 0xf0],
        );
    }

    #[test]
    fn decode_shared_witnesses() {
        // This program is exactly the output from the `encode_shared_witnesses` test.
        // The point of this is to make sure that our witness-unsharing logic doesn't
        // get confused here and try to read two witnesses when there are only one.
        let eqwits = vec![
            0xc9, 0xc4, 0x6d, 0xb8, 0x82, 0x30, 0x11, 0xe2, 0x0d, 0xea, 0xdb, 0xee, 0xf0,
        ];
        let mut iter = BitIter::from(&eqwits[..]);
        RedeemNode::<crate::jet::Core>::decode(&mut iter).unwrap();
    }

    #[test]
    fn unshared_child() {
        // # id1 and id2 should be shared, but are not!
        // id1 = iden          :: A -> A # cmr dbfefcfc...
        // id2 = iden          :: A -> A # cmr dbfefcfc...
        // cp3 = comp id1 id2  :: A -> A # cmr c1ae55b5...
        // main = comp cp3 cp3 :: A -> A # cmr 314e2879...
        let bad = [0xc1, 0x08, 0x04, 0x00, 0x00, 0x74, 0x74, 0x74];
        let mut iter = BitIter::from(&bad[..]);
        let err = RedeemNode::<crate::jet::Core>::decode(&mut iter).unwrap_err();
        assert!(matches!(
            err,
            Error::Decode(crate::decode::Error::SharingNotMaximal)
        ));
    }

    #[test]
    fn witness_consumed() {
        // "main = unit", but with a witness attached. Found by fuzzer.
        let badwit = vec![0x27, 0x00];
        let mut iter = BitIter::from(&badwit[..]);
        if let Err(Error::InconsistentWitnessLength) =
            RedeemNode::<crate::jet::Core>::decode(&mut iter)
        {
            // ok
        } else {
            panic!("accepted program with bad witness length")
        }
    }
}
