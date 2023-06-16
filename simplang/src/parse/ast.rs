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

use santiago::grammar::{Associativity, Grammar};
use santiago::lexer::{Lexeme, LexerRules};
use simplicity::bitcoin_hashes::hex::FromHex;
use simplicity::{BitIter, Cmr};
use simplicity::core::Value;
use simplicity::jet::Jet;
use simplicity::types;

use crate::parse::error::{Error, ErrorType};
use crate::Position;

/// A single non-empty line of a program, of the form x = y :: t
///
/// A program is simply a list of such lines
#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub struct Line<J: Jet> {
    /// The name of the expression being named on the line
    pub name: String,
    /// The actual expression, if present (missing for type declarations)
    pub expression: Option<Expression<J>>,
    /// The type of the expression, if given (inferred if missing)
    pub arrow: Option<(Type, Type)>,
}

/// An expression, as represented in the AST
#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub struct Expression<J: Jet> {
    pub variant: ExpressionVariant<J>,
    pub position: Position,
}

/// An expression, as represented in the AST
#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub enum ExpressionVariant<J: Jet> {
    /// A reference to another expression
    Symbol(String),
    /// The `iden` combinator
    Iden,
    /// The `unit` combinator
    Unit,
    /// The `injl` combinator
    Injl(Box<Expression<J>>),
    /// The `injr` combinator
    Injr(Box<Expression<J>>),
    /// The `take` combinator
    Take(Box<Expression<J>>),
    /// The `drop` combinator
    Drop(Box<Expression<J>>),
    /// The `comp` combinator
    Comp(Box<Expression<J>>, Box<Expression<J>>),
    /// The `case` combinator
    Case(Box<Expression<J>>, Box<Expression<J>>),
    /// The left-assert combinator (`case` with a hidden right branch)
    AssertL(Box<Expression<J>>, Cmr),
    /// The right-assert combinator (`case` with a hidden left branch)
    AssertR(Box<Expression<J>>, Cmr),
    /// The `pair` combinator
    Pair(Box<Expression<J>>, Box<Expression<J>>),
    /// The `disconnect` combinator
    Disconnect(Box<Expression<J>>, Box<Expression<J>>),
    /// The `witness` combinator -- witness data is not populated!
    Witness,
    /// The `fail` combinator
    Fail(Cmr, Cmr),
    /// A jet
    Jet(J),
    /// Constant-word jet
    Word {
        data: Value,
        bit_length: usize,
    },
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
    /// An exponential type 2^n where n is one of 2, 4, 8, 16, ..., 1024.
    TwoN(u32),
}

impl Type {
    /// Convert to a Simplicity type
    pub fn reify<J: Jet>(self, ctx: &simplicity::Context<J>) -> types::Type {
        match self {
            Type::Name(s) => types::Type::free(s),
            Type::One => types::Type::unit(),
            Type::Two => types::Type::sum(types::Type::unit(), types::Type::unit()),
            Type::Product(left, right) => types::Type::product(left.reify(ctx), right.reify(ctx)),
            Type::Sum(left, right) => types::Type::sum(left.reify(ctx), right.reify(ctx)),
            Type::TwoN(n) => ctx.nth_power_of_2(n as usize), // cast OK as we are only using tiny numbers
        }
    }
}

/// Takes a program as a string and parses it into an AST (actually, a vector
/// of lines, each of which is individually an AST)
pub fn parse<J: Jet>(input: &str) -> Result<Vec<Line<J>>, Vec<Error>> {
    let lexer_rules = lexer_rules();
    let grammar = grammar::<J>();

    let lexemes = match santiago::lexer::lex(&lexer_rules, &input) {
        Ok(lexemes) => lexemes,
        Err(err) => return Err(vec![err.into()]),
    };
    match santiago::parser::parse(&grammar, &lexemes) {
        Ok(trees) => {
            assert_eq!(trees.len(), 1, "ambiguous parse (this is a bug)");
            match trees[0].as_abstract_syntax_tree() {
                Ast::Program(lines) => Ok(lines),
                Ast::Error(errs) => Err(errs),
                x => unreachable!("Parsed program into non-program non-error {:?}; this is a bug.", x),
            }
        },
        Err(err) => Err(vec![err.into()]),
    }
}

/// Check a list of AST elements for errors; if any are errors, combine them and return the result
fn propagate_errors<J: Jet>(ast: &[Ast<J>]) -> Option<Ast<J>> {
    let mut e = vec![];
    for elem in ast {
        if let Ast::Error(ref errs) = elem {
            e.extend(errs.iter().cloned());
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
    /// A type->type arrow
    Arrow(Type, Type),
    /// An error occurred during parsing
    Error(Vec<Error>),
    /// An expression
    Expression(Expression<J>),
    /// A jet
    Jet {
        jet: J,
        position: Position,
    },
    /// A full expression line
    Line(Line<J>),
    /// A hex or binary literal
    Literal {
        data: Vec<u8>,
        bit_length: usize,
        /// for error reporting
        lexeme: std::rc::Rc<Lexeme>,
    },
    /// A numeric value (used only in type expressions)
    Number {
        value: u32,
        position: Position,
    },
    /// The top-level program
    Program(Vec<Line<J>>),
    /// A symbol
    String {
        value: String,
        position: Position,
    },
    /// A type
    Type(Type),
    /// Dummy value used internally in the parser when building the tree.
    ///
    /// Any parse objects which have no information in themselves (e.g. the
    /// plus or star symbols) but which are just used to shape the parse
    /// tree, get mapped to this value and then discarded. 
    Dummy {
        position: Position,
    },
    /// Dummy value used when manipulating the tree in-place, to replace
    /// data that we move out of the tree
    Replaced,
}

impl<J: Jet> Ast<J> {
    /// Creates an `Ast` from nothing
    fn from_0(toks: &[Self], ast: Self) -> Self {
        if let Some(e) = propagate_errors(&toks) { return e; }
        assert!(toks.is_empty());
        ast
    }

    /// Creates an `Ast` from a single sub-AST
    fn from_1<T, F1, F>(
        toks: &mut [Self],
        convert: F1,
        unconvert: F,
    ) -> Self
    where
        F1: FnOnce(&mut Self) -> T,
        F: FnOnce(T) -> Self,
    {
        if let Some(e) = propagate_errors(&toks) { return e; }
        assert_eq!(toks.len(), 1);
        unconvert(convert(&mut toks[0]))
    }

    /// Creates an `Ast` from two sub-`Ast`s with a dummy in between
    fn from_2<T, U, F1, F2, F>(
        toks: &mut [Self],
        convert1: F1,
        convert2: F2,
        unconvert: F,
    ) -> Self
    where
        F1: FnOnce(&mut Self) -> T,
        F2: FnOnce(&mut Self) -> U,
        F: FnOnce(T, U) -> Self,
    {
        if let Some(e) = propagate_errors(&toks) { return e; }
        assert_eq!(toks.len(), 3);
        toks[1].expect_dummy();
        unconvert(
            convert1(&mut toks[0]),
            convert2(&mut toks[2]),
        )
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
        if let Some(e) = propagate_errors(&toks) { return e; }
        assert_eq!(toks.len(), 5);
        toks[1].expect_dummy();
        toks[3].expect_dummy();
        unconvert(
            convert1(&mut toks[0]),
            convert2(&mut toks[2]),
            convert3(&mut toks[4]),
        )
    }

    /// Creates an AST from a combinator with no arguments
    fn from_comb_0(toks: &[Self], comb: ExpressionVariant<J>) -> Self {
        if let Some(e) = propagate_errors(&toks) { return e; }
        assert_eq!(toks.len(), 1);
        let position = toks[0].expect_dummy();
        Ast::Expression(Expression { variant: comb, position })
    }

    /// Creates an AST from a combinator with one argument
    fn from_comb_1<F: FnOnce(Box<Expression<J>>) -> ExpressionVariant<J>>(
        toks: &mut [Self],
        comb: F,
    ) -> Self {
        if let Some(e) = propagate_errors(&toks) { return e; }
        assert_eq!(toks.len(), 2);
        let position = toks[0].expect_dummy();
        let sub = Box::new(toks[1].expect_expression());
        Ast::Expression(Expression { variant: comb(sub), position })
    }

    /// Creates an AST from a combinator with two arguments
    fn from_comb_2<F: FnOnce(Box<Expression<J>>, Box<Expression<J>>) -> ExpressionVariant<J>>(
        toks: &mut [Self],
        comb: F,
    ) -> Self {
        if let Some(e) = propagate_errors(&toks) { return e; }
        assert_eq!(toks.len(), 3);
        let position = toks[0].expect_dummy();
        let left = Box::new(toks[1].expect_expression());
        let right = Box::new(toks[2].expect_expression());
        Ast::Expression(Expression { variant: comb(left, right), position })
    }

    /// Creates an AST from a single lexeme
    fn from_lexeme(lexemes: &[&std::rc::Rc<Lexeme>]) -> Self {
        assert_eq!(lexemes.len(), 1);
        Ast::Dummy { position: (&lexemes[0].position).into() }
    }

    fn expect_arrow(&mut self) -> (Type, Type) {
        let replaced = mem::replace(self, Ast::Replaced);
        if let Ast::Arrow(a, b) = replaced {
            (a, b)
        } else {
            panic!("Expected arrow, got {:?}", self);
        }
    }

    /// Checks that a given AST element is a dummy value
    fn expect_dummy(&self) -> Position {
        if let Ast::Dummy { position } = self {
            *position
        } else {
            panic!("Expected dummy, got {:?}", self);
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

    /// Checks that a given AST element is a dummy value
    fn expect_jet(&mut self) -> (J, Position) {
        let replaced = mem::replace(self, Ast::Replaced);
        if let Ast::Jet {jet, position } = replaced {
            (jet, position)
        } else {
            panic!("Expected jet, got {:?}", replaced);
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

    fn expect_literal(&mut self) -> (Vec<u8>, usize, std::rc::Rc<Lexeme>) {
        let replaced = mem::replace(self, Ast::Replaced);
        if let Ast::Literal { data, bit_length, lexeme } = replaced {
            (data, bit_length, lexeme)
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

    fn expect_string(&mut self) -> String {
        let replaced = mem::replace(self, Ast::Replaced);
        if let Ast::String { value, .. } = replaced {
            value
        } else {
            panic!("Expected symbol, got {:?}", replaced);
        }
    }

    /// Checks that a given AST element is a type, and returns it if so
    ///
    /// Replaces the original value with a dummy, on the assumption that
    /// it lives in a vector and therefore can't be simply moved.
    fn expect_type(&mut self) -> Type {
        let replaced = mem::replace(self, Ast::Replaced);
        if let Ast::Type(ty) = replaced {
            ty
        } else {
            panic!("Expected type, got {:?}", replaced);
        }
    }

    fn parse_cmr(&mut self) -> Result<Cmr, Ast<J>> {
        let replaced = mem::replace(self, Ast::Replaced);
        if let Ast::String { value, position } = replaced {
            Cmr::from_str(&value).map_err(|err| Ast::Error(vec![Error {
                ty: ErrorType::BadCmr(err),
                position,
            }]))
        } else {
            panic!("Expected type, got {:?}", replaced);
        }
     }
}

fn lexer_rules() -> LexerRules {
    santiago::lexer_rules!(
        // base combinators
        "DEFAULT" | "UNIT" = string "unit";
        "DEFAULT" | "IDEN" = string "iden";
        "DEFAULT" | "COMP" = string "comp";
        "DEFAULT" | "INJL" = string "injl";
        "DEFAULT" | "INJR" = string "injr";
        "DEFAULT" | "CASE" = string "case";
        "DEFAULT" | "TAKE" = string "take";
        "DEFAULT" | "DROP" = string "drop";
        "DEFAULT" | "PAIR" = string "pair";
        // witness and asserts
        "DEFAULT" | "DISCONNECT" = string "disconnect";
        "DEFAULT" | "WITNESS" = string "witness";
        "DEFAULT" | "ASSERTL" = string "assertl";
        "DEFAULT" | "ASSERTR" = string "assertr";
        "DEFAULT" | "FAIL" = string "fail";
        // Constant words (other jets defined below)
        "DEFAULT" | "WORD" = string "word_jet";
        // Literals
        "DEFAULT" | "BINLIT" = pattern r"0b[01]+";
        "DEFAULT" | "HEXLIT" = pattern r"0x[0-9a-f]+";
        // Type constructors
        "DEFAULT" | "(" = string "(";
        "DEFAULT" | ")" = string ")";
        "DEFAULT" | "+" = string "+";
        "DEFAULT" | "*" = string "*";
        "DEFAULT" | "^" = string "^";
        "DEFAULT" | "->" = string "->";
        "DEFAULT" | "::" = string "::";
        // Numbers (used in type constructors)
        "DEFAULT" | "NUMBER" = pattern "[1-9][0-9]*";
        // Assignment
        "DEFAULT" | "=" = string "=";

        // Comments (single-line comments and also (potentially-nested) C-style comments)
        // In all cases we skip over comments entirely in the lexer.
        "DEFAULT" | "LINE_COMMENT" = pattern r"(--|#|//).*" => |lexer| lexer.skip();
        "DEFAULT" | "/*" = string "/*" => |lexer| {
            lexer.push_state("C_COMMENT");
            lexer.skip()
        };
        "C_COMMENT" | "/*" = string "/*" => |lexer| {
            lexer.push_state("C_COMMENT");
            lexer.skip()
        };
        "C_COMMENT" | "*/" = string "*/" => |lexer| {
            lexer.pop_state();
            lexer.skip()
        };
        "C_COMMENT" | "C_COMMENT_STAR" = string "*" => |lexer| lexer.skip();
        "C_COMMENT" | "C_COMMENT_SLASH" = string "/" => |lexer| lexer.skip();
        "C_COMMENT" | "C_COMMENT_NON_STAR_SLASH" = pattern r"[^*/]+" => |lexer| lexer.skip();

        // Jets are alphanumerics which start with the prefix jet_
        "DEFAULT" | "JET" = pattern r"jet_[a-zA-Z0-9_]+";
        // Basically everything else is fair game as a symbol (which is a placeholder
        // for an expression or type). That is, any string is permissible except one
        // that contains whitespace or any of < > + * # " ^ ( ).
        "DEFAULT" | "SYMBOL" = pattern r#"[^\s<>+*"^()]+"#;

        // Skip all other whitespace (newlines are handled before the parser is even
        // invoked).
        "DEFAULT" | "NEWLINE" = pattern r"\n+";
        "DEFAULT" | "WS" = pattern r"\s" => |lexer| lexer.skip();
    )
}

fn grammar<J: Jet>() -> Grammar<Ast<J>> {
    let ctx = simplicity::Context::<J>::new();
    santiago::grammar!(
        "program" => empty => |toks| Ast::from_0(&toks, Ast::Program(vec![]));
        "program" => rules "line" => |mut toks| Ast::from_1(
            &mut toks,
            Ast::expect_line,
            |line| Ast::Program(vec![line]),
        );
        "program" => rules "NEWLINE" "program" => |mut toks| Ast::from_1(
            &mut toks[1..],
            Ast::expect_program,
            |prog| Ast::Program(prog),
        );
        "program" => rules "line" "NEWLINE" "program" => |mut toks| Ast::from_2(
            &mut toks,
            Ast::expect_line,
            Ast::expect_program,
            |line, tail| {
                let mut prog = Vec::with_capacity(1 + tail.len());
                prog.push(line);
                prog.extend(tail);
                Ast::Program(prog)
            }
        );
        "line" => rules "symbol" "::" "arrow" => |mut toks| Ast::from_2(
            &mut toks,
            Ast::expect_string,
            Ast::expect_arrow,
            |symb, arrow| Ast::Line(Line {
                name: symb,
                expression: None,
                arrow: Some(arrow),
            })
        );
        "line" => rules "symbol" "=" "expr" => |mut toks| Ast::from_2(
            &mut toks,
            Ast::expect_string,
            Ast::expect_expression,
            |symb, expr| Ast::Line(Line {
                name: symb,
                expression: Some(expr),
                arrow: None,
            })
        );
        "line" => rules "symbol" "=" "expr" "::" "arrow" => |mut toks| Ast::from_3(
            &mut toks,
            Ast::expect_string,
            Ast::expect_expression,
            Ast::expect_arrow,
            |symb, expr, arrow| Ast::Line(Line {
                name: symb,
                expression: Some(expr),
                arrow: Some(arrow),
            }),
        );
        "arrow" => rules "type" "->" "type" => |mut toks| Ast::from_2(
            &mut toks,
            Ast::expect_type,
            Ast::expect_type,
            Ast::Arrow,
        );

        "type" => rules "symbol" => |mut toks| Ast::from_1(
            &mut toks,
            Ast::expect_string,
            |name| Ast::Type(Type::Name(name)),
        );
        "type" => rules "NUMBER" => |toks| {
            assert_eq!(toks.len(), 1);
            if let Ast::Number { value, position } = toks[0] {
                match value {
                    1 => Ast::Type(Type::One),
                    2 => Ast::Type(Type::Two),
                    x => Ast::Error(vec![Error::bad_type_number(x, position)]),
                }
            } else {
                unreachable!("expected number, got something else")
            }
        };
        "type" => rules "NUMBER" "^" "NUMBER" => |mut toks| {
            assert_eq!(toks.len(), 3);
            let base = mem::replace(&mut toks[0], Ast::Replaced);
            let exp = mem::replace(&mut toks[2], Ast::Replaced);
            toks[1].expect_dummy();
            if let (Ast::Number { value: base, position }, Ast::Number { value: exp, .. }) = (base, exp) {
                match (base, exp) {
                    (2, 0) => Ast::Type(Type::One),
                    (2, 1) => Ast::Type(Type::Two),
                    (2, 2) => Ast::Type(Type::TwoN(1)),
                    (2, 4) => Ast::Type(Type::TwoN(2)),
                    (2, 8) => Ast::Type(Type::TwoN(3)),
                    (2, 16) => Ast::Type(Type::TwoN(4)),
                    (2, 32) => Ast::Type(Type::TwoN(5)),
                    (2, 64) => Ast::Type(Type::TwoN(6)),
                    (2, 128) => Ast::Type(Type::TwoN(7)),
                    (2, 256) => Ast::Type(Type::TwoN(8)),
                    (2, 512) => Ast::Type(Type::TwoN(9)),
                    (x, y) => Ast::Error(vec![Error::bad_exp_type_number(x, y, position)]),
                }
            } else {
                unreachable!("expected number, got something else")
            }
        };
        "type" => rules "(" "type" ")" => |mut toks| Ast::from_1(
            &mut toks[1..2],
            Ast::expect_type,
            Ast::Type,
        );
        "type" => rules "type" "+" "type" => |mut toks| Ast::from_2(
            &mut toks,
            Ast::expect_type,
            Ast::expect_type,
            |t1, t2| Ast::Type(Type::Sum(Box::new(t1), Box::new(t2))),
        );
        "type" => rules "type" "*" "type" => |mut toks| Ast::from_2(
            &mut toks,
            Ast::expect_type,
            Ast::expect_type,
            |t1, t2| Ast::Type(Type::Product(Box::new(t1), Box::new(t2))),
        );

        "expr" => rules "symbol" => |mut toks| {
            if let Some(e) = propagate_errors(&toks) { return e; }
            assert_eq!(toks.len(), 1);
            if let Ast::String { value, position } = mem::replace(&mut toks[0], Ast::Replaced) {
                Ast::Expression(Expression {
                    variant: ExpressionVariant::Symbol(value),
                    position,
                })
            } else {
                unreachable!("expected string, got something else")
            }
        };
        "expr" => rules "(" "expr" ")" => |mut toks| Ast::from_1(
            &mut toks[1..2],
            Ast::expect_expression,
            Ast::Expression,
        );
        "expr" => rules "UNIT" => |toks| Ast::from_comb_0(&toks, ExpressionVariant::Unit);
        "expr" => rules "IDEN" => |toks| Ast::from_comb_0(&toks, ExpressionVariant::Iden);
        "expr" => rules "COMP" "expr" "expr" => |mut toks| Ast::from_comb_2(&mut toks, ExpressionVariant::Comp);
        "expr" => rules "INJL" "expr" => |mut toks| Ast::from_comb_1(&mut toks, ExpressionVariant::Injl);
        "expr" => rules "INJR" "expr" => |mut toks| Ast::from_comb_1(&mut toks, ExpressionVariant::Injr);
        "expr" => rules "CASE" "expr" "expr" => |mut toks| Ast::from_comb_2(&mut toks, ExpressionVariant::Case);
        "expr" => rules "TAKE" "expr" => |mut toks| Ast::from_comb_1(&mut toks, ExpressionVariant::Take);
        "expr" => rules "DROP" "expr" => |mut toks| Ast::from_comb_1(&mut toks, ExpressionVariant::Drop);
        "expr" => rules "PAIR" "expr" "expr" => |mut toks| Ast::from_comb_2(&mut toks, ExpressionVariant::Pair);

        "expr" => rules "ASSERTL" "expr" "symbol" => |mut toks| {
            assert_eq!(toks.len(), 3);
            let cmr = match toks[2].parse_cmr() {
                Ok(cmr) => cmr,
                Err(e) => return e,
            };
            Ast::from_comb_1(&mut toks[0..2], |sub| ExpressionVariant::AssertL(sub, cmr))
        };
        "expr" => rules "ASSERTR" "symbol" "expr" => |mut toks| {
            assert_eq!(toks.len(), 3);
            toks.swap(1, 2);
            let cmr = match toks[2].parse_cmr() {
                Ok(cmr) => cmr,
                Err(e) => return e,
            };
            Ast::from_comb_1(&mut toks[0..2], |sub| ExpressionVariant::AssertR(sub, cmr))
        };

        "expr" => rules "WITNESS" => |mut toks| Ast::from_comb_0(&mut toks, ExpressionVariant::Witness);
        "expr" => rules "DISCONNECT" "expr" "expr" => |mut toks| Ast::from_comb_2(&mut toks, ExpressionVariant::Disconnect);

        "expr" => rules "FAIL" "symbol" "symbol" => |mut toks| {
            if let Some(e) = propagate_errors(&toks) { return e; }
            assert_eq!(toks.len(), 3);
            let position = toks[0].expect_dummy();
            let cmr1 = match toks[1].parse_cmr() {
                Ok(cmr) => cmr,
                Err(e) => return e,
            };
            let cmr2 = match toks[2].parse_cmr() {
                Ok(cmr) => cmr,
                Err(e) => return e,
            };
            Ast::Expression(Expression {
                variant: ExpressionVariant::Fail(cmr1, cmr2),
                position,
            })
        };

        "LITERAL" => rules "HEXLIT";
        "LITERAL" => rules "BINLIT";
        // this is a move closure since it consumes `ctx`
        "expr" => rules "WORD" "LITERAL" => move |mut toks| {
            if let Some(e) = propagate_errors(&toks) { return e; }
            assert_eq!(toks.len(), 2);
            let position = toks[0].expect_dummy();
            let (data, bit_length, lexeme) = toks[1].expect_literal();
            let mut iter = BitIter::from(data).take(bit_length);

            if bit_length.count_ones() != 1 || bit_length > 1 << 32 {
                return Ast::Error(vec![Error::bad_word_length(
                    lexeme.raw.clone(),
                    (&lexeme.position).into()
                )]);
            }
            let ty = ctx.nth_power_of_2(bit_length.trailing_zeros() as usize)
                .final_data()
                .unwrap();
            // unwrap ok here since literally every sequence of bits is a valid
            // value for the given type
            let value = simplicity::decode_value(&ty, &mut iter).unwrap();
            Ast::Expression(Expression {
                variant: ExpressionVariant::Word { data: value, bit_length },
                position,
            })
        };

        "expr" => rules "JET" => |mut toks| {
            if let Some(e) = propagate_errors(&toks) { return e; }
            assert_eq!(toks.len(), 1);
            let (jet, position) = toks[0].expect_jet();
            Ast::Expression(Expression {
                variant: ExpressionVariant::Jet(jet),
                position
            })
        };

        // Map rules to their corresponding Lexemes
        "UNIT" => lexemes "UNIT" => Ast::from_lexeme;
        "IDEN" => lexemes "IDEN" => Ast::from_lexeme;
        "COMP" => lexemes "COMP" => Ast::from_lexeme;
        "INJL" => lexemes "INJL" => Ast::from_lexeme;
        "INJR" => lexemes "INJR" => Ast::from_lexeme;
        "CASE" => lexemes "CASE" => Ast::from_lexeme;
        "TAKE" => lexemes "TAKE" => Ast::from_lexeme;
        "DROP" => lexemes "DROP" => Ast::from_lexeme;
        "PAIR" => lexemes "PAIR" => Ast::from_lexeme;
        "ASSERTL" => lexemes "ASSERTL" => Ast::from_lexeme;
        "ASSERTR" => lexemes "ASSERTR" => Ast::from_lexeme;
        "WITNESS" => lexemes "WITNESS" => Ast::from_lexeme;
        "DISCONNECT" => lexemes "DISCONNECT" => Ast::from_lexeme;
        "FAIL" => lexemes "FAIL" => Ast::from_lexeme;
        "JET" => lexemes "JET" => |lexemes| {
            assert_eq!(lexemes.len(), 1);
            match J::from_str(&lexemes[0].raw[4..]) {
                Ok(jet) => Ast::Jet {
                    jet,
                    position: (&lexemes[0].position).into(),
                },
                Err(..) => Ast::Error(vec![Error::unknown_jet(
                    lexemes[0].raw.clone(),
                    (&lexemes[0].position).into(),
                )]),
            }
        };
        "WORD" => lexemes "WORD" => Ast::from_lexeme;
        "(" => lexemes "(" => Ast::from_lexeme;
        ")" => lexemes ")" => Ast::from_lexeme;
        "+" => lexemes "+" => Ast::from_lexeme;
        "*" => lexemes "*" => Ast::from_lexeme;
        "^" => lexemes "^" => Ast::from_lexeme;
        "::" => lexemes "::" => Ast::from_lexeme;
        "->" => lexemes "->" => Ast::from_lexeme;
        "BINLIT" => lexemes "BINLIT" => |lexemes| {
            assert_eq!(lexemes.len(), 1);
            let mut data = vec![];
            let bit_length = lexemes[0].raw.len() - 2;
            let mut x = 0;
            for (n, ch) in lexemes[0].raw.chars().skip(2).enumerate() {
                match ch {
                    '0' => {},
                    '1' => x |= 1 << (7 - (n % 8)),
                    _ => unreachable!(),
                }
                if n % 8 == 7 {
                    data.push(x);
                    x = 0;
                }
            }
            Ast::Literal { data, bit_length, lexeme: lexemes[0].clone() }
        };
        "HEXLIT" => lexemes "HEXLIT" => |lexemes| {
            assert_eq!(lexemes.len(), 1);
            // FIXME we could be much more efficient about odd-length strings here.
            let mut string = lexemes[0].raw[2..].to_string();
            let bit_length = string.len() * 4;
            if string.len() % 2 == 1 {
                string.push('0');
            }
            Ast::Literal {
                data: Vec::from_hex(&string).unwrap(),
                bit_length,
                lexeme: lexemes[0].clone(),
            }
        };
        "NUMBER" => lexemes "NUMBER" => |lexemes| {
            assert_eq!(lexemes.len(), 1);
            let n = str::parse(&lexemes[0].raw).unwrap();
            Ast::Number {
                value: n,
                position: (&lexemes[0].position).into(),
            }
        };
        "=" => lexemes "=" => Ast::from_lexeme;
        "NEWLINE" => lexemes "NEWLINE" => Ast::from_lexeme;
        "symbol" => lexemes "SYMBOL" => |lexemes| {
            assert_eq!(lexemes.len(), 1);
            Ast::String {
                value: lexemes[0].raw.clone(),
                position: (&lexemes[0].position).into(),
            }
        };

        // Define associativity rules for type constructors
        Associativity::Left => rules "*" "+";
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    use simplicity::jet::Elements;

    #[test]
    fn fixed_vectors() {
        // Single line
        parse::<Elements>("a = b").unwrap();
        // Bad lex
        parse::<Elements>("?P<").unwrap_err();
        // Witness
        parse::<Elements>("U = witness").unwrap();
        // Case with nested unit
        parse::<Elements>("ABC = case unit injl DEF").unwrap();
        // word hex
        parse::<Elements>("U = word_jet 0xabcd").unwrap();
        // word bin
        parse::<Elements>("U = word_jet 0b0101001011111000").unwrap();
    }

    #[test]
    fn simple_program() {
        parse::<Elements>("
            v2 = unit :: B -> 1                -- 62274a89
            v1 = pair v2 v2 :: B -> (1 * 1)    -- 822d5a17
        ").unwrap();
    }
}

