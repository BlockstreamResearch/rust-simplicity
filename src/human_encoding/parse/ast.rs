// SPDX-License-Identifier: CC0-1.0

//! Parsing

use std::str;
use std::sync::Arc;

use logos::{Lexer, Logos};

use crate::human_encoding::{Error, ErrorSet, Position, WitnessOrHole};
use crate::jet::Jet;
use crate::value::Word;
use crate::{node, types};
use crate::{BitIter, FailEntropy};

/// A single non-empty line of a program, of the form x = y :: t
///
/// A program is simply a list of such lines
#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub struct Line {
    /// Position of the first character of the line.
    pub position: Position,
    /// The name of the expression being named on the line.
    pub name: Arc<str>,
    /// The actual expression, if present (missing for type declarations).
    pub expression: Option<Expression>,
    /// The type of the expression, if given (inferred if missing).
    pub arrow: (Option<Type>, Option<Type>),
}

/// An expression, as represented in the AST
#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub struct Expression {
    pub inner: ExprInner,
    pub position: Position,
}

impl Expression {
    fn reference(name: Arc<str>, position: Position) -> Self {
        Expression {
            inner: ExprInner::Reference(name),
            position,
        }
    }
}

/// An expression, as represented in the AST
#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub enum ExprInner {
    /// A reference to another expression
    Reference(Arc<str>),
    /// A left assertion (referring to the CMR of an expression on the right)
    AssertL(Arc<Expression>, AstCmr),
    /// A right assertion (referring to the CMR of an expression on the left)
    AssertR(AstCmr, Arc<Expression>),
    /// An inline expression
    Inline(node::Inner<Arc<Expression>, Arc<Expression>, WitnessOrHole>),
}

/// A CMR, as represented in the AST
#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub enum AstCmr {
    Expr(Arc<Expression>),
    Literal,
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
    pub fn reify<'brand>(self, ctx: &types::Context<'brand>) -> types::Type<'brand> {
        match self {
            Type::Name(s) => types::Type::free(ctx, s),
            Type::One => types::Type::unit(ctx),
            Type::Two => {
                let unit_ty = types::Type::unit(ctx);
                types::Type::sum(ctx, unit_ty.shallow_clone(), unit_ty)
            }
            Type::Product(left, right) => {
                let left = left.reify(ctx);
                let right = right.reify(ctx);
                types::Type::product(ctx, left, right)
            }
            Type::Sum(left, right) => {
                let left = left.reify(ctx);
                let right = right.reify(ctx);
                types::Type::sum(ctx, left, right)
            }
            Type::TwoTwoN(n) => types::Type::two_two_n(ctx, n as usize), // cast OK as we are only using tiny numbers
        }
    }
}

/// Token type produced by the logos lexer.
#[derive(Logos, Debug, Clone, PartialEq)]
#[logos(skip r"[ \t\r\n]+")] // skip whitespace
#[logos(skip r"--[^\n]*")] // skip line comments
enum Token {
    // Punctuatiions
    #[token(":=")]
    Assign,
    #[token("->")]
    Arrow,
    #[token("#{")]
    HashBrace,
    #[token("(")]
    LParen,
    #[token(")")]
    RParen,
    #[token("+")]
    Plus,
    #[token("*")]
    Star,
    #[token(":")]
    Colon,
    #[token("}")]
    RBrace,
    #[token("?")]
    Question,

    // Keywords
    #[token("const")]
    Const,
    #[token("assertl")]
    AssertL,
    #[token("assertr")]
    AssertR,
    #[token("fail")]
    Fail,
    #[token("disconnect")]
    Disconnect,
    #[token("case")]
    Case,
    #[token("comp")]
    Comp,
    #[token("pair")]
    Pair,
    #[token("injl")]
    InjL,
    #[token("injr")]
    InjR,
    #[token("take")]
    Take,
    #[token("drop")]
    Drop,
    #[token("unit")]
    Unit,
    #[token("iden")]
    Iden,
    #[token("witness")]
    Witness,

    // Jet names
    #[regex(r"jet_[a-z0-9_]+", |lex| lex.slice().to_owned())]
    Jet(String),

    // Literals
    #[token("_")]
    Underscore,
    #[regex(r"0b[01]+", |lex| lex.slice().to_owned())]
    BinLiteral(String),
    #[regex(r"0x[0-9a-f]+", |lex| lex.slice().to_owned())]
    HexLiteral(String),

    // CMR literal
    #[regex(r"#[a-fA-F0-9]{64}", |lex| lex.slice().to_owned())]
    CmrLiteral(String),

    // Types
    #[token("1")]
    One,
    #[token("2")]
    Two,
    #[regex(r"2\^[1-9][0-9]*", |lex| lex.slice().to_owned())]
    TwoExp(String),

    // Symbols
    #[regex(r"[a-zA-Z_\-.'][0-9a-zA-Z_\-.']*", priority = 1, callback = |lex| lex.slice().to_owned())]
    Symbol(String),
}

/// A token together with its source position
#[derive(Debug, Clone)]
struct Spanned {
    token: Token,
    position: Position,
}

/// Lex the entire input into a vector of spanned tokens
fn lex_all(input: &str) -> Result<Vec<Spanned>, ErrorSet> {
    let mut lexer: Lexer<'_, Token> = Token::lexer(input);
    let mut tokens = Vec::new();
    while let Some(result) = lexer.next() {
        let span = lexer.span();
        // Compute line-column position
        let position = offset_to_position(input, span.start);
        match result {
            Ok(token) => tokens.push(Spanned { token, position }),
            Err(()) => {
                return Err(ErrorSet::single(
                    position,
                    Error::LexFailed(format!(
                        "unexpected character `{}`",
                        &input[span.start..span.end]
                    )),
                ));
            }
        }
    }
    Ok(tokens)
}

/// Convert a byte offset into line-column
fn offset_to_position(input: &str, offset: usize) -> Position {
    let mut line: usize = 1;
    let mut col: usize = 1;
    for (i, ch) in input.char_indices() {
        if i >= offset {
            break;
        }
        if ch == '\n' {
            line += 1;
            col = 1;
        } else {
            col += 1;
        }
    }
    Position::new(line, col)
}

struct Parser {
    tokens: Vec<Spanned>,
    pos: usize,
}

impl Parser {
    fn new(tokens: Vec<Spanned>) -> Self {
        Parser { tokens, pos: 0 }
    }

    fn peek(&self) -> Option<&Token> {
        self.tokens.get(self.pos).map(|s| &s.token)
    }

    fn current_position(&self) -> Position {
        self.tokens
            .get(self.pos)
            .map(|s| s.position)
            .unwrap_or_default()
    }

    /// Advance and return the consumed spanned token
    fn advance(&mut self) -> &Spanned {
        let s = &self.tokens[self.pos];
        self.pos += 1;
        s
    }

    /// Consume a token if it matches, returning true on success
    fn eat(&mut self, expected: &Token) -> bool {
        if self.peek() != Some(expected) {
            return false;
        }
        self.pos += 1;
        true
    }

    /// Consume a token that must match, or return an error
    fn expect(&mut self, expected: &Token) -> Result<Position, ErrorSet> {
        if self.peek() != Some(expected) {
            return Err(ErrorSet::single(
                self.current_position(),
                Error::ParseFailed(self.peek_raw_description()),
            ));
        }
        let pos = self.current_position();
        self.pos += 1;
        Ok(pos)
    }

    /// Whether we are at the end of input
    fn at_end(&self) -> bool {
        self.pos >= self.tokens.len()
    }

    /// Human-readable description of the current token for error messages
    fn peek_raw_description(&self) -> Option<String> {
        self.tokens.get(self.pos).map(|s| match &s.token {
            Token::Assign => ":=".to_owned(),
            Token::Arrow => "->".to_owned(),
            Token::HashBrace => "#{".to_owned(),
            Token::LParen => "(".to_owned(),
            Token::RParen => ")".to_owned(),
            Token::Plus => "+".to_owned(),
            Token::Star => "*".to_owned(),
            Token::Colon => ":".to_owned(),
            Token::RBrace => "}".to_owned(),
            Token::Question => "?".to_owned(),
            Token::Const => "const".to_owned(),
            Token::AssertL => "assertl".to_owned(),
            Token::AssertR => "assertr".to_owned(),
            Token::Fail => "fail".to_owned(),
            Token::Disconnect => "disconnect".to_owned(),
            Token::Case => "case".to_owned(),
            Token::Comp => "comp".to_owned(),
            Token::Pair => "pair".to_owned(),
            Token::InjL => "injl".to_owned(),
            Token::InjR => "injr".to_owned(),
            Token::Take => "take".to_owned(),
            Token::Drop => "drop".to_owned(),
            Token::Unit => "unit".to_owned(),
            Token::Iden => "iden".to_owned(),
            Token::Witness => "witness".to_owned(),
            Token::Jet(ref s) => s.clone(),
            Token::Underscore => "_".to_owned(),
            Token::BinLiteral(ref s) | Token::HexLiteral(ref s) => s.clone(),
            Token::CmrLiteral(ref s) => s.clone(),
            Token::One => "1".to_owned(),
            Token::Two => "2".to_owned(),
            Token::TwoExp(ref s) => s.clone(),
            Token::Symbol(ref s) => s.clone(),
        })
    }
}

/// Takes a program as a string and parses it into an AST
pub fn parse_line_vector<J: Jet + 'static>(input: &str) -> Result<Vec<Line>, ErrorSet> {
    let tokens = lex_all(input)?;
    let mut parser = Parser::new(tokens);
    let mut lines = Vec::new();
    while !parser.at_end() {
        lines.push(parse_line::<J>(&mut parser)?);
    }
    Ok(lines)
}

/// Parse a line
fn parse_line<J: Jet + 'static>(p: &mut Parser) -> Result<Line, ErrorSet> {
    let (name, position) = parse_symbol_value(p)?;

    if p.eat(&Token::Assign) {
        // symbol ":=" expr  (optionally followed by ":" arrow)
        let expr = parse_expr::<J>(p)?;
        let arrow = if p.eat(&Token::Colon) {
            parse_arrow(p)?
        } else {
            (None, None)
        };
        return Ok(Line {
            position,
            name,
            expression: Some(expr),
            arrow,
        });
    }

    if p.eat(&Token::Colon) {
        // symbol ":" arrow
        let arrow = parse_arrow(p)?;
        return Ok(Line {
            position,
            name,
            expression: None,
            arrow,
        });
    }

    Err(ErrorSet::single(
        p.current_position(),
        Error::ParseFailed(p.peek_raw_description()),
    ))
}

/// Parse an arrow (type -> type)
fn parse_arrow(p: &mut Parser) -> Result<(Option<Type>, Option<Type>), ErrorSet> {
    let src = parse_type(p)?;
    p.expect(&Token::Arrow)?;
    let tgt = parse_type(p)?;
    Ok((src, tgt))
}

/// Parse an expression
fn parse_expr<J: Jet + 'static>(p: &mut Parser) -> Result<Expression, ErrorSet> {
    let position = p.current_position();

    match p.peek().cloned() {
        Some(Token::LParen) => {
            p.advance();
            let inner = parse_expr::<J>(p)?;
            p.expect(&Token::RParen)?;
            Ok(inner)
        }
        Some(Token::Question) => {
            p.advance();
            let (name, sym_pos) = parse_symbol_value(p)?;
            Ok(Expression {
                inner: ExprInner::Inline(node::Inner::Witness(WitnessOrHole::TypedHole(name))),
                position: sym_pos,
            })
        }
        Some(Token::Const) => {
            p.advance();
            let (data, bit_length, lit_pos) = parse_literal(p)?;
            let mut iter = BitIter::from(data);
            if bit_length.count_ones() != 1 || bit_length > 1 << 31 {
                return Err(ErrorSet::single(
                    lit_pos,
                    Error::BadWordLength { bit_length },
                ));
            }
            let word = Word::from_bits(&mut iter, bit_length.trailing_zeros()).unwrap();
            Ok(Expression {
                inner: ExprInner::Inline(node::Inner::Word(word)),
                position: lit_pos,
            })
        }
        Some(Token::AssertL) => {
            p.advance();
            let left = parse_expr::<J>(p)?;
            let cmr = parse_cmr::<J>(p)?;
            Ok(Expression {
                inner: ExprInner::AssertL(Arc::new(left), cmr),
                position,
            })
        }
        Some(Token::AssertR) => {
            p.advance();
            let cmr = parse_cmr::<J>(p)?;
            let right = parse_expr::<J>(p)?;
            Ok(Expression {
                inner: ExprInner::AssertR(cmr, Arc::new(right)),
                position,
            })
        }
        Some(Token::Fail) => {
            p.advance();
            let (value, bit_length, lit_pos) = parse_literal(p)?;
            if bit_length < 128 {
                return Err(ErrorSet::single(
                    lit_pos,
                    Error::EntropyInsufficient { bit_length },
                ));
            }
            if bit_length > 512 {
                return Err(ErrorSet::single(
                    lit_pos,
                    Error::EntropyTooMuch { bit_length },
                ));
            }
            let mut entropy = [0; 64];
            entropy[..value.len()].copy_from_slice(&value[..]);
            let entropy = FailEntropy::from_byte_array(entropy);
            Ok(Expression {
                inner: ExprInner::Inline(node::Inner::Fail(entropy)),
                position,
            })
        }
        // Nullary?
        Some(Token::Unit) => {
            p.advance();
            Ok(Expression {
                inner: ExprInner::Inline(node::Inner::Unit),
                position,
            })
        }
        Some(Token::Iden) => {
            p.advance();
            Ok(Expression {
                inner: ExprInner::Inline(node::Inner::Iden),
                position,
            })
        }
        Some(Token::Witness) => {
            p.advance();
            Ok(Expression {
                inner: ExprInner::Inline(node::Inner::Witness(WitnessOrHole::Witness)),
                position,
            })
        }
        Some(Token::Jet(ref name)) => {
            let jet_name = name.clone();
            p.advance();
            let Ok(jet) = J::parse(&jet_name[4..]) else {
                return Err(ErrorSet::single(position, Error::UnknownJet(jet_name)));
            };
            Ok(Expression {
                inner: ExprInner::Inline(node::Inner::Jet(Box::new(jet))),
                position,
            })
        }
        // Unary
        Some(Token::InjL) => {
            p.advance();
            let child = Arc::new(parse_expr::<J>(p)?);
            Ok(Expression {
                inner: ExprInner::Inline(node::Inner::InjL(child)),
                position,
            })
        }
        Some(Token::InjR) => {
            p.advance();
            let child = Arc::new(parse_expr::<J>(p)?);
            Ok(Expression {
                inner: ExprInner::Inline(node::Inner::InjR(child)),
                position,
            })
        }
        Some(Token::Take) => {
            p.advance();
            let child = Arc::new(parse_expr::<J>(p)?);
            Ok(Expression {
                inner: ExprInner::Inline(node::Inner::Take(child)),
                position,
            })
        }
        Some(Token::Drop) => {
            p.advance();
            let child = Arc::new(parse_expr::<J>(p)?);
            Ok(Expression {
                inner: ExprInner::Inline(node::Inner::Drop(child)),
                position,
            })
        }
        // Binary
        Some(Token::Case) => {
            p.advance();
            let left = Arc::new(parse_expr::<J>(p)?);
            let right = Arc::new(parse_expr::<J>(p)?);
            Ok(Expression {
                inner: ExprInner::Inline(node::Inner::Case(left, right)),
                position,
            })
        }
        Some(Token::Comp) => {
            p.advance();
            let left = Arc::new(parse_expr::<J>(p)?);
            let right = Arc::new(parse_expr::<J>(p)?);
            Ok(Expression {
                inner: ExprInner::Inline(node::Inner::Comp(left, right)),
                position,
            })
        }
        Some(Token::Pair) => {
            p.advance();
            let left = Arc::new(parse_expr::<J>(p)?);
            let right = Arc::new(parse_expr::<J>(p)?);
            Ok(Expression {
                inner: ExprInner::Inline(node::Inner::Pair(left, right)),
                position,
            })
        }
        Some(Token::Disconnect) => {
            p.advance();
            let left = Arc::new(parse_expr::<J>(p)?);
            let right = Arc::new(parse_expr::<J>(p)?);
            Ok(Expression {
                inner: ExprInner::Inline(node::Inner::Disconnect(left, right)),
                position,
            })
        }
        // Symbol reference
        Some(Token::Symbol(_)) | Some(Token::Underscore) => {
            let (name, sym_pos) = parse_symbol_value(p)?;
            Ok(Expression::reference(name, sym_pos))
        }
        _ => Err(ErrorSet::single(
            position,
            Error::ParseFailed(p.peek_raw_description()),
        )),
    }
}

/// Parse a CMR (either an expression in #{} or a literal)
fn parse_cmr<J: Jet + 'static>(p: &mut Parser) -> Result<AstCmr, ErrorSet> {
    if p.eat(&Token::HashBrace) {
        let expr = parse_expr::<J>(p)?;
        p.expect(&Token::RBrace)?;
        return Ok(AstCmr::Expr(Arc::new(expr)));
    }

    if let Some(Token::CmrLiteral(_)) = p.peek() {
        p.advance();
        return Ok(AstCmr::Literal);
    }

    Err(ErrorSet::single(
        p.current_position(),
        Error::ParseFailed(p.peek_raw_description()),
    ))
}

/// Parse a literal (underscore, binary, or hex)
fn parse_literal(p: &mut Parser) -> Result<(Vec<u8>, usize, Position), ErrorSet> {
    let position = p.current_position();
    match p.peek().cloned() {
        Some(Token::Underscore) => {
            p.advance();
            Ok((vec![], 0, position))
        }
        Some(Token::BinLiteral(ref raw)) => {
            let s = &raw[2..];
            let bit_length = s.len();
            let mut data = Vec::with_capacity(s.len().div_ceil(8));
            let mut x: u8 = 0;
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
            p.advance();
            Ok((data, bit_length, position))
        }
        Some(Token::HexLiteral(ref raw)) => {
            let s = &raw[2..];
            let bit_length = s.len() * 4;
            let mut data = Vec::with_capacity(s.len().div_ceil(2));
            for idx in 0..s.len() / 2 {
                data.push(u8::from_str_radix(&s[2 * idx..2 * idx + 2], 16).unwrap());
            }
            if s.len() % 2 == 1 {
                data.push(u8::from_str_radix(&s[s.len() - 1..], 16).unwrap() << 4);
            }
            p.advance();
            Ok((data, bit_length, position))
        }
        _ => Err(ErrorSet::single(
            position,
            Error::ParseFailed(p.peek_raw_description()),
        )),
    }
}

/// Parse a type expression, left-associative for both + and *
fn parse_type(p: &mut Parser) -> Result<Option<Type>, ErrorSet> {
    let mut lhs = parse_type_atom(p)?;
    loop {
        if p.peek() == Some(&Token::Plus) {
            p.advance();
            let rhs = parse_type_atom(p)?;
            lhs = lhs
                .zip(rhs)
                .map(|(l, r)| Type::Sum(Box::new(l), Box::new(r)));
            continue;
        }
        if p.peek() == Some(&Token::Star) {
            p.advance();
            let rhs = parse_type_atom(p)?;
            lhs = lhs
                .zip(rhs)
                .map(|(l, r)| Type::Product(Box::new(l), Box::new(r)));
            continue;
        }
        break;
    }
    Ok(lhs)
}

/// Parse a type atom, including one or more `?`s.
fn parse_type_atom(p: &mut Parser) -> Result<Option<Type>, ErrorSet> {
    let mut atom = parse_type_atom_inner(p)?;
    // Repeated `?` denotes nested optionals, e.g. `2??` is 1 + (1 + 2).
    while atom.is_some() && p.eat(&Token::Question) {
        atom = atom.map(|ty| Type::Sum(Box::new(Type::One), Box::new(ty)));
    }
    Ok(atom)
}

fn parse_type_atom_inner(p: &mut Parser) -> Result<Option<Type>, ErrorSet> {
    match p.peek().cloned() {
        Some(Token::One) => {
            p.advance();
            Ok(Some(Type::One))
        }
        Some(Token::Two) => {
            p.advance();
            Ok(Some(Type::Two))
        }
        Some(Token::TwoExp(ref raw)) => {
            let raw = raw.clone();
            let position = p.current_position();
            p.advance();
            let exp_str = &raw[2..];
            match str::parse::<u32>(exp_str) {
                Ok(0) => Ok(Some(Type::One)),
                Ok(1) => Ok(Some(Type::Two)),
                Ok(2) => Ok(Some(Type::TwoTwoN(1))),
                Ok(4) => Ok(Some(Type::TwoTwoN(2))),
                Ok(8) => Ok(Some(Type::TwoTwoN(3))),
                Ok(16) => Ok(Some(Type::TwoTwoN(4))),
                Ok(32) => Ok(Some(Type::TwoTwoN(5))),
                Ok(64) => Ok(Some(Type::TwoTwoN(6))),
                Ok(128) => Ok(Some(Type::TwoTwoN(7))),
                Ok(256) => Ok(Some(Type::TwoTwoN(8))),
                Ok(512) => Ok(Some(Type::TwoTwoN(9))),
                Ok(y) => Err(ErrorSet::single(position, Error::Bad2ExpNumber(y))),
                Err(_) => Err(ErrorSet::single(position, Error::NumberOutOfRange(raw))),
            }
        }
        Some(Token::LParen) => {
            p.advance();
            let ty = parse_type(p)?;
            p.expect(&Token::RParen)?;
            Ok(ty)
        }
        Some(Token::Symbol(_)) | Some(Token::Underscore) => {
            let (name, _pos) = parse_symbol_value(p)?;
            if name.as_ref() == "_" {
                Ok(None)
            } else {
                Ok(Some(Type::Name(name.as_ref().to_owned())))
            }
        }
        _ => Err(ErrorSet::single(
            p.current_position(),
            Error::ParseFailed(p.peek_raw_description()),
        )),
    }
}

/// Consume a token that represents a symbol name and return it
fn parse_symbol_value(p: &mut Parser) -> Result<(Arc<str>, Position), ErrorSet> {
    let position = p.current_position();
    match p.peek().cloned() {
        Some(Token::Symbol(ref s)) => {
            let s: Arc<str> = Arc::from(s.as_str());
            p.advance();
            Ok((s, position))
        }
        Some(Token::Underscore) => {
            p.advance();
            Ok((Arc::from("_"), position))
        }
        _ => Err(ErrorSet::single(
            position,
            Error::ParseFailed(p.peek_raw_description()),
        )),
    }
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
