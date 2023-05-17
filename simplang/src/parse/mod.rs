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
mod error;

use simplicity::{CommitNode, Context, Cmr, jet::Jet, types::{self, arrow::Arrow, Type}};
use std::collections::HashMap;
use std::rc::Rc;
use std::sync::{Mutex, MutexGuard};

use crate::{Position, Program};

pub use error::{Error, ErrorType};

struct ParseContext<J: Jet> {
    sctx: Mutex<Context<J>>,
    expr_map: HashMap<String, ParsedExpression<J>>,
    node_map: Mutex<HashMap<String, Rc<CommitNode<J>>>>,
    cmr_map: Mutex<HashMap<Cmr, String>>,
}

impl<J: Jet> ParseContext<J> {
    fn new() -> Self {
        ParseContext {
            sctx: Mutex::new(Context::new()),
            expr_map: HashMap::new(),
            node_map: Mutex::new(HashMap::new()),
            cmr_map: Mutex::new(HashMap::new()),
        }
    }

    fn sctx(&self) -> MutexGuard<Context<J>> {
        self.sctx.lock().unwrap()
    }
}

fn push_error<T>(
    res: Result<T, simplicity::types::Error>,
    position: Position,
    errors: &mut Vec<Error>,
) {
    if let Err(err) = res {
        errors.push(Error {
            ty: ErrorType::TypeCheck(err),
            position: position,
        })
    }
}

enum ParsedExpression<J: Jet> {
    Missing,
    Ast {
        expr: ast::Expression<J>,
        arrow: Arrow,
    },
}

fn resolve_1<J: Jet, F: FnOnce(&mut Context<J>, Rc<CommitNode<J>>) -> Rc<CommitNode<J>>>(
    ctx: &ParseContext<J>,
    expr: &ast::Expression<J>,
    comb: F,
) -> Result<Rc<CommitNode<J>>, Vec<Error>> {
    resolve(ctx, expr).map(|res| comb(&mut ctx.sctx(), res))
}

fn resolve_2<J: Jet, F: FnOnce(&mut Context<J>, Rc<CommitNode<J>>, Rc<CommitNode<J>>) -> Result<Rc<CommitNode<J>>, types::Error>>(
    ctx: &ParseContext<J>,
    left_expr: &ast::Expression<J>,
    right_expr: &ast::Expression<J>,
    position: Position,
    comb: F,
) -> Result<Rc<CommitNode<J>>, Vec<Error>> {
     match (resolve(ctx, left_expr), resolve(ctx, right_expr)) {
        (Ok(left), Ok(right)) => comb(&mut ctx.sctx(), left, right).map_err(|err| vec![Error {
            ty: ErrorType::TypeCheck(err),
            position,
        }]),
        (Ok(_), Err(rerr)) => Err(rerr),
        (Err(lerr), Ok(_)) => Err(lerr),
        (Err(mut lerr), Err(rerr)) => {
            lerr.extend(rerr);
            Err(lerr)
        }
    }
}

fn resolve<J: Jet>(
    ctx: &ParseContext<J>,
    astexpr: &ast::Expression<J>,
) -> Result<Rc<CommitNode<J>>, Vec<Error>> {
    match astexpr.variant {
        ast::ExpressionVariant::Symbol(ref refname) => {
            match ctx.expr_map.get(refname) {
                Some(ParsedExpression::Missing) => Err(vec![Error::incomplete_symbol(refname.to_owned(), astexpr.position)]),
                Some(ParsedExpression::Ast { ref expr, arrow }) => {
                    let res = resolve(ctx, expr)?;
                    let mut errors = vec![];
                    push_error(
                        res.arrow().source.unify(&arrow.source, "unifying source types"),
                        astexpr.position,
                        &mut errors,
                    );
                    push_error(
                        res.arrow().target.unify(&arrow.target, "unifying target types"),
                        astexpr.position,
                        &mut errors,
                    );
                    if errors.is_empty() {
                        ctx.node_map.lock().unwrap().insert(refname.clone(), Rc::clone(&res));
                        ctx.cmr_map.lock().unwrap().insert(res.cmr(), refname.clone());
                        Ok(res)
                    } else {
                        Err(errors)
                    }
                },
                None => Err(vec![Error::undefined_symbol(refname.to_owned(), astexpr.position)]),
            }
        },
        ast::ExpressionVariant::Unit => Ok(CommitNode::unit(&mut ctx.sctx())),
        ast::ExpressionVariant::Iden => Ok(CommitNode::iden(&mut ctx.sctx())),
        ast::ExpressionVariant::Comp(ref left, ref right) => resolve_2(ctx, left, right, astexpr.position, CommitNode::comp),
        ast::ExpressionVariant::Injl(ref comb) => resolve_1(ctx, comb, CommitNode::injl),
        ast::ExpressionVariant::Injr(ref comb) => resolve_1(ctx, comb, CommitNode::injr),
        ast::ExpressionVariant::Case(ref left, ref right) => resolve_2(ctx, left, right, astexpr.position, CommitNode::case),
        ast::ExpressionVariant::AssertL(ref comb, cmr) => resolve(ctx, comb).and_then(|sub| {
            CommitNode::assertl(&mut ctx.sctx(), sub, cmr).map_err(|err| vec![Error {
                ty: ErrorType::TypeCheck(err),
                position: astexpr.position,
            }])
        }),
        ast::ExpressionVariant::AssertR(ref comb, cmr) => resolve(ctx, comb).and_then(|sub| {
            CommitNode::assertr(&mut ctx.sctx(), cmr, sub).map_err(|err| vec![Error {
                ty: ErrorType::TypeCheck(err),
                position: astexpr.position,
            }])
        }),
        ast::ExpressionVariant::Drop(ref comb) => resolve_1(ctx, comb, CommitNode::drop),
        ast::ExpressionVariant::Take(ref comb) => resolve_1(ctx, comb, CommitNode::take),
        ast::ExpressionVariant::Pair(ref left, ref right) => resolve_2(ctx, left, right, astexpr.position, CommitNode::pair),
        ast::ExpressionVariant::Disconnect(ref left, ref right) => resolve_2(ctx, left, right, astexpr.position, CommitNode::disconnect),
        ast::ExpressionVariant::Fail(cmr1, cmr2) => Ok(CommitNode::fail(&mut ctx.sctx(), cmr1, cmr2)),
        ast::ExpressionVariant::Hidden(cmr) => Ok(CommitNode::hidden(&mut ctx.sctx(), cmr)),
        ast::ExpressionVariant::Witness => Ok(CommitNode::witness(&mut ctx.sctx())),
        ast::ExpressionVariant::Word { ref data, .. } => Ok(CommitNode::const_word(&mut ctx.sctx(), data.clone())),
        ast::ExpressionVariant::Jet(jet) => Ok(CommitNode::jet(&mut ctx.sctx(), jet.clone())),
    }
}

pub fn parse<J: Jet>(program: &str) -> Result<Program<J>, Vec<Error>> {
    let mut ctx = ParseContext::<J>::new();
    let mut errors = vec![];

    // First read all expressions into hashmap
    for line in ast::parse(program)? {
        let entry = ctx.expr_map.entry(line.name).or_insert(ParsedExpression::Missing);

        if let Some(expr) = line.expression {
            let mut sctx = ctx.sctx.lock().unwrap();
            *entry = ParsedExpression::Ast {
                expr,
                arrow: line.arrow.map(|(src, tgt)| Arrow {
                    source: src.reify(&mut sctx),
                    target: tgt.reify(&mut sctx),
                }).unwrap_or(Arrow {
                    source: Type::free("unnamed_src".into()),
                    target: Type::free("unnamed_tgt".into()),
                }),
            };
        }
    }
    // Then try to resolve everything
    let mut final_result = None;
    match ctx.expr_map.get("main") {
        Some(ParsedExpression::Missing) => errors.push(Error::incomplete_symbol("main".to_owned(), Position::default())),
        Some(ParsedExpression::Ast { expr, arrow }) => {
            match resolve(&ctx, expr) {
                Ok(res) => {
                    push_error(
                        res.arrow().source.unify(&arrow.source, "unifying source types"),
                        expr.position,
                        &mut errors,
                    );
                    push_error(
                        res.arrow().target.unify(&arrow.target, "unifying target types"),
                        expr.position,
                        &mut errors,
                    );
                    ctx.node_map.lock().unwrap().insert("main".to_owned(), Rc::clone(&res));
                    ctx.cmr_map.lock().unwrap().insert(res.cmr(), "main".to_owned());
                    final_result = Some((res, expr.position));
                }
                Err(es) => errors.extend(es),
            }
        }
        None => errors.push(Error::no_main()),
    }

    // Finally, set main's type to 1->1
    if let Some((ref main, position)) = final_result {
        let sctx = ctx.sctx.lock().unwrap();
        push_error(
            main.arrow().source.unify(&sctx.unit_ty(), "setting main source to 1"),
            position,
            &mut errors,
        );
        push_error(
            main.arrow().target.unify(&sctx.unit_ty(), "setting main target to 1"),
            position,
            &mut errors,
        );
    }

    if errors.is_empty() {
        Ok(Program {
            root_node: final_result.unwrap().0,
            node_map: ctx.node_map.into_inner().unwrap(),
            cmr_name_map: ctx.cmr_map.into_inner().unwrap(),
        })
    } else {
        Err(errors)
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    use simplicity::jet::Elements;

    fn error_contains<T>(res: Result<T, Vec<Error>>, s: &str) {
        match res {
            Ok(_) => panic!("expected error, but result was Ok"),
            Err(v) => {
                for e in v {
                    let e_msg = e.to_string();
                    assert!(
                        e_msg.contains(s),
                        "Did not find search string {} in error string: {}",
                        s,
                        e_msg,
                    );
                }
            },
        }
    }

    #[test]
    fn errors() {
        error_contains(parse::<Elements>(""), "does not have `main`");
        error_contains(parse::<Elements>("a = unit"), "does not have `main`");
        error_contains(parse::<Elements>("main = a"), "undefined symbol `a`");
        error_contains(parse::<Elements>("main = unit :: 1 -> 2"), "attempted to unify unequal types 1 and 2");
        error_contains(parse::<Elements>("main = pair unit unit"), "setting main target to 1");
        error_contains(parse::<Elements>("main = hidden xyzz"), "bad hex string length 4");
        error_contains(parse::<Elements>("main = hidden"), "could not parse");
    }

    #[test]
    fn simple_program() {
        parse::<Elements>("main = unit").unwrap();

        match parse::<simplicity::jet::Core>("
            wit1 = witness :: 1 -> 2^32
            wit2 = witness :: 1 -> 2^32

            wits_are_equal = comp (pair wit1 wit2) jet_eq_32 :: 1 -> 2
            main = comp wits_are_equal jet_verify            :: 1 -> 1
        ") {
            Ok(prog) => {
                assert_eq!(
                    prog.root().cmr().to_string(),
                    "e552742731de7f5c3c83cd54176c0ca6acf6dbd3c37bef7da132eb06f3856d06",
                );
            },
            Err(errs) => {
                for err in errs {
                    println!("Error: {}", err);
                }
            }
        }

    }

    #[test]
    fn bip340_program() {
        parse::<Elements>("main = unit").unwrap();

        let prog = parse::<Elements>("
            # Witnesses
            wit1 = witness :: 1 -> 2^512

            # Constants
            const1 = word_jet 0xf9308a019258c31049344f85f89d5229b531c845836f99b08601f113bce036f90000000000000000000000000000000000000000000000000000000000000000 :: 1 -> 2^512 # f254d6e9

            # Program code
            pr1 = pair const1 wit1    :: 1 -> (2^512 * 2^512) # 31ae2960
            jt2 = jet_bip_0340_verify :: (2^512 * 2^512) -> 1 # af924cbe

            main = comp pr1 jt2       :: 1 -> 1               # 7bc56cb1
        ").unwrap();

        assert_eq!(
            prog.root().cmr().to_string(),
            "7bc56cb16d84999b977b58e1bc71dbe9edcc33650afc8a6ee05cfef8d608132b",
        );
    }
}

