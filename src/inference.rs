use crate::core::commit::{CommitNodeInner, RefWrapper};
use crate::core::iter::{DagIterable, PostOrderIter};
use crate::core::node::NodeType;
use crate::core::types::{RcVar, Type, Variable, VariableFactory, VariableInner, VariableType};
use crate::jet::Application;
use crate::Error;
use std::collections::HashMap;
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

// TODO: Remove once local type checking is implemented
/// Storage of the unification arrows for each node of a Simplicity DAG.
#[derive(Debug)]
pub(crate) struct Arrows<'a, Witness, App: Application> {
    node_to_arrow: HashMap<RefWrapper<'a, Witness, App>, UnificationArrow>,
}

impl<'a, Witness, App: Application> Arrows<'a, Witness, App> {
    /// Return the finalized type of the given node.
    /// To work, this method must be called on nodes in post order!
    pub fn finalize(&self, node: &RefWrapper<'a, Witness, App>) -> Result<NodeType, Error> {
        let arrow = self
            .node_to_arrow
            .get(node)
            .expect("Type finalizer was not initialized to include the given node");
        let source_ty = match finalize(arrow.source.clone()) {
            Ok(ty) => ty,
            Err(error) => return Err(error),
        };
        let target_ty = match finalize(arrow.target.clone()) {
            Ok(ty) => ty,
            Err(error) => return Err(error),
        };

        Ok(NodeType {
            source: source_ty,
            target: target_ty,
        })
    }
}

/// Return the initialized unification arrows of a Simplicity DAG.
pub(crate) fn get_arrows<Witness, App: Application>(
    it: PostOrderIter<RefWrapper<Witness, App>>,
) -> Result<Arrows<Witness, App>, Error> {
    let mut node_to_arrow: HashMap<RefWrapper<Witness, App>, UnificationArrow> = HashMap::new();
    let pow2s = Variable::powers_of_two();
    let mut naming = VariableFactory::new();

    for untyped_node in it {
        let arrow = UnificationArrow {
            source: naming.free_variable(),
            target: naming.free_variable(),
        };

        if let Some(left) = untyped_node.get_left() {
            let left_arrow = node_to_arrow.get(&left).unwrap();

            if let Some(right) = untyped_node.get_right() {
                let right_arrow = node_to_arrow.get(&right).unwrap();

                match &untyped_node.0.inner {
                    CommitNodeInner::Comp(_, _) => {
                        unify(arrow.source.clone(), left_arrow.source.clone())?;
                        unify(left_arrow.target.clone(), right_arrow.source.clone())?;
                        unify(arrow.target.clone(), right_arrow.target.clone())?;
                    }
                    CommitNodeInner::Case(_, _)
                    | CommitNodeInner::AssertL(_, _)
                    | CommitNodeInner::AssertR(_, _) => {
                        let a = naming.free_variable();
                        let b = naming.free_variable();
                        let c = naming.free_variable();

                        let sum_a_b = VariableType::Sum(a.clone(), b.clone());
                        let prod_sum_a_b_c =
                            VariableType::Product(Variable::bound(sum_a_b), c.clone());
                        bind(&arrow.source, prod_sum_a_b_c)?;

                        if let CommitNodeInner::AssertR(_, _) = &untyped_node.0.inner {
                        } else {
                            bind(
                                &find_root(left_arrow.source.clone()),
                                VariableType::Product(a.clone(), c.clone()),
                            )?;
                            unify(arrow.target.clone(), left_arrow.target.clone())?;
                        }
                        if let CommitNodeInner::AssertL(_, _) = &untyped_node.0.inner {
                        } else {
                            bind(
                                &find_root(right_arrow.source.clone()),
                                VariableType::Product(b.clone(), c.clone()),
                            )?;
                            unify(arrow.target.clone(), right_arrow.target.clone())?;
                        }
                    }
                    CommitNodeInner::Pair(_, _) => {
                        unify(arrow.source.clone(), left_arrow.source.clone())?;
                        unify(arrow.source.clone(), right_arrow.source.clone())?;
                        bind(
                            &arrow.target,
                            VariableType::Product(
                                left_arrow.target.clone(),
                                right_arrow.target.clone(),
                            ),
                        )?;
                    }
                    CommitNodeInner::Disconnect(_, _) => {
                        let a = naming.free_variable();
                        let b = naming.free_variable();
                        let c = naming.free_variable();
                        let d = naming.free_variable();

                        let prod_256_a =
                            Variable::bound(VariableType::Product(pow2s[8].clone(), a.clone()));
                        let prod_b_c = Variable::bound(VariableType::Product(b.clone(), c.clone()));
                        unify(left_arrow.source.clone(), prod_256_a)?;
                        unify(left_arrow.target.clone(), prod_b_c)?;
                        unify(right_arrow.source.clone(), c)?;
                        unify(right_arrow.target.clone(), d.clone())?;

                        let prod_b_d = Variable::bound(VariableType::Product(b, d));
                        unify(arrow.source.clone(), a)?;
                        unify(arrow.target.clone(), prod_b_d)?;
                    }
                    _ => unreachable!(),
                }
            } else {
                match &untyped_node.0.inner {
                    CommitNodeInner::InjL(_) => {
                        unify(arrow.source.clone(), left_arrow.source.clone())?;
                        let sum_b_c =
                            VariableType::Sum(left_arrow.target.clone(), naming.free_variable());
                        bind(&arrow.target, sum_b_c)?;
                    }
                    CommitNodeInner::InjR(_) => {
                        unify(arrow.source.clone(), left_arrow.source.clone())?;
                        let sum_b_c =
                            VariableType::Sum(naming.free_variable(), left_arrow.target.clone());
                        bind(&arrow.target, sum_b_c)?;
                    }
                    CommitNodeInner::Take(_) => {
                        unify(arrow.target.clone(), left_arrow.target.clone())?;
                        let prod_a_b = VariableType::Product(
                            left_arrow.source.clone(),
                            naming.free_variable(),
                        );
                        bind(&arrow.source, prod_a_b)?;
                    }
                    CommitNodeInner::Drop(_) => {
                        unify(arrow.target.clone(), left_arrow.target.clone())?;
                        let prod_a_b = VariableType::Product(
                            naming.free_variable(),
                            left_arrow.source.clone(),
                        );
                        bind(&arrow.source, prod_a_b)?;
                    }
                    _ => unreachable!(),
                }
            }
        } else {
            match &untyped_node.0.inner {
                CommitNodeInner::Iden => unify(arrow.source.clone(), arrow.target.clone())?,
                CommitNodeInner::Unit => bind(&arrow.target, VariableType::Unit)?,
                CommitNodeInner::Witness(_)
                | CommitNodeInner::Fail(_, _)
                | CommitNodeInner::Hidden(_) => {
                    // no type constraints
                }
                CommitNodeInner::Jet(jet) => {
                    bind(&arrow.source, jet.source_ty.to_type(&pow2s))?;
                    bind(&arrow.target, jet.target_ty.to_type(&pow2s))?;
                }
                _ => unreachable!(),
            }
        }

        debug_assert!(!node_to_arrow.contains_key(&untyped_node));
        node_to_arrow.insert(untyped_node, arrow);
    }

    Ok(Arrows { node_to_arrow })
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
