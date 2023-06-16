// Simplicity "Human-Readable" Language
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

//! Parsing Errors

use std::{error, fmt};

use santiago::lexer::Lexeme;
use simplicity::types;

use crate::Position;

/// An individual error
#[derive(Clone, Debug)]
pub struct Error {
    pub position: Position,
    pub ty: ErrorType,
}

impl Error {
    pub fn no_main() -> Error {
        Error {
            position: Position::default(),
            ty: ErrorType::NoMain,
        }
    }

    pub fn bad_type_number(n: u32, position: Position) -> Error {
        Error { position, ty: ErrorType::BadTypeNumber(n) }
    }

    pub fn bad_exp_type_number(base: u32, exp: u32, position: Position) -> Error {
        Error { position, ty: ErrorType::BadExpTypeNumber{ base, exp } }
    }

    pub fn bad_word_length(word: String, position: Position) -> Error {
        Error { position, ty: ErrorType::BadWordLength { word } }
    }

    pub fn incomplete_symbol(name: String, position: Position) -> Error {
        Error {
            position,
            ty: ErrorType::Incomplete(name),
        }
    }

    pub fn undefined_symbol(name: String, position: Position) -> Error {
        Error {
            position,
            ty: ErrorType::Undefined(name),
        }
    }

    pub fn unknown_jet(name: String, position: Position) -> Error {
        Error {
            position,
            ty: ErrorType::UnknownJet(name),
        }
    }
}

impl<T> From<santiago::parser::ParseError<T>> for Error {
    fn from(e: santiago::parser::ParseError<T>) -> Error {
        let lex = e.at.map(|rc| (*rc).clone());
        let position = lex
            .as_ref()
            .map(|lex| (&lex.position).into())
            .unwrap_or_default();
        Error {
            position,
            ty: ErrorType::ParseFailed(lex),
        }
    }
}

impl From<santiago::lexer::LexerError> for Error {
    fn from(e: santiago::lexer::LexerError) -> Error {
        Error {
            position: e.position.into(),
            ty: ErrorType::LexFailed(e.message),
        }
    }
}

impl error::Error for Error {
    fn cause(&self) -> Option<&(dyn error::Error + 'static)> {
        match self.ty {
            ErrorType::BadCmr(ref e) => Some(e),
            ErrorType::BadExpTypeNumber { .. } => None,
            ErrorType::BadTypeNumber(_) => None,
            ErrorType::BadWordLength { .. } => None,
            ErrorType::NoMain => None,
            ErrorType::ParseFailed(_) => None,
            ErrorType::LexFailed(_) => None,
            ErrorType::Incomplete(_) => None,
            ErrorType::TypeCheck(ref e) => Some(e),
            ErrorType::Undefined(_) => None,
            ErrorType::UnknownJet(_) => None,
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "line {} column {}: {}", self.position.line, self.position.column, self.ty)
    }
}

#[derive(Clone, Debug)]
pub enum ErrorType {
    /// A number was used as a type but it was not 1 or 2
    BadCmr(simplicity::bitcoin_hashes::hex::Error),
    /// A number of the form x^y was used as a type but it was not 2^n for an allowed n
    BadExpTypeNumber {
        base: u32,
        exp: u32,
    },
    /// A number was used as a type but it was not 1 or 2
    BadTypeNumber(u32),
    /// A constant word had a length which was not an allowable power of 2
    BadWordLength {
        word: String,
    },
    /// Program did not have a `main` expression
    NoMain,
    /// Parsing failed (the parser provides us some extra information, but beyond
    /// the line and column, it does not seem very useful to a user, so we drop it).
    ParseFailed(Option<Lexeme>),
    /// Lexing failed; here santiago provides us an error message which is useful
    LexFailed(String),
    /// Expression referred to symbol which is declared (has a type) but has no definition
    Incomplete(String),
    /// Simplicity type-checking error
    TypeCheck(types::Error),
    /// Expression referred to an undefined symbol
    Undefined(String),
    /// A given jet is not a known jet
    UnknownJet(String),
}

impl fmt::Display for ErrorType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ErrorType::BadCmr(ref e) => fmt::Display::fmt(e, f),
            ErrorType::BadWordLength { ref word } => write!(f, "word length {} is not a valid power of 2 ({})", word.len(), word),
            ErrorType::BadExpTypeNumber { base, exp } => write!(f, "types may be 2^n for n a power of 2, but not {}^{}", base, exp),
            ErrorType::BadTypeNumber(x) => write!(f, "types may be 1 or 2, but not {}", x),
            ErrorType::NoMain => f.write_str("program does not have `main` expression"),
            ErrorType::ParseFailed(None) => f.write_str("could not parse"),
            ErrorType::ParseFailed(Some(ref lex)) => write!(f, "could not parse `{}`", lex.raw),
            ErrorType::LexFailed(ref msg) => write!(f, "could not parse: {}", msg),
            ErrorType::Incomplete(ref s) => write!(f, "reference to incomplete symbol `{}`", s),
            ErrorType::TypeCheck(ref e) => fmt::Display::fmt(e, f),
            ErrorType::Undefined(ref s) => write!(f, "reference to undefined symbol `{}`", s),
            ErrorType::UnknownJet(ref s) => write!(f, "unknown jet `{}`", s),
        }
    }
}

