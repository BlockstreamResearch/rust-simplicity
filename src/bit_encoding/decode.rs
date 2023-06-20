// Rust Simplicity Library
// Written in 2020 by
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

//! # Decoding
//!
//! Functionality to decode Simplicity programs.
//! Refer to [`crate::encode`] for information on the encoding.

use crate::core::iter::WitnessIterator;
use crate::dag::{Dag, DagLike, InternalSharing};
use crate::jet::Jet;
use crate::merkle::cmr::Cmr;
use crate::node::{
    CommitNode, ConstructNode, CoreConstructible, JetConstructible, NoWitness, WitnessConstructible,
};
use crate::types;
use crate::{BitIter, Context, FailEntropy, Value};
use std::sync::Arc;
use std::{error, fmt};

use super::bititer::u2;

type ArcNode<J> = Arc<ConstructNode<J>>;

/// Decoding error
#[non_exhaustive]
#[derive(Debug)]
pub enum Error {
    /// Node made a back-reference past the beginning of the program
    BadIndex,
    /// Both children of a node are hidden
    BothChildrenHidden,
    /// Program must not be empty
    EmptyProgram,
    /// Bitstream ended early
    EndOfStream,
    /// Hidden node occurred outside of a case combinator
    HiddenNode,
    /// Tried to parse a jet but the name wasn't recognized
    InvalidJet,
    /// Number exceeded 32 bits
    NaturalOverflow,
    /// Program is not encoded in canonical order
    NotInCanonicalOrder,
    /// Program encoded a stop code
    StopCode,
    /// Tried to allocate too many nodes in a program
    TooManyNodes(usize),
    /// Type-checking error
    Type(crate::types::Error),
}

impl From<super::bititer::EarlyEndOfStreamError> for Error {
    fn from(_: super::bititer::EarlyEndOfStreamError) -> Error {
        Error::EndOfStream
    }
}

impl From<crate::types::Error> for Error {
    fn from(e: crate::types::Error) -> Error {
        Error::Type(e)
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::BadIndex => {
                f.write_str("node made a back-reference past the beginning of the program")
            }
            Error::BothChildrenHidden => f.write_str("both children of a case node are hidden"),
            Error::EmptyProgram => f.write_str("empty program"),
            Error::EndOfStream => f.write_str("bitstream ended early"),
            Error::HiddenNode => write!(f, "hidden node occurred outside of a case combinator"),
            Error::InvalidJet => write!(f, "unrecognized jet"),
            Error::NaturalOverflow => f.write_str("encoded number exceeded 32 bits"),
            Error::NotInCanonicalOrder => f.write_str("program not in canonical order"),
            Error::StopCode => f.write_str("program contained a stop code"),
            Error::TooManyNodes(k) => {
                write!(f, "program has too many nodes ({})", k)
            }
            Error::Type(ref e) => fmt::Display::fmt(e, f),
        }
    }
}

impl error::Error for Error {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match *self {
            Error::BadIndex => None,
            Error::BothChildrenHidden => None,
            Error::EmptyProgram => None,
            Error::EndOfStream => None,
            Error::HiddenNode => None,
            Error::InvalidJet => None,
            Error::NaturalOverflow => None,
            Error::NotInCanonicalOrder => None,
            Error::StopCode => None,
            Error::TooManyNodes(..) => None,
            Error::Type(ref e) => Some(e),
        }
    }
}

#[derive(Debug)]
enum DecodeNode<J: Jet> {
    Iden,
    Unit,
    InjL(usize),
    InjR(usize),
    Take(usize),
    Drop(usize),
    Comp(usize, usize),
    Case(usize, usize),
    Pair(usize, usize),
    Disconnect(usize, usize),
    Witness,
    Fail(FailEntropy),
    Hidden(Cmr),
    Jet(J),
    Word(Arc<Value>),
}

impl<'d, J: Jet> DagLike for (usize, &'d [DecodeNode<J>]) {
    type Node = DecodeNode<J>;

    fn data(&self) -> &DecodeNode<J> {
        &self.1[self.0]
    }

    fn as_dag_node(&self) -> Dag<Self> {
        let nodes = &self.1;
        match self.1[self.0] {
            DecodeNode::Iden
            | DecodeNode::Unit
            | DecodeNode::Fail(..)
            | DecodeNode::Hidden(..)
            | DecodeNode::Jet(..)
            | DecodeNode::Word(..) => Dag::Nullary,
            DecodeNode::InjL(i)
            | DecodeNode::InjR(i)
            | DecodeNode::Take(i)
            | DecodeNode::Drop(i) => Dag::Unary((i, nodes)),
            DecodeNode::Comp(li, ri) | DecodeNode::Case(li, ri) | DecodeNode::Pair(li, ri) => {
                Dag::Binary((li, nodes), (ri, nodes))
            }
            DecodeNode::Disconnect(li, ri) => Dag::Disconnect((li, nodes), (ri, nodes)),
            DecodeNode::Witness => Dag::Witness,
        }
    }
}

/// Decode a Simplicity program from bits, without the witness data.
///
/// If witness data is present, it should be encoded after the program, and the
/// user must deserialize it separately.
pub fn decode_program<I: Iterator<Item = u8>, J: Jet>(
    bits: &mut BitIter<I>,
) -> Result<Arc<CommitNode<J>>, Error> {
    let root = decode_expression(bits)?;
    root.finalize_types().map_err(Error::from)
}

pub fn decode_expression<I: Iterator<Item = u8>, J: Jet>(
    bits: &mut BitIter<I>,
) -> Result<ArcNode<J>, Error> {
    let len = bits.read_natural(None)?;

    if len == 0 {
        return Err(Error::EmptyProgram);
    }
    // FIXME: check maximum length of DAG that is allowed by consensus
    if len > 1_000_000 {
        return Err(Error::TooManyNodes(len));
    }

    let mut context = Context::new();

    let mut nodes = Vec::with_capacity(len);
    for _ in 0..len {
        let new_node = decode_node(bits, nodes.len())?;
        nodes.push(new_node);
    }

    enum Converted<J: Jet> {
        Node(ArcNode<J>),
        Hidden(Cmr),
    }
    use Converted::{Hidden, Node};
    impl<J: Jet> Converted<J> {
        fn get(&self) -> Result<&ArcNode<J>, Error> {
            match self {
                Node(arc) => Ok(arc),
                Hidden(_) => Err(Error::HiddenNode),
            }
        }
    }

    // Convert the DecodeNode structure into a CommitNode structure
    let mut converted = Vec::<Converted<J>>::with_capacity(len);
    for data in (nodes.len() - 1, &nodes[..]).post_order_iter::<InternalSharing>() {
        // Check canonical order as we go
        if data.index != data.node.0 {
            return Err(Error::NotInCanonicalOrder);
        }

        let new = match nodes[data.node.0] {
            DecodeNode::Unit => Node(ArcNode::unit(&mut context)),
            DecodeNode::Iden => Node(ArcNode::iden(&mut context)),
            DecodeNode::InjL(i) => Node(ArcNode::injl(&mut context, converted[i].get()?)),
            DecodeNode::InjR(i) => Node(ArcNode::injr(&mut context, converted[i].get()?)),
            DecodeNode::Take(i) => Node(ArcNode::take(&mut context, converted[i].get()?)),
            DecodeNode::Drop(i) => Node(ArcNode::drop_(&mut context, converted[i].get()?)),
            DecodeNode::Comp(i, j) => Node(ArcNode::comp(
                &mut context,
                converted[i].get()?,
                converted[j].get()?,
            )?),
            DecodeNode::Case(i, j) => {
                // Case is a special case, since it uniquely is allowed to have hidden
                // children (but only one!) in which case it becomes an assertion.
                match (&converted[i], &converted[j]) {
                    (Node(left), Node(right)) => Node(ArcNode::case(&mut context, left, right)?),
                    (Node(left), Hidden(cmr)) => Node(ArcNode::assertl(&mut context, left, *cmr)?),
                    (Hidden(cmr), Node(right)) => {
                        Node(ArcNode::assertr(&mut context, *cmr, right)?)
                    }
                    (Hidden(_), Hidden(_)) => return Err(Error::BothChildrenHidden),
                }
            }
            DecodeNode::Pair(i, j) => Node(ArcNode::pair(
                &mut context,
                converted[i].get()?,
                converted[j].get()?,
            )?),
            DecodeNode::Disconnect(i, j) => Node(ArcNode::disconnect(
                &mut context,
                converted[i].get()?,
                converted[j].get()?,
            )?),
            DecodeNode::Witness => Node(ArcNode::witness(&mut context, NoWitness)),
            DecodeNode::Fail(entropy) => Node(ArcNode::fail(&mut context, entropy)),
            DecodeNode::Hidden(cmr) => Hidden(cmr),
            DecodeNode::Jet(j) => Node(ArcNode::jet(&mut context, j)),
            DecodeNode::Word(ref w) => Node(ArcNode::const_word(&mut context, Arc::clone(w))),
        };
        converted.push(new);
    }

    converted[len - 1].get().map(Arc::clone)
}

/// Decode a single Simplicity node from bits and
/// insert it into a hash map at its index for future reference by ancestor nodes.
fn decode_node<I: Iterator<Item = u8>, J: Jet>(
    bits: &mut BitIter<I>,
    index: usize,
) -> Result<DecodeNode<J>, Error> {
    // First bit: 1 for jets/words, 0 for normal combinators
    if bits.read_bit()? {
        // Second bit: 1 for jets, 0 for words
        if bits.read_bit()? {
            match J::decode(bits) {
                Err(crate::Error::Decode(e)) => Err(e),
                Err(_) => unreachable!(), // FIXME need to update generated jet code to directly return a decode::Error
                Ok(jet) => Ok(DecodeNode::Jet(jet)),
            }
        } else {
            let depth = bits.read_natural(Some(32))?;
            let word = decode_power_of_2(bits, 1 << (depth - 1))?;
            Ok(DecodeNode::Word(Arc::new(word)))
        }
    } else {
        // Bits 2 and 3: code
        match bits.read_u2()? {
            u2::_0 => {
                let subcode = bits.read_u2()?;
                let i_abs = index - bits.read_natural(Some(index))?;
                let j_abs = index - bits.read_natural(Some(index))?;

                // Bits 4 and 5: subcode
                match subcode {
                    u2::_0 => Ok(DecodeNode::Comp(i_abs, j_abs)),
                    u2::_1 => Ok(DecodeNode::Case(i_abs, j_abs)),
                    u2::_2 => Ok(DecodeNode::Pair(i_abs, j_abs)),
                    u2::_3 => Ok(DecodeNode::Disconnect(i_abs, j_abs)),
                }
            }
            u2::_1 => {
                let subcode = bits.read_u2()?;
                let i_abs = index - bits.read_natural(Some(index))?;
                // Bits 4 and 5: subcode
                match subcode {
                    u2::_0 => Ok(DecodeNode::InjL(i_abs)),
                    u2::_1 => Ok(DecodeNode::InjR(i_abs)),
                    u2::_2 => Ok(DecodeNode::Take(i_abs)),
                    u2::_3 => Ok(DecodeNode::Drop(i_abs)),
                }
            }
            u2::_2 => {
                // Bits 4 and 5: subcode
                match bits.read_u2()? {
                    u2::_0 => Ok(DecodeNode::Iden),
                    u2::_1 => Ok(DecodeNode::Unit),
                    u2::_2 => Ok(DecodeNode::Fail(bits.read_fail_entropy()?)),
                    u2::_3 => Err(Error::StopCode),
                }
            }
            u2::_3 => {
                // Bit 4: subcode
                if bits.read_bit()? {
                    Ok(DecodeNode::Witness)
                } else {
                    Ok(DecodeNode::Hidden(bits.read_cmr()?))
                }
            }
        }
    }
}

/// Implementation of [`WitnessIterator`] for an underlying [`BitIter`].
#[derive(Debug)]
pub struct WitnessDecoder<'a, I: Iterator<Item = u8>> {
    pub bits: &'a mut BitIter<I>,
    pub max_n: usize,
}

impl<'a, I: Iterator<Item = u8>> WitnessDecoder<'a, I> {
    /// Create a new witness decoder for the given bit iterator.
    ///
    /// # Usage
    ///
    /// This method must be used **after** the program serialization has been read by the iterator.
    pub fn new(bits: &'a mut BitIter<I>) -> Result<Self, Error> {
        let bit_len = match bits.next() {
            Some(false) => 0,
            Some(true) => bits.read_natural(None)?,
            None => return Err(Error::EndOfStream),
        };
        let n_start = bits.n_total_read();

        Ok(Self {
            bits,
            max_n: n_start + bit_len,
        })
    }
}

impl<'a, I: Iterator<Item = u8>> WitnessIterator for WitnessDecoder<'a, I> {
    fn next(&mut self, ty: &types::Final) -> Result<Value, crate::Error> {
        self.bits.read_value(ty).map_err(crate::Error::from)
    }

    fn finish(self) -> Result<(), crate::Error> {
        if self.bits.n_total_read() != self.max_n {
            Err(crate::Error::InconsistentWitnessLength)
        } else {
            Ok(())
        }
    }
}

/// Decode a value from bits, of the form 2^exp
///
/// # Panics
///
/// Panics if `exp` itself is not a power of 2
pub fn decode_power_of_2<I: Iterator<Item = bool>>(
    iter: &mut I,
    exp: usize,
) -> Result<Value, Error> {
    assert_eq!(exp.count_ones(), 1, "exp must be a power of 2");

    struct StackElem {
        value: Value,
        width: usize,
    }

    let mut stack = Vec::with_capacity(32);
    for _ in 0..exp {
        // Read next bit
        let bit = Value::u1(u8::from(iter.next().ok_or(Error::EndOfStream)?));
        stack.push(StackElem {
            value: bit,
            width: 1,
        });

        while stack.len() >= 2 && stack[stack.len() - 1].width == stack[stack.len() - 2].width {
            let right = stack.pop().unwrap();
            let left = stack.pop().unwrap();
            stack.push(StackElem {
                value: Value::Prod(Box::new(left.value), Box::new(right.value)),
                width: left.width * 2,
            });
        }
    }
    Ok(stack.pop().unwrap().value)
}

/// Decode a natural number from bits.
/// If a bound is specified, then the decoding terminates before trying to decode a larger number.
pub fn decode_natural<I: Iterator<Item = bool>>(
    iter: &mut I,
    bound: Option<usize>,
) -> Result<usize, Error> {
    let mut recurse_depth = 0;
    loop {
        match iter.next() {
            Some(true) => recurse_depth += 1,
            Some(false) => break,
            None => return Err(Error::EndOfStream),
        }
    }

    let mut len = 0;
    loop {
        let mut n = 1;
        for _ in 0..len {
            let bit = match iter.next() {
                Some(false) => 0,
                Some(true) => 1,
                None => return Err(Error::EndOfStream),
            };
            n = 2 * n + bit;
        }

        if recurse_depth == 0 {
            if let Some(bound) = bound {
                if n > bound {
                    return Err(Error::BadIndex);
                }
            }
            return Ok(n);
        } else {
            len = n;
            if len > 31 {
                return Err(Error::NaturalOverflow);
            }
            recurse_depth -= 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::encode;
    use crate::exec::BitMachine;
    use crate::jet::Core;
    use crate::BitWriter;
    use crate::CommitNode;
    use bitcoin_hashes::hex::ToHex;

    fn assert_program_deserializable<J: Jet>(
        prog_bytes: &[u8],
        cmr_str: &str,
        amr_str: Option<&str>,
        imr_str: Option<&str>,
    ) -> std::rc::Rc<crate::CommitNode<J>> {
        let prog_hex = prog_bytes.to_hex();

        let mut iter = BitIter::from(prog_bytes);
        let prog = match decode_expression::<_, J>(&mut iter) {
            Ok(prog) => prog,
            Err(e) => panic!("program {} failed: {}", prog_hex, e),
        };

        assert_eq!(
            prog.cmr().to_string(),
            cmr_str,
            "CMR mismatch (got {} expected {}) for program {}",
            prog.cmr().to_string(),
            cmr_str,
            prog_hex,
        );
        if amr_str.is_some() || imr_str.is_some() {
            let fprog = prog
                .finalize_types()
                .expect("finalizing types")
                .finalize_no_witnesses()
                .expect("can't be finalized without witnesses; can't check AMR or IMR");
            if let Some(amr) = amr_str {
                assert_eq!(
                    fprog.amr().to_string(),
                    amr,
                    "AMR mismatch (got {} expected {}) for program {}",
                    fprog.amr().to_string(),
                    amr,
                    prog_hex,
                );
            }
            if let Some(imr) = imr_str {
                assert_eq!(
                    fprog.imr().to_string(),
                    imr,
                    "IMR mismatch (got {} expected {}) for program {}",
                    fprog.imr().to_string(),
                    imr,
                    prog_hex,
                );
            }
        }

        let prog = crate::CommitNode::from_node(prog.as_ref());
        let mut reser_sink = Vec::<u8>::new();
        let mut w = BitWriter::from(&mut reser_sink);
        prog.encode(&mut w)
            .expect("reserializing program into vector");
        assert_eq!(
            prog_bytes,
            &reser_sink[..],
            "program {} reserialized as {}",
            prog_hex,
            reser_sink.to_hex(),
        );

        prog
    }

    fn assert_program_not_deserializable<J: Jet>(prog: &[u8], err: Error) {
        let prog_hex = prog.to_hex();
        let err_str = err.to_string();

        let mut iter = BitIter::from(prog);
        match decode_program::<_, J>(&mut iter) {
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
    fn canonical_order() {
        // "main = comp unit iden", but with the iden serialized before the unit
        // To obtain this test vector I temporarily swapped `get_left` and `get_right`
        // in the implementation of `PostOrderIter`
        assert_program_not_deserializable::<Core>(&[0xa8, 0x48, 0x10], Error::NotInCanonicalOrder);

        // "main = iden", but prefixed by some unused nodes, the first of which is also iden.
        assert_program_not_deserializable::<Core>(
            &[0xc1, 0x00, 0x06, 0x20],
            Error::NotInCanonicalOrder,
        );
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
            Some("e053520f0c3219d1cabd705b4523ccd05c8d703a70f6f3994a20774a42b5ccfc"),
            Some("7b0ad0514279280d5c2ac1da729222936b8768d9f465c6c6ade3b0ed7dc97263"),
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
            Some("eaf95c23d967563132b65e43578fe08dae2a29ac66775ddd37af3ac7de28678b"),
            Some("d2927a9a54ddea8359ee00aa27e0aa1e354cc6924b090c759e2ed686712700a0"),
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
            Some("ea1ee417816a57b80739520c7319c33a39a5f4ce7b59856e69f768d5d8f174a6"),
            Some("f262f83f1c9341390e015e4c5126f3954e17a1f275af73da2948eaf4797fda48"),
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
            Some("d5b05a5da87ee490312279496e12e17bc987c98219d8961bc3a7c3ec95a7ce1e"),
            Some("3579ae2a05bbe689f16bd3ff29d840ae8aa8bbad70f6de27b7473746637abeb6"),
        );
    }

    #[test]
    fn hidden_node() {
        // main = hidden deadbeefdeadbeefdeadbeefdeadbeefdeadbeefdeadbeefdeadbeefdeadbeef
        #[rustfmt::skip]
        let hidden = [
            0x36, 0xf5, 0x6d, 0xf7, 0x7e, 0xf5, 0x6d, 0xf7,
            0x7e, 0xf5, 0x6d, 0xf7, 0x7e, 0xf5, 0x6d, 0xf7,
            0x7e, 0xf5, 0x6d, 0xf7, 0x7e, 0xf5, 0x6d, 0xf7,
            0x7e, 0xf5, 0x6d, 0xf7, 0x7e, 0xf5, 0x6d, 0xf7,
            78,
        ];
        assert_program_not_deserializable::<Core>(&hidden, Error::HiddenNode);

        // main = comp witness hidden deadbeefdeadbeefdeadbeefdeadbeefdeadbeefdeadbeefdeadbeefdeadbeef
        let hidden = [
            0xae, 0xdb, 0xd5, 0xb7, 0xdd, 0xfb, 0xd5, 0xb7, 0xdd, 0xfb, 0xd5, 0xb7, 0xdd, 0xfb,
            0xd5, 0xb7, 0xdd, 0xfb, 0xd5, 0xb7, 0xdd, 0xfb, 0xd5, 0xb7, 0xdd, 0xfb, 0xd5, 0xb7,
            0xdd, 0xfb, 0xd5, 0xb7, 0xdd, 0xe0, 0x80,
        ];
        assert_program_not_deserializable::<Core>(&hidden, Error::HiddenNode);
    }

    #[test]
    fn case_both_children_hidden() {
        // h1 = hidden deadbeefdeadbeefdeadbeefdeadbeefdeadbeefdeadbeefdeadbeefdeadbeef
        // main = case h1 h1
        #[rustfmt::skip]
        let hidden = [
            0x8d, 0xbd, 0x5b, 0x7d, 0xdf, 0xbd, 0x5b, 0x7d,
            0xdf, 0xbd, 0x5b, 0x7d, 0xdf, 0xbd, 0x5b, 0x7d,
            0xdf, 0xbd, 0x5b, 0x7d, 0xdf, 0xbd, 0x5b, 0x7d,
            0xdf, 0xbd, 0x5b, 0x7d, 0xdf, 0xbd, 0x5b, 0x7d,
            0xde, 0x10,
        ];
        assert_program_not_deserializable::<Core>(&hidden, Error::BothChildrenHidden);
    }

    #[test]
    fn shared_witnesses() {
        // main = witness :: A -> B
        assert_program_deserializable::<Core>(
            &[0x38],
            "bf12681a76fc7c00c63e583c25cc97237337d6aca30d3f4a664075445385c648",
            Some("e7e02bc77e86cfd4dd48b3ccea00a60d519e0d8cfcc021826e193116a15eaa1c"),
            Some("78dcc84f6accf009d29ac434fa095f0b175cf9813b0efff0e2fcc6b8dc9196ae"),
        );

        // # Program which demands two 32-bit witnesses, the first one == the second + 1
        // wit1 = witness :: 1 -> 2^32
        // wit2 = witness :: 1 -> 2^32
        //
        // wit_diff = comp (comp (pair wit1 wit2) jet_subtract_32) (drop iden) :: 1 -> 2^32
        // diff_is_one = comp (pair wit_diff jet_one_32) jet_eq_32             :: 1 -> 2
        // main = comp diff_is_one jet_verify                                  :: 1 -> 1
        #[rustfmt::skip]
        let diff1s = vec![
            (
                vec![
                    0xda, 0xe2, 0x39, 0xa3, 0x10, 0x42, 0x0e, 0x05,
                    0x71, 0x88, 0xa3, 0x6d, 0xc4, 0x11, 0x80, 0x80
                ],
                // CMR not checked against C code, since C won't give us any data without witnesses
                "a2ad9852818c0dc9307b476464cb9366c5c97896ba128f2f526b51910218293c",
                None,
                None,
            ),
            // Same program but with each `witness` replaced by `comp iden witness`, which
            // is semantically the same but will trip up naive witness-unsharing logic.
            (
                vec![
                    0xde, 0x87, 0x04, 0x08, 0xe6, 0x8c, 0x41, 0x08,
                    0x38, 0x15, 0xc6, 0x22, 0x8d, 0xb7, 0x10, 0x46,
                    0x02, 0x00,
                ],
                // CMR not checked against C code, since C won't give us any data without witnesses
                "f4583eca2a35aa48e8895235b58cfe90ba2196fbf7722b7d847c3c55eb6bdc0e",
                None,
                None,
            )
        ];

        for (diff1, cmr, amr, imr) in diff1s {
            let diff1_prog = assert_program_deserializable::<Core>(&diff1, cmr, amr, imr);

            // Attempt to finalize, providing 32-bit witnesses 0, 1, ..., and then
            // counting how many were consumed afterward.
            let mut counter = 0..100;
            let mut witness_iter = (&mut counter).rev().map(Value::u32);
            let diff1_final = diff1_prog.finalize(&mut witness_iter, true).unwrap();
            assert_eq!(counter, 0..98);

            // Execute the program to confirm that it worked
            let mut mac = BitMachine::for_program(&diff1_final);
            mac.exec(&diff1_final, &()).unwrap();
        }
    }

    #[test]
    fn root_unit_to_unit() {
        // main = jet_eq_32 :: 2^64 -> 2 # 7387d279
        let justjet = vec![0x6d, 0xb8, 0x80];
        // Should be able to decode this as a CommitNode...
        let mut iter = BitIter::from(&justjet[..]);
        CommitNode::<Core>::decode::<_>(&mut iter).unwrap();
        // ...but not as a program
        let mut iter = BitIter::from(&justjet[..]);
        decode_program::<_, Core>(&mut iter).unwrap_err();
    }

    #[test]
    fn extra_nodes() {
        // main = comp unit unit # but with an extra unconnected `unit` stuck on the beginning
        // I created this unit test by hand
        assert_program_not_deserializable::<Core>(&[0xa9, 0x48, 0x00], Error::NotInCanonicalOrder);
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
        // Note: we cannot use `assert_program_deserializable` since the encoded program includes
        //       witness data, which we don't parse and won't reseriaize.
        let mut iter = BitIter::from(&schnorr0[..]);
        let prog =
            decode_program::<_, crate::jet::Elements>(&mut iter).expect("can't decode schnorr0");

        // Matches C source code
        #[rustfmt::skip]
        assert_eq!(
            prog.cmr(),
            Cmr::from_byte_array([
                0x7b, 0xc5, 0x6c, 0xb1, 0x6d, 0x84, 0x99, 0x9b,
                0x97, 0x7b, 0x58, 0xe1, 0xbc, 0x71, 0xdb, 0xe9,
                0xed, 0xcc, 0x33, 0x65, 0x0a, 0xfc, 0x8a, 0x6e,
                0xe0, 0x5c, 0xfe, 0xf8, 0xd6, 0x08, 0x13, 0x2b,
            ]),
        );
    }

    #[test]
    fn decode_fixed_natural() {
        let tries = vec![
            (1, vec![false]),
            (2, vec![true, false, false]),
            (3, vec![true, false, true]),
            (4, vec![true, true, false, false, false, false]),
            (5, vec![true, true, false, false, false, true]),
            (6, vec![true, true, false, false, true, false]),
            (7, vec![true, true, false, false, true, true]),
            (8, vec![true, true, false, true, false, false, false]),
            (15, vec![true, true, false, true, true, true, true]),
            (
                16,
                vec![
                    true, true, true, false, // len: 1
                    false, // len: 2
                    false, false, // len: 4
                    false, false, false, false,
                ],
            ),
            // 31
            (
                31,
                vec![
                    true, true, true, false, // len: 1
                    false, // len: 2
                    false, false, // len: 4
                    true, true, true, true,
                ],
            ),
            // 32
            (
                32,
                vec![
                    true, true, true, false, // len: 1
                    false, // len: 2
                    false, true, // len: 5
                    false, false, false, false, false,
                ],
            ),
            // 2^15
            (
                32768,
                vec![
                    true, true, true, false, // len: 1
                    true,  // len: 3
                    true, true, true, // len: 15
                    false, false, false, false, false, false, false, false, false, false, false,
                    false, false, false, false,
                ],
            ),
            (
                65535,
                vec![
                    true, true, true, false, // len: 1
                    true,  // len: 3
                    true, true, true, // len: 15
                    true, true, true, true, true, true, true, true, true, true, true, true, true,
                    true, true,
                ],
            ),
            (
                65536,
                vec![
                    true, true, true, true, false, // len: 1
                    false, // len: 2
                    false, false, // len: 4
                    false, false, false, false, // len: 16
                    false, false, false, false, false, false, false, false, false, false, false,
                    false, false, false, false, false,
                ],
            ),
        ];

        for (natural, bitvec) in tries {
            let truncated = bitvec[0..bitvec.len() - 1].to_vec();
            assert!(matches!(
                decode_natural(&mut truncated.into_iter(), None),
                Err(Error::EndOfStream)
            ));

            let mut sink = Vec::<u8>::new();

            let mut w = BitWriter::from(&mut sink);
            encode::encode_natural(natural, &mut w).expect("encoding to vector");
            w.flush_all().expect("flushing");
            assert_eq!(w.n_total_written(), bitvec.len());

            let decoded_natural = decode_natural(&mut BitIter::from(sink.into_iter()), None)
                .expect("decoding from vector");
            assert_eq!(natural, decoded_natural);
        }
    }
}
