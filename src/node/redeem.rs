// SPDX-License-Identifier: CC0-1.0

use crate::analysis::NodeBounds;
use crate::dag::{DagLike, InternalSharing, MaxSharing, PostOrderIterItem};
use crate::jet::Jet;
use crate::types::{self, arrow::FinalArrow};
use crate::{encode, WitnessNode};
use crate::{Amr, BitIter, BitWriter, Cmr, Error, FirstPassImr, Imr, Value};

use super::{
    Commit, CommitData, CommitNode, Construct, ConstructNode, Constructible, Converter, Inner,
    Marker, NoDisconnect, NoWitness, Node, Witness, WitnessData,
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
    /// struct has a \<J\> parameter, since it forces the choice of jets to
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
            Inner::Iden => (
                Amr::iden(&arrow),
                FirstPassImr::iden(),
                NodeBounds::iden(arrow.source.bit_width()),
            ),
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
                    left.arrow.target.bit_width() - right.arrow.source.bit_width(),
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

    /// Convert a [`RedeemNode`] back into a [`WitnessNode`]
    /// by loosening the finalized types, witness data and disconnected branches.
    pub fn to_witness_node(&self) -> Arc<WitnessNode<J>> {
        struct ToWitness<J> {
            inference_context: types::Context,
            phantom: PhantomData<J>,
        }

        impl<J: Jet> Converter<Redeem<J>, Witness<J>> for ToWitness<J> {
            type Error = ();

            fn convert_witness(
                &mut self,
                _: &PostOrderIterItem<&Node<Redeem<J>>>,
                witness: &Arc<Value>,
            ) -> Result<Option<Arc<Value>>, Self::Error> {
                Ok(Some(witness.clone()))
            }

            fn convert_disconnect(
                &mut self,
                _: &PostOrderIterItem<&Node<Redeem<J>>>,
                right: Option<&Arc<Node<Witness<J>>>>,
                _: &Arc<RedeemNode<J>>,
            ) -> Result<Option<Arc<Node<Witness<J>>>>, Self::Error> {
                Ok(right.cloned())
            }

            fn convert_data(
                &mut self,
                _: &PostOrderIterItem<&Node<Redeem<J>>>,
                inner: Inner<
                    &Arc<Node<Witness<J>>>,
                    J,
                    &Option<Arc<WitnessNode<J>>>,
                    &Option<Arc<Value>>,
                >,
            ) -> Result<WitnessData<J>, Self::Error> {
                let inner = inner
                    .map(|node| node.cached_data())
                    .map_witness(|maybe_value| maybe_value.clone());
                Ok(WitnessData::from_inner(&self.inference_context, inner)
                    .expect("types were already finalized"))
            }
        }

        self.convert::<InternalSharing, _, _>(&mut ToWitness {
            inference_context: types::Context::new(),
            phantom: PhantomData,
        })
        .unwrap()
    }

    /// Decode a Simplicity program from bits, including the witness data.
    pub fn decode<I1, I2>(
        mut program: BitIter<I1>,
        mut witness: BitIter<I2>,
    ) -> Result<Arc<Self>, Error>
    where
        I1: Iterator<Item = u8>,
        I2: Iterator<Item = u8>,
    {
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
                let arrow = data.node.data.arrow();
                let target_ty = arrow.target.finalize()?;
                self.bits.read_value(&target_ty).map_err(Error::from)
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
        let construct = crate::decode::decode_expression(&mut program)?;
        program
            .close()
            .map_err(crate::decode::Error::BitIter)
            .map_err(Error::Decode)?;
        construct.set_arrow_to_program()?;

        // Importantly, we  use `InternalSharing` here to make sure that we respect
        // the sharing choices that were actually encoded in the bitstream.
        let program: Arc<Self> =
            construct.convert::<InternalSharing, _, _>(&mut DecodeFinalizer {
                bits: &mut witness,
                phantom: PhantomData,
            })?;

        // 3. Check that we read exactly as much witness data as we expected
        witness
            .close()
            .map_err(crate::decode::Error::BitIter)
            .map_err(Error::Decode)?;

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

    /// Encode the program to bits.
    ///
    /// Includes witness data. Returns the number of written bits.
    pub fn encode<W1, W2>(
        &self,
        prog: &mut BitWriter<W1>,
        witness: &mut BitWriter<W2>,
    ) -> io::Result<usize>
    where
        W1: io::Write,
        W2: io::Write,
    {
        let sharing_iter = self.post_order_iter::<MaxSharing<Redeem<J>>>();
        let program_bits = encode::encode_program(self, prog)?;
        prog.flush_all()?;
        let witness_bits =
            encode::encode_witness(sharing_iter.into_witnesses().map(Arc::as_ref), witness)?;
        witness.flush_all()?;
        Ok(program_bits + witness_bits)
    }

    /// Encode the program and witness data to byte vectors.
    pub fn encode_to_vec(&self) -> (Vec<u8>, Vec<u8>) {
        let mut ret_1 = vec![];
        let mut ret_2 = vec![];
        self.encode(
            &mut BitWriter::new(&mut ret_1),
            &mut BitWriter::new(&mut ret_2),
        )
        .unwrap();
        (ret_1, ret_2)
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
        witness_bytes: &[u8],
        cmr_str: &str,
        amr_str: &str,
        imr_str: &str,
    ) -> Arc<RedeemNode<J>> {
        let prog_hex = prog_bytes.as_hex();
        let witness_hex = witness_bytes.as_hex();

        let prog = BitIter::from(prog_bytes);
        let witness = BitIter::from(witness_bytes);
        let prog = match RedeemNode::<J>::decode(prog, witness) {
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

        let (reser_prog, reser_witness) = prog.encode_to_vec();
        assert_eq!(
            prog_bytes,
            &reser_prog[..],
            "program {} reserialized as {}",
            prog_hex,
            reser_prog.as_hex(),
        );
        assert_eq!(
            witness_bytes,
            &reser_witness[..],
            "witness {} reserialized as {}",
            witness_hex,
            reser_witness.as_hex(),
        );

        prog
    }

    fn assert_program_not_deserializable<J: Jet>(
        prog_bytes: &[u8],
        witness_bytes: &[u8],
        err: &dyn fmt::Display,
    ) {
        let prog_hex = prog_bytes.as_hex();
        let witness_hex = witness_bytes.as_hex();
        let err_str = err.to_string();

        let prog = BitIter::from(prog_bytes);
        let witness = BitIter::from(witness_bytes);
        match RedeemNode::<J>::decode(prog, witness) {
            Ok(prog) => panic!(
                "Program {} wit {} succeded (expected error {}). Program parsed as:\n{}",
                prog_hex, witness_hex, err, prog
            ),
            Err(e) if e.to_string() == err_str => {} // ok
            Err(e) => panic!(
                "Program {} wit {} failed with error {} (expected error {})",
                prog_hex, witness_hex, e, err
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
        let iter = BitIter::from(&eqwits[..]);
        let eqwits_prog = CommitNode::<Core>::decode(iter).unwrap();

        let eqwits_final = eqwits_prog
            .finalize(&mut SimpleFinalizer::new(std::iter::repeat(Value::u32(
                0xDEADBEEF,
            ))))
            .unwrap();
        let output = eqwits_final.encode_to_vec();

        assert_eq!(
            output,
            (
                [0xc9, 0xc4, 0x6d, 0xb8, 0x82, 0x30, 0x10].into(),
                [0xde, 0xad, 0xbe, 0xef].into(),
            ),
            "output {} {}",
            output.0.as_hex(),
            output.1.as_hex()
        );
    }

    #[test]
    fn decode_shared_witnesses() {
        // This program is exactly the output from the `encode_shared_witnesses` test.
        // The point of this is to make sure that our witness-unsharing logic doesn't
        // get confused here and try to read two witnesses when there are only one.
        assert_program_deserializable::<Core>(
            &[0xc9, 0xc4, 0x6d, 0xb8, 0x82, 0x30, 0x10],
            &[0xde, 0xad, 0xbe, 0xef],
            "e398797db2b1c742dfa69a3925583e55e55a81a1e5150a0241a8d9e85e112475",
            "78280372ee1d0e0aaf8b343030f3bbdb2f954b22f0c4f77f6f04dd8ec506fca6",
            "ef660061d9d3bd87d208fb7ab82ed095141440fd00cc2b3fa7ddeb360769ab6b",
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
            &[0xc1, 0x08, 0x04, 0x00],
            &[],
            &Error::Decode(crate::decode::Error::SharingNotMaximal),
        );
    }

    #[test]
    fn witness_consumed() {
        // "main = unit", but with a witness attached. Found by fuzzer.
        let prog = BitIter::from(&[0x24][..]);
        let wit = BitIter::from(&[0x00][..]);
        match RedeemNode::<Core>::decode(prog, wit) {
            Err(Error::Decode(crate::decode::Error::BitIter(
                crate::BitIterCloseError::TrailingBytes { first_byte: 0 },
            ))) => {} // ok,
            Err(e) => panic!("got incorrect error {e}"),
            Ok(_) => panic!("accepted program with bad witness length"),
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
            &[],
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
            &[],
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
            &[],
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
            &[],
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
                0xd3, 0x69, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x3b, 0x78, 0xce,
                0x56, 0x3f, 0x89, 0xa0, 0xed, 0x94, 0x14, 0xf5, 0xaa, 0x28, 0xad, 0x0d, 0x96, 0xd6, 0x79, 0x5f,
                0x9c, 0x63, 0x47, 0x07, 0x02, 0xc0, 0xe2, 0x8d, 0x88, 0x10, 
            ],
            &[
                0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x3b, 0x78, 0xce, 0x56, 0x3f,
                0x89, 0xa0, 0xed, 0x94, 0x14, 0xf5, 0xaa, 0x28, 0xad, 0x0d, 0x96, 0xd6, 0x79, 0x5f, 0x9c, 0x63,
                0xdb, 0x86, 0x8d, 0x45, 0xa0, 0xbc, 0x1d, 0x19, 0x01, 0x30, 0x2b, 0xc8, 0x7a, 0x87, 0x1c, 0xf1,
                0x58, 0xe2, 0xbd, 0xe2, 0xcf, 0xa6, 0x45, 0xa8, 0x95, 0xc1, 0xb4, 0x5d, 0x68, 0xea, 0x24, 0xc0, 
            ],
            "e2a6e4a223c0da97ebbf5f401e2d622535c3ed538e70f344318e9e5e4c2e02af",
            "5cd9d4af96740d2f27ffd94649304923a20939ae42b02828fa7982457fac3c22",
            "6f9dba991638478e2eec6388e51024410fd442264e8b212344aef802d8ff858b",
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
            &[],
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
            0x00, 0x00, 0xe2, 0x8d, 0x8c, 0x04, 0x00,
        ];
        #[rustfmt::skip]
        let schnorr0_wit = vec![
            0xe9, 0x07, 0x83, 0x1f, 0x80, 0x84, 0x8d, 0x10, 0x69, 0xa5, 0x37, 0x1b, 0x40, 0x24, 0x10, 0x36,
            0x4b, 0xdf, 0x1c, 0x5f, 0x83, 0x07, 0xb0, 0x08, 0x4c, 0x55, 0xf1, 0xce, 0x2d, 0xca, 0x82, 0x15,
            0x25, 0xf6, 0x6a, 0x4a, 0x85, 0xea, 0x8b, 0x71, 0xe4, 0x82, 0xa7, 0x4f, 0x38, 0x2d, 0x2c, 0xe5,
            0xeb, 0xee, 0xe8, 0xfd, 0xb2, 0x17, 0x2f, 0x47, 0x7d, 0xf4, 0x90, 0x0d, 0x31, 0x05, 0x36, 0xc0,
        ];
        assert_program_deserializable::<crate::jet::Elements>(
            &schnorr0,
            &schnorr0_wit,
            "e3b25e13f91fcbf95407f126b3aed5b4543399cb9c3d426fe98a10f8d33ef6c3",
            "9049bcda62ffe6b7146b91af7a4522adbb93f0cd50dd390d12b9fe12ae836c8c",
            "6b18b11f6367528afbe1abaf41553ce68bb417da2dcdd2832964399148548ac7",
        );
    }
}
