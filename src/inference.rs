use crate::core::commit::CommitNodeInner;
use crate::core::redeem::NodeType;
use crate::core::types::{RcVar, Type, Variable, VariableFactory, VariableInner, VariableType};
use crate::jet::Jet;
use crate::{CommitNode, Error};
use std::convert::TryFrom;
use std::rc::Rc;
use std::sync::Arc;
use std::{cmp, fmt, mem};

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
fn unify(mut x: RcVar, mut y: RcVar, hint: &'static str) -> Result<(), Error> {
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
        VariableInner::Bound(y_ty, _) => bind(&x, y_ty, hint),
        VariableInner::EqualTo(..) => unreachable!("A root node cannot have a parent"),
        VariableInner::Finalized(..) => unreachable!("No finalized types at this stage"),
    }
}

/// Bind variable `x` to type `ty`.
///
/// Fails if `x` is already bound to a type that is incompatible with `ty`.
fn bind(x: &RcVar, ty: VariableType, hint: &'static str) -> Result<(), Error> {
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
                unify(x1, y1, hint)?;
                unify(x2, y2, hint)
            }
            _ => Err(Error::Unification(hint)),
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

/// Source and target type of a node during type inference
#[derive(Clone, Debug)]
pub struct UnificationArrow {
    /// Source type of the node
    source: RcVar,
    /// Target type of the node
    target: RcVar,
}

impl fmt::Display for UnificationArrow {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} → {}", self.source.borrow(), self.target.borrow())
    }
}

impl<J: Jet> TryFrom<&CommitNode<J>> for NodeType {
    type Error = Error;

    /// Return the finalized type of the given `node`.
    /// To work, this method must be called on nodes in post order!
    fn try_from(node: &CommitNode<J>) -> Result<Self, Self::Error> {
        let source_ty = match finalize(node.arrow.source.clone()) {
            Ok(ty) => ty,
            Err(error) => return Err(error),
        };
        let target_ty = match finalize(node.arrow.target.clone()) {
            Ok(ty) => ty,
            Err(error) => return Err(error),
        };

        Ok(NodeType {
            source: source_ty,
            target: target_ty,
        })
    }
}

/// Return a unification arrow that is initialized for the given `node`.
pub(crate) fn get_arrow<J: Jet>(
    node: &CommitNodeInner<J>,
    naming: &mut VariableFactory,
) -> Result<UnificationArrow, Error> {
    let arrow = UnificationArrow {
        source: naming.free_variable(),
        target: naming.free_variable(),
    };

    if let Some(left) = node.get_left() {
        let left_arrow = &left.arrow;

        if let Some(right) = node.get_right() {
            let right_arrow = &right.arrow;

            match node {
                CommitNodeInner::Comp(_, _) => {
                    unify(
                        arrow.source.clone(),
                        left_arrow.source.clone(),
                        "Cannot fail",
                    )?;
                    unify(
                        left_arrow.target.clone(),
                        right_arrow.source.clone(),
                        "Composition: Left target = right source",
                    )?;
                    unify(
                        arrow.target.clone(),
                        right_arrow.target.clone(),
                        "Cannot fail",
                    )?;
                }
                CommitNodeInner::Case(_, _)
                | CommitNodeInner::AssertL(_, _)
                | CommitNodeInner::AssertR(_, _) => {
                    let a = naming.free_variable();
                    let b = naming.free_variable();
                    let c = naming.free_variable();

                    let sum_a_b = VariableType::Sum(a.clone(), b.clone());
                    let prod_sum_a_b_c = VariableType::Product(Variable::bound(sum_a_b), c.clone());
                    bind(&arrow.source, prod_sum_a_b_c, "Cannot fail")?;

                    if let CommitNodeInner::AssertR(_, _) = node {
                    } else {
                        bind(
                            &find_root(left_arrow.source.clone()),
                            VariableType::Product(a, c.clone()),
                            "Case: Left source = A × C",
                        )?;
                        unify(
                            arrow.target.clone(),
                            left_arrow.target.clone(),
                            "Cannot fail",
                        )?;
                    }
                    if let CommitNodeInner::AssertL(_, _) = node {
                    } else {
                        bind(
                            &find_root(right_arrow.source.clone()),
                            VariableType::Product(b, c),
                            "Case: Right source = B × C",
                        )?;
                        unify(
                            arrow.target.clone(),
                            right_arrow.target.clone(),
                            "Case: Left target = right target",
                        )?;
                    }
                }
                CommitNodeInner::Pair(_, _) => {
                    unify(
                        arrow.source.clone(),
                        left_arrow.source.clone(),
                        "Cannot fail",
                    )?;
                    unify(
                        left_arrow.source.clone(),
                        right_arrow.source.clone(),
                        "Pair: Left source = Right source",
                    )?;
                    bind(
                        &arrow.target,
                        VariableType::Product(
                            left_arrow.target.clone(),
                            right_arrow.target.clone(),
                        ),
                        "Cannot fail",
                    )?;
                }
                CommitNodeInner::Disconnect(_, _) => {
                    let a = naming.free_variable();
                    let b = naming.free_variable();
                    let c = naming.free_variable();
                    let d = naming.free_variable();

                    let pow2s = Variable::powers_of_two();
                    let prod_256_a =
                        Variable::bound(VariableType::Product(pow2s[8].clone(), a.clone()));
                    let prod_b_c = Variable::bound(VariableType::Product(b.clone(), c.clone()));
                    let prod_b_d = Variable::bound(VariableType::Product(b, d.clone()));

                    unify(arrow.source.clone(), a, "Cannot fail")?;
                    unify(arrow.target.clone(), prod_b_d, "Cannot fail")?;
                    unify(right_arrow.source.clone(), c, "Cannot fail")?;
                    unify(right_arrow.target.clone(), d, "Cannot fail")?;

                    unify(
                        left_arrow.source.clone(),
                        prod_256_a,
                        "Disconnect: Left source = 2^256 × A",
                    )?;
                    unify(
                        left_arrow.target.clone(),
                        prod_b_c,
                        "Disconnect: Left target = B × C",
                    )?;
                }
                _ => unreachable!(),
            }
        } else {
            match node {
                CommitNodeInner::InjL(_) => {
                    unify(
                        arrow.source.clone(),
                        left_arrow.source.clone(),
                        "Cannot fail",
                    )?;
                    let sum_b_c =
                        VariableType::Sum(left_arrow.target.clone(), naming.free_variable());
                    bind(&arrow.target, sum_b_c, "Cannot fail")?;
                }
                CommitNodeInner::InjR(_) => {
                    unify(
                        arrow.source.clone(),
                        left_arrow.source.clone(),
                        "Cannot fail",
                    )?;
                    let sum_b_c =
                        VariableType::Sum(naming.free_variable(), left_arrow.target.clone());
                    bind(&arrow.target, sum_b_c, "Cannot fail")?;
                }
                CommitNodeInner::Take(_) => {
                    unify(
                        arrow.target.clone(),
                        left_arrow.target.clone(),
                        "Cannot fail",
                    )?;
                    let prod_a_b =
                        VariableType::Product(left_arrow.source.clone(), naming.free_variable());
                    bind(&arrow.source, prod_a_b, "Cannot fail")?;
                }
                CommitNodeInner::Drop(_) => {
                    unify(
                        arrow.target.clone(),
                        left_arrow.target.clone(),
                        "Cannot fail",
                    )?;
                    let prod_a_b =
                        VariableType::Product(naming.free_variable(), left_arrow.source.clone());
                    bind(&arrow.source, prod_a_b, "Cannot fail")?;
                }
                _ => unreachable!(),
            }
        }
    } else {
        match node {
            CommitNodeInner::Iden => {
                unify(arrow.source.clone(), arrow.target.clone(), " Cannot fail")?
            }
            CommitNodeInner::Unit => bind(&arrow.target, VariableType::Unit, "Cannot fail")?,
            CommitNodeInner::Witness | CommitNodeInner::Fail(..) | CommitNodeInner::Hidden(..) => {
                // no type constraints
            }
            CommitNodeInner::Jet(jet) => {
                let pow2s = Variable::powers_of_two();
                bind(
                    &arrow.source,
                    jet.source_ty().to_type(&pow2s),
                    "Cannot fail",
                )?;
                bind(
                    &arrow.target,
                    jet.target_ty().to_type(&pow2s),
                    "Cannot fail",
                )?;
            }
            _ => unreachable!(),
        }
    }

    Ok(arrow)
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
        bind(&x, VariableType::Sum(x1, x2), "Cannot fail").unwrap();
        bind(
            &y,
            VariableType::Product(pow2s[8].clone(), naming.free_variable()),
            "Cannot fail",
        )
        .unwrap();

        unify(x, y, "Always fails").unwrap_err();
    }
}
