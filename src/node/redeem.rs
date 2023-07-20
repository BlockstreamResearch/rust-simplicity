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

use super::{
    Commit, CommitData, CommitNode, Construct, ConstructNode, Converter, Inner, Marker,
    NoDisconnect, NoWitness, Node,
};

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
    type Disconnect = Arc<RedeemNode<J>>;
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
    pub fn new(arrow: FinalArrow, inner: Inner<&Arc<Self>, J, &Arc<Self>, Arc<Value>>) -> Self {
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
            Inner::Jet(jet) => (Amr::jet(jet), FirstPassImr::jet(jet), NodeBounds::jet(jet)),
            Inner::Word(ref val) => (
                Amr::const_word(val),
                FirstPassImr::const_word(val),
                NodeBounds::const_word(val),
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

            fn convert_disconnect(
                &mut self,
                _: &PostOrderIterItem<&RedeemNode<J>>,
                _: Option<&Arc<CommitNode<J>>>,
                _: &Arc<RedeemNode<J>>,
            ) -> Result<NoDisconnect, Self::Error> {
                Ok(NoDisconnect)
            }

            fn convert_data(
                &mut self,
                data: &PostOrderIterItem<&RedeemNode<J>>,
                inner: Inner<&Arc<CommitNode<J>>, J, &NoDisconnect, &NoWitness>,
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

        impl<'bits, J: Jet, I: Iterator<Item = u8>> Converter<Construct<J>, Redeem<J>>
            for DecodeFinalizer<'bits, J, I>
        {
            type Error = Error;
            fn convert_witness(
                &mut self,
                data: &PostOrderIterItem<&ConstructNode<J>>,
                _: &NoWitness,
            ) -> Result<Arc<Value>, Self::Error> {
                let target_ty = data.node.data.arrow().target.finalize()?;
                self.bits
                    .read_value(&target_ty)
                    .map(Arc::new)
                    .map_err(Error::from)
            }

            fn convert_disconnect(
                &mut self,
                _: &PostOrderIterItem<&ConstructNode<J>>,
                right: Option<&Arc<RedeemNode<J>>>,
                _: &Option<Arc<ConstructNode<J>>>,
            ) -> Result<Arc<RedeemNode<J>>, Self::Error> {
                if let Some(child) = right {
                    Ok(Arc::clone(child))
                } else {
                    Err(Error::DisconnectRedeemTime)
                }
            }

            fn convert_data(
                &mut self,
                data: &PostOrderIterItem<&ConstructNode<J>>,
                inner: Inner<&Arc<RedeemNode<J>>, J, &Arc<RedeemNode<J>>, &Arc<Value>>,
            ) -> Result<Arc<RedeemData<J>>, Self::Error> {
                let arrow = data.node.data.arrow().finalize()?;
                let converted_data = inner
                    .map(|node| node.cached_data())
                    .map_disconnect(|node| node.cached_data())
                    .map_witness(Arc::clone);
                Ok(Arc::new(RedeemData::new(arrow, converted_data)))
            }
        }

        // 1. Decode program without witnesses as ConstructNode
        let construct = crate::decode::decode_expression(bits)?;
        construct.set_arrow_to_program()?;

        // 2. Convert to RedeemNode, reading witnesses as we go
        let witness_len = if bits.read_bit()? {
            bits.read_natural(None)?
        } else {
            0
        };
        let witness_start = bits.n_total_read();

        // Importantly, we  use `InternalSharing` here to make sure that we respect
        // the sharing choices that were actually encoded in the bitstream.
        let program: Arc<Self> =
            construct.convert::<InternalSharing, _, _>(&mut DecodeFinalizer {
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

    use hex::DisplayHex;
    use std::fmt;

    use crate::jet::Core;
    use crate::node::SimpleFinalizer;

    fn assert_program_deserializable<J: Jet>(
        prog_bytes: &[u8],
        cmr_str: &str,
        amr_str: &str,
        imr_str: &str,
    ) -> Arc<RedeemNode<J>> {
        let prog_hex = prog_bytes.as_hex();

        let mut iter = BitIter::from(prog_bytes);
        let prog = match RedeemNode::<J>::decode(&mut iter) {
            Ok(prog) => prog,
            Err(e) => panic!("program {} failed: {}", prog_hex, e),
        };

        assert_eq!(
            prog.cmr().to_string(),
            cmr_str,
            "CMR mismatch (got {} expected {}) for program {}",
            prog.cmr(),
            cmr_str,
            prog_hex,
        );

        assert_eq!(
            prog.amr().to_string(),
            amr_str,
            "AMR mismatch (got {} expected {}) for program {}",
            prog.amr(),
            amr_str,
            prog_hex,
        );
        assert_eq!(
            prog.imr().to_string(),
            imr_str,
            "IMR mismatch (got {} expected {}) for program {}",
            prog.imr(),
            imr_str,
            prog_hex,
        );

        let reser_sink = prog.encode_to_vec();
        assert_eq!(
            prog_bytes,
            &reser_sink[..],
            "program {} reserialized as {}",
            prog_hex,
            reser_sink.as_hex(),
        );

        prog
    }

    fn assert_program_not_deserializable<J: Jet>(prog: &[u8], err: &dyn fmt::Display) {
        let prog_hex = prog.as_hex();
        let err_str = err.to_string();

        let mut iter = BitIter::from(prog);
        match RedeemNode::<J>::decode(&mut iter) {
            Ok(prog) => panic!(
                "Program {} succeded (expected error {}). Program parsed as:\n{}",
                prog_hex, err, prog
            ),
            Err(e) if e.to_string() == err_str => {} // ok
            Err(e) => panic!(
                "Program {} failed with error {} (expected error {})",
                prog_hex, e, err
            ),
        };
    }

    #[test]
    fn encode_shared_witnesses() {
        // # Program code:
        // wit1 = witness :: 1 -> 2^32
        // wit2 = witness :: 1 -> 2^32
        //
        // wits_are_equal = comp (pair wit1 wit2) jet_eq_32 :: 1 -> 2
        // main = comp wits_are_equal jet_verify            :: 1 -> 1
        let eqwits = [0xcd, 0xdc, 0x51, 0xb6, 0xe2, 0x08, 0xc0, 0x40];
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
        assert_program_deserializable::<Core>(
            &[
                0xc9, 0xc4, 0x6d, 0xb8, 0x82, 0x30, 0x11, 0xe2, 0x0d, 0xea, 0xdb, 0xee, 0xf0,
            ],
            "e552742731de7f5c3c83cd54176c0ca6acf6dbd3c37bef7da132eb06f3856d06",
            "6522c14daf762467232b55c37df25c17f80d7c12000ce8a5dcbbd67b879d192f",
            "bbb31a66836ab67de2ef0efa1ce82078b36c9dbcf6af77822b87f7743dc88a8f",
        );
    }

    #[test]
    fn unshared_child() {
        // # id1 and id2 should be shared, but are not!
        // id1 = iden          :: A -> A # cmr dbfefcfc...
        // id2 = iden          :: A -> A # cmr dbfefcfc...
        // cp3 = comp id1 id2  :: A -> A # cmr c1ae55b5...
        // main = comp cp3 cp3 :: A -> A # cmr 314e2879...
        assert_program_not_deserializable::<Core>(
            &[0xc1, 0x08, 0x04, 0x00, 0x00, 0x74, 0x74, 0x74],
            &Error::Decode(crate::decode::Error::SharingNotMaximal),
        );
    }

    #[test]
    fn witness_consumed() {
        // "main = unit", but with a witness attached. Found by fuzzer.
        let badwit = [0x27, 0x00];
        let mut iter = BitIter::from(&badwit[..]);
        if let Err(Error::InconsistentWitnessLength) =
            RedeemNode::<crate::jet::Core>::decode(&mut iter)
        {
            // ok
        } else {
            panic!("accepted program with bad witness length")
        }
    }

    #[test]
    fn shared_grandchild() {
        // # This program repeats the node `cp2` three times; during iteration it will
        // # be placed on the stack as part of the initial `comp` combinator, but by
        // # the time we get to it, it will have already been yielded. Makes sure this
        // # does not confuse the iteration logic and break the decoded program structure.
        // id1 = iden
        // cp2 = comp id1 id1
        // cp3 = comp cp2 cp2
        // main = comp cp3 cp2
        assert_program_deserializable::<Core>(
            &[0xc1, 0x00, 0x00, 0x01, 0x00],
            "c2c86be0081a9c75af49098f359c7efdfa7ccbd0459adb11bcf676b80c8644b1",
            "e053520f0c3219d1cabd705b4523ccd05c8d703a70f6f3994a20774a42b5ccfc",
            "7b0ad0514279280d5c2ac1da729222936b8768d9f465c6c6ade3b0ed7dc97263",
        );
    }

    #[test]
    #[rustfmt::skip]
    fn assert_lr() {
        // asst = assertl unit deadbeefdeadbeefdeadbeefdeadbeefdeadbeefdeadbeefdeadbeefdeadbeef
        // input0 = pair (injl unit) unit
        // main = comp input0 asst
        assert_program_deserializable::<Core>(
            &[
                0xcd, 0x24, 0x08, 0x4b, 0x6f, 0x56, 0xdf, 0x77,
                0xef, 0x56, 0xdf, 0x77, 0xef, 0x56, 0xdf, 0x77,
                0xef, 0x56, 0xdf, 0x77, 0xef, 0x56, 0xdf, 0x77,
                0xef, 0x56, 0xdf, 0x77, 0xef, 0x56, 0xdf, 0x77,
                0xef, 0x56, 0xdf, 0x77, 0x86, 0x01, 0x80,
            ],
            "c7194362a5480900dd44f9f647a49b8adcb92a25fb293c920e6bbcf6977cf63d",
            "eaf95c23d967563132b65e43578fe08dae2a29ac66775ddd37af3ac7de28678b",
            "d2927a9a54ddea8359ee00aa27e0aa1e354cc6924b090c759e2ed686712700a0",
        );


        // asst = assertr deadbeefdeadbeefdeadbeefdeadbeefdeadbeefdeadbeefdeadbeefdeadbeef unit
        // input1 = pair (injr unit) unit
        // main = comp input1 asst
        assert_program_deserializable::<Core>(
            &[
                0xcd, 0x25, 0x08, 0x6d, 0xea, 0xdb, 0xee, 0xfd,
                0xea, 0xdb, 0xee, 0xfd, 0xea, 0xdb, 0xee, 0xfd,
                0xea, 0xdb, 0xee, 0xfd, 0xea, 0xdb, 0xee, 0xfd,
                0xea, 0xdb, 0xee, 0xfd, 0xea, 0xdb, 0xee, 0xfd,
                0xea, 0xdb, 0xee, 0xf4, 0x86, 0x01, 0x80,
            ],
            "8e471ac519e0b16a2b7dda7e8d68165f260cae4823861ddc494b7c73a615b212",
            "ea1ee417816a57b80739520c7319c33a39a5f4ce7b59856e69f768d5d8f174a6",
            "f262f83f1c9341390e015e4c5126f3954e17a1f275af73da2948eaf4797fda48",
        );
    }

    #[test]
    #[rustfmt::skip]
    fn disconnect() {
        // id1 = iden                 :: (2^256 * B) -> (2^256 * B)                       # cmr dbfefcfc...
        // pr2 = pair id1 id1         :: (2^256 * B) -> ((2^256 * B) * (2^256 * B))       # cmr a62c628c...
        // disc3 = disconnect pr2 pr2 :: B -> ((2^256 * B) * ((2^256 * B) * (2^256 * B))) # cmr d81d6f28...
        // ut4 = unit                 :: ((2^256 * B) * ((2^256 * B) * (2^256 * B))) -> 1 # cmr 62274a89...
        // main = comp disc3 ut4      :: B -> 1                                           # cmr a453360c...
        assert_program_deserializable::<Core>(
            &[0xc5, 0x02, 0x06, 0x24, 0x10],
            "a453360c0825cc2d3c4c907d67b174273b0e0386c7e5ecdb28394a8f37fd68b9",
            "d5b05a5da87ee490312279496e12e17bc987c98219d8961bc3a7c3ec95a7ce1e",
            "3579ae2a05bbe689f16bd3ff29d840ae8aa8bbad70f6de27b7473746637abeb6",
        );
    }

    #[test]
    #[rustfmt::skip]
    #[cfg(feature = "elements")]
    fn disconnect2() {
        // Program that Russell posted on IRC 2023-06-22 that tickled `Dag::right_child`
        // bug that had been introduced that day. Probably not the most minimal test
        // vector but might as well include it as it seems like an interesting program.
        //
        // # Witnesses
        // wit1 = witness :: 1 -> 2^512
        //
        // # Constants
        // const1 = word_jet 0x00000000000000000000003b78ce563f89a0ed9414f5aa28ad0d96d6795f9c63 :: 1 -> 2^256 # cmr a9e3dbca...
        //
        // # Program code
        // id1 = iden                 :: (2^256 * 1) -> (2^256 * 1)     # cmr dbfefcfc...
        // jt2 = jet_sig_all_hash     :: 1 -> 2^256                     # cmr 9902bc0f...
        // disc3 = disconnect id1 jt2 :: 1 -> 2^512                     # cmr 6968f10e...
        // pr4 = pair const1 disc3    :: 1 -> (2^256 * 2^512)           # cmr 378ad609...
        // pr5 = pair pr4 wit1        :: 1 -> ((2^256 * 2^512) * 2^512) # cmr 0d51ff00...
        // jt6 = jet_check_sig_verify :: ((2^256 * 2^512) * 2^512) -> 1 # cmr 297459d8...
        //
        // main = comp pr5 jt6        :: 1 -> 1                         # cmr 14a5e0cc...
        assert_program_deserializable::<crate::jet::Elements>(
            &[
                0xd3, 0x69, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                0x00, 0x00, 0x00, 0x00, 0x00, 0x3b, 0x78, 0xce,
                0x56, 0x3f, 0x89, 0xa0, 0xed, 0x94, 0x14, 0xf5,
                0xaa, 0x28, 0xad, 0x0d, 0x96, 0xd6, 0x79, 0x5f,
                0x9c, 0x63, 0x47, 0x07, 0x02, 0xc0, 0xe2, 0x8d,
                0x88, 0x11, 0xe9, 0x00, 0x00, 0x00, 0x00, 0x00,
                0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x1d,
                0xbc, 0x67, 0x2b, 0x1f, 0xc4, 0xd0, 0x76, 0xca,
                0x0a, 0x7a, 0xd5, 0x14, 0x56, 0x86, 0xcb, 0x6b,
                0x3c, 0xaf, 0xce, 0x31, 0xed, 0xc3, 0x46, 0xa2,
                0xd0, 0x5e, 0x0e, 0x8c, 0x80, 0x98, 0x15, 0xe4,
                0x3d, 0x43, 0x8e, 0x78, 0xac, 0x71, 0x5e, 0xf1,
                0x67, 0xd3, 0x22, 0xd4, 0x4a, 0xe0, 0xda, 0x2e,
                0xb4, 0x75, 0x12, 0x60, 0x00,
            ],
            "14a5e0cc13da9acdd5f758ae7186802137143e06c8dcba10019ffec790359ee7",
            "1a2a1cb30027f64bd9a639bf385834f415cac889ea654b0e04e81b210d3502d4",
            "6de2a91db704fa287d8724da2e43a087bbd91334b0fb7160b231129356dc03b8",
        );
    }

    #[test]
    #[rustfmt::skip]
    #[cfg(feature = "elements")]
    fn disconnect3() {
        // Yet another disconnect-based program that hit a bug in our AMR computation
        // (passing left arrow in place of right arrow to the AMR constructor.)
        // # Program code
        // id1 = iden                 :: (2^256 * 1) -> (2^256 * 1) # cmr dbfefcfc...
        // ut2 = unit                 :: 1 -> 1                     # cmr 62274a89...
        // jl3 = injl ut2             :: 1 -> 2                     # cmr bd0cce93...
        // disc4 = disconnect id1 jl3 :: 1 -> (2^256 * 2)           # cmr 6968f10e...
        // ut5 = unit                 :: (2^256 * 2) -> 1           # cmr 62274a89...
        // main = comp disc4 ut5      :: 1 -> 1                     # cmr a8c9cc7a...
        assert_program_deserializable::<crate::jet::Elements>(
            &[0xc9, 0x09, 0x20, 0x74, 0x90, 0x40],
            "a8c9cc7a83518d0886afe1078d88eabca8353509e8c2e3b5c72cf559c713c9f5",
            "97f77a7e7d7f3b2b1ac790bf54b39d47d6db8dcab7ed3c0a48df12f2c940af58",
            "ed8152948589d65e0dea6d84f90eb752f63df818041f46bdc8f959f33299cbd3",
        );
    }

    #[test]
    #[cfg(feature = "elements")]
    fn decode_schnorr() {
        #[rustfmt::skip]
        let schnorr0 = vec![
            0xc6, 0xd5, 0xf2, 0x61, 0x14, 0x03, 0x24, 0xb1, 0x86, 0x20, 0x92, 0x68, 0x9f, 0x0b, 0xf1, 0x3a,
            0xa4, 0x53, 0x6a, 0x63, 0x90, 0x8b, 0x06, 0xdf, 0x33, 0x61, 0x0c, 0x03, 0xe2, 0x27, 0x79, 0xc0,
            0x6d, 0xf2, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0xe2, 0x8d, 0x8c, 0x04, 0x7a, 0x40, 0x1d, 0x20, 0xf0, 0x63, 0xf0, 0x10, 0x91, 0xa2,
            0x0d, 0x34, 0xa6, 0xe3, 0x68, 0x04, 0x82, 0x06, 0xc9, 0x7b, 0xe3, 0x8b, 0xf0, 0x60, 0xf6, 0x01,
            0x09, 0x8a, 0xbe, 0x39, 0xc5, 0xb9, 0x50, 0x42, 0xa4, 0xbe, 0xcd, 0x49, 0x50, 0xbd, 0x51, 0x6e,
            0x3c, 0x90, 0x54, 0xe9, 0xe7, 0x05, 0xa5, 0x9c, 0xbd, 0x7d, 0xdd, 0x1f, 0xb6, 0x42, 0xe5, 0xe8,
            0xef, 0xbe, 0x92, 0x01, 0xa6, 0x20, 0xa6, 0xd8, 0x00
        ];
        assert_program_deserializable::<crate::jet::Elements>(
            &schnorr0,
            "7bc56cb16d84999b977b58e1bc71dbe9edcc33650afc8a6ee05cfef8d608132b",
            "02796d1d7d906a15d0a1ebed9d702e334b21e9cc52a578c24fca0fb7be82fac0",
            "00ee3d3e7b7a65fcd77b6309c4d8464f176f13c92d8d69238eb0158f70c8a4df",
        );
    }
}
