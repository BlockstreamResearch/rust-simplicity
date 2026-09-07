// SPDX-License-Identifier: CC0-1.0

mod c_env;
mod environment;
#[cfg(test)]
mod tests;

pub use environment::BitcoinEnv;

use super::init::bitcoin::Bitcoin;
use super::JetEnvironment;
use simplicity_sys::c_jets::c_env::bitcoin::CTxEnv;
use simplicity_sys::c_jets::frame_ffi::CFrameItem;

impl<T: core::borrow::Borrow<bitcoin::Transaction>> JetEnvironment for BitcoinEnv<T> {
    type Jet = Bitcoin;
    type CJetEnvironment = CTxEnv;

    fn c_jet_env(&self) -> &Self::CJetEnvironment {
        self.c_tx_env()
    }

    fn c_jet_ptr(
        jet: &Self::Jet,
    ) -> fn(&mut CFrameItem, CFrameItem, &Self::CJetEnvironment) -> bool {
        super::init::bitcoin::c_jet_ptr(jet)
    }
}
