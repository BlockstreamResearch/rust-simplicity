
use std::rc::Rc;
use std::sync::Arc;
use std::cell::RefCell;
use std::{fmt, mem};
use std::collections::HashSet;

use {Node, Value};

#[derive(Clone)]
enum Type {
    Unit,
    Sum(RcVar, RcVar),
    Product(RcVar, RcVar),
}

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum FinalType {
    Unit,
    Sum(Arc<FinalType>, Arc<FinalType>),
    Product(Arc<FinalType>, Arc<FinalType>),
}

impl fmt::Display for FinalType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            FinalType::Unit => f.write_str("ε"),
            FinalType::Sum(ref a, ref b) => write!(f, "({} + {})", a, b),
            FinalType::Product(ref a, ref b) => write!(f, "({} × {})", a, b),
        }
    }
}

impl FinalType {
    fn bit_width(&self) -> usize {
        match *self {
            FinalType::Unit => 0,
            FinalType::Sum(ref s, ref t) => 1 + s.bit_width() + t.bit_width(),
            FinalType::Product(ref s, ref t) => s.bit_width() + t.bit_width(),
        }
    }

    fn from_var(var: RcVar) -> Arc<FinalType> {
        let var = find_root(var);
        let mut var_borr = var.borrow_mut();

        let existing_type = match var_borr.var {
            Variable::Free => Type::Unit,
            Variable::Bound(ref ty, ref mut occurs_check) => {
                assert!(!*occurs_check); // TODO set an error here
                *occurs_check = true;
                ty.clone()
            },
            Variable::EqualTo(..) => unreachable!(),
            Variable::Finalized(ref done) => {
                return done.clone()
            }
        };

        let (sub1, sub2) = match existing_type {
            Type::Unit => {
                let ret = Arc::new(FinalType::Unit);
                var_borr.var = Variable::Finalized(ret.clone());
                return ret;
            },
            Type::Sum(ref sub1, ref sub2) => (sub1.clone(), sub2.clone()),
            Type::Product(ref sub1, ref sub2) => (sub1.clone(), sub2.clone()),
        };

        let sub1 = find_root(sub1.clone());
        let sub2 = find_root(sub2.clone());

        let sub1_borr = sub1.borrow_mut();
        let final1 = match sub1_borr.var {
            Variable::Free => Arc::new(FinalType::Unit),
            Variable::Bound(..) => {
                drop(sub1_borr);
                FinalType::from_var(sub1.clone())
            },
            Variable::EqualTo(..) => unreachable!(),
            Variable::Finalized(ref f1) => {
                let ret = f1.clone();
                drop(sub1_borr);
                ret
            }
        };

        let sub2_borr = sub2.borrow_mut();
        let final2 = match sub2_borr.var {
            Variable::Free => Arc::new(FinalType::Unit),
            Variable::Bound(..) => {
                drop(sub2_borr);
                FinalType::from_var(sub2)
            },
            Variable::EqualTo(..) => unreachable!(),
            Variable::Finalized(ref f2) => {
                let ret = f2.clone();
                drop(sub2_borr);
                ret
            }
        };

        let ret = match existing_type {
            Type::Unit => unreachable!(),
            Type::Sum(..) => Arc::new(FinalType::Sum(final1, final2)),
            Type::Product(..) => Arc::new(FinalType::Product(final1, final2)),
        };
        var_borr.var = Variable::Finalized(ret.clone());
        ret
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

struct UnificationVar {
    var: Variable,
    rank: usize,
}

type RcVar = Rc<RefCell<UnificationVar>>;

impl UnificationVar {
    fn free() -> UnificationVar {
        UnificationVar {
            var: Variable::Free,
            rank: 0,
        }
    }

    fn bind(&mut self, ty: Type) {
        // Cloning a `Variable` is cheap, as the nontrivial variants merely
        // hold `Rc`s
        let self_var = self.var.clone();
        match self_var {
            Variable::Free => self.var = Variable::Bound(ty, false),
            Variable::EqualTo(..) => unreachable!(
                "Tried to bind unification variable which was not \
                 the representative of its equivalence class"
            ),
            Variable::Finalized(..) => unreachable!(),
            Variable::Bound(self_ty, _) => {
                match (self_ty, ty) {
                    (Type::Unit, Type::Unit) => {},
                    (Type::Sum(al1, al2), Type::Sum(be1, be2))
                        | (Type::Product(al1, al2), Type::Product(be1, be2))
                        => {
                            unify(al1, be1);
                            unify(al2, be2);
                        },
                    (a, b) => {
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
                        panic!("unification failure {} vs {}", self_s, b_s)
                    }
                }
            },
        }
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

fn unify(mut alpha: RcVar, mut beta: RcVar) {
    alpha = find_root(alpha);
    beta = find_root(beta);

    // Already unified, done
    if Rc::ptr_eq(&alpha, &beta) {
        return;
    }

    // Adjust ranks for union-find path halving
    if alpha.borrow().rank < beta.borrow().rank {
        mem::swap(&mut alpha, &mut beta);
    } else if alpha.borrow().rank == beta.borrow().rank {
        alpha.borrow_mut().rank += 1;
    }

    // Do the unification
    let mut be_borr = beta.borrow_mut();
    let be_var = mem::replace(&mut be_borr.var, Variable::EqualTo(alpha));

    let mut al_borr = match be_borr.var {
        Variable::EqualTo(ref mut alpha) => alpha.borrow_mut(),
        _ => unreachable!(),
    };

    match be_var {
        Variable::Free => {} // nothing to do
        Variable::Bound(be_type, _) => al_borr.bind(be_type),
        Variable::EqualTo(..) => unreachable!(),
        Variable::Finalized(..) => unreachable!(),
    }
}

#[derive(Clone)]
struct UnificationArrow {
    source: Rc<RefCell<UnificationVar>>,
    target: Rc<RefCell<UnificationVar>>,
}

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub struct TypedNode {
    pub node: Node<Arc<TypedNode>, Option<Value>>,
    pub index: usize,
    pub source_ty: Arc<FinalType>,
    pub target_ty: Arc<FinalType>,
}

impl TypedNode {
    fn graph_print_internal(&self, drawn: &mut HashSet<usize>) {
        if drawn.insert(self.index) {
            println!(
                "{} [label=\"{}\\n{}\\n{}→{}\"];",
                self.index,
                match self.node {
                    Node::Iden => "iden",
                    Node::Unit => "unit",
                    Node::InjL(..) => "injl",
                    Node::InjR(..) => "injr",
                    Node::Take(..) => "take",
                    Node::Drop(..) => "drop",
                    Node::Comp(..) => "comp",
                    Node::Case(..) => "case",
                    Node::Pair(..) => "pair",
                    Node::Disconnect(..) => "disconnect",
                    Node::Witness(..) => "witness",
                    Node::Hidden(..) => "hidden",
                    Node::Fail(..) => "fail",
                },
                self.index,
                self.source_ty,
                self.target_ty,
            );
            match self.node {
                Node::Iden | Node::Unit | Node::Witness(..) | Node::Hidden(..) | Node::Fail(..) => {},
                Node::InjL(ref i)
                    | Node::InjR(ref i)
                    | Node::Take(ref i)
                    | Node::Drop(ref i) => {
                        println!("  {} -> {};", self.index, i.index);
                        i.graph_print_internal(drawn);
                    }
                Node::Comp(ref i, ref j)
                | Node::Case(ref i, ref j)
                | Node::Pair(ref i, ref j)
                | Node::Disconnect(ref i, ref j) => {
                    println!("  {} -> {} [color=red];", self.index, i.index);
                    println!("  {} -> {} [color=blue];", self.index, j.index);
                    i.graph_print_internal(drawn);
                    j.graph_print_internal(drawn);
                }
            }
        }
    }

    pub fn graph_print(&self) {
        println!("digraph {{");
        self.graph_print_internal(&mut HashSet::new());
        println!("}}");
    }
}

impl fmt::Display for TypedNode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}] ", self.index)?;
        match self.node {
            Node::Iden => f.write_str("iden")?,
            Node::Unit => f.write_str("unit")?,
            Node::InjL(ref i) => write!(f, "injl({})", i.index)?,
            Node::InjR(ref i) => write!(f, "injr({})", i.index)?,
            Node::Take(ref i) => write!(f, "take({})", i.index)?,
            Node::Drop(ref i) => write!(f, "drop({})", i.index)?,
            Node::Comp(ref i, ref j)
                => write!(f, "comp({}, {})", i.index, j.index)?,
            Node::Case(ref i, ref j)
                => write!(f, "case({}, {})", i.index, j.index)?,
            Node::Pair(ref i, ref j)
                => write!(f, "pair({}, {})", i.index, j.index)?,
            Node::Disconnect(ref i, ref j)
                => write!(f, "disconnect({}, {})", i.index, j.index)?,
            Node::Witness(..) => f.write_str("witness")?,
            Node::Hidden(..) => f.write_str("hidden")?,
            Node::Fail(..) => f.write_str("fail")?,
        }
        write!(
            f,
            ": {} → {}",
            self.source_ty,
            self.target_ty,
        )
    }
}

pub struct FullPrint<'a>(&'a TypedNode);
impl<'a> fmt::Display for FullPrint<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}]", self.0.index)?;
        match self.0.node {
            Node::Iden => f.write_str("iden"),
            Node::Unit => f.write_str("unit"),
            Node::InjL(ref i) => write!(f, "injl({})", FullPrint(i)),
            Node::InjR(ref i) => write!(f, "injr({})", FullPrint(i)),
            Node::Take(ref i) => write!(f, "take({})", FullPrint(i)),
            Node::Drop(ref i) => write!(f, "drop({})", FullPrint(i)),
            Node::Comp(ref i, ref j)
                => write!(f, "comp({}, {})", FullPrint(i), FullPrint(j)),
            Node::Case(ref i, ref j)
                => write!(f, "case({}, {})", FullPrint(i), FullPrint(j)),
            Node::Pair(ref i, ref j)
                => write!(f, "pair({}, {})", FullPrint(i), FullPrint(j)),
            Node::Disconnect(ref i, ref j)
                => write!(f, "disconnect({}, {})", FullPrint(i), FullPrint(j)),
            Node::Witness(..) => f.write_str("witness"),
            Node::Hidden(..) => f.write_str("hidden"),
            Node::Fail(..) => f.write_str("fail"),
        }?;
        write!(
            f,
            ": {} → {}",
            self.0.source_ty,
            self.0.target_ty,
        )
    }
}

pub fn type_check<BitStream: Iterator<Item = bool>>(
    program: &[Node<usize, ()>],
    mut witness_iter: Option<BitStream>,
) -> Arc<TypedNode> {
    if program.is_empty() {
        panic!("cannot deal with empty program")
    }

    let mut rcs = Vec::<Rc<UnificationArrow>>::with_capacity(program.len());
    let mut finals = Vec::<Arc<TypedNode>>::with_capacity(program.len());

    // Compute most general unifier for all types in the DAG
    for k in 0..program.len() {
        let node = UnificationArrow {
            source: Rc::new(RefCell::new(UnificationVar::free())),
            target: Rc::new(RefCell::new(UnificationVar::free())),
        };

        match program[k]{
            Node::Iden => unify(node.source.clone(), node.target.clone()),
            Node::Unit => node.target.borrow_mut().bind(Type::Unit),
            Node::InjL(i) => {
                unify(node.source.clone(), rcs[k - i].source.clone());
                let target_type = Type::Sum(
                    rcs[k - i].target.clone(),
                    Rc::new(RefCell::new(UnificationVar::free())),
                );
                node.target.borrow_mut().bind(target_type);
            },
            Node::InjR(i) => {
                unify(node.source.clone(), rcs[k - i].source.clone());
                let target_type = Type::Sum(
                    Rc::new(RefCell::new(UnificationVar::free())),
                    rcs[k - i].target.clone(),
                );
                node.target.borrow_mut().bind(target_type);
            },
            Node::Take(i) => {
                unify(node.target.clone(), rcs[k - i].target.clone());
                let target_type = Type::Product(
                    rcs[k - i].source.clone(),
                    Rc::new(RefCell::new(UnificationVar::free())),
                );
                node.source.borrow_mut().bind(target_type);
            },
            Node::Drop(i) => {
                unify(node.target.clone(), rcs[k - i].target.clone());
                let target_type = Type::Product(
                    Rc::new(RefCell::new(UnificationVar::free())),
                    rcs[k - i].source.clone(),
                );
                node.source.borrow_mut().bind(target_type);
            },
            Node::Comp(i, j) => {
                unify(node.source.clone(), rcs[k - i].source.clone());
                unify(rcs[k - i].target.clone(), rcs[k - j].source.clone());
                unify(node.target.clone(), rcs[k - j].target.clone());
            },
            Node::Case(i, j) => {
                let var1 = Rc::new(RefCell::new(UnificationVar::free()));
                let var2 = Rc::new(RefCell::new(UnificationVar::free()));
                let var3 = Rc::new(RefCell::new(UnificationVar::free()));

                let sum_12_ty = Type::Sum(var1.clone(), var2.clone());
                let mut sum12_var = UnificationVar::free();
                sum12_var.bind(sum_12_ty);
                let sum12_var = Rc::new(RefCell::new(sum12_var));

                let source_ty = Type::Product(sum12_var, var3.clone());
                node.source.borrow_mut().bind(source_ty);
                if let Node::Hidden(..) = program[k - i] {
                } else {
                    find_root(rcs[k - i].source.clone())
                        .borrow_mut()
                        .bind(Type::Product(var1.clone(), var3.clone()));
                    unify(node.target.clone(), rcs[k - i].target.clone());
                }
                if let Node::Hidden(..) = program[k - j] {
                } else {
                    find_root(rcs[k - j].source.clone())
                        .borrow_mut()
                        .bind(Type::Product(var2.clone(), var3.clone()));
                    unify(node.target.clone(), rcs[k - j].target.clone());
                }
            },
            Node::Pair(i, j) => {
                unify(node.source.clone(), rcs[k - i].source.clone());
                unify(node.source.clone(), rcs[k - j].source.clone());
                node.target.borrow_mut().bind(Type::Product(
                    rcs[k - i].target.clone(),
                    rcs[k - j].target.clone(),
                ));
            },
            Node::Witness(..) => {
                // No type constraints
            },
            Node::Hidden(..) => {
                // No type constraints
            },
            ref x => unimplemented!("haven't implemented {:?}", x),
        };

        rcs.push(Rc::new(node));
    }

    // Finalize, setting all unconstrained types to `Unit` and doing the
    // occurs check
    for k in 0..program.len() {
        let source_ty = FinalType::from_var(rcs[k].source.clone());
        let target_ty = FinalType::from_var(rcs[k].target.clone());
        let final_node = TypedNode {
            node: match program[k] {
                Node::Iden => Node::Iden,
                Node::Unit => Node::Unit,
                Node::InjL(i) => Node::InjL(finals[k - i].clone()),
                Node::InjR(i) => Node::InjR(finals[k - i].clone()),
                Node::Take(i) => Node::Take(finals[k - i].clone()),
                Node::Drop(i) => Node::Drop(finals[k - i].clone()),
                Node::Comp(i, j) => Node::Comp(
                    finals[k - i].clone(),
                    finals[k - j].clone(),
                ),
                Node::Case(i, j) => Node::Case(
                    finals[k - i].clone(),
                    finals[k - j].clone(),
                ),
                Node::Pair(i, j) => Node::Pair(
                    finals[k - i].clone(),
                    finals[k - j].clone(),
                ),
                Node::Disconnect(i, j) => Node::Disconnect(
                    finals[k - i].clone(),
                    finals[k - j].clone(),
                ),
                Node::Witness(..) => {
                    if let Some(ref mut iter) = witness_iter {
                        Node::Witness(Some(
                            Value::from_witness(iter, &target_ty)
                                .expect("decoding witness")
                        ))
                    } else {
                        Node::Witness(None)
                    }
                },
                Node::Hidden(hash) => Node::Hidden(hash),
                ref x => unreachable!("{:?}", x),
            },
            index: k,
            source_ty: source_ty,
            target_ty: target_ty,
        };

        finals.push(Arc::new(final_node));
    }

    finals.pop().expect("nonempty program")
}
