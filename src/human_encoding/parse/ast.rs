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

use std::mem;
use std::str::FromStr;
use std::sync::Arc;

use crate::human_encoding::{Error, ErrorSet, Position, WitnessOrHole};
use crate::jet::Jet;
use crate::{node, types};
use crate::{BitIter, Cmr, FailEntropy};
use santiago::grammar::{Associativity, Grammar};
use santiago::lexer::{Lexeme, LexerRules};

/// A single non-empty line of a program, of the form x = y :: t
///
/// A program is simply a list of such lines
#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub struct Line<J> {
    /// Position of the first character of the line.
    pub position: Position,
    /// The name of the expression being named on the line.
    pub name: Arc<str>,
    /// The actual expression, if present (missing for type declarations).
    pub expression: Option<Expression<J>>,
    /// The type of the expression, if given (inferred if missing).
    pub arrow: (Option<Type>, Option<Type>),
}

/// An expression, as represented in the AST
#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub struct Expression<J> {
    pub inner: ExprInner<J>,
    pub position: Position,
}

impl<J: Jet> Expression<J> {
    fn reference(name: Arc<str>, position: Position) -> Self {
        Expression {
            inner: ExprInner::Reference(name),
            position,
        }
    }
}

/// An expression, as represented in the AST
#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub enum ExprInner<J> {
    /// A reference to another expression
    Reference(Arc<str>),
    /// A left assertion (referring to the CMR of an expression on the right)
    AssertL(Arc<Expression<J>>, AstCmr<J>),
    /// A right assertion (referring to the CMR of an expression on the left)
    AssertR(AstCmr<J>, Arc<Expression<J>>),
    /// An inline expression
    Inline(node::Inner<Arc<Expression<J>>, J, node::NoDisconnect, WitnessOrHole>),
}

/// A CMR, as represented in the AST
#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub enum AstCmr<J> {
    Expr(Arc<Expression<J>>),
    Literal(Cmr),
}

/// A type, as represented in the AST
#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub enum Type {
    /// A named type variable
    Name(String),
    /// The unit type 1
    One,
    /// The bit type 1+1
    Two,
    /// A product type (A * B)
    Product(Box<Type>, Box<Type>),
    /// A sum type (A + B)
    Sum(Box<Type>, Box<Type>),
    /// An exponential type 2^(2^n).
    TwoTwoN(u32),
}

impl Type {
    /// Convert to a Simplicity type
    pub fn reify(self) -> types::Type {
        match self {
            Type::Name(s) => types::Type::free(s),
            Type::One => types::Type::unit(),
            Type::Two => types::Type::sum(types::Type::unit(), types::Type::unit()),
            Type::Product(left, right) => types::Type::product(left.reify(), right.reify()),
            Type::Sum(left, right) => types::Type::sum(left.reify(), right.reify()),
            Type::TwoTwoN(n) => types::Type::two_two_n(n as usize), // cast OK as we are only using tiny numbers
        }
    }
}

/// Takes a program as a string and parses it into an AST (actually, a vector
/// of lines, each of which is individually an AST)
pub fn parse_line_vector<J: Jet + 'static>(input: &str) -> Result<Vec<Line<J>>, ErrorSet> {
    let lexer_rules = lexer_rules();
    let grammar = grammar::<J>();

    let lexemes = match santiago::lexer::lex(&lexer_rules, input) {
        Ok(lexemes) => lexemes,
        Err(err) => return Err(err.into()),
    };
    let trees = santiago::parser::parse(&grammar, &lexemes)?;
    assert_eq!(trees.len(), 1, "ambiguous parse (this is a bug)");
    match trees[0].as_abstract_syntax_tree() {
        Ast::Program(lines) => Ok(lines),
        Ast::Error(errs) => Err(errs),
        x => unreachable!(
            "Parsed program into non-program non-error {:?}; this is a bug.",
            x
        ),
    }
}

/// Check a list of AST elements for errors; if any are errors, combine them and return the result
fn propagate_errors<J: Jet>(ast: &[Ast<J>]) -> Option<Ast<J>> {
    let mut e = ErrorSet::new();
    for elem in ast {
        if let Ast::Error(errs) = elem {
            e.merge(errs);
        }
    }
    if e.is_empty() {
        None
    } else {
        Some(Ast::Error(e))
    }
}

/// Main AST structure
///
/// This structure is never really instantiated; during construction it is
/// continually collapsed until in the end it will be in either the `Program`
/// or `Error` variant.
#[derive(Debug, Clone)]
enum Ast<J: Jet> {
    Combinator {
        comb: node::Inner<(), J, node::NoDisconnect, WitnessOrHole>,
        position: Position,
    },
    /// A type->type arrow
    Arrow(Option<Type>, Option<Type>),
    /// A #{expr} or #abcd CMR
    Cmr(AstCmr<J>),
    /// An error occurred during parsing
    Error(ErrorSet),
    /// An expression
    Expression(Expression<J>),
    /// A full expression line
    Line(Line<J>),
    /// A hex or binary literal
    Literal {
        data: Vec<u8>,
        bit_length: usize,
        position: Position,
    },
    /// The top-level program
    Program(Vec<Line<J>>),
    /// A symbol
    Symbol { value: Arc<str>, position: Position },
    /// A type
    Type(Option<Type>),
    /// Dummy value used internally in the parser when building the tree.
    ///
    /// Any parse objects which have no information in themselves (e.g. the
    /// plus or star symbols) but which are just used to shape the parse
    /// tree, get mapped to this value and then discarded.
    Dummy { position: Position },
    /// Dummy value used when manipulating the tree in-place, to replace
    /// data that we move out of the tree
    Replaced,
}

impl<J: Jet> Ast<J> {
    /// Creates an `Ast` from a single sub-AST
    fn from_1<T, F1, F>(toks: &mut [Self], convert: F1, unconvert: F) -> Self
    where
        F1: FnOnce(&mut Self) -> T,
        F: FnOnce(T) -> Self,
    {
        if let Some(e) = propagate_errors(toks) {
            return e;
        }
        assert_eq!(toks.len(), 1);
        unconvert(convert(&mut toks[0]))
    }

    /// Creates an `Ast` from two sub-`Ast`s with a dummy in between
    fn from_2<T, U, F1, F2, F>(toks: &mut [Self], convert1: F1, convert2: F2, unconvert: F) -> Self
    where
        F1: FnOnce(&mut Self) -> T,
        F2: FnOnce(&mut Self) -> U,
        F: FnOnce(T, U) -> Self,
    {
        if let Some(e) = propagate_errors(toks) {
            return e;
        }
        assert_eq!(toks.len(), 3);
        toks[1].expect_position();
        unconvert(convert1(&mut toks[0]), convert2(&mut toks[2]))
    }

    /// Creates an `Ast` from three sub-`Ast`s with dummies in between
    fn from_3<T, U, V, F1, F2, F3, F>(
        toks: &mut [Self],
        convert1: F1,
        convert2: F2,
        convert3: F3,
        unconvert: F,
    ) -> Self
    where
        F1: FnOnce(&mut Self) -> T,
        F2: FnOnce(&mut Self) -> U,
        F3: FnOnce(&mut Self) -> V,
        F: FnOnce(T, U, V) -> Self,
    {
        if let Some(e) = propagate_errors(toks) {
            return e;
        }
        assert_eq!(toks.len(), 5);
        toks[1].expect_position();
        toks[3].expect_position();
        unconvert(
            convert1(&mut toks[0]),
            convert2(&mut toks[2]),
            convert3(&mut toks[4]),
        )
    }

    /// Creates an AST from a combinator with no arguments
    fn from_combinator(mut toks: Vec<Self>) -> Self {
        if let Some(e) = propagate_errors(&toks) {
            return e;
        }

        let (comb, position) = match mem::replace(&mut toks[0], Ast::Replaced) {
            Ast::Combinator { comb, position } => (comb, position),
            _ => unreachable!(),
        };

        // This stupid construction is needed to avoid borrowck rules
        // around borrowing tok[1] and tok[2] mutably at the same time.
        let arcs: Vec<Arc<_>> = toks[1..]
            .iter_mut()
            .map(Ast::expect_expression)
            .map(Arc::new)
            .collect();
        let inner = comb.map_left_right(|_| Arc::clone(&arcs[0]), |_| Arc::clone(&arcs[1]));
        Ast::Expression(Expression {
            inner: ExprInner::Inline(inner),
            position,
        })
    }

    /// Creates an AST from a dummy lexeme
    fn from_dummy_lexeme(lexemes: &[&std::rc::Rc<Lexeme>]) -> Self {
        assert_eq!(lexemes.len(), 1);
        let position = (&lexemes[0].position).into();
        Ast::Dummy { position }
    }

    /// Creates an AST from a lexeme which forms a complete type
    fn from_type_lexeme(lexemes: &[&std::rc::Rc<Lexeme>]) -> Self {
        assert_eq!(lexemes.len(), 1);
        let position = &lexemes[0].position;
        match lexemes[0].raw.as_str() {
            "1" => Ast::Type(Some(Type::One)),
            "2" => Ast::Type(Some(Type::Two)),
            other => {
                assert_eq!(&other[..2], "2^");
                match str::parse::<u32>(&other[2..]) {
                    // TODO how many of these should we support?
                    Ok(0) => Ast::Type(Some(Type::One)),
                    Ok(1) => Ast::Type(Some(Type::Two)),
                    Ok(2) => Ast::Type(Some(Type::TwoTwoN(1))),
                    Ok(4) => Ast::Type(Some(Type::TwoTwoN(2))),
                    Ok(8) => Ast::Type(Some(Type::TwoTwoN(3))),
                    Ok(16) => Ast::Type(Some(Type::TwoTwoN(4))),
                    Ok(32) => Ast::Type(Some(Type::TwoTwoN(5))),
                    Ok(64) => Ast::Type(Some(Type::TwoTwoN(6))),
                    Ok(128) => Ast::Type(Some(Type::TwoTwoN(7))),
                    Ok(256) => Ast::Type(Some(Type::TwoTwoN(8))),
                    Ok(512) => Ast::Type(Some(Type::TwoTwoN(9))),
                    Ok(y) => Ast::Error(ErrorSet::single(position, Error::Bad2ExpNumber(y))),
                    Err(_) => Ast::Error(ErrorSet::single(
                        position,
                        Error::NumberOutOfRange(other.to_owned()),
                    )),
                }
            }
        }
    }

    /// Creates an AST from a combinator lexeme
    fn from_combinator_lexeme(lexemes: &[&std::rc::Rc<Lexeme>]) -> Self {
        assert_eq!(lexemes.len(), 1);
        let position = (&lexemes[0].position).into();
        let comb = match lexemes[0].raw.as_str() {
            "unit" => node::Inner::Unit,
            "iden" => node::Inner::Iden,
            "injl" => node::Inner::InjL(()),
            "injr" => node::Inner::InjR(()),
            "take" => node::Inner::Take(()),
            "drop" => node::Inner::Drop(()),
            "comp" => node::Inner::Comp((), ()),
            "case" => node::Inner::Case((), ()),
            "assertl" => node::Inner::AssertL((), Cmr::unit()),
            "assertr" => node::Inner::AssertR(Cmr::unit(), ()),
            "pair" => node::Inner::Pair((), ()),
            "disconnect" => node::Inner::Disconnect((), node::NoDisconnect),
            "witness" => node::Inner::Witness(WitnessOrHole::Witness),
            "fail" => node::Inner::Fail(FailEntropy::ZERO),
            other => {
                assert_eq!(&other[..4], "jet_");
                if let Ok(jet) = J::from_str(&other[4..]) {
                    node::Inner::Jet(jet)
                } else {
                    return Ast::Error(ErrorSet::single(
                        position,
                        Error::UnknownJet(other.to_owned()),
                    ));
                }
            }
        };
        Ast::Combinator { comb, position }
    }

    /// Creates an AST from a literal lexeme
    fn from_literal_lexeme(lexemes: &[&std::rc::Rc<Lexeme>]) -> Self {
        assert_eq!(lexemes.len(), 1);
        let position = (&lexemes[0].position).into();

        if lexemes[0].raw == "_" {
            return Ast::Literal {
                data: vec![],
                bit_length: 0,
                position,
            };
        }

        let s = &lexemes[0].raw[2..];
        if &lexemes[0].raw[..2] == "0x" {
            let bit_length = s.len() * 4;
            let mut data = Vec::with_capacity((s.len() + 1) / 2);
            for idx in (0..s.len() / 2).map(|n| n * 2) {
                data.push(u8::from_str_radix(&s[idx..idx + 1], 16).unwrap());
            }
            if s.len() % 2 == 1 {
                data.push(u8::from_str_radix(&s[s.len() - 1..], 16).unwrap() << 4);
            }

            Ast::Literal {
                data,
                bit_length,
                position,
            }
        } else {
            assert_eq!(&lexemes[0].raw[..2], "0b");

            let bit_length = s.len();
            let mut data = Vec::with_capacity((s.len() + 7) / 8);
            let mut x = 0;
            for (n, ch) in s.chars().enumerate() {
                match ch {
                    '0' => {}
                    '1' => x |= 1 << (7 - (n % 8)),
                    _ => unreachable!(),
                }
                if n % 8 == 7 {
                    data.push(x);
                    x = 0;
                }
            }
            if s.len() % 8 != 0 {
                data.push(x);
            }

            Ast::Literal {
                data,
                bit_length,
                position,
            }
        }
    }

    /// Creates an AST from a CMR literal lexeme
    fn from_cmr_literal_lexeme(lexemes: &[&std::rc::Rc<Lexeme>]) -> Self {
        assert_eq!(lexemes.len(), 1);
        assert_eq!(lexemes[0].raw.len(), 65);

        Ast::Cmr(AstCmr::Literal(
            Cmr::from_str(&lexemes[0].raw[1..]).unwrap(),
        ))
    }

    fn expect_arrow(&mut self) -> (Option<Type>, Option<Type>) {
        let replaced = mem::replace(self, Ast::Replaced);
        if let Ast::Arrow(a, b) = replaced {
            (a, b)
        } else {
            panic!("Expected arrow, got {:?}", self);
        }
    }

    /// Checks that a given AST element is a dummy value
    fn expect_position(&self) -> Position {
        match self {
            Ast::Combinator { position, .. } => *position,
            Ast::Dummy { position } => *position,
            Ast::Literal { position, .. } => *position,
            Ast::Symbol { position, .. } => *position,
            _ => panic!("Expected some element with position, got {:?}", self),
        }
    }

    fn expect_cmr(&mut self) -> AstCmr<J> {
        let replaced = mem::replace(self, Ast::Replaced);
        if let Ast::Cmr(cmr) = replaced {
            cmr
        } else {
            panic!("Expected CMR, got {:?}", replaced);
        }
    }

    fn expect_expression(&mut self) -> Expression<J> {
        let replaced = mem::replace(self, Ast::Replaced);
        if let Ast::Expression(exp) = replaced {
            exp
        } else {
            panic!("Expected expression, got {:?}", replaced);
        }
    }

    fn expect_line(&mut self) -> Line<J> {
        let replaced = mem::replace(self, Ast::Replaced);
        if let Ast::Line(ell) = replaced {
            ell
        } else {
            panic!("Expected line, got {:?}", replaced);
        }
    }

    fn expect_literal(&mut self) -> (Vec<u8>, usize, Position) {
        let replaced = mem::replace(self, Ast::Replaced);
        if let Ast::Literal {
            data,
            bit_length,
            position,
        } = replaced
        {
            (data, bit_length, position)
        } else {
            panic!("Expected literal, got {:?}", replaced);
        }
    }

    fn expect_program(&mut self) -> Vec<Line<J>> {
        let replaced = mem::replace(self, Ast::Replaced);
        if let Ast::Program(lines) = replaced {
            lines
        } else {
            panic!("Expected program, got {:?}", replaced);
        }
    }

    fn expect_symbol(&mut self) -> (Arc<str>, Position) {
        let replaced = mem::replace(self, Ast::Replaced);
        if let Ast::Symbol { value, position } = replaced {
            (value, position)
        } else {
            panic!("Expected symbol, got {:?}", replaced);
        }
    }

    /// Checks that a given AST element is a type, and returns it if so
    ///
    /// Replaces the original value with a dummy, on the assumption that
    /// it lives in a vector and therefore can't be simply moved.
    fn expect_type(&mut self) -> Option<Type> {
        let replaced = mem::replace(self, Ast::Replaced);
        if let Ast::Type(ty) = replaced {
            ty
        } else {
            panic!("Expected type, got {:?}", replaced);
        }
    }
}

fn lexer_rules() -> LexerRules {
    santiago::lexer_rules!(
        // Base combinators and jets
        "DEFAULT" | "CONST"  = string "const";
        "DEFAULT" | "NULLARY" = pattern "unit|iden|witness|jet_[a-z0-9_]*";
        "DEFAULT" | "UNARY"   = pattern "injl|injr|take|drop|disconnect";
        "DEFAULT" | "BINARY"  = pattern "case|comp|pair";
        // Assertions
        "DEFAULT" | "ASSERTL" = string "assertl";
        "DEFAULT" | "ASSERTR" = string "assertr";
        "DEFAULT" | "FAIL" = string "fail";
        // Literals
        "DEFAULT" | "_" = string "_";
        "DEFAULT" | "LITERAL" = pattern r"0b[01]+|0x[0-9a-f]+";

        // Symbols (expression names). Essentially any alphanumeric that does not
        // start with a numbera and isn't a reserved symbol (i.e. one of the above)
        // patterns. Dash, underscore and dot are also allowed anywhere in a symbol.
        "DEFAULT" | "SYMBOL" = pattern r"[a-zA-Z_\-.'][0-9a-zA-Z_\-.']*";

        // Type/arrow symbols
        "DEFAULT" | "(" = string "(";
        "DEFAULT" | ")" = string ")";
        "DEFAULT" | "+" = string "+";
        "DEFAULT" | "*" = string "*";
        "DEFAULT" | "->" = string "->";
        "DEFAULT" | ":" = string ":";
        "DEFAULT" | "1" = string "1";
        "DEFAULT" | "2" = string "2";
        "DEFAULT" | "2EXP" = pattern "2\\^[1-9][0-9]*";

        // Assignment
        "DEFAULT" | ":=" = string ":=";

        // CMR and holes
        "DEFAULT" | "CMRLIT" = pattern "#[a-fA-F0-9]{64}";
        "DEFAULT" | "#{" = string "#{";
        "DEFAULT" | "}" = string "}";
        "DEFAULT" | "?" = string "?";

        // Comments (single-line comments only).
        "DEFAULT" | "LINE_COMMENT" = pattern r"--.*\n" => |lexer| lexer.skip();

        // No whitespace is significant except to separate other tokens.
        "DEFAULT" | "WS" = pattern r"\s" => |lexer| lexer.skip();
    )
}

fn grammar<J: Jet + 'static>() -> Grammar<Ast<J>> {
    santiago::grammar!(
        "program" => empty => |_| Ast::Program(vec![]);
        "program" => rules "line" "program" => |mut toks| {
            if let Some(e) = propagate_errors(&toks) { return e; }
            let line = toks[0].expect_line();
            let prog = toks[1].expect_program();

            let mut new_prog = Vec::with_capacity(1 + prog.len());
            new_prog.push(line);
            new_prog.extend(prog);
            Ast::Program(new_prog)
        };

        "line" => rules "symbol" ":" "arrow" => |mut toks| Ast::from_2(
            &mut toks,
            Ast::expect_symbol,
            Ast::expect_arrow,
            |symb, arrow| Ast::Line(Line {
                position: symb.1,
                name: symb.0,
                expression: None,
                arrow,
            })
        );
        "line" => rules "symbol" ":=" "expr" => |mut toks| Ast::from_2(
            &mut toks,
            Ast::expect_symbol,
            Ast::expect_expression,
            |symb, expr| Ast::Line(Line {
                position: symb.1,
                name: symb.0,
                expression: Some(expr),
                arrow: (None, None),
            })
        );
        "line" => rules "symbol" ":=" "expr" ":" "arrow" => |mut toks| Ast::from_3(
            &mut toks,
            Ast::expect_symbol,
            Ast::expect_expression,
            Ast::expect_arrow,
            |symb, expr, arrow| Ast::Line(Line {
                position: symb.1,
                name: symb.0,
                expression: Some(expr),
                arrow,
            }),
        );

        "arrow" => rules "type" "->" "type" => |mut toks| Ast::from_2(
            &mut toks,
            Ast::expect_type,
            Ast::expect_type,
            Ast::Arrow,
        );

        "expr" => rules "symbol" => |mut toks| {
            if let Some(e) = propagate_errors(&toks) { return e; }
            assert_eq!(toks.len(), 1);
            if let Ast::Symbol { value, position } = mem::replace(&mut toks[0], Ast::Replaced) {
                Ast::Expression(Expression::reference(value, position))
            } else {
                unreachable!("expected string, got something else")
            }
        };
        "expr" => rules "?" "symbol" => |mut toks| {
            if let Some(e) = propagate_errors(&toks) { return e; }
            assert_eq!(toks.len(), 2);
            if let Ast::Symbol { value, position } = mem::replace(&mut toks[1], Ast::Replaced) {
                Ast::Expression(Expression {
                    inner: ExprInner::Inline(node::Inner::Witness(WitnessOrHole::TypedHole(value))),
                    position,
                })
            } else {
                unreachable!("expected string, got something else")
            }
        };
        "expr" => rules "(" "expr" ")" => |toks| toks[1].clone();
        "expr" => rules "nullary" => Ast::from_combinator;
        "expr" => rules "unary" "expr" => Ast::from_combinator;
        "expr" => rules "binary" "expr" "expr" => Ast::from_combinator;

        // TODO should we allow CMRs as literals for constant words?
        "expr" => rules "const" "literal" => |mut toks| {
            if let Some(e) = propagate_errors(&toks) { return e; }
            assert_eq!(toks.len(), 2);
            let (data, bit_length, position) = toks[1].expect_literal();
            let mut iter = BitIter::from(data);

            if bit_length.count_ones() != 1 || bit_length > 1 << 32 {
                return Ast::Error(ErrorSet::single(
                    position,
                    Error::BadWordLength { bit_length },
                ));
            }
            let ty = types::Type::two_two_n(bit_length.trailing_zeros() as usize)
                .final_data()
                .unwrap();
            // unwrap ok here since literally every sequence of bits is a valid
            // value for the given type
            let value = iter.read_value(&ty).unwrap();
            Ast::Expression(Expression {
                inner: ExprInner::Inline(node::Inner::Word(Arc::new(value))),
                position,
            })
        };

        "expr" => rules "assertl" "expr" "cmr" => |mut toks| {
            if let Some(e) = propagate_errors(&toks) { return e; }
            assert_eq!(toks.len(), 3);
            let exp1 = toks[1].expect_expression();
            let cmr2 = toks[2].expect_cmr();
            Ast::Expression(Expression {
                inner: ExprInner::AssertL(Arc::new(exp1), cmr2),
                position: toks[0].expect_position(),
            })
        };
        "expr" => rules "assertr" "cmr" "expr" => |mut toks| {
            if let Some(e) = propagate_errors(&toks) { return e; }
            assert_eq!(toks.len(), 3);
            let cmr1 = toks[1].expect_cmr();
            let exp2 = toks[2].expect_expression();
            Ast::Expression(Expression {
                inner: ExprInner::AssertR(cmr1, Arc::new(exp2)),
                position: toks[0].expect_position(),
            })
        };

        "expr" => rules "fail" "literal" => |mut toks| {
            if let Some(e) = propagate_errors(&toks) { return e; }
            assert_eq!(toks.len(), 2);
            let (value, bit_length, position) = toks[1].expect_literal();
            if bit_length < 128 {
                Ast::Error(ErrorSet::single(
                    position,
                    Error::EntropyInsufficient { bit_length },
                ))
            } else if bit_length > 512 {
                Ast::Error(ErrorSet::single(
                    position,
                    Error::EntropyTooMuch { bit_length },
                ))
            } else {
                let mut entropy = [0; 64];
                entropy[..value.len()].copy_from_slice(&value[..]);
                let entropy = FailEntropy::from_byte_array(entropy);
                Ast::Expression(Expression {
                    inner: ExprInner::Inline(node::Inner::Fail(entropy)),
                    position,
                })
            }
        };

        "cmr" => rules "#{" "expr" "}" => |mut toks| {
            if let Some(e) = propagate_errors(&toks) { return e; }
            assert_eq!(toks.len(), 3);
            let exp = toks[1].expect_expression();
            Ast::Cmr(AstCmr::Expr(Arc::new(exp)))
        };
        "cmr" => rules "CMRLIT";


        "type" => rules "symbol" => |mut toks| Ast::from_1(
            &mut toks,
            Ast::expect_symbol,
            |name| {
                if name.0.as_ref() == "_" {
                    Ast::Type(None)
                } else {
                    // Type names are stored as Strings, but we normally use Arc<str>
                    // in the parser. So we need to do an extra conversion.
                    Ast::Type(Some(Type::Name(name.0.as_ref().to_owned())))
                }
            },
        );
        "type" => rules "1";
        "type" => rules "2";
        "type" => rules "2EXP";
        "type" => rules "(" "type" ")" => |mut toks| Ast::from_1(
            &mut toks[1..2],
            Ast::expect_type,
            Ast::Type,
        );
        "type" => rules "type" "+" "type" => |mut toks| Ast::from_2(
            &mut toks,
            Ast::expect_type,
            Ast::expect_type,
            |t1, t2| Ast::Type(t1.zip(t2).map(|(t1, t2)| Type::Sum(Box::new(t1), Box::new(t2)))),
        );
        "type" => rules "type" "*" "type" => |mut toks| Ast::from_2(
            &mut toks,
            Ast::expect_type,
            Ast::expect_type,
            |t1, t2| Ast::Type(t1.zip(t2).map(|(t1, t2)| Type::Product(Box::new(t1), Box::new(t2)))),
        );

        // Turn lexemes into rules
        "nullary" => lexemes "NULLARY" => Ast::from_combinator_lexeme;
        "unary" => lexemes "UNARY" => Ast::from_combinator_lexeme;
        "binary" => lexemes "BINARY" => Ast::from_combinator_lexeme;
        "const" => lexemes "CONST" => Ast::from_dummy_lexeme;
        "assertl" => lexemes "ASSERTL" => Ast::from_combinator_lexeme;
        "assertr" => lexemes "ASSERTR" => Ast::from_combinator_lexeme;
        "fail" => lexemes "FAIL" => Ast::from_combinator_lexeme;

        "literal" => lexemes "LITERAL" => Ast::from_literal_lexeme;
        "literal" => lexemes "_" => Ast::from_literal_lexeme;

        "#{" => lexemes "#{" => Ast::from_dummy_lexeme;
        "}" => lexemes "}" => Ast::from_dummy_lexeme;
        "CMRLIT" => lexemes "CMRLIT" => Ast::from_cmr_literal_lexeme;

        "symbol" => lexemes "_" => |lexemes| {
            assert_eq!(lexemes.len(), 1);
            Ast::Symbol {
                value: Arc::from(lexemes[0].raw.as_str()),
                position: (&lexemes[0].position).into(),
            }
        };
        "symbol" => lexemes "SYMBOL" => |lexemes| {
            assert_eq!(lexemes.len(), 1);
            Ast::Symbol {
                value: Arc::from(lexemes[0].raw.as_str()),
                position: (&lexemes[0].position).into(),
            }
        };

        "(" => lexemes "(" => Ast::from_dummy_lexeme;
        ")" => lexemes ")" => Ast::from_dummy_lexeme;
        "+" => lexemes "+" => Ast::from_dummy_lexeme;
        "*" => lexemes "*" => Ast::from_dummy_lexeme;
        ":" => lexemes ":" => Ast::from_dummy_lexeme;
        "->" => lexemes "->" => Ast::from_dummy_lexeme;

        "1" => lexemes "1" => Ast::from_type_lexeme;
        "2" => lexemes "2" => Ast::from_type_lexeme;
        "2EXP" => lexemes "2EXP" => Ast::from_type_lexeme;

        ":=" => lexemes ":=" => Ast::from_dummy_lexeme;

        "#" => lexemes "#" => Ast::from_dummy_lexeme;
        "?" => lexemes "?" => Ast::from_dummy_lexeme;

        // Define associativity rules for type constructors
        Associativity::Left => rules "+";
        Associativity::Left => rules "*";
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::jet::Core;

    #[test]
    fn fixed_vectors() {
        // Single line
        parse_line_vector::<Core>("a := b").unwrap();
        // Bad lex
        parse_line_vector::<Core>("?P<").unwrap_err();
        // Witness
        parse_line_vector::<Core>("U := witness").unwrap();
        // Name with type
        parse_line_vector::<Core>("U : T -> 1").unwrap();
        parse_line_vector::<Core>("U : 2 -> 1").unwrap();
        parse_line_vector::<Core>("U : 2^2 -> 1").unwrap();
        parse_line_vector::<Core>("U : 2^512 -> 1").unwrap();
        parse_line_vector::<Core>("U : (2^512) -> 1").unwrap();
        parse_line_vector::<Core>("U : (2^512 * 2^512) -> 1").unwrap();
        parse_line_vector::<Core>("U : 1 -> (2^512 * 2^512)").unwrap();
        // Witness with type and expression
        parse_line_vector::<Core>("U := witness : 1 -> 1").unwrap();
        parse_line_vector::<Core>("U := witness : _ -> 1").unwrap();
        parse_line_vector::<Core>("U := witness : 1 -> _").unwrap();
        parse_line_vector::<Core>("U := witness : _ -> _").unwrap();
        // Case with nested unit
        parse_line_vector::<Core>("ABC := case unit injl DEF").unwrap();
        // word hex
        parse_line_vector::<Core>("U := const 0xabcd").unwrap();
        // word bin
        parse_line_vector::<Core>("U := const 0b0101001011111000").unwrap();

        // asserts
        parse_line_vector::<Core>(
            "U := assertl unit #abcd1234abcd1234abcd1234abcd1234abcd1234abcd1234abcd1234abcd1234",
        )
        .unwrap();
        parse_line_vector::<Core>("U := assertl unit #{comp iden iden}").unwrap();
        parse_line_vector::<Core>(
            "U := assertr #abcd1234abcd1234abcd1234abcd1234abcd1234abcd1234abcd1234abcd1234 unit",
        )
        .unwrap();
        parse_line_vector::<Core>("U := assertr #{comp iden iden} unit").unwrap();
    }

    #[test]
    fn simple_program() {
        parse_line_vector::<Core>(
            "
            v2 := unit : B -> 1                -- 62274a89
            v1 := pair v2 v2 : B -> (1 * 1)    -- 822d5a17
        ",
        )
        .unwrap();
    }
}
