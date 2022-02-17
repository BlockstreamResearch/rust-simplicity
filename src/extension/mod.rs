// Rust Simplicity Library
// Written in 2020 by
//   Andrew Poelstra <apoelstra@blockstream.com>
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

//! # Extensions
//!
//! Extensions to the Simplicity language needed for blockchain support
//!

#[cfg(feature = "bitcoin")]
pub mod bitcoin;
pub mod dummy;
#[cfg(feature = "elements")]
pub mod elements;
pub mod jets;

use std::{fmt, io};

use crate::bititer::BitIter;
use crate::cmr::Cmr;
use crate::encode;
use crate::exec;
use crate::Error;

#[cfg(not(feature = "bitcoin"))]
pub use self::dummy as bitcoin;

/// Types used by Bitcoin/Elements primitives and hardcoded jets
///
/// The type inference engine does not have direct access to Bitcoin/Elements
/// node types (these do not even exist without the correct feature flag);
/// therefore we need accessors in this module whichcan introspect the node
/// and determine the correct source/target type.
///
/// These accessors (`source_type` and `target_type`) encode their types in
/// a prefix (Polish) notation, where + and * represent sum and product types
/// respectively, and base types are represented by:
///
/// | char | type         |
/// |------|--------------|
/// | `1`  | unit         |
/// | `2`  | single bit   |
/// | `i`  | 32-bit word  |
/// | `l`  | 64-bit word  |
/// | `h`  | 256-bit word |
///
#[derive(Clone)]
pub struct TypeName(pub &'static [u8]);

impl Iterator for TypeName {
    type Item = u8;
    fn next(&mut self) -> Option<u8> {
        if self.0.is_empty() {
            None
        } else {
            let ret = Some(self.0[0]);
            self.0 = &self.0[1..];
            ret
        }
    }
}

/// Trait representing an extension (Bitcoin or Elements) to Simplicity
pub trait Jet: Sized + fmt::Display {
    /// Transaction environment
    type TxEnv;
    type JetErr: ExtError;

    /// Decode a node from a bit iterator
    fn decode<I: Iterator<Item = u8>>(iter: &mut BitIter<I>) -> Result<Self, Error>;

    /// Encode a node into a bit writer
    fn encode<W: encode::BitWrite>(&self, w: &mut W) -> io::Result<usize>;

    /// Execute the node in a Bit Machine; assuming the surrounding
    /// program has typechecked, this cannot fail
    fn exec(&self, mac: &mut exec::BitMachine, txenv: &Self::TxEnv) -> Result<(), Self::JetErr>;

    /// Return the CMR of the node
    fn cmr(&self) -> Cmr;

    /// Return the WMR of the node
    fn wmr(&self) -> Cmr;

    /// The name of the source type of this node
    fn source_type(&self) -> TypeName;

    /// The name of the target type of this node
    fn target_type(&self) -> TypeName;
}

/// Errors encountered while executing a jet
pub trait ExtError: std::error::Error {}
