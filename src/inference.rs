use crate::core::types::{RcVar, Type, Variable, VariableFactory, VariableInner, VariableType};
use crate::core::{Term, TypedNode, TypedProgram, UntypedProgram};
use crate::jet::Application;
use crate::merkle::cmr;
use crate::Error;
use std::rc::Rc;
use std::sync::Arc;
use std::{cmp, mem};

/// Find the representative of the set of variable `x`
/// using _path halving_ of the union-find algorithm.
fn find_root(mut x: RcVar) -> RcVar {
    loop {
        // Clone inner to un-borrow `x`
        let x_var = x.borrow().inner.clone();
        let parent = if let VariableInner::EqualTo(parent) = x_var {
            parent
        } else {
            // If there is no parent, then return the current node.
            return x;
        };

        // Clone inner to un-borrow `parent`
        let parent_var = parent.borrow().inner.clone();
        if let VariableInner::EqualTo(grandparent) = parent_var {
            // Update the parent pointer to the grandparent, and go to the grandparent.
            x.borrow_mut().inner = VariableInner::EqualTo(grandparent.clone());
            x = grandparent;
        } else {
            // If there is no grandparent, then return the parent.
            return parent;
        }
    }
}

/// Unify the sets of variables `x` and `y`
/// using _union by rank_ of the union-find algorithm.
fn unify(mut x: RcVar, mut y: RcVar) -> Result<(), Error> {
    x = find_root(x);
    y = find_root(y);

    // x and y are already in the same set
    if Rc::ptr_eq(&x, &y) {
        return Ok(());
    }

    // Adjust ranks for union-find path halving
    let rank_ord = x.borrow().rank.cmp(&y.borrow().rank);
    match rank_ord {
        // If x.rank < y.rank, then swap x and y to ensure x.rank ≥ y.rank
        // Always unify tree with smaller rank into tree with larger rank
        cmp::Ordering::Less => mem::swap(&mut x, &mut y),
        // If x.rank = y.rank, then increment the rank of x
        // The rank of x increases by making y its child
        cmp::Ordering::Equal => x.borrow_mut().rank += 1,
        _ => {}
    }

    // Make x the parent of y
    let old_y_var = mem::replace(&mut y.borrow_mut().inner, VariableInner::EqualTo(x.clone()));
    match old_y_var {
        VariableInner::Free(_) => Ok(()),
        // If y was already bound to a type, then x must be bound, too
        VariableInner::Bound(be_type, _) => bind(&x, be_type),
        VariableInner::EqualTo(..) => unreachable!("A root node cannot have a parent"),
        VariableInner::Finalized(..) => unreachable!("No finalized types at this stage"),
    }
}

/// Bind variable `x` to type `ty`.
///
/// Fails if `x` is already bound to a type that is incompatible with `ty`.
fn bind(x: &RcVar, ty: VariableType) -> Result<(), Error> {
    // Clone inner to un-borrow `x`
    let x_var = x.borrow().inner.clone();
    match x_var {
        VariableInner::Free(_) => {
            x.borrow_mut().inner = VariableInner::Bound(ty, false);
            Ok(())
        }
        VariableInner::Bound(self_ty, _) => match (self_ty, ty) {
            (VariableType::Unit, VariableType::Unit) => Ok(()),
            (VariableType::Sum(x1, x2), VariableType::Sum(y1, y2))
            | (VariableType::Product(x1, x2), VariableType::Product(y1, y2)) => {
                unify(x1, y1)?;
                unify(x2, y2)
            }
            _ => Err(Error::TypeCheck),
        },
        VariableInner::EqualTo(..) => unreachable!("Can only bind root nodes"),
        VariableInner::Finalized(..) => unreachable!("No finalized types at this stage"),
    }
}

/// Converts variable `x` into a finalized type.
///
/// Free variables are finalized as _unit_.
/// Bound variables are recursively finalized as the units, sums and products of their types.
///
/// Fails if a variable occurs recursively in the type it is bound to _(occurs check)_.
fn finalize(x: RcVar) -> Result<Arc<Type>, Error> {
    let x = find_root(x);

    let variable_type = match x.borrow_mut().inner {
        VariableInner::Free(_) => VariableType::Unit,
        VariableInner::Bound(ref ty, ref mut occurs_check) => {
            if *occurs_check {
                return Err(Error::OccursCheck);
            }
            *occurs_check = true;
            ty.clone()
        }
        VariableInner::EqualTo(..) => unreachable!("A root cannot have a parent"),
        VariableInner::Finalized(ref final_type) => return Ok(final_type.clone()),
    };

    let final_type = match variable_type {
        VariableType::Unit => Type::unit(),
        VariableType::Sum(y, z) => Type::sum(finalize(y)?, finalize(z)?),
        VariableType::Product(y, z) => Type::product(finalize(y)?, finalize(z)?),
    };

    x.borrow_mut().inner = VariableInner::Finalized(final_type.clone());
    Ok(final_type)
}

/// Reference to the source and target type of a node during type inference
#[derive(Clone, Debug)]
struct UnificationArrow {
    /// Source type of the node
    source: RcVar,
    /// Target type of the node
    target: RcVar,
}

/// Attach types to all nodes in a program
pub(crate) fn type_check<Witness, App: Application>(
    program: UntypedProgram<Witness, App>,
) -> Result<TypedProgram<Witness, App>, Error> {
    let nodes = program.0;
    if nodes.is_empty() {
        return Ok(TypedProgram(vec![]));
    }

    let mut arrows: Vec<UnificationArrow> = Vec::with_capacity(nodes.len());
    let pow2s = Variable::powers_of_two();
    let mut naming = VariableFactory::new();

    for (idx, node) in nodes.iter().enumerate() {
        let arrow = UnificationArrow {
            source: naming.free_variable(),
            target: naming.free_variable(),
        };

        match node {
            Term::Iden => unify(arrow.source.clone(), arrow.target.clone())?,
            Term::Unit => bind(&arrow.target, VariableType::Unit)?,
            Term::InjL(i) => {
                let i = idx - i;
                unify(arrow.source.clone(), arrows[i].source.clone())?;
                let sum_b_c = VariableType::Sum(arrows[i].target.clone(), naming.free_variable());
                bind(&arrow.target, sum_b_c)?;
            }
            Term::InjR(i) => {
                let i = idx - i;
                unify(arrow.source.clone(), arrows[i].source.clone())?;
                let sum_b_c = VariableType::Sum(naming.free_variable(), arrows[i].target.clone());
                bind(&arrow.target, sum_b_c)?;
            }
            Term::Take(i) => {
                let i = idx - i;
                unify(arrow.target.clone(), arrows[i].target.clone())?;
                let prod_a_b =
                    VariableType::Product(arrows[i].source.clone(), naming.free_variable());
                bind(&arrow.source, prod_a_b)?;
            }
            Term::Drop(i) => {
                let i = idx - i;
                unify(arrow.target.clone(), arrows[i].target.clone())?;
                let prod_a_b =
                    VariableType::Product(naming.free_variable(), arrows[i].source.clone());
                bind(&arrow.source, prod_a_b)?;
            }
            Term::Comp(i, j) => {
                let (i, j) = (idx - i, idx - j);
                unify(arrow.source.clone(), arrows[i].source.clone())?;
                unify(arrows[i].target.clone(), arrows[j].source.clone())?;
                unify(arrow.target.clone(), arrows[j].target.clone())?;
            }
            Term::Case(i, j) | Term::AssertL(i, j) | Term::AssertR(i, j) => {
                let (i, j) = (idx - i, idx - j);
                let a = naming.free_variable();
                let b = naming.free_variable();
                let c = naming.free_variable();

                let sum_a_b = VariableType::Sum(a.clone(), b.clone());
                let prod_sum_a_b_c = VariableType::Product(Variable::bound(sum_a_b), c.clone());
                bind(&arrow.source, prod_sum_a_b_c)?;

                if let Term::AssertR(..) = node {
                } else {
                    bind(
                        &find_root(arrows[i].source.clone()),
                        VariableType::Product(a.clone(), c.clone()),
                    )?;
                    unify(arrow.target.clone(), arrows[i].target.clone())?;
                }
                if let Term::AssertL(..) = node {
                } else {
                    bind(
                        &find_root(arrows[j].source.clone()),
                        VariableType::Product(b.clone(), c.clone()),
                    )?;
                    unify(arrow.target.clone(), arrows[j].target.clone())?;
                }
            }
            Term::Pair(i, j) => {
                let (i, j) = (idx - i, idx - j);
                unify(arrow.source.clone(), arrows[i].source.clone())?;
                unify(arrow.source.clone(), arrows[j].source.clone())?;
                bind(
                    &arrow.target,
                    VariableType::Product(arrows[i].target.clone(), arrows[j].target.clone()),
                )?;
            }
            Term::Disconnect(i, j) => {
                let (i, j) = (idx - i, idx - j);
                let a = naming.free_variable();
                let b = naming.free_variable();
                let c = naming.free_variable();
                let d = naming.free_variable();

                let prod_256_a =
                    Variable::bound(VariableType::Product(pow2s[8].clone(), a.clone()));
                let prod_b_c = Variable::bound(VariableType::Product(b.clone(), c.clone()));
                unify(arrows[i].source.clone(), prod_256_a)?;
                unify(arrows[i].target.clone(), prod_b_c)?;
                unify(arrows[j].source.clone(), c)?;
                unify(arrows[j].target.clone(), d.clone())?;

                let prod_b_d = Variable::bound(VariableType::Product(b, d));
                unify(arrow.source.clone(), a)?;
                unify(arrow.target.clone(), prod_b_d)?;
            }
            Term::Jet(jet) => {
                bind(&arrow.source, jet.source_ty.to_type(&pow2s))?;
                bind(&arrow.target, jet.target_ty.to_type(&pow2s))?;
            }
            Term::Witness(..) | Term::Hidden(..) | Term::Fail(..) => {
                // No type constraints
            }
        }

        arrows.push(arrow);
    }

    let mut typed_nodes = Vec::with_capacity(nodes.len());

    for (index, term) in nodes.into_iter().enumerate() {
        typed_nodes.push(TypedNode {
            cmr: cmr::compute_cmr(&typed_nodes, &term, index),
            term,
            source_ty: finalize(arrows[index].source.clone())?,
            target_ty: finalize(arrows[index].target.clone())?,
            index,
        });
    }

    Ok(TypedProgram(typed_nodes))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::types::Variable;

    #[test]
    fn type_error() {
        let mut naming = VariableFactory::new();
        let pow2s = Variable::powers_of_two();
        let x = naming.free_variable();
        let y = naming.free_variable();

        let x1 = naming.free_variable();
        let x2 = naming.free_variable();
        bind(&x, VariableType::Sum(x1, x2)).unwrap();
        bind(
            &y,
            VariableType::Product(pow2s[8].clone(), naming.free_variable()),
        )
        .unwrap();

        unify(x, y).unwrap_err();
    }
}
