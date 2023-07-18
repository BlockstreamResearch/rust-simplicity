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
            Error::TypeCheck(ref e) => Some(e),
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
    /// Simplicity type-checking error
    TypeCheck(types::Error),
}

impl From<types::Error> for Error {
    fn from(e: types::Error) -> Self {
        Error::TypeCheck(e)
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::TypeCheck(ref e) => fmt::Display::fmt(e, f),
        }
    }
}
