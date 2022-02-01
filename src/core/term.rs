use super::types;
use crate::Error;
use crate::{cmr, extension};
use std::collections::HashMap;
use std::fmt;
use std::hash::{Hash, Hasher};
use std::rc::Rc;

/// Simplicity expression node, including Bitcoin/Elements extensions
///
/// If Bitcoin/Elements support is not compiled (see `bitcoin` and
/// `elements` features) programs using these extensions will fail to
/// parse.
///
/// All references being relative indices in the context of a program.
/// For ex: InjL(2) at index 7, represents InjL(x) where x is a node
/// at index 5.
/// This is used for representing a final constructed simplicity program.
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub enum Term<Witness, Extension> {
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
    Jet(extension::jets::JetsNode),
}

/// Simplicity expression node, including Bitcoin/Elements extensions
///
/// If Bitcoin/Elements support is not compiled (see `bitcoin` and
/// `elements` features) programs using these extensions will fail to
/// parse.
///
/// The structure stores the simplicity program as a directed acyclic graph(DAG).
/// This structure is useful for creating simplicity programs recursively.
#[derive(Clone, Eq, PartialEq, PartialOrd, Ord, Hash, Debug)]
pub enum DagTerm<Witness, Extension> {
    Iden,
    Unit,
    InjL(Rc<DagTerm<Witness, Extension>>),
    InjR(Rc<DagTerm<Witness, Extension>>),
    Take(Rc<DagTerm<Witness, Extension>>),
    Drop(Rc<DagTerm<Witness, Extension>>),
    Comp(
        Rc<DagTerm<Witness, Extension>>,
        Rc<DagTerm<Witness, Extension>>,
    ),
    Case(
        Rc<DagTerm<Witness, Extension>>,
        Rc<DagTerm<Witness, Extension>>,
    ),
    Pair(
        Rc<DagTerm<Witness, Extension>>,
        Rc<DagTerm<Witness, Extension>>,
    ),
    Disconnect(
        Rc<DagTerm<Witness, Extension>>,
        Rc<DagTerm<Witness, Extension>>,
    ),
    Witness(Witness),
    Fail([u8; 32], [u8; 32]),
    Hidden(cmr::Cmr),
    Ext(Extension),
    Jet(extension::jets::JetsNode),
}

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct UnTypedProg<Witness, Extension>(pub Vec<Term<Witness, Extension>>);

impl<Witness, Extension> UnTypedProg<Witness, Extension> {
    /// Whether this is the null program
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    /// The number of (shared) terms in the program
    pub fn len(&self) -> usize {
        self.0.len()
    }

    /// Returns an iterator over the (shared) terms in the program
    pub fn iter(&self) -> impl Iterator<Item = &Term<Witness, Extension>> {
        self.0.iter()
    }
}

impl<Witness, Extension> DagTerm<Witness, Extension> {
    /// Create a DAG representation from an untyped representation
    pub fn from_untyped_prog(untyped_prog: UnTypedProg<Witness, Extension>) -> Rc<Self> {
        assert!(
            !untyped_prog.0.is_empty(),
            "Untyped Program len must be greater than 0"
        );
        let mut dag: Vec<Rc<DagTerm<_, _>>> = vec![];
        for (index, term) in untyped_prog.0.into_iter().enumerate() {
            let dag_term = match term {
                Term::Iden => Rc::new(DagTerm::Iden),
                Term::Unit => Rc::new(DagTerm::Unit),
                Term::InjL(l) => Rc::new(DagTerm::InjL(Rc::clone(&dag[index - l]))),
                Term::InjR(r) => Rc::new(DagTerm::InjR(Rc::clone(&dag[index - r]))),
                Term::Take(l) => Rc::new(DagTerm::Take(Rc::clone(&dag[index - l]))),
                Term::Drop(r) => Rc::new(DagTerm::Drop(Rc::clone(&dag[index - r]))),
                Term::Comp(l, r) => Rc::new(DagTerm::Comp(
                    Rc::clone(&dag[index - l]),
                    Rc::clone(&dag[index - r]),
                )),
                Term::Case(l, r) => Rc::new(DagTerm::Case(
                    Rc::clone(&dag[index - l]),
                    Rc::clone(&dag[index - r]),
                )),
                Term::Pair(l, r) => Rc::new(DagTerm::Pair(
                    Rc::clone(&dag[index - l]),
                    Rc::clone(&dag[index - r]),
                )),
                Term::Disconnect(l, r) => Rc::new(DagTerm::Disconnect(
                    Rc::clone(&dag[index - l]),
                    Rc::clone(&dag[index - r]),
                )),
                Term::Witness(w) => Rc::new(DagTerm::Witness(w)),
                //TODO: understand how Fail works and rename `a` and `b`
                Term::Fail(a, b) => Rc::new(DagTerm::Fail(a, b)),
                Term::Hidden(c) => Rc::new(DagTerm::Hidden(c)),
                Term::Ext(e) => Rc::new(DagTerm::Ext(e)),
                Term::Jet(j) => Rc::new(DagTerm::Jet(j)),
            };
            dag.push(dag_term);
        }
        Rc::clone(dag.last().unwrap())
    }
}

// A direct comparison for Rc<> results in comparison
// of underllying inner values whereas we desire to
// compare the referececs.
#[derive(Debug)]
struct RcWrapper<Witness, Extension> {
    rc: Rc<DagTerm<Witness, Extension>>,
}

impl<Witness, Extension> PartialEq for RcWrapper<Witness, Extension> {
    fn eq(&self, other: &Self) -> bool {
        Rc::ptr_eq(&self.rc, &other.rc)
    }
}

impl<Witness, Extension> Eq for RcWrapper<Witness, Extension> {}

impl<Witness, Extension> From<Rc<DagTerm<Witness, Extension>>> for RcWrapper<Witness, Extension> {
    fn from(dag: Rc<DagTerm<Witness, Extension>>) -> Self {
        Self { rc: dag }
    }
}

impl<Witness, Extension> Hash for RcWrapper<Witness, Extension>
where
    Witness: Hash,
    Extension: Hash,
{
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.rc.hash(state);
    }
}

impl<Witness, Extension> DagTerm<Witness, Extension>
where
    Witness: Hash + Clone,
    Extension: Hash + Clone,
{
    /// Convert a DagTerm into into a untyped program vec.
    pub fn into_untyped_prog(self) -> UnTypedProg<Witness, Extension> {
        // helper function to recrusively compute the index positions
        // of the children.
        fn into_helper<Witness, Extension>(
            dag: Rc<DagTerm<Witness, Extension>>,
            index_map: &mut HashMap<RcWrapper<Witness, Extension>, usize>,
            prog: &mut Vec<Term<Witness, Extension>>,
        ) -> usize
        where
            RcWrapper<Witness, Extension>: Hash,
            Witness: Clone,
            Extension: Clone,
        {
            // insert one child into prog
            macro_rules! insert_one_child {
                ($term: expr, $l : expr) => {
                    match index_map.get(&RcWrapper::from(Rc::clone($l))) {
                        Some(ind) => prog.push($term(prog.len() - ind)),
                        None => {
                            let ind = into_helper(Rc::clone($l), index_map, prog);
                            prog.push($term(prog.len() - ind));
                        }
                    }
                };
            }

            // insert two children into prog
            macro_rules! insert_two_child {
                ($term: expr, $l : expr, $r: expr) => {{
                    let l_ind = match index_map.get(&RcWrapper::from(Rc::clone($l))) {
                        Some(ind) => *ind,
                        None => into_helper(Rc::clone($l), index_map, prog),
                    };
                    let r_ind = match index_map.get(&RcWrapper::from(Rc::clone($r))) {
                        Some(ind) => *ind,
                        None => into_helper(Rc::clone($r), index_map, prog),
                    };
                    prog.push($term(prog.len() - l_ind, prog.len() - r_ind));
                }};
            }

            if index_map.contains_key(&RcWrapper::from(Rc::clone(&dag))) {
                return *index_map.get(&RcWrapper::from(Rc::clone(&dag))).unwrap();
            }
            match dag.as_ref() {
                DagTerm::Unit => prog.push(Term::Unit),
                DagTerm::Iden => prog.push(Term::Iden),
                DagTerm::InjL(l) => insert_one_child!(Term::InjL, l),
                DagTerm::InjR(r) => insert_one_child!(Term::InjR, r),
                DagTerm::Take(l) => insert_one_child!(Term::Take, l),
                DagTerm::Drop(r) => insert_one_child!(Term::Drop, r),
                DagTerm::Comp(l, r) => insert_two_child!(Term::Comp, l, r),
                DagTerm::Case(l, r) => insert_two_child!(Term::Case, l, r),
                DagTerm::Pair(l, r) => insert_two_child!(Term::Pair, l, r),
                DagTerm::Disconnect(l, r) => insert_two_child!(Term::Disconnect, l, r),
                DagTerm::Witness(ref w) => prog.push(Term::Witness(w.clone())),
                DagTerm::Fail(a, b) => prog.push(Term::Fail(*a, *b)),
                DagTerm::Hidden(cmr) => prog.push(Term::Hidden(*cmr)),
                DagTerm::Ext(ref e) => prog.push(Term::Ext(e.clone())),
                DagTerm::Jet(ref j) => prog.push(Term::Jet(*j)),
            }
            // insert the current node remembering it's index for reusing
            index_map.insert(RcWrapper::from(dag), prog.len() - 1);
            prog.len() - 1
        }

        let mut prog = vec![];
        let mut index_map = HashMap::new();
        let _len = into_helper(Rc::new(self), &mut index_map, &mut prog);

        UnTypedProg(prog)
    }
}

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub enum Value {
    Unit,
    SumL(Box<Value>),
    SumR(Box<Value>),
    Prod(Box<Value>, Box<Value>),
}

impl Value {
    #![allow(clippy::len_without_is_empty)]
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
        if n > 15 {
            panic!("{} out of range for Value::u4", n);
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
        let w1 = (n & 0xffff_ffff) as u32;
        Value::Prod(Box::new(Value::u32(w0)), Box::new(Value::u32(w1)))
    }

    /// Convert the value to a byte array.
    pub fn into_bits(self) -> Vec<bool> {
        let mut ret = vec![];
        fn into_bit_helper(v: Value, ret: &mut Vec<bool>) {
            match v {
                Value::Unit => {}
                Value::SumL(l) => {
                    ret.push(false);
                    into_bit_helper(*l, ret);
                }
                Value::SumR(r) => {
                    ret.push(true);
                    into_bit_helper(*r, ret);
                }
                Value::Prod(l, r) => {
                    into_bit_helper(*l, ret);
                    into_bit_helper(*r, ret);
                }
            }
        }
        into_bit_helper(self, &mut ret);
        ret
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
    pub fn from_bits_and_type<Bits: Iterator<Item = bool>>(
        bits: &mut Bits,
        ty: &types::FinalType,
    ) -> Result<Value, Error> {
        match ty.ty {
            types::FinalTypeInner::Unit => Ok(Value::Unit),
            types::FinalTypeInner::Sum(ref l, ref r) => match bits.next() {
                Some(false) => Ok(Value::SumL(Box::new(Value::from_bits_and_type(bits, &*l)?))),
                Some(true) => Ok(Value::SumR(Box::new(Value::from_bits_and_type(bits, &*r)?))),
                None => Err(Error::EndOfStream),
            },
            types::FinalTypeInner::Product(ref l, ref r) => Ok(Value::Prod(
                Box::new(Value::from_bits_and_type(&mut *bits, &*l)?),
                Box::new(Value::from_bits_and_type(bits, &*r)?),
            )),
        }
    }
}
