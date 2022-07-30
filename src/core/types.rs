use crate::merkle::common::{MerkleRoot, TypeMerkleRoot};
use crate::merkle::tmr::Tmr;
use std::{cell::RefCell, cmp, fmt, rc::Rc, sync::Arc};

#[derive(Clone, Debug)]
pub(crate) enum Type {
    Unit,
    Sum(RcVar, RcVar),
    Product(RcVar, RcVar),
}

impl Type {
    pub(crate) fn into_rcvar(self) -> RcVar {
        Rc::new(RefCell::new(UnificationVar::concrete(self)))
    }
}

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum FinalTypeInner {
    Unit,
    Sum(Arc<FinalType>, Arc<FinalType>),
    Product(Arc<FinalType>, Arc<FinalType>),
}

#[derive(Clone, PartialOrd, Ord, Debug)]
pub struct FinalType {
    pub ty: FinalTypeInner,
    pub bit_width: usize,
    /// The annotated type merkle root of the type
    pub tmr: Tmr,
    /// cached display result in order to avoid repeat computation
    pub display: String,
}

impl FinalType {
    pub fn unit() -> Self {
        let ty = FinalTypeInner::Unit;

        Self {
            tmr: Tmr::get_iv(&ty),
            ty,
            bit_width: 0,
            display: "1".to_owned(),
        }
    }

    pub fn sum(a: Arc<Self>, b: Arc<Self>) -> Self {
        let ty = FinalTypeInner::Sum(a.clone(), b.clone());

        Self {
            tmr: Tmr::get_iv(&ty).update(a.tmr, b.tmr),
            ty,
            bit_width: 1 + cmp::max(a.bit_width, b.bit_width),
            display: if a.ty == FinalTypeInner::Unit && b.ty == FinalTypeInner::Unit {
                "2".to_owned()
            } else {
                format!("({} + {})", a.display, b.display)
            },
        }
    }

    pub fn prod(a: Arc<Self>, b: Arc<Self>) -> Self {
        let ty = FinalTypeInner::Product(a.clone(), b.clone());

        Self {
            tmr: Tmr::get_iv(&ty).update(a.tmr, b.tmr),
            ty,
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
        }
    }
}

impl fmt::Display for FinalType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.display)
    }
}

impl PartialEq for FinalType {
    fn eq(&self, other: &Self) -> bool {
        self.tmr.eq(&other.tmr)
    }
}

impl Eq for FinalType {}

impl std::hash::Hash for FinalType {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.tmr.hash(state)
    }
}

#[derive(Clone)]
pub(crate) enum Variable {
    /// Free variable
    Free,
    /// Bound to some type (which may itself contain other free variables,
    /// or not). Contains a boolean which is only used by the finalization
    /// function, for the occurs-check
    Bound(Type, bool),
    /// Equal to another variable (the included `RcVar` is the "parent"
    /// pointer in union-find terms)
    EqualTo(RcVar),
    /// Complete type has been set in place
    Finalized(Arc<FinalType>),
}

impl fmt::Debug for Variable {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Variable::Free => f.write_str("?"),
            Variable::Bound(ref ty, b) => write!(f, "[{:?}/{}]", ty, b),
            Variable::EqualTo(ref other) => write!(f, "={:?}", other),
            Variable::Finalized(..) => unimplemented!(),
        }
    }
}

pub(crate) struct UnificationVar {
    pub var: Variable,
    pub rank: usize,
}

impl fmt::Debug for UnificationVar {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}]{:?}", self.rank, self.var)
    }
}

pub(crate) type RcVar = Rc<RefCell<UnificationVar>>;

impl UnificationVar {
    pub fn free() -> UnificationVar {
        UnificationVar {
            var: Variable::Free,
            rank: 0,
        }
    }

    pub fn concrete(ty: Type) -> UnificationVar {
        UnificationVar {
            var: Variable::Bound(ty, false),
            rank: 0,
        }
    }
}
