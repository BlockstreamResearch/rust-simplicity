// Simplicity "Human-Readable" Language
// Written in 2023 by
//   Andrew Poelstra <simplicity@wpsoftware.net>
//
// To the extent possible under law, the author(s) have dedicated all
// copyright and related and neighboring rights to this software to
// the public domain worldwide. This software is distributed without
// any warranty.
//
// You should have received a copy of the CC0 Public Domain Dedication
// along with this software.
// If not, see <http://creativecommons.org/publicdomain/zero/1.0/>.
//

//! Parsing

mod ast;

use crate::dag::{Dag, DagLike, InternalSharing};
use crate::jet::Jet;
use crate::node::{self, NoDisconnect, NoWitness};
use crate::types::Type;
use crate::Cmr;
use std::collections::HashMap;
use std::mem;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;

use super::named_node::{NamedCommitNode, NamedConstructNode, Namer};
use super::Position;

use super::{Error, ErrorSet};

#[derive(Clone)]
struct UnresolvedExpression<J: Jet> {
    inner: UnresolvedInner<J>,
    position: Position,
}

impl<J: Jet> UnresolvedExpression<J> {
    fn from_name(name: Arc<str>, position: Position) -> Self {
        UnresolvedExpression {
            inner: UnresolvedInner::NoExpr {
                name,
                user_source_types: vec![],
                user_target_types: vec![],
            },
            position,
        }
    }

    fn from_inline_expression(expr_inner: ast::ExprInner<J>, position: Position) -> Self {
        UnresolvedExpression {
            inner: UnresolvedInner::Inline { expr_inner },
            position,
        }
    }

    fn add_expression(
        &mut self,
        expr_inner: ast::ExprInner<J>,
        position: Position,
    ) -> Result<(), ErrorSet> {
        match self.inner {
            UnresolvedInner::NoExpr {
                ref name,
                ref mut user_source_types,
                ref mut user_target_types,
            } => {
                self.inner = UnresolvedInner::Named {
                    name: Arc::clone(name),
                    user_source_types: mem::take(user_source_types),
                    user_target_types: mem::take(user_target_types),
                    expr_inner,
                };
                Ok(())
            }
            UnresolvedInner::Inline { .. } => unreachable!(),
            UnresolvedInner::Named { ref name, .. } => Err(ErrorSet::single(
                position,
                Error::NameRepeated(Arc::clone(name)),
            )),
        }
    }

    fn add_source_type(&mut self, ty: Type) {
        match self.inner {
            UnresolvedInner::NoExpr {
                ref mut user_source_types,
                ..
            } => {
                user_source_types.push(ty);
            }
            UnresolvedInner::Inline { .. } => unreachable!(),
            UnresolvedInner::Named {
                ref mut user_source_types,
                ..
            } => {
                user_source_types.push(ty);
            }
        }
    }

    fn add_target_type(&mut self, ty: Type) {
        match self.inner {
            UnresolvedInner::NoExpr {
                ref mut user_target_types,
                ..
            } => {
                user_target_types.push(ty);
            }
            UnresolvedInner::Inline { .. } => unreachable!(),
            UnresolvedInner::Named {
                ref mut user_target_types,
                ..
            } => {
                user_target_types.push(ty);
            }
        }
    }
}

#[derive(Clone)]
enum UnresolvedInner<J: Jet> {
    NoExpr {
        name: Arc<str>,
        user_source_types: Vec<Type>,
        user_target_types: Vec<Type>,
    },
    Inline {
        expr_inner: ast::ExprInner<J>,
    },
    Named {
        name: Arc<str>,
        user_source_types: Vec<Type>,
        user_target_types: Vec<Type>,
        expr_inner: ast::ExprInner<J>,
    },
}

struct ResolvedExpression<J: Jet> {
    inner: ResolvedInner<J>,
    position: Position,

    name: Option<Arc<str>>,
    user_source_types: Arc<[Type]>,
    user_target_types: Arc<[Type]>,

    in_degree: AtomicUsize,
}

enum ResolvedCmr<J: Jet> {
    Expr(Arc<ResolvedExpression<J>>),
    Literal(Cmr),
}

enum ResolvedInner<J: Jet> {
    /// A reference to a missing expression
    Missing { name: Arc<str> },
    /// A reference to a name with no associated expression
    NoExpr { name: Arc<str> },
    /// A reference to another expression
    Reference(Arc<ResolvedExpression<J>>),
    /// A left assertion (referring to the CMR of an expression on the right)
    AssertL(Arc<ResolvedExpression<J>>, ResolvedCmr<J>),
    /// A right assertion (referring to the CMR of an expression on the left)
    AssertR(ResolvedCmr<J>, Arc<ResolvedExpression<J>>),
    /// An inline expression
    Inline(node::Inner<Arc<ResolvedExpression<J>>, J, NoDisconnect, NoWitness>),
}

pub fn parse<J: Jet + 'static>(
    program: &str,
) -> Result<HashMap<Arc<str>, Arc<NamedCommitNode<J>>>, ErrorSet> {
    let mut errors = ErrorSet::new();

    // **
    // Step 1: Read expressions into HashMap, checking for dupes and illegal names.
    // **
    let mut unresolved_map = HashMap::<Arc<str>, UnresolvedExpression<J>>::new();
    for line in ast::parse_line_vector(program)? {
        if line.name.as_ref() == "_" || line.name.starts_with("prim") {
            errors.add(line.position, Error::NameIllegal(Arc::clone(&line.name)));
            continue;
        }

        let entry =
            unresolved_map
                .entry(line.name.clone())
                .or_insert(UnresolvedExpression::from_name(
                    Arc::clone(&line.name),
                    line.position,
                ));

        if let Some(expr) = line.expression {
            if let Err(eset) = entry.add_expression(expr.inner, expr.position) {
                errors.merge(&eset)
            }
        }
        if let Some(ty) = line.arrow.0 {
            entry.add_source_type(ty.reify());
        }
        if let Some(ty) = line.arrow.1 {
            entry.add_target_type(ty.reify());
        }
    }

    // **
    // Step 2: Resolve every name.
    // We need to manually recurse rather than using `dag::PostOrderIter` because
    // we may have multiple disconnected components.
    // **
    let mut resolved_map =
        HashMap::<Arc<str>, Arc<ResolvedExpression<J>>>::with_capacity(unresolved_map.len());

    while let Some(name) = unresolved_map.keys().next() {
        let name = Arc::clone(name);
        let expr = unresolved_map.remove(&name).unwrap();

        #[derive(Clone)]
        struct StackItem<J: Jet> {
            expr: UnresolvedExpression<J>,
            name: Option<Arc<str>>,
            done_children: bool,
        }

        // Stack of unresolved entries, which are named expressions from `resolved_map`
        // as well as children of those named expressions (which themselves may be
        // either named or inline).
        let mut stack = vec![];
        // Stack of resolved inline entries. Somewhat confusingly, when resolving inline
        // expressions, we put them on the stack and use some careful accounting to make
        // sure that they're popped in the right place. When resolving named expressions,
        // we just stick the result into `resolved_map` and later refer to it by name.
        //
        // The reason we need to do both is that we are working with a "DAG forest" where
        // named expressions may be accessible from multiple roots. This means that when
        // we encounter a named expression, that expression might've already been resolved,
        // and there is no "intrinsic" way to distinguish these cases. So tracking state
        // using a stack would be pretty complicated.
        //
        // On the other hand, inline expressions do not have names or any other identifying
        // characteristics except the order in which they appear. So for these we need to
        // use the `inline_stack` to keep track of which ones we've already resolved..
        let mut inline_stack: Vec<Arc<ResolvedExpression<J>>> = vec![];
        stack.push(StackItem {
            expr,
            name: Some(Arc::clone(&name)),
            done_children: false,
        });

        while let Some(mut stack_item) = stack.pop() {
            // First, recurse on any children
            if !stack_item.done_children {
                stack_item.done_children = true;
                stack.push(stack_item.clone());

                let push_ast_expr = |stack: &mut Vec<_>, expr: &ast::Expression<_>| {
                    stack.push(StackItem {
                        expr: UnresolvedExpression::from_inline_expression(
                            expr.inner.clone(),
                            expr.position,
                        ),
                        name: None,
                        done_children: false,
                    })
                };

                match stack_item.expr.inner {
                    UnresolvedInner::NoExpr { .. } => {}
                    UnresolvedInner::Inline { ref expr_inner, .. }
                    | UnresolvedInner::Named { ref expr_inner, .. } => match expr_inner {
                        ast::ExprInner::Reference(ref ref_name) => {
                            if let Some(child) = unresolved_map.remove(ref_name) {
                                stack.push(StackItem {
                                    expr: child,
                                    name: Some(Arc::clone(ref_name)),
                                    done_children: false,
                                });
                            }
                        }
                        ast::ExprInner::AssertL(ref left, ref cmr) => {
                            push_ast_expr(&mut stack, left);
                            if let ast::AstCmr::Expr(ref right) = cmr {
                                push_ast_expr(&mut stack, right);
                            }
                        }
                        ast::ExprInner::AssertR(ref cmr, ref right) => {
                            if let ast::AstCmr::Expr(ref left) = cmr {
                                push_ast_expr(&mut stack, left);
                            }
                            push_ast_expr(&mut stack, right);
                        }
                        ast::ExprInner::Inline(ref inner) => match inner.as_dag() {
                            Dag::Nullary => {}
                            Dag::Unary(child) => push_ast_expr(&mut stack, child),
                            Dag::Binary(left, right) => {
                                push_ast_expr(&mut stack, left);
                                push_ast_expr(&mut stack, right);
                            }
                        },
                    },
                }
                continue;
            }

            let mut convert_expr_inner = |expr_inner: &ast::ExprInner<J>| match expr_inner {
                ast::ExprInner::Reference(ref ref_name) => {
                    if let Some(referent) = resolved_map.get(ref_name) {
                        referent.in_degree.fetch_add(1, Ordering::SeqCst);
                        ResolvedInner::Reference(Arc::clone(referent))
                    } else {
                        ResolvedInner::Missing {
                            name: Arc::clone(ref_name),
                        }
                    }
                }
                ast::ExprInner::AssertL(_, ref cmr) => {
                    let left = inline_stack.pop().unwrap();
                    left.in_degree.fetch_add(1, Ordering::SeqCst);

                    let right = match cmr {
                        ast::AstCmr::Expr(..) => {
                            let right = inline_stack.pop().unwrap();
                            right.in_degree.fetch_add(1, Ordering::SeqCst);
                            ResolvedCmr::Expr(right)
                        }
                        ast::AstCmr::Literal(cmr) => ResolvedCmr::Literal(*cmr),
                    };
                    ResolvedInner::AssertL(left, right)
                }
                ast::ExprInner::AssertR(ref cmr, _) => {
                    let left = match cmr {
                        ast::AstCmr::Expr(..) => {
                            let left = inline_stack.pop().unwrap();
                            left.in_degree.fetch_add(1, Ordering::SeqCst);
                            ResolvedCmr::Expr(left)
                        }
                        ast::AstCmr::Literal(cmr) => ResolvedCmr::Literal(*cmr),
                    };

                    let right = inline_stack.pop().unwrap();
                    right.in_degree.fetch_add(1, Ordering::SeqCst);
                    ResolvedInner::AssertR(left, right)
                }
                ast::ExprInner::Inline(ref inner) => ResolvedInner::Inline(
                    inner
                        .as_ref()
                        .map(|_| {
                            let child = inline_stack.pop().unwrap();
                            child.in_degree.fetch_add(1, Ordering::SeqCst);
                            child
                        })
                        .copy_disconnect()
                        .copy_witness(),
                ),
            };

            // Then, convert the node. At this point if we are missing any children
            // it is because there was a resolution error, i.e. the expression
            // references a child that doesn't exist.
            let resolved: ResolvedExpression<J> = match stack_item.expr.inner {
                UnresolvedInner::NoExpr {
                    ref name,
                    ref user_source_types,
                    ref user_target_types,
                } => ResolvedExpression {
                    inner: ResolvedInner::NoExpr {
                        name: Arc::clone(name),
                    },
                    position: stack_item.expr.position,
                    name: Some(Arc::clone(name)),
                    user_source_types: Arc::from(&user_source_types[..]),
                    user_target_types: Arc::from(&user_target_types[..]),
                    in_degree: AtomicUsize::new(0),
                },
                UnresolvedInner::Inline { ref expr_inner } => ResolvedExpression {
                    inner: convert_expr_inner(expr_inner),
                    position: stack_item.expr.position,
                    name: None,
                    user_source_types: Arc::from([]),
                    user_target_types: Arc::from([]),
                    in_degree: AtomicUsize::new(0),
                },
                UnresolvedInner::Named {
                    ref expr_inner,
                    ref name,
                    ref user_source_types,
                    ref user_target_types,
                } => ResolvedExpression {
                    inner: convert_expr_inner(expr_inner),
                    position: stack_item.expr.position,
                    name: Some(Arc::clone(name)),
                    user_source_types: Arc::from(&user_source_types[..]),
                    user_target_types: Arc::from(&user_target_types[..]),
                    in_degree: AtomicUsize::new(0),
                },
            };

            if let Some(name) = stack_item.name {
                resolved_map.insert(name, Arc::new(resolved));
            } else {
                inline_stack.push(Arc::new(resolved));
            }
        }
        assert!(inline_stack.is_empty());
    }
    drop(unresolved_map);

    // ** Step 3: convert each DAG of names/expressions into a DAG of NamedNodes.
    impl<'a, J: Jet> DagLike for &'a ResolvedExpression<J> {
        type Node = ResolvedExpression<J>;
        fn data(&self) -> &ResolvedExpression<J> {
            self
        }

        fn as_dag_node(&self) -> Dag<Self> {
            match self.inner {
                ResolvedInner::Missing { .. } | ResolvedInner::NoExpr { .. } => Dag::Nullary,
                ResolvedInner::Reference(ref child) => Dag::Unary(child),
                ResolvedInner::AssertL(ref left, ResolvedCmr::Expr(ref right))
                | ResolvedInner::AssertR(ResolvedCmr::Expr(ref left), ref right) => {
                    Dag::Binary(left, right)
                }
                ResolvedInner::AssertL(ref child, ResolvedCmr::Literal(..))
                | ResolvedInner::AssertR(ResolvedCmr::Literal(..), ref child) => Dag::Unary(child),
                ResolvedInner::Inline(ref inner) => inner.as_dag().map(|node| &**node),
            }
        }
    }

    let mut roots = HashMap::<Arc<str>, Arc<NamedCommitNode<J>>>::new();
    for (name, expr) in &resolved_map {
        if expr.in_degree.load(Ordering::SeqCst) > 0 {
            continue;
        }

        let mut namer = Namer::new();
        let mut converted: Vec<Option<Arc<NamedConstructNode<J>>>> = vec![];
        for data in expr.as_ref().post_order_iter::<InternalSharing>() {
            let left = data
                .left_index
                .and_then(|idx| converted[idx].as_ref().map(Arc::clone));
            let right = data
                .right_index
                .and_then(|idx| converted[idx].as_ref().map(Arc::clone));

            let maybe_inner = match data.node.inner {
                ResolvedInner::Missing { ref name, .. } => {
                    errors.add(data.node.position, Error::NameMissing(Arc::clone(name)));
                    None
                }
                ResolvedInner::NoExpr { ref name, .. } => {
                    errors.add(data.node.position, Error::NameIncomplete(Arc::clone(name)));
                    None
                }
                ResolvedInner::Reference(..) => {
                    converted.push(left);
                    continue;
                }
                ResolvedInner::AssertL(..) => left.zip(right).map(|(left, right)| {
                    let cmr = right.cmr();
                    node::Inner::AssertL(left, cmr)
                }),
                ResolvedInner::AssertR(..) => left.zip(right).map(|(left, right)| {
                    let cmr = left.cmr();
                    node::Inner::AssertR(cmr, right)
                }),
                ResolvedInner::Inline(ref inner) => inner
                    .as_ref()
                    .map_left_right(|_| left, |_| right)
                    .copy_disconnect()
                    .copy_witness()
                    .transpose(),
            };

            let inner = match maybe_inner {
                Some(inner) => inner,
                None => {
                    converted.push(None);
                    continue;
                }
            };

            let name = data
                .node
                .name
                .as_ref()
                .map(Arc::clone)
                .unwrap_or_else(|| Arc::from(namer.assign_name(inner.as_ref()).as_str()));

            let node = NamedConstructNode::new(
                Arc::clone(&name),
                data.node.position,
                Arc::clone(&data.node.user_source_types),
                Arc::clone(&data.node.user_target_types),
                inner,
            );

            match node {
                Ok(node) => {
                    let node = Arc::new(node);
                    converted.push(Some(node));
                }
                Err(e) => {
                    errors.add(data.node.position, e);
                    converted.push(None);
                }
            }
        }

        if let Some(Some(root)) = converted.pop() {
            let finalized = if &**name == "main" {
                root.finalize_types_main()
            } else {
                root.finalize_types_non_main()
            };

            match finalized {
                Ok(root) => {
                    roots.insert(Arc::clone(name), root);
                }
                Err(errs) => errors.merge(&errs),
            }
        }
    }

    // From each root, count the number of ways that each witness node can be reached
    for root in roots.values() {
        let mut counts: Vec<HashMap<Arc<str>, usize>> = vec![];
        for data in root.as_ref().post_order_iter::<InternalSharing>() {
            let mut new_counts = HashMap::new();

            if let Some(idx) = data.left_index {
                for (name, count) in &counts[idx] {
                    *new_counts.entry(Arc::clone(name)).or_insert(0) += count;
                }
            }
            if let Some(idx) = data.right_index {
                for (name, count) in &counts[idx] {
                    *new_counts.entry(Arc::clone(name)).or_insert(0) += count;
                }
            }

            if matches!(
                data.node.inner(),
                node::Inner::Disconnect(_, _) | node::Inner::Witness(_)
            ) {
                *new_counts.entry(Arc::clone(data.node.name())).or_insert(0) += 1;
            }

            counts.push(new_counts);
        }

        let accumulated = counts.last().unwrap();
        for (name, count) in accumulated {
            if *count > 1 {
                errors.add_no_position(Error::WitnessDisconnectRepeated {
                    name: Arc::clone(name),
                    count: *count,
                });
            }
        }
    }

    if errors.is_empty() {
        Ok(roots)
    } else {
        Err(errors)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::jet::{Core, Jet};

    fn assert_single_cmr<J: Jet>(s: &str, cmr: &str) {
        match parse::<J>(s) {
            Ok(forest) => {
                assert_eq!(forest.len(), 1);
                let main = &forest["main"];
                assert_eq!(main.cmr().to_string(), cmr);
            }
            Err(errs) => {
                println!("\nErrors:\n{}", errs);
                panic!("Failed to parse program:\n{}", s);
            }
        }
    }

    fn error_contains<T>(res: Result<T, ErrorSet>, s: &str) {
        match res {
            Ok(_) => panic!("expected error, but result was Ok"),
            Err(errs) => {
                let e_msg = errs.to_string();
                assert!(
                    e_msg.contains(s),
                    "Did not find search string {} in error string: {}",
                    s,
                    e_msg,
                );
            }
        }
    }

    #[test]
    fn errors() {
        error_contains(
            parse::<Core>("main := a"),
            "name `a` is referred to but does not exist",
        );
        error_contains(
            parse::<Core>("main := unit : 1 -> 2"),
            "failed to apply bound `2` to existing bound `1`",
        );
        error_contains(
            parse::<Core>("main := pair unit unit"),
            "failed to apply bound `1 × 1` to existing bound `1`",
        );
    }

    #[test]
    fn simple_program() {
        assert_single_cmr::<Core>(
            "main := unit",
            "62274a89833ece8ba5ff57b28118c0063d3d4a85dd25aae06f87617604402715",
        );

        assert_single_cmr::<Core>(
            "
                wit1 := witness : 1 -> 2^32
                wit2 := witness : 1 -> 2^32

                wits_are_equal := comp (pair wit1 wit2) jet_eq_32 : 1 -> 2
                main := comp wits_are_equal jet_verify            : 1 -> 1
            ",
            "e552742731de7f5c3c83cd54176c0ca6acf6dbd3c37bef7da132eb06f3856d06",
        );
    }

    #[test]
    fn circular_program() {
        error_contains(
            parse::<Core>("main := comp main unit"),
            "name `main` is referred to but does not exist",
        );
    }

    #[test]
    #[cfg(feature = "elements")]
    fn bip340_program() {
        use crate::jet::Elements;

        parse::<Elements>("main := unit").unwrap();

        assert_single_cmr::<Elements>("
                -- Witnesses
                wit1 := witness : 1 -> 2^512

                -- Constants
                const1 := const 0xf9308a019258c31049344f85f89d5229b531c845836f99b08601f113bce036f90000000000000000000000000000000000000000000000000000000000000000 : 1 -> 2^512 -- ef40cb78

                -- Program code
                pr1 := pair const1 wit1    : 1 -> (2^512 * 2^512) -- 8080ed5e
                jt2 := jet_bip_0340_verify : (2^512 * 2^512) -> 1 -- af924cbe

                main := comp pr1 jt2       : 1 -> 1               -- 3f6422da
            ",
            "3f6422da75bf12565329872840d33adacbb49d49f2285a9f7133b79e034d8899",
        );
    }
}
