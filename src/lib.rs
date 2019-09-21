pub mod encode;
pub mod types;

use std::fmt;

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum Node<Ref, Witness> {
    Iden,
    Unit,
    InjL(Ref),
    InjR(Ref),
    Take(Ref),
    Drop(Ref),
    Comp(Ref, Ref),
    Case(Ref, Ref),
    Pair(Ref, Ref),
    Disconnect(Ref, Ref),
    Witness(Witness),
    Fail([u8; 32], [u8; 32]),
    Hidden([u8; 32]),
}

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub enum Value {
    Unit,
    SumL(Box<Value>),
    SumR(Box<Value>),
    Prod(Box<Value>, Box<Value>),
}

impl Value {
    pub fn len(&self) -> usize {
        match *self {
            Value::Unit => 0,
            Value::SumL(ref s) => 1 + s.len(),
            Value::SumR(ref s) => 1 + s.len(),
            Value::Prod(ref s, ref t) => s.len() + t.len(),
        }
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
    ) -> Result<Value, encode::Error> {
        match ty.ty {
            types::FinalTypeInner::Unit => Ok(Value::Unit),
            types::FinalTypeInner::Sum(ref l, ref r) => match bits.next() {
                Some(false) => Ok(Value::SumL(Box::new(Value::from_witness(bits, &*l)?))),
                Some(true) => Ok(Value::SumR(Box::new(Value::from_witness(bits, &*r)?))),
                None => Err(encode::Error::EndOfStream),
            },
            types::FinalTypeInner::Product(ref l, ref r) => Ok(Value::Prod(
                Box::new(Value::from_witness(&mut *bits, &*l)?),
                Box::new(Value::from_witness(bits, &*r)?),
            )),
        }
    }
}
