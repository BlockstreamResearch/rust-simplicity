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

extern crate bitcoin_hashes;
extern crate byteorder;

pub mod bititer;
pub mod cmr;
pub mod encode;
pub mod exec;
pub mod extension;
pub mod program;
pub mod types;

use std::fmt;

pub use program::Program;

/// De/serialization error
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum Error {
    /// Unable to unify types in a DAG
    TypeCheck,
    /// A recursive type was inferred, violating the "occurs check" of the
    /// type inference engine
    OccursCheck,
    /// Node made a back-reference past the beginning of the program
    BadIndex,
    /// Number exceeded 32 bits
    NaturalOverflow,
    /// Non-'case' nodes may not have hidden children
    NonCaseHiddenChild,
    /// 'case' nodes may have at most one hidden child
    CaseMultipleHiddenChildren,
    /// Bitstream ended early
    EndOfStream,
    /// Tried to allocate too many nodes in a program
    TooManyNodes(usize),
    /// Unrecognized node
    ParseError(&'static str),
}

/// Simplicity expression node, including Bitcoin/Elements extensions
///
/// If Bitcoin/Elements support is not compiled (see `bitcoin` and
/// `elements` features) programs using these extensions will fail to
/// parse.
///
/// While programs are *serialized* with all references being relative
/// indices, they are represented in this type as having absolute
/// indices starting from 0. This means that this type only really makes
/// sense in the context of a complete program. Expressions/partial
/// programs will need a different type.
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum Node<Witness, Extension> {
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
    Witness(Witness),
    Fail([u8; 32], [u8; 32]),
    Hidden(cmr::Cmr),
    Ext(Extension),
    Jet(extension::jets::Node),
}

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub enum Value {
    Unit,
    SumL(Box<Value>),
    SumR(Box<Value>),
    Prod(Box<Value>, Box<Value>),
}

impl Value {
    /// The length, in bits, of the value when encoded in the Bit Machine
    pub fn len(&self) -> usize {
        match *self {
            Value::Unit => 0,
            Value::SumL(ref s) => 1 + s.len(),
            Value::SumR(ref s) => 1 + s.len(),
            Value::Prod(ref s, ref t) => s.len() + t.len(),
        }
    }

    /// Encode a single bit as a value. Will panic if the input is out of range
    pub fn u1(n: u8) -> Value {
        match n {
            0 => Value::SumL(Box::new(Value::Unit)),
            1 => Value::SumR(Box::new(Value::Unit)),
            x => panic!("{} out of range for Value::u1", x),
        }
    }

    /// Encode a two-bit number as a value. Will panic if the input is out of range
    pub fn u2(n: u8) -> Value {
        let b0 = (n & 2) / 2;
        let b1 = n & 1;
        if n > 3 {
            panic!("{} out of range for Value::u2", n);
        }
        Value::Prod(Box::new(Value::u1(b0)), Box::new(Value::u1(b1)))
    }

    /// Encode a four-bit number as a value. Will panic if the input is out of range
    pub fn u4(n: u8) -> Value {
        let w0 = (n & 12) / 4;
        let w1 = n & 3;
        if n > 7 {
            panic!("{} out of range for Value::u2", n);
        }
        Value::Prod(Box::new(Value::u2(w0)), Box::new(Value::u2(w1)))
    }

    /// Encode an eight-bit number as a value
    pub fn u8(n: u8) -> Value {
        let w0 = n >> 4;
        let w1 = n & 0xf;
        Value::Prod(Box::new(Value::u4(w0)), Box::new(Value::u4(w1)))
    }

    /// Encode a 16-bit number as a value
    pub fn u16(n: u16) -> Value {
        let w0 = (n >> 8) as u8;
        let w1 = (n & 0xff) as u8;
        Value::Prod(Box::new(Value::u8(w0)), Box::new(Value::u8(w1)))
    }

    /// Encode a 32-bit number as a value
    pub fn u32(n: u32) -> Value {
        let w0 = (n >> 16) as u16;
        let w1 = (n & 0xffff) as u16;
        Value::Prod(Box::new(Value::u16(w0)), Box::new(Value::u16(w1)))
    }

    /// Encode a 32-bit number as a value
    pub fn u64(n: u64) -> Value {
        let w0 = (n >> 32) as u32;
        let w1 = (n & 0xffffffff) as u32;
        Value::Prod(Box::new(Value::u32(w0)), Box::new(Value::u32(w1)))
    }

    /// Convenience constructor for a left sum of a value
    pub fn sum_l(a: Value) -> Value {
        Value::SumL(Box::new(a))
    }

    /// Convenience constructor for a right sum of a value
    pub fn sum_r(a: Value) -> Value {
        Value::SumR(Box::new(a))
    }

    /// Convenience constructor for product of two values
    pub fn prod(a: Value, b: Value) -> Value {
        Value::Prod(Box::new(a), Box::new(b))
    }
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Value::Unit => f.write_str("ε"),
            Value::SumL(ref sub) => {
                f.write_str("0")?;
                if **sub != Value::Unit {
                    write!(f, "{}", sub)
                } else {
                    Ok(())
                }
            }
            Value::SumR(ref sub) => {
                f.write_str("1")?;
                if **sub != Value::Unit {
                    write!(f, "{}", sub)
                } else {
                    Ok(())
                }
            }
            Value::Prod(ref l, ref r) => {
                write!(f, "({},", l)?;
                write!(f, "{})", r)
            }
        }
    }
}

impl Value {
    pub fn from_witness<Bits: Iterator<Item = bool>>(
        bits: &mut Bits,
        ty: &types::FinalType,
    ) -> Result<Value, Error> {
        match ty.ty {
            types::FinalTypeInner::Unit => Ok(Value::Unit),
            types::FinalTypeInner::Sum(ref l, ref r) => match bits.next() {
                Some(false) => Ok(Value::SumL(Box::new(Value::from_witness(bits, &*l)?))),
                Some(true) => Ok(Value::SumR(Box::new(Value::from_witness(bits, &*r)?))),
                None => Err(Error::EndOfStream),
            },
            types::FinalTypeInner::Product(ref l, ref r) => Ok(Value::Prod(
                Box::new(Value::from_witness(&mut *bits, &*l)?),
                Box::new(Value::from_witness(bits, &*r)?),
            )),
        }
    }
}
