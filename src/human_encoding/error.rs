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

use santiago::lexer::Lexeme;
use std::collections::BTreeMap;
use std::sync::{Arc, Mutex};
use std::{error, fmt, iter};

use crate::types;

use super::Position;

/// A set of errors found in a human-readable encoding of a Simplicity program.
#[derive(Clone, Debug, Default)]
pub struct ErrorSet {
    context: Option<Arc<str>>,
    line_map: Arc<Mutex<Vec<usize>>>,
    errors: BTreeMap<Option<Position>, Vec<Error>>,
}

impl<T> From<santiago::parser::ParseError<T>> for ErrorSet {
    fn from(e: santiago::parser::ParseError<T>) -> Self {
        let lex = e.at.map(|rc| (*rc).clone());
        match lex.as_ref().map(|lex| &lex.position).map(Position::from) {
            Some(pos) => ErrorSet::single(pos, Error::ParseFailed(lex)),
            None => ErrorSet::single_no_position(Error::ParseFailed(lex)),
        }
    }
}

impl From<santiago::lexer::LexerError> for ErrorSet {
    fn from(e: santiago::lexer::LexerError) -> Self {
        ErrorSet::single(e.position, Error::LexFailed(e.message))
    }
}

impl ErrorSet {
    /// Constructs a new empty error set.
    pub fn new() -> Self {
        ErrorSet::default()
    }

    /// Returns the first (and presumably most important) error in the set, if it
    /// is non-empty, along with its position.
    pub fn first_error(&self) -> Option<(Option<Position>, &Error)> {
        self.errors.iter().next().map(|(a, b)| (*a, &b[0]))
    }

    /// Constructs a new error set with a single error in it.
    pub fn single<P: Into<Position>, E: Into<Error>>(position: P, err: E) -> Self {
        let mut errors = BTreeMap::default();
        errors.insert(Some(position.into()), vec![err.into()]);
        ErrorSet {
            context: None,
            line_map: Arc::new(Mutex::new(vec![])),
            errors,
        }
    }

    /// Constructs a new error set with a single error in it.
    pub fn single_no_position<E: Into<Error>>(err: E) -> Self {
        let mut errors = BTreeMap::default();
        errors.insert(None, vec![err.into()]);
        ErrorSet {
            context: None,
            line_map: Arc::new(Mutex::new(vec![])),
            errors,
        }
    }

    /// Adds an error to the error set.
    pub fn add<P: Into<Position>, E: Into<Error>>(&mut self, position: P, err: E) {
        self.errors
            .entry(Some(position.into()))
            .or_insert(vec![])
            .push(err.into());
    }

    /// Adds an error to the error set.
    pub fn add_no_position<E: Into<Error>>(&mut self, err: E) {
        self.errors.entry(None).or_insert(vec![]).push(err.into());
    }

    /// Merges another set of errors into the current set.
    ///
    /// # Panics
    ///
    /// Panics if the two sets have different contexts attached.
    pub fn merge(&mut self, other: &Self) {
        match (self.context.as_ref(), other.context.as_ref()) {
            (None, None) => {}
            (Some(_), None) => {}
            (None, Some(b)) => self.context = Some(Arc::clone(b)),
            (Some(a), Some(b)) => {
                assert_eq!(a, b, "cannot merge error sets for different source input");
            }
        };

        for (pos, errs) in &other.errors {
            self.errors
                .entry(*pos)
                .or_insert(vec![])
                .extend(errs.iter().cloned());
        }
    }

    /// Attaches the input code to the error set, so that error messages can include
    /// line numbers etc.
    ///
    /// # Panics
    ///
    /// Panics if it is called twice on the same error set. You should call this once
    /// with the complete input code.
    pub fn add_context(&mut self, s: Arc<str>) {
        if self.context.is_some() {
            panic!("tried to add context to the same error context twice");
        }
        self.context = Some(s);
    }

    /// Returns a boolean indicating whether the set is empty.
    pub fn is_empty(&self) -> bool {
        self.errors.is_empty()
    }

    /// Returns the number of errors currently in the set.
    pub fn len(&self) -> usize {
        self.errors.len()
    }

    /// Converts the error set into a result.
    ///
    /// If the set is empty, returns Ok with the given value. Otherwise
    /// returns Err with itself.
    pub fn into_result<T>(self, ok: T) -> Result<T, Self> {
        if self.is_empty() {
            Ok(ok)
        } else {
            Err(self)
        }
    }

    /// Converts the error set into a result.
    ///
    /// If the set is empty, returns Ok with the result of calling the given closure.
    /// Otherwise returns Err with itself.
    pub fn into_result_with<T, F: FnOnce() -> T>(self, okfn: F) -> Result<T, Self> {
        if self.is_empty() {
            Ok(okfn())
        } else {
            Err(self)
        }
    }
}

impl error::Error for ErrorSet {
    fn cause(&self) -> Option<&(dyn error::Error + 'static)> {
        match self.first_error()?.1 {
            Error::Bad2ExpNumber(..) => None,
            Error::BadWordLength { .. } => None,
            Error::EntropyInsufficient { .. } => None,
            Error::EntropyTooMuch { .. } => None,
            Error::HoleAtCommitTime { .. } => None,
            Error::NameIllegal(_) => None,
            Error::NameIncomplete(_) => None,
            Error::NameMissing(_) => None,
            Error::NameRepeated(_) => None,
            Error::NoMain => None,
            Error::ParseFailed(_) => None,
            Error::LexFailed(_) => None,
            Error::NumberOutOfRange(_) => None,
            Error::TypeCheck(ref e) => Some(e),
            Error::Undefined(_) => None,
            Error::UnknownJet(_) => None,
            Error::WitnessDisconnectRepeated { .. } => None,
        }
    }
}

impl fmt::Display for ErrorSet {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut line_map = self.line_map.lock().unwrap();
        if line_map.is_empty() {
            if let Some(ref s) = self.context {
                *line_map = iter::repeat(0)
                    .take(2)
                    .chain(
                        s.char_indices()
                            .filter_map(|(n, ch)| if ch == '\n' { Some(n) } else { None }),
                    )
                    .collect();
            }
        }

        for (pos, errs) in &self.errors {
            if let Some(pos) = pos {
                for err in errs {
                    if let Some(ref s) = self.context {
                        let end = line_map.get(pos.line + 1).copied().unwrap_or(s.len());
                        let line = &s[line_map[pos.line] + 1..end];
                        writeln!(f, "{:5} | {}", pos.line, line)?;
                        writeln!(f, "      | {:>width$}", "^", width = pos.column)?;
                        writeln!(f, "      \\-- {}", err)?;
                        writeln!(f)?;
                    } else {
                        writeln!(f, "{:4}:{:2}: {}", pos.line, pos.column, err,)?;
                        writeln!(f)?;
                    }
                }
            } else {
                for err in errs {
                    writeln!(f, "Error: {}", err)?;
                }
            }
        }
        Ok(())
    }
}

/// An individual error.
///
/// Generally this structure should not be used on its own, but only wrapped in an
/// [`ErrorSet`]. This is because in the human-readable encoding errors it is usually
/// possible to continue past individual errors, and the user would prefer to see as
/// many as possible at once.
#[derive(Clone, Debug)]
pub enum Error {
    /// A number of the form 2^y was used as a type but y was not an allowed value
    Bad2ExpNumber(u32),
    /// A constant word had a length which was not an allowable power of 2
    BadWordLength { bit_length: usize },
    /// A "fail" node was provided with less than 128 bits of entropy
    EntropyInsufficient { bit_length: usize },
    /// A "fail" node was provided with more than 512 bits of entropy
    EntropyTooMuch { bit_length: usize },
    /// When converting to a `CommitNode`, there were unfilled holes which prevent
    /// us from knowing the whole program.
    HoleAtCommitTime {
        name: Arc<str>,
        arrow: types::arrow::Arrow,
    },
    /// An expression name was not allowed to be used as a name.
    NameIllegal(Arc<str>),
    /// An expression was given a type, but no actual expression was provided.
    NameIncomplete(Arc<str>),
    /// An expression was referenced but did not refer to anything.
    NameMissing(Arc<str>),
    /// An expression name was used for multiple expressions.
    NameRepeated(Arc<str>),
    /// Program did not have a `main` expression
    NoMain,
    /// Parsing failed (the parser provides us some extra information, but beyond
    /// the line and column, it does not seem very useful to a user, so we drop it).
    ParseFailed(Option<Lexeme>),
    /// Lexing failed; here santiago provides us an error message which is useful
    LexFailed(String),
    /// A number was parsed in some context but was out of range.
    NumberOutOfRange(String),
    /// Simplicity type-checking error
    TypeCheck(types::Error),
    /// Expression referred to an undefined symbol
    Undefined(String),
    /// A given jet is not a known jet
    UnknownJet(String),
    /// A witness or disconnect node was accessible from multiple paths.
    WitnessDisconnectRepeated { name: Arc<str>, count: usize },
}

impl From<types::Error> for Error {
    fn from(e: types::Error) -> Self {
        Error::TypeCheck(e)
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::BadWordLength { bit_length } => {
                write!(f, "word length {} is not a valid power of 2", bit_length)
            }
            Error::Bad2ExpNumber(exp) => {
                write!(f, "types may be 2^n for n a power of 2, but not 2^{}", exp)
            }
            Error::EntropyInsufficient { bit_length } => write!(
                f,
                "fail node has insufficient entropy ({} bits, need 128)",
                bit_length
            ),
            Error::EntropyTooMuch { bit_length } => write!(
                f,
                "fail node has too much entropy ({} bits, max 512)",
                bit_length
            ),
            Error::HoleAtCommitTime {
                ref name,
                ref arrow,
            } => write!(
                f,
                "unfilled hole ?{} at commitment time; type arrow {}",
                name, arrow
            ),
            Error::NameIllegal(ref s) => {
                write!(f, "name `{}` is not allowed in this context", s)
            }
            Error::NameIncomplete(ref s) => write!(f, "name `{}` has no expression", s),
            Error::NameMissing(ref s) => {
                write!(f, "name `{}` is referred to but does not exist", s)
            }
            Error::NameRepeated(ref s) => write!(f, "name `{}` occured mulitple times", s),
            Error::NoMain => f.write_str("program does not define `main`"),
            Error::NumberOutOfRange(ref n) => {
                write!(f, "number {} was out of allowable range", n)
            }
            Error::ParseFailed(None) => f.write_str("could not parse"),
            Error::ParseFailed(Some(ref lex)) => write!(f, "could not parse `{}`", lex.raw),
            Error::LexFailed(ref msg) => write!(f, "could not parse: {}", msg),
            Error::TypeCheck(ref e) => fmt::Display::fmt(e, f),
            Error::Undefined(ref s) => write!(f, "reference to undefined symbol `{}`", s),
            Error::UnknownJet(ref s) => write!(f, "unknown jet `{}`", s),
            Error::WitnessDisconnectRepeated { ref name, count } => write!(
                f,
                "witness/disconnect node {} was accessible by {} distinct paths from the same root",
                name, count,
            ),
        }
    }
}
