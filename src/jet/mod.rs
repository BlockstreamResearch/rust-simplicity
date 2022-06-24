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
//! [`Application`]s extend Simplicity by [`JetNode`]s, i.e., special nodes that read a value,
//! process it internally, and write an output value.  This jet evaluation happens in a black-box
//! manner:  In terms of the Bit Machine, it is a one-step process.
//!
//! In practice, jets call foreign C code that is equivalent to some Simplicity DAG.
//! This speeds up evaluation tremendously.
//! Equivalence of C and Simplicity is proved using the _Verified Software Toolchain_.

pub mod application;
mod init;
pub mod type_name;

#[cfg(feature = "bitcoin")]
pub use init::bitcoin;
pub use init::core;
#[cfg(feature = "elements")]
pub use init::elements;

use crate::bititer::BitIter;
use crate::encode::BitWriter;
use crate::exec::BitMachine;
use crate::jet::type_name::TypeName;
use crate::merkle::cmr::Cmr;
use crate::merkle::common::MerkleRoot;
use crate::merkle::imr::Imr;
use std::hash::Hash;
use std::io::Write;

/// Applications extend Simplicity by providing [`JetNode`]s
/// with custom (de)serialization and execution.
pub trait Application: Sized + 'static {
    /// Environment for jets to read from
    type Environment;
    /// Custom application errors
    type Error: AppError;
    /// Enumeration of all jet names
    type JetName: Clone + Eq + Ord + std::hash::Hash + std::fmt::Debug + std::fmt::Display;

    /// Decode a jet from bits.
    ///
    /// Every jet has a unique serialization _in context of its application_.
    /// Two jets of _different_ applications may share the same serialization.
    fn decode_jet<I: Iterator<Item = u8>>(
        iter: &mut BitIter<I>,
    ) -> Result<&'static JetNode<Self>, crate::Error>;

    /// Encode a jet as bits.
    ///
    /// Encoding is the reverse of decoding, and vice versa.
    /// Returns the number of written bits.
    fn encode_jet<W: Write>(jet: &JetNode<Self>, w: &mut BitWriter<W>) -> std::io::Result<usize>;

    /// Execute a jet on the Bit Machine.
    ///
    /// During its execution, the jet may read from the environment and throw custom errors.
    fn exec_jet(
        jet: &JetNode<Self>,
        mac: &mut BitMachine,
        env: &Self::Environment,
    ) -> Result<(), Self::Error>;
}

/// Application error
pub trait AppError: std::error::Error {}

/// Jet node that belongs to some [`Application`].
///
/// Jets are single nodes that read an input,
/// process it internally using foreign C code _(black box)_,
/// and produce an output.
/// Jets may read values from an _environment_.
///
/// Jets are **always** leaf nodes in a Simplicity DAG.
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct JetNode<App: Application> {
    pub(crate) name: App::JetName,
    cmr: Cmr,
    pub(crate) source_ty: TypeName,
    pub(crate) target_ty: TypeName,
}

impl<App: Application> JetNode<App> {
    /// Return the CMR of the jet.
    ///
    /// Every jet has a CMR that is _usually_ unique.
    /// Jet primitives commit to their application name and to their jet name in their CMR.
    /// Jet macros commit to their DAG's root CMR _(usually unique)_ and to their cost.
    pub fn cmr(&self) -> Cmr {
        self.cmr
    }

    /// Return the IMR of the jet.
    ///
    /// A jet's CMR equals its IMR.
    /// Jets may not include `witness` or `disconnect` nodes as a result.
    pub fn imr(&self) -> Imr {
        self.cmr.into_inner().into()
    }
}

impl<App: Application> std::fmt::Display for JetNode<App> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(&self.name, f)
    }
}
