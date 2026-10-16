// SPDX-License-Identifier: CC0-1.0

mod c_env;
mod environment;
#[cfg(test)]
mod tests;

pub use environment::{ElementsEnv, ElementsUtxo};

use super::init::elements::Elements;
use super::JetEnvironment;
use simplicity_sys::c_jets::frame_ffi::CFrameItem;
use simplicity_sys::CElementsTxEnv;

/// Type alias for the Elements transaction environment.
pub type ElementsTxEnv = ElementsEnv<std::sync::Arc<elements::Transaction>>;

impl<T: core::borrow::Borrow<elements::Transaction>> JetEnvironment for ElementsEnv<T> {
    type Jet = Elements;
    type CJetEnvironment = CElementsTxEnv;

    fn c_jet_env(&self) -> &Self::CJetEnvironment {
        self.c_tx_env()
    }

    fn c_jet_ptr(
        jet: &Self::Jet,
    ) -> fn(&mut CFrameItem, CFrameItem, &Self::CJetEnvironment) -> bool {
        super::init::elements::c_jet_ptr(jet)
    }
}
