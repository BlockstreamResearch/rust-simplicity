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
use crate::dag::{DagLike, InternalSharing, MaxSharing};
use crate::jet::Jet;
use crate::types::{self, arrow::FinalArrow};
use crate::{Amr, BitIter, Cmr, Error, FirstPassImr, Imr, Value};

use super::{CommitData, CommitNode, Inner, NoWitness, Node, NodeData};

use std::collections::HashSet;
use std::marker::PhantomData;
use std::sync::Arc;

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Debug, Hash)]
pub enum Redeem {}

impl<J: Jet> NodeData<J> for Redeem {
    type CachedData = Arc<RedeemData<J>>;
    type Witness = Arc<Value>;
    type SharingId = Imr;

    fn compute_sharing_id(_: Cmr, cached_data: &Arc<RedeemData<J>>) -> Option<Imr> {
        Some(cached_data.imr)
    }
}

pub type RedeemNode<J> = Node<Redeem, J>;

#[derive(Clone, Debug)]
pub struct RedeemData<J: Jet> {
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

impl<J: Jet> PartialEq for RedeemData<J> {
    fn eq(&self, other: &Self) -> bool {
        self.imr == other.imr
    }
}
impl<J: Jet> Eq for RedeemData<J> {}

impl<J: Jet> std::hash::Hash for RedeemData<J> {
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
                Amr::disconnect(&arrow, &left.arrow, left.amr, right.amr),
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
        self.convert::<MaxSharing<Redeem, J>, _, _, _, _>(
            |data, converted| {
                let converted_data = converted.map(|node| node.cached_data());
                Ok(Arc::new(CommitData::from_final(
                    data.node.data.arrow.shallow_clone(),
                    converted_data,
                )))
            },
            |_, _| Ok(NoWitness),
        )
    }

    /// Decode a Simplicity program from bits, including the witness data.
    pub fn decode<I: Iterator<Item = u8>>(bits: &mut BitIter<I>) -> Result<Arc<Self>, Error> {
        // 1. Decode program without witnesses as CommitNode
        let commit = crate::decode::decode_program(bits)?;

        // 2. Convert to RedeemNode, reading witnesses as we go
        let witness_len = if bits.read_bit()? {
            bits.read_natural(None)?
        } else {
            0
        };
        let witness_start = bits.n_total_read();

        // Importantly, we do this directly using convert() rather than calling
        // `.finalize()` because we need to use `InternalSharing` to make sure
        // that we respect the sharing choices that were actually encoded in the
        // bitstream.
        let program: Arc<Self> = commit.convert::<InternalSharing, _, _, _, _>(
            |data, converted| {
                let converted_data = converted.map(|node| node.cached_data());
                Ok(Arc::new(RedeemData::new(
                    data.node.arrow().shallow_clone(),
                    converted_data,
                )))
            },
            |data, _| {
                bits.read_value(&data.node.data.arrow().target)
                    .map(Arc::new)
            },
        )?;

        // 3. Check that we read exactly as much witness data as we expected
        if bits.n_total_read() != witness_start + witness_len {
            return Err(Error::InconsistentWitnessLength);
        }

        // 4. Check sharing
        let mut imrs: HashSet<Imr> = HashSet::new();
        for data in program.as_ref().post_order_iter::<InternalSharing>() {
            if !imrs.insert(data.node.imr()) {
                return Err(Error::SharingNotMaximal);
            }
        }

        Ok(program)
    }
}
