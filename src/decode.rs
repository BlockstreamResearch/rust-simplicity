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

use crate::bititer::BitIter;
use crate::core::term::UntypedProgram;
use crate::core::types::{FinalType, FinalTypeInner, TypedProgram};
use crate::jet::Application;
use crate::merkle::cmr::Cmr;
use crate::Value;
use crate::{Error, Term};

/// Decode an untyped Simplicity program from bits.
pub fn decode_program_no_witness<I: Iterator<Item = u8>, App: Application>(
    iter: &mut BitIter<I>,
) -> Result<UntypedProgram<(), App>, Error> {
    let prog_len = decode_natural(iter, None)?;

    // FIXME: check maximum length of DAG that is allowed by consensus
    if prog_len > 1_000_000 {
        return Err(Error::TooManyNodes(prog_len));
    }

    let mut program = Vec::with_capacity(prog_len);
    for _ in 0..prog_len {
        decode_node(&mut program, iter)?;
    }

    let program = UntypedProgram(program);

    if program.has_canonical_order() {
        Ok(program)
    } else {
        Err(Error::ParseError("Program is not in canonical order!"))
    }
}

/// Decode witness data from bits.
pub fn decode_witness<Wit, App: Application, I: Iterator<Item = u8>>(
    program: &TypedProgram<Wit, App>,
    iter: &mut BitIter<I>,
) -> Result<Vec<Value>, Error> {
    let bit_len = match iter.next() {
        Some(false) => 0,
        Some(true) => decode_natural(iter, None)?,
        None => return Err(Error::EndOfStream),
    };
    let mut witness = Vec::new();
    let n_start = iter.n_total_read();

    for node in &program.0 {
        if let Term::Witness(_old_witness) = &node.node {
            witness.push(decode_value(&node.target_ty, iter)?);
        }
    }

    if iter.n_total_read() - n_start > bit_len {
        Err(Error::ParseError(
            "Witness bit string is longer than defined in its preamble!",
        ))
    } else {
        Ok(witness)
    }
}

/// Decode a value from bits, based on the given type.
pub fn decode_value<I: Iterator<Item = bool>>(
    ty: &FinalType,
    iter: &mut I,
) -> Result<Value, Error> {
    let value = match ty.ty {
        FinalTypeInner::Unit => Value::Unit,
        FinalTypeInner::Sum(ref l, ref r) => match iter.next() {
            Some(false) => Value::SumL(Box::new(decode_value(l, iter)?)),
            Some(true) => Value::SumR(Box::new(decode_value(r, iter)?)),
            None => return Err(Error::EndOfStream),
        },
        FinalTypeInner::Product(ref l, ref r) => Value::Prod(
            Box::new(decode_value(l, iter)?),
            Box::new(decode_value(r, iter)?),
        ),
    };

    Ok(value)
}

/// Decode an untyped Simplicity term from bits and add it to the given program.
fn decode_node<I: Iterator<Item = u8>, App: Application>(
    program: &mut Vec<Term<(), App>>,
    iter: &mut BitIter<I>,
) -> Result<(), Error> {
    match iter.next() {
        None => return Err(Error::EndOfStream),
        Some(true) => return decode_jet(program, iter),
        Some(false) => {}
    };

    let code = match iter.read_bits_be(2) {
        Some(n) => n,
        None => return Err(Error::EndOfStream),
    };
    let subcode = match iter.read_bits_be(if code < 3 { 2 } else { 1 }) {
        Some(n) => n,
        None => return Err(Error::EndOfStream),
    };
    let node = if code <= 1 {
        let idx = program.len();
        let i = decode_natural(iter, Some(idx))?;

        if code == 0 {
            let j = decode_natural(iter, Some(idx))?;

            match subcode {
                0 => Term::Comp(i, j),
                1 => {
                    let mut node = Term::Case(i, j);
                    let mut left_hidden = false;

                    if let Term::Hidden(..) = program[idx - i] {
                        node = Term::AssertR(i, j);
                        left_hidden = true;
                    }
                    if let Term::Hidden(..) = program[idx - j] {
                        if left_hidden {
                            return Err(Error::CaseMultipleHiddenChildren);
                        }

                        node = Term::AssertL(i, j);
                    }

                    node
                }
                2 => Term::Pair(i, j),
                3 => Term::Disconnect(i, j),
                _ => unreachable!(),
            }
        } else {
            match subcode {
                0 => Term::InjL(i),
                1 => Term::InjR(i),
                2 => Term::Take(i),
                3 => Term::Drop(i),
                _ => unreachable!(),
            }
        }
    } else if code == 2 {
        match subcode {
            0 => Term::Iden,
            1 => Term::Unit,
            2 => Term::Fail(Cmr::from(decode_hash(iter)?), Cmr::from(decode_hash(iter)?)),
            3 => return Err(Error::ParseError("01011 (stop code)")),
            _ => unreachable!(),
        }
    } else if code == 3 {
        match subcode {
            0 => Term::Hidden(Cmr::from(decode_hash(iter)?)),
            1 => Term::Witness(()),
            _ => unreachable!(),
        }
    } else {
        unreachable!()
    };

    program.push(node);
    Ok(())
}

/// Decode a Simplicity jet from bits.
fn decode_jet<I: Iterator<Item = u8>, App: Application>(
    program: &mut Vec<Term<(), App>>,
    iter: &mut BitIter<I>,
) -> Result<(), Error> {
    let node = Term::Jet(App::decode_jet(iter)?);
    program.push(node);
    Ok(())
}

/// Decode a 256-bit hash from bits.
fn decode_hash<I: Iterator<Item = u8>>(iter: &mut BitIter<I>) -> Result<[u8; 32], Error> {
    let mut h = [0; 32];

    for b in &mut h {
        match iter.read_bits_be(8) {
            Some(n) => *b = n as u8,
            None => return Err(Error::EndOfStream),
        }
    }

    Ok(h)
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
    use crate::encode::BitWriter;

    #[test]
    fn decode_fixed() {
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
            assert_matches!(
                decode_natural(&mut truncated.into_iter(), None),
                Err(Error::EndOfStream)
            );

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
