use crate::merkle::common::{MerkleRoot, TypeMerkleRoot};
use crate::merkle::tmr::Tmr;
use crate::util::replace_to_buffer;
use std::collections::VecDeque;
use std::iter::FromIterator;
use std::{cell::RefCell, cmp, fmt, rc::Rc, sync::Arc};

/// Finalized Simplicity type.
///
/// _Unit_ is the base type and is for _unit_ values.
///
/// The _sum_ of types A and B is for values
/// that are the _left sum_ of a value of type A or the _right sum_ of a value of type B.
///
/// The _product_ of types A and B is for values
/// that are the _product_ of a value of type A and a value of type B.
///
/// _(see [`crate::core::Value`])_
#[derive(Clone, PartialOrd, Ord, Debug)]
pub struct Type {
    /// Underlying type with references to sub-types
    pub inner: TypeInner,
    /// Bit width of the type
    pub bit_width: usize,
    /// Annotated Type Merkle root of the type
    pub tmr: Tmr,
    /// Cached display result in order to avoid repeat computation
    pub display: String,
}

impl Type {
    /// Return a unit type.
    pub fn unit() -> Arc<Self> {
        let inner = TypeInner::Unit;

        Arc::new(Self {
            tmr: Tmr::get_iv(&inner),
            inner,
            bit_width: 0,
            display: "1".to_owned(),
        })
    }

    /// Return the sum of the given two types.
    pub fn sum(a: Arc<Self>, b: Arc<Self>) -> Arc<Self> {
        let inner = TypeInner::Sum(a.clone(), b.clone());

        Arc::new(Self {
            tmr: Tmr::get_iv(&inner).update(a.tmr, b.tmr),
            inner,
            bit_width: 1 + cmp::max(a.bit_width, b.bit_width),
            display: if a.inner == TypeInner::Unit && b.inner == TypeInner::Unit {
                "2".to_owned()
            } else {
                format!("({} + {})", a.display, b.display)
            },
        })
    }

    /// Return the product of the given two types.
    pub fn product(a: Arc<Self>, b: Arc<Self>) -> Arc<Self> {
        let inner = TypeInner::Product(a.clone(), b.clone());

        Arc::new(Self {
            tmr: Tmr::get_iv(&inner).update(a.tmr, b.tmr),
            inner,
            bit_width: a.bit_width + b.bit_width,
            display: if a.display == b.display {
                match a.display.as_str() {
                    "2" => "2^2".to_owned(),
                    "2^2" => "2^4".to_owned(),
                    "2^4" => "2^8".to_owned(),
                    "2^8" => "2^16".to_owned(),
                    "2^16" => "2^32".to_owned(),
                    "2^32" => "2^64".to_owned(),
                    "2^64" => "2^128".to_owned(),
                    "2^128" => "2^256".to_owned(),
                    _ => format!("({} × {})", a.display, b.display),
                }
            } else {
                format!("({} × {})", a.display, b.display)
            },
        })
    }
}

impl fmt::Display for Type {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(&self.display)
    }
}

impl PartialEq for Type {
    fn eq(&self, other: &Self) -> bool {
        self.tmr.eq(&other.tmr)
    }
}

impl Eq for Type {}

impl std::hash::Hash for Type {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.tmr.hash(state)
    }
}

/// Finalized Simplicity type without metadata (see [`Type`])
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum TypeInner {
    /// Unit type
    Unit,
    /// Sum of two types
    Sum(Arc<Type>, Arc<Type>),
    /// Product of two types
    Product(Arc<Type>, Arc<Type>),
}

/// Variable Simplicity type.
///
/// In contrast to [`Type`],
/// a [`VariableType`] contains variables that can be internally mutated.
#[derive(Clone, Debug)]
pub(crate) enum VariableType {
    /// Unit type
    Unit,
    /// Sum of two types
    Sum(RcVar, RcVar),
    /// Product of two types
    Product(RcVar, RcVar),
}

impl VariableType {
    fn uncompressed_display<W: fmt::Write>(&self, w: &mut W) -> fmt::Result {
        match self {
            VariableType::Unit => w.write_str("1"),
            VariableType::Sum(a, b) => {
                w.write_str("(")?;
                a.borrow().inner.uncompressed_display(w)?;
                w.write_str(" + ")?;
                b.borrow().inner.uncompressed_display(w)?;
                w.write_str(")")
            }
            VariableType::Product(a, b) => {
                w.write_str("(")?;
                a.borrow().inner.uncompressed_display(w)?;
                w.write_str(" × ")?;
                b.borrow().inner.uncompressed_display(w)?;
                w.write_str(")")
            }
            // Uncomment to use intermediate compression: more allocation of smaller strings
            // VariableType::Sum(a, b) => write!(w, "({} + {})", a, b),
            // VariableType::Product(a, b) => write!(w, "({} × {})", a, b),
        }
    }
}

fn into_compressed_display(mut buffer1: String) -> String {
    let mut buffer2 = String::with_capacity(buffer1.len());

    replace_to_buffer(&buffer1, &mut buffer2, "(1 + 1)", "2");
    buffer1.clear();
    replace_to_buffer(&buffer2, &mut buffer1, "(2 × 2)", "2^2");
    buffer2.clear();
    replace_to_buffer(&buffer1, &mut buffer2, "(2^2 × 2^2)", "2^4");
    buffer1.clear();
    replace_to_buffer(&buffer2, &mut buffer1, "(2^4 × 2^4)", "2^8");
    buffer2.clear();
    replace_to_buffer(&buffer1, &mut buffer2, "(2^8 × 2^8)", "2^16");
    buffer1.clear();
    replace_to_buffer(&buffer2, &mut buffer1, "(2^16 × 2^16)", "2^32");
    buffer2.clear();
    replace_to_buffer(&buffer1, &mut buffer2, "(2^32 × 2^32)", "2^64");
    buffer1.clear();
    replace_to_buffer(&buffer2, &mut buffer1, "(2^64 × 2^64)", "2^128");
    buffer2.clear();
    replace_to_buffer(&buffer1, &mut buffer2, "(2^128 × 2^128)", "2^256");

    buffer2
}

impl fmt::Display for VariableType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut buffer1 = String::new();
        self.uncompressed_display(&mut buffer1)?;
        let buffer2 = into_compressed_display(buffer1);

        write!(f, "{}", buffer2)
    }
}

/// Variable that can be cheaply cloned and internally mutated
pub(crate) type RcVar = Rc<RefCell<Variable>>;

/// Unification variable
#[derive(Clone)]
pub(crate) struct Variable {
    /// Underlying variable
    pub inner: VariableInner,
    /// Rank for union-first algorithm
    pub rank: usize,
}

impl Variable {
    /// Return a free variable.
    pub fn free(id: String) -> RcVar {
        Rc::new(RefCell::new(Self {
            inner: VariableInner::Free(id),
            rank: 0,
        }))
    }

    /// Return a variable that is bound to the given type.
    pub fn bound(ty: VariableType) -> RcVar {
        Rc::new(RefCell::new(Self {
            inner: VariableInner::Bound(ty, false),
            rank: 0,
        }))
    }

    /// Return an array `pow2s` of types such that `pow2s[i] = 2^i` holds for 0 ≤ `i` < 9.
    pub fn powers_of_two() -> [RcVar; 9] {
        let two_0 = Variable::bound(VariableType::Unit);
        let two_1 = Variable::bound(VariableType::Sum(two_0.clone(), two_0));
        let two_2 = Variable::bound(VariableType::Product(two_1.clone(), two_1.clone()));
        let two_4 = Variable::bound(VariableType::Product(two_2.clone(), two_2.clone()));
        let two_8 = Variable::bound(VariableType::Product(two_4.clone(), two_4.clone()));
        let two_16 = Variable::bound(VariableType::Product(two_8.clone(), two_8.clone()));
        let two_32 = Variable::bound(VariableType::Product(two_16.clone(), two_16.clone()));
        let two_64 = Variable::bound(VariableType::Product(two_32.clone(), two_32.clone()));
        let two_128 = Variable::bound(VariableType::Product(two_64.clone(), two_64.clone()));
        let two_256 = Variable::bound(VariableType::Product(two_128.clone(), two_128.clone()));

        [
            two_1, two_2, two_4, two_8, two_16, two_32, two_64, two_128, two_256,
        ]
    }
}

impl fmt::Debug for Variable {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}]{:?}", self.rank, self.inner)
    }
}

impl fmt::Display for Variable {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut buffer1 = String::new();
        self.inner.uncompressed_display(&mut buffer1)?;
        let buffer2 = into_compressed_display(buffer1);

        write!(f, "{}", buffer2)
    }
}

/// Unification variable without metadata (see [`Variable`])
#[derive(Clone)]
pub(crate) enum VariableInner {
    /// Free variable with some identifier
    Free(String),
    /// Variable bound to some variable type.
    /// Contains a Boolean to check if this variable already occurred _(occurs check)_
    Bound(VariableType, bool),
    /// Variable equal to another variable.
    /// In the union-find algorithm, this is the _parent_
    EqualTo(RcVar),
    /// Variable bound to some finalized type
    Finalized(Arc<Type>),
}

impl fmt::Debug for VariableInner {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            VariableInner::Free(id) => fmt::Debug::fmt(id, f),
            VariableInner::Bound(ty, b) => write!(f, "[{:?}/{}]", ty, b),
            VariableInner::EqualTo(parent) => write!(f, "={:?}", parent),
            VariableInner::Finalized(ty) => fmt::Debug::fmt(ty, f),
        }
    }
}

impl VariableInner {
    fn uncompressed_display<W: fmt::Write>(&self, w: &mut W) -> fmt::Result {
        match self {
            VariableInner::Free(id) => w.write_str(id),
            VariableInner::Bound(ty, _) => ty.uncompressed_display(w),
            // Uncomment to use intermediate compression: more allocation of smaller strings
            // VariableInner::Bound(ty, _) => write!(w, "{}", ty),
            VariableInner::EqualTo(parent) => parent.borrow().inner.uncompressed_display(w),
            VariableInner::Finalized(ty) => write!(w, "{}", ty),
        }
    }
}

/// Factory for creating free variables with fresh names.
/// Identifiers are assigned sequentially as follows:
/// `A`, `B`, `C`, ... `Z`, `AA`, `AB`, `AC`, ...
pub(crate) struct VariableFactory {
    next_id: usize,
}

impl VariableFactory {
    /// Create a new factory.
    pub fn new() -> Self {
        Self { next_id: 1 }
    }

    /// Return a free variable with a fresh name.
    pub fn free_variable(&mut self) -> RcVar {
        let mut n = self.next_id;
        self.next_id += 1;
        let mut ascii_bytes = VecDeque::new();

        while n / 26 > 0 {
            ascii_bytes.push_front((n % 26) as u8 + 65);
            n /= 26;
        }

        ascii_bytes.push_front((n % 26) as u8 + 64);
        let id = String::from_utf8(Vec::from_iter(ascii_bytes.into_iter())).unwrap();
        Variable::free(id)
    }
}
