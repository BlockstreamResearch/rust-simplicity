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

//! # Simplicity applications
//!
//! Applications provide jets to Simplicity.
//! Their implementation usually requires certain features to be enabled.

//#[cfg(feature = "bitcoin")]
//mod bitcoin;
mod core;
//#[cfg(feature = "elements")]
//mod elements;

//#[cfg(feature = "bitcoin")]
//pub use self::bitcoin::{Bitcoin, BitcoinEnv, BitcoinError};
pub use self::core::{Core, CoreError};
//#[cfg(feature = "elements")]
//pub use self::elements::{Elements, ElementsEnv, ElementsError, ElementsUtxo};
