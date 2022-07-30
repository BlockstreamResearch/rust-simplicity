use crate::core::types::{RcVar, Type, Variable, VariableInner, VariableType};
use crate::core::{Term, TypedNode, TypedProgram, UntypedProgram};
use crate::jet::Application;
use crate::merkle::cmr;
use crate::Error;
use std::rc::Rc;
use std::sync::Arc;
use std::{cmp, mem};

fn find_root(mut node: RcVar) -> RcVar {
    loop {
        // Double-assignment needed for pre-NLL borrowck reasons
        let parent = match node.borrow().inner {
            VariableInner::EqualTo(ref parent) => Some(parent.clone()),
            _ => None,
        };
        let parent = match parent {
            Some(x) => x,
            _ => break node,
        };

        // Extra scope for pre-NLL borrowck reasons
        {
            let parent_borr = parent.borrow();
            if let VariableInner::EqualTo(ref grandparent) = parent_borr.inner {
                node.borrow_mut().inner = VariableInner::EqualTo(grandparent.clone());
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
        mem::replace(&mut be_borr.inner, VariableInner::EqualTo(alpha.clone()))
    };
    match be_var {
        VariableInner::Free => {} // nothing to do
        VariableInner::Bound(be_type, _) => bind(&alpha, be_type)?,
        VariableInner::EqualTo(..) => unreachable!(),
        VariableInner::Finalized(..) => unreachable!(),
    }

    Ok(())
}

fn bind(rcvar: &RcVar, ty: VariableType) -> Result<(), Error> {
    // Cloning a `Variable` is cheap, as the nontrivial variants merely
    // hold `Rc`s
    let self_var = rcvar.borrow().inner.clone();
    match self_var {
        VariableInner::Free => {
            rcvar.borrow_mut().inner = VariableInner::Bound(ty, false);
            Ok(())
        }
        VariableInner::EqualTo(..) => unreachable!(
            "Tried to bind unification variable which was not \
             the representative of its equivalence class"
        ),
        VariableInner::Finalized(..) => unreachable!(),
        VariableInner::Bound(self_ty, _) => match (self_ty, ty) {
            (VariableType::Unit, VariableType::Unit) => Ok(()),
            (VariableType::Sum(al1, al2), VariableType::Sum(be1, be2))
            | (VariableType::Product(al1, al2), VariableType::Product(be1, be2)) => {
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

fn finalize(var: RcVar) -> Result<Arc<Type>, Error> {
    let var = find_root(var);
    let mut var_borr = var.borrow_mut();

    let existing_type = match var_borr.inner {
        VariableInner::Free => VariableType::Unit,
        VariableInner::Bound(ref ty, ref mut occurs_check) => {
            if *occurs_check {
                return Err(Error::OccursCheck);
            }
            *occurs_check = true;
            ty.clone()
        }
        VariableInner::EqualTo(..) => unreachable!(),
        VariableInner::Finalized(ref done) => return Ok(done.clone()),
    };

    let (sub1, sub2) = match existing_type {
        VariableType::Unit => {
            let ret = Type::unit();
            var_borr.inner = VariableInner::Finalized(ret.clone());
            return Ok(ret);
        }
        VariableType::Sum(ref sub1, ref sub2) => (sub1.clone(), sub2.clone()),
        VariableType::Product(ref sub1, ref sub2) => (sub1.clone(), sub2.clone()),
    };
    drop(var_borr);

    let sub1 = find_root(sub1);
    let sub2 = find_root(sub2);

    let sub1_borr = sub1.borrow_mut();
    let final1 = match sub1_borr.inner {
        VariableInner::Free => {
            drop(sub1_borr);
            Type::unit()
        }
        VariableInner::Bound(..) => {
            drop(sub1_borr);
            finalize(sub1.clone())?
        }
        VariableInner::EqualTo(..) => unreachable!(),
        VariableInner::Finalized(ref f1) => {
            let ret = f1.clone();
            drop(sub1_borr);
            ret
        }
    };
    // drop(sub1_borr);
    let sub2_borr = sub2.borrow_mut();
    let final2 = match sub2_borr.inner {
        VariableInner::Free => {
            drop(sub2_borr);
            Type::unit()
        }
        VariableInner::Bound(..) => {
            drop(sub2_borr);
            finalize(sub2.clone())?
        }
        VariableInner::EqualTo(..) => unreachable!(),
        VariableInner::Finalized(ref f2) => {
            let ret = f2.clone();
            drop(sub2_borr);
            ret
        }
    };
    // drop(sub2_borr);

    let ret = match existing_type {
        VariableType::Unit => unreachable!(),
        VariableType::Sum(..) => Type::sum(final1, final2),
        VariableType::Product(..) => Type::product(final1, final2),
    };
    var.borrow_mut().inner = VariableInner::Finalized(ret.clone());
    Ok(ret)
}

#[derive(Clone, Debug)]
struct UnificationArrow {
    source: RcVar,
    target: RcVar,
}

/// Attach types to all nodes in a program
pub(crate) fn type_check<Witness, App: Application>(
    program: UntypedProgram<Witness, App>,
) -> Result<TypedProgram<Witness, App>, Error> {
    let vec_nodes = program.0;
    if vec_nodes.is_empty() {
        return Ok(TypedProgram(vec![]));
    }

    let mut rcs = Vec::<Rc<UnificationArrow>>::with_capacity(vec_nodes.len());
    let mut finals = Vec::<TypedNode<Witness, App>>::with_capacity(vec_nodes.len());
    let pow2s = Variable::powers_of_two();

    // Compute most general unifier for all types in the DAG
    for (idx, program_node) in vec_nodes.iter().enumerate() {
        let node = UnificationArrow {
            source: Variable::free(),
            target: Variable::free(),
        };

        match program_node {
            Term::Iden => unify(node.source.clone(), node.target.clone())?,
            Term::Unit => bind(&node.target, VariableType::Unit)?,
            Term::InjL(i) => {
                let i = idx - i;
                unify(node.source.clone(), rcs[i].source.clone())?;
                let target_type = VariableType::Sum(rcs[i].target.clone(), Variable::free());
                bind(&node.target, target_type)?;
            }
            Term::InjR(i) => {
                let i = idx - i;
                unify(node.source.clone(), rcs[i].source.clone())?;
                let target_type = VariableType::Sum(Variable::free(), rcs[i].target.clone());
                bind(&node.target, target_type)?;
            }
            Term::Take(i) => {
                let i = idx - i;
                unify(node.target.clone(), rcs[i].target.clone())?;
                let target_type = VariableType::Product(rcs[i].source.clone(), Variable::free());
                bind(&node.source, target_type)?;
            }
            Term::Drop(i) => {
                let i = idx - i;
                unify(node.target.clone(), rcs[i].target.clone())?;
                let target_type = VariableType::Product(Variable::free(), rcs[i].source.clone());
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
                let var1 = Variable::free();
                let var2 = Variable::free();
                let var3 = Variable::free();

                let sum12_ty = VariableType::Sum(var1.clone(), var2.clone());
                let sum12_var = Variable::free();
                bind(&sum12_var, sum12_ty)?;

                let source_ty = VariableType::Product(sum12_var, var3.clone());
                bind(&node.source, source_ty)?;
                if let Term::Hidden(..) = vec_nodes[i] {
                } else {
                    bind(
                        &find_root(rcs[i].source.clone()),
                        VariableType::Product(var1.clone(), var3.clone()),
                    )?;
                    unify(node.target.clone(), rcs[i].target.clone())?;
                }
                if let Term::Hidden(..) = vec_nodes[j] {
                } else {
                    bind(
                        &find_root(rcs[j].source.clone()),
                        VariableType::Product(var2.clone(), var3.clone()),
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
                    VariableType::Product(rcs[i].target.clone(), rcs[j].target.clone()),
                )?;
            }
            Term::Disconnect(i, j) => {
                let (i, j) = (idx - i, idx - j);
                // See chapter 6 (Delegation) of TR
                // Be careful, this order changed! https://github.com/ElementsProject/simplicity/pull/46
                let var_a = Variable::free();
                let var_b = Variable::free();
                let var_c = Variable::free();
                let var_d = Variable::free();

                let s_source =
                    Variable::bound(VariableType::Product(pow2s[8].clone(), var_a.clone()));
                let s_target = Variable::bound(VariableType::Product(var_b.clone(), var_c.clone()));
                unify(rcs[i].source.clone(), s_source)?;
                unify(rcs[i].target.clone(), s_target)?;

                let node_target = Variable::bound(VariableType::Product(var_b, var_d.clone()));
                unify(node.source.clone(), var_a)?;
                unify(node.target.clone(), node_target)?;

                unify(rcs[j].source.clone(), var_c)?;
                unify(rcs[j].target.clone(), var_d)?;
            }
            Term::Jet(jet) => {
                bind(&node.source, jet.source_ty.to_type(&pow2s[..]))?;

                bind(&node.target, jet.target_ty.to_type(&pow2s[..]))?;
            }
            Term::Witness(..) | Term::Hidden(..) | Term::Fail(..) => {
                // No type constraints
            }
        };
        rcs.push(Rc::new(node));
        // dbg!(&rcs);
    }

    for (index, term) in vec_nodes.into_iter().enumerate() {
        finals.push(TypedNode {
            cmr: cmr::compute_cmr(&finals, &term, index),
            term,
            source_ty: finalize(rcs[index].source.clone())?,
            target_ty: finalize(rcs[index].target.clone())?,
            index,
        });
    }

    Ok(TypedProgram(finals))
}
