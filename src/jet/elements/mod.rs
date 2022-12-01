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

mod c_env;
mod environment;
#[allow(dead_code)]
mod exec;
#[cfg(test)]
mod tests;

use crate::jet::Elements;
pub use environment::{ElementsEnv, ElementsUtxo};
use simplicity_sys::c_jets::CTxEnv;

use super::JetEnvironment;

impl std::fmt::Display for Elements {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(self, f)
    }
}

impl JetEnvironment for ElementsEnv {
    fn c_tx_env(&self) -> CTxEnv {
        CTxEnv::ElementsTxEnv(self.c_tx_env())
    }
}
