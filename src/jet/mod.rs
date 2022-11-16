// Rust Simplicity Library
// Written in 2022 by
//   Christian Lewe <clewe@blockstream.com>
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

//! # Simplicity jets
//!
//! Jets are special nodes that read a value,
//! process it internally, and write an output value.
//! This evaluation happens in a black-box manner:
//! In terms of the Bit Machine, it is a one-step process.
//!
//! In practice, jets call foreign C code that is equivalent to some Simplicity DAG.
//! This speeds up evaluation tremendously.
//! Equivalence of C and Simplicity is proved using the _Verified Software Toolchain_.
//! Programs are also smaller in size because jets replace large, equivalent Simplicity DAGs.

#[cfg(feature = "bitcoin")]
pub mod bitcoin;
pub mod core;
mod init;
pub mod type_name;

#[cfg(feature = "bitcoin")]
pub use init::bitcoin::Bitcoin;
pub use init::core::Core;
// #[cfg(feature = "elements")]
// pub use init::elements;

use crate::bititer::BitIter;
use crate::bitwriter::BitWriter;
use crate::exec::BitMachine;
use crate::jet::type_name::TypeName;
use crate::merkle::cmr::Cmr;
use crate::merkle::common::MerkleRoot;
use crate::merkle::imr::Imr;
use crate::Error;
use std::hash::Hash;
use std::io::Write;

/// Generic error that a jet failed during its execution.
///
/// Failure could be due to a failed assertion, an illegal input, etc.
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Debug, Hash)]
pub struct JetFailed;

impl std::fmt::Display for JetFailed {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str("Jet failed during execution")
    }
}

impl std::error::Error for JetFailed {}

/// Family of jets that share an encoding scheme and execution environment.
///
/// Jets are single nodes that read an input,
/// process it internally using foreign C code _(black box)_,
/// and produce an output.
/// Jets may read values from their _environment_.
///
/// Jets are **always** leaves in a Simplicity DAG.
pub trait Jet: Copy + Eq + Ord + Hash + std::fmt::Debug + std::fmt::Display {
    /// Environment for jet to read from
    type Environment;

    /// Return the CMR of the jet.
    fn cmr(&self) -> Cmr;

    /// Return the IMR of the jet.
    fn imr(&self) -> Imr {
        self.cmr().into_inner().into()
    }

    /// Return the source type of the jet.
    fn source_ty(&self) -> TypeName;

    /// Return the target type of the jet.
    fn target_ty(&self) -> TypeName;

    /// Encode the jet to bits.
    fn encode<W: Write>(&self, w: &mut BitWriter<W>) -> std::io::Result<usize>;

    /// Decode a jet from bits.
    fn decode<I: Iterator<Item = u8>>(bits: &mut BitIter<I>) -> Result<Self, Error>;

    /// Execute the jet on the Bit Machine, using the given environment.
    fn exec(&self) -> fn(&mut BitMachine, &Self::Environment) -> Result<(), JetFailed>;
}
