//FIXME: Remove this later
#![allow(dead_code)]

use std::{cell::RefCell, cmp, fmt, mem, rc::Rc, sync::Arc};

use crate::cmr::{tag, Tmr};
use crate::extension;
use crate::extension::Jet as ExtNode;
use crate::Error;
use crate::Term;

use super::term::UnTypedProg;

#[derive(Clone, Debug)]
enum Type {
    Unit,
    Sum(RcVar, RcVar),
    Product(RcVar, RcVar),
}

impl Type {
    fn into_rcvar(self) -> RcVar {
        Rc::new(RefCell::new(UnificationVar::concrete(self)))
    }
}

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum FinalTypeInner {
    Unit,
    Sum(Arc<FinalType>, Arc<FinalType>),
    Product(Arc<FinalType>, Arc<FinalType>),
}

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub struct FinalType {
    pub ty: FinalTypeInner,
    pub bit_width: usize,
    /// The annotated type merkle root of the type
    pub tmr: Tmr,
    /// cached display result in order to avoid repeat computation
    pub display: String,
}

impl FinalType {
    fn unit() -> Self {
        Self {
            ty: FinalTypeInner::Unit,
            bit_width: 0,
            tmr: tag::unit_type_tmr(),
            display: "1".to_owned(),
        }
    }

    fn sum(a: Arc<Self>, b: Arc<Self>) -> Self {
        Self {
            ty: FinalTypeInner::Sum(a.clone(), b.clone()),
            bit_width: 1 + cmp::max(a.bit_width, b.bit_width),
            tmr: tag::sum_type_tmr().update(a.tmr, b.tmr),
            display: if a.ty == FinalTypeInner::Unit && b.ty == FinalTypeInner::Unit {
                "2".to_owned()
            } else {
                format!("({} + {})", a.display, b.display)
            },
        }
    }

    fn prod(a: Arc<Self>, b: Arc<Self>) -> Self {
        Self {
            ty: FinalTypeInner::Product(a.clone(), b.clone()),
            bit_width: a.bit_width + b.bit_width,
            tmr: tag::prod_type_tmr().update(a.tmr, b.tmr),
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

pub(crate) fn pow2_types() -> [Arc<FinalType>; 11] {
    let word0 = Arc::new(FinalType::unit());
    let word1 = Arc::new(FinalType::sum(Arc::clone(&word0), Arc::clone(&word0)));
    let word2 = Arc::new(FinalType::prod(Arc::clone(&word1), Arc::clone(&word1)));
    let word4 = Arc::new(FinalType::prod(Arc::clone(&word2), Arc::clone(&word2)));
    let word8 = Arc::new(FinalType::prod(Arc::clone(&word4), Arc::clone(&word4)));
    let word16 = Arc::new(FinalType::prod(Arc::clone(&word8), Arc::clone(&word8)));
    let word32 = Arc::new(FinalType::prod(Arc::clone(&word16), Arc::clone(&word16)));
    let word64 = Arc::new(FinalType::prod(Arc::clone(&word32), Arc::clone(&word32)));
    let word128 = Arc::new(FinalType::prod(Arc::clone(&word64), Arc::clone(&word64)));
    let word256 = Arc::new(FinalType::prod(Arc::clone(&word128), Arc::clone(&word128)));
    let word512 = Arc::new(FinalType::prod(Arc::clone(&word256), Arc::clone(&word256)));

    [
        word0, word1, word2, word4, word8, word16, word32, word64, word128, word256, word512,
    ]
}

impl fmt::Display for FinalType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.display)
    }
}

impl FinalType {
    pub fn bit_width(&self) -> usize {
        self.bit_width
    }

    fn from_var(var: RcVar) -> Result<Arc<FinalType>, Error> {
        let var = find_root(var);
        let mut var_borr = var.borrow_mut();

        let existing_type = match var_borr.var {
            Variable::Free => Type::Unit,
            Variable::Bound(ref ty, ref mut occurs_check) => {
                if *occurs_check {
                    return Err(Error::OccursCheck);
                }
                *occurs_check = true;
                ty.clone()
            }
            Variable::EqualTo(..) => unreachable!(),
            Variable::Finalized(ref done) => return Ok(done.clone()),
        };

        let (sub1, sub2) = match existing_type {
            Type::Unit => {
                let ret = Arc::new(FinalType::unit());
                var_borr.var = Variable::Finalized(ret.clone());
                return Ok(ret);
            }
            Type::Sum(ref sub1, ref sub2) => (sub1.clone(), sub2.clone()),
            Type::Product(ref sub1, ref sub2) => (sub1.clone(), sub2.clone()),
        };
        drop(var_borr);

        let sub1 = find_root(sub1);
        let sub2 = find_root(sub2);

        let sub1_borr = sub1.borrow_mut();
        let final1 = match sub1_borr.var {
            Variable::Free => {
                drop(sub1_borr);
                Arc::new(FinalType::unit())
            }
            Variable::Bound(..) => {
                drop(sub1_borr);
                FinalType::from_var(sub1.clone())?
            }
            Variable::EqualTo(..) => unreachable!(),
            Variable::Finalized(ref f1) => {
                let ret = f1.clone();
                drop(sub1_borr);
                ret
            }
        };
        // drop(sub1_borr);
        let sub2_borr = sub2.borrow_mut();
        let final2 = match sub2_borr.var {
            Variable::Free => {
                drop(sub2_borr);
                Arc::new(FinalType::unit())
            }
            Variable::Bound(..) => {
                drop(sub2_borr);
                FinalType::from_var(sub2.clone())?
            }
            Variable::EqualTo(..) => unreachable!(),
            Variable::Finalized(ref f2) => {
                let ret = f2.clone();
                drop(sub2_borr);
                ret
            }
        };
        // drop(sub2_borr);

        let ret = match existing_type {
            Type::Unit => unreachable!(),
            Type::Sum(..) => Arc::new(FinalType::sum(final1, final2)),
            Type::Product(..) => Arc::new(FinalType::prod(final1, final2)),
        };
        var.borrow_mut().var = Variable::Finalized(ret.clone());
        Ok(ret)
    }
}

#[derive(Clone)]
enum Variable {
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

struct UnificationVar {
    var: Variable,
    rank: usize,
}

impl fmt::Debug for UnificationVar {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}]{:?}", self.rank, self.var)
    }
}

type RcVar = Rc<RefCell<UnificationVar>>;

impl UnificationVar {
    fn free() -> UnificationVar {
        UnificationVar {
            var: Variable::Free,
            rank: 0,
        }
    }

    fn concrete(ty: Type) -> UnificationVar {
        UnificationVar {
            var: Variable::Bound(ty, false),
            rank: 0,
        }
    }
}

fn bind(rcvar: &RcVar, ty: Type) -> Result<(), Error> {
    // Cloning a `Variable` is cheap, as the nontrivial variants merely
    // hold `Rc`s
    let self_var = rcvar.borrow().var.clone();
    match self_var {
        Variable::Free => {
            rcvar.borrow_mut().var = Variable::Bound(ty, false);
            Ok(())
        }
        Variable::EqualTo(..) => unreachable!(
            "Tried to bind unification variable which was not \
             the representative of its equivalence class"
        ),
        Variable::Finalized(..) => unreachable!(),
        Variable::Bound(self_ty, _) => match (self_ty, ty) {
            (Type::Unit, Type::Unit) => Ok(()),
            (Type::Sum(al1, al2), Type::Sum(be1, be2))
            | (Type::Product(al1, al2), Type::Product(be1, be2)) => {
                unify(al1, be1)?;
                unify(al2, be2)
            }
            // FIXME output a sane error
            _ => {
                //            (a, b) => {
                /*
                let self_s = match a {
                    Type::Unit => "unit",
                    Type::Sum(..) => "sum",
                    Type::Product(..) => "prod",
                };
                let b_s = match b {
                    Type::Unit => "unit",
                    Type::Sum(..) => "sum",
                    Type::Product(..) => "prod",
                };
                */
                Err(Error::TypeCheck)
            }
        },
    }
}

fn find_root(mut node: RcVar) -> RcVar {
    loop {
        // Double-assignment needed for pre-NLL borrowck reasons
        let parent = match node.borrow().var {
            Variable::EqualTo(ref parent) => Some(parent.clone()),
            _ => None,
        };
        let parent = match parent {
            Some(x) => x,
            _ => break node,
        };

        // Extra scope for pre-NLL borrowck reasons
        {
            let parent_borr = parent.borrow();
            if let Variable::EqualTo(ref grandparent) = parent_borr.var {
                node.borrow_mut().var = Variable::EqualTo(grandparent.clone());
            }
        }
        node = parent;
    }
}

fn unify(mut alpha: RcVar, mut beta: RcVar) -> Result<(), Error> {
    alpha = find_root(alpha);
    beta = find_root(beta);

    // Already unified, done
    if Rc::ptr_eq(&alpha, &beta) {
        return Ok(());
    }

    // Adjust ranks for union-find path halving
    let rank_ord = { alpha.borrow().rank.cmp(&beta.borrow().rank) };
    match rank_ord {
        cmp::Ordering::Less => mem::swap(&mut alpha, &mut beta),
        cmp::Ordering::Equal => alpha.borrow_mut().rank += 1,
        _ => {}
    }

    // Do the unification
    let be_var = {
        let mut be_borr = beta.borrow_mut();
        mem::replace(&mut be_borr.var, Variable::EqualTo(alpha.clone()))
    };
    match be_var {
        Variable::Free => {} // nothing to do
        Variable::Bound(be_type, _) => bind(&alpha, be_type)?,
        Variable::EqualTo(..) => unreachable!(),
        Variable::Finalized(..) => unreachable!(),
    }

    Ok(())
}

#[derive(Clone, Debug)]
struct UnificationArrow {
    source: Rc<RefCell<UnificationVar>>,
    target: Rc<RefCell<UnificationVar>>,
}

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub struct TypedNode<Witness, Ext> {
    pub node: Term<Witness, Ext>,
    pub source_ty: Arc<FinalType>,
    pub target_ty: Arc<FinalType>,
}

/// Convenience method for converting a type for an extension
/// field from a name to an actual `Type`
fn type_from_name<I: Iterator<Item = u8>>(n: &mut I, pow2s: &[RcVar]) -> Type {
    match n.next() {
        Some(b'1') => Type::Unit,
        Some(b'2') => {
            let unit = Type::Unit.into_rcvar();
            Type::Sum(unit.clone(), unit)
        }
        Some(b'i') => Type::Product(pow2s[4].clone(), pow2s[4].clone()),
        Some(b'l') => Type::Product(pow2s[5].clone(), pow2s[5].clone()),
        Some(b'h') => Type::Product(pow2s[7].clone(), pow2s[7].clone()),
        Some(b'+') => Type::Sum(
            type_from_name(&mut *n, pow2s).into_rcvar(),
            type_from_name(&mut *n, pow2s).into_rcvar(),
        ),
        Some(b'*') => Type::Product(
            type_from_name(&mut *n, pow2s).into_rcvar(),
            type_from_name(&mut *n, pow2s).into_rcvar(),
        ),
        Some(x) => panic!("Do not understand byte {} in type name", x as char),
        None => panic!("unexpected end of string parsing type"),
    }
}

/// Attach types to all nodes in a program
pub fn type_check<Witness, Ext: extension::Jet>(
    program: UnTypedProg<Witness, Ext>,
) -> Result<Vec<TypedNode<Witness, Ext>>, Error> {
    let vec_nodes = program.0;
    if vec_nodes.is_empty() {
        return Ok(vec![]);
    }

    let two_0 = Type::Unit.into_rcvar();
    let two_1 = Type::Sum(two_0.clone(), two_0).into_rcvar();
    let two_2 = Type::Product(two_1.clone(), two_1.clone()).into_rcvar();
    let two_4 = Type::Product(two_2.clone(), two_2.clone()).into_rcvar();
    let two_8 = Type::Product(two_4.clone(), two_4.clone()).into_rcvar();
    let two_16 = Type::Product(two_8.clone(), two_8.clone()).into_rcvar();
    let two_32 = Type::Product(two_16.clone(), two_16.clone()).into_rcvar();
    let two_64 = Type::Product(two_32.clone(), two_32.clone()).into_rcvar();
    let two_128 = Type::Product(two_64.clone(), two_64.clone()).into_rcvar();
    let two_256 = Type::Product(two_128.clone(), two_128.clone()).into_rcvar();
    // pow2s[i] = 2^(2^i)
    let pow2s = [
        two_1, two_2, two_4, two_8, two_16, two_32, two_64, two_128, two_256,
    ];

    let mut rcs = Vec::<Rc<UnificationArrow>>::with_capacity(vec_nodes.len());
    let mut finals = Vec::<TypedNode<Witness, Ext>>::with_capacity(vec_nodes.len());

    // Compute most general unifier for all types in the DAG
    for (idx, program_node) in vec_nodes.iter().enumerate() {
        let node = UnificationArrow {
            source: Rc::new(RefCell::new(UnificationVar::free())),
            target: Rc::new(RefCell::new(UnificationVar::free())),
        };

        match program_node {
            Term::Iden => unify(node.source.clone(), node.target.clone())?,
            Term::Unit => bind(&node.target, Type::Unit)?,
            Term::InjL(i) => {
                let i = idx - i;
                unify(node.source.clone(), rcs[i].source.clone())?;
                let target_type = Type::Sum(
                    rcs[i].target.clone(),
                    Rc::new(RefCell::new(UnificationVar::free())),
                );
                bind(&node.target, target_type)?;
            }
            Term::InjR(i) => {
                let i = idx - i;
                unify(node.source.clone(), rcs[i].source.clone())?;
                let target_type = Type::Sum(
                    Rc::new(RefCell::new(UnificationVar::free())),
                    rcs[i].target.clone(),
                );
                bind(&node.target, target_type)?;
            }
            Term::Take(i) => {
                let i = idx - i;
                unify(node.target.clone(), rcs[i].target.clone())?;
                let target_type = Type::Product(
                    rcs[i].source.clone(),
                    Rc::new(RefCell::new(UnificationVar::free())),
                );
                bind(&node.source, target_type)?;
            }
            Term::Drop(i) => {
                let i = idx - i;
                unify(node.target.clone(), rcs[i].target.clone())?;
                let target_type = Type::Product(
                    Rc::new(RefCell::new(UnificationVar::free())),
                    rcs[i].source.clone(),
                );
                bind(&node.source, target_type)?;
            }
            Term::Comp(i, j) => {
                let (i, j) = (idx - i, idx - j);
                unify(node.source.clone(), rcs[i].source.clone())?;
                unify(rcs[i].target.clone(), rcs[j].source.clone())?;
                unify(node.target.clone(), rcs[j].target.clone())?;
            }
            Term::Case(i, j) | Term::AssertL(i, j) | Term::AssertR(i, j) => {
                let (i, j) = (idx - i, idx - j);
                let var1 = Rc::new(RefCell::new(UnificationVar::free()));
                let var2 = Rc::new(RefCell::new(UnificationVar::free()));
                let var3 = Rc::new(RefCell::new(UnificationVar::free()));

                let sum12_ty = Type::Sum(var1.clone(), var2.clone());
                let sum12_var = Rc::new(RefCell::new(UnificationVar::free()));
                bind(&sum12_var, sum12_ty)?;

                let source_ty = Type::Product(sum12_var, var3.clone());
                bind(&node.source, source_ty)?;
                if let Term::Hidden(..) = vec_nodes[i] {
                } else {
                    bind(
                        &find_root(rcs[i].source.clone()),
                        Type::Product(var1.clone(), var3.clone()),
                    )?;
                    unify(node.target.clone(), rcs[i].target.clone())?;
                }
                if let Term::Hidden(..) = vec_nodes[j] {
                } else {
                    bind(
                        &find_root(rcs[j].source.clone()),
                        Type::Product(var2.clone(), var3.clone()),
                    )?;
                    unify(node.target.clone(), rcs[j].target.clone())?;
                }
            }
            Term::Pair(i, j) => {
                let (i, j) = (idx - i, idx - j);
                unify(node.source.clone(), rcs[i].source.clone())?;
                unify(node.source.clone(), rcs[j].source.clone())?;
                bind(
                    &node.target,
                    Type::Product(rcs[i].target.clone(), rcs[j].target.clone()),
                )?;
            }
            Term::Disconnect(i, j) => {
                let (i, j) = (idx - i, idx - j);
                // See chapter 6 (Delegation) of TR
                // Be careful, this order changed! https://github.com/ElementsProject/simplicity/pull/46
                let var_a = Rc::new(RefCell::new(UnificationVar::free()));
                let var_b = Rc::new(RefCell::new(UnificationVar::free()));
                let var_c = Rc::new(RefCell::new(UnificationVar::free()));
                let var_d = Rc::new(RefCell::new(UnificationVar::free()));

                let s_source = Type::Product(pow2s[8].clone(), var_a.clone()).into_rcvar();
                let s_target = Type::Product(var_b.clone(), var_c.clone()).into_rcvar();
                unify(rcs[i].source.clone(), s_source)?;
                unify(rcs[i].target.clone(), s_target)?;

                let node_target = Type::Product(var_b, var_d.clone()).into_rcvar();
                unify(node.source.clone(), var_a)?;
                unify(node.target.clone(), node_target)?;

                unify(rcs[j].source.clone(), var_c)?;
                unify(rcs[j].target.clone(), var_d)?;
            }
            Term::Witness(..) => {
                // No type constraints
            }
            Term::Hidden(..) => {
                // No type constraints
            }
            Term::Ext(ref bn) => {
                bind(
                    &node.source,
                    type_from_name(&mut bn.source_type(), &pow2s[..]),
                )?;
                bind(
                    &node.target,
                    type_from_name(&mut bn.target_type(), &pow2s[..]),
                )?;
            }
            Term::Jet(ref jt) => {
                bind(
                    &node.source,
                    type_from_name(&mut jt.source_type(), &pow2s[..]),
                )?;

                bind(
                    &node.target,
                    type_from_name(&mut jt.target_type(), &pow2s[..]),
                )?;
            }
            Term::Fail(..) => unimplemented!("Cannot typecheck a program with `Fail` in it"),
        };
        rcs.push(Rc::new(node));
        // dbg!(&rcs);
    }

    // Finalize, setting all unconstrained types to `Unit` and doing the
    // occurs check. (All the magic happens inside `FinalType::from_var`.)
    for (idx, node) in vec_nodes.into_iter().enumerate() {
        finals.push(TypedNode {
            node: node,
            source_ty: FinalType::from_var(rcs[idx].source.clone())?,
            target_ty: FinalType::from_var(rcs[idx].target.clone())?,
        });
    }

    Ok(finals)
}
