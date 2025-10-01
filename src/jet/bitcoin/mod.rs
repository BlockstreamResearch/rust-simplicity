// SPDX-License-Identifier: CC0-1.0

mod c_env;
mod environment;

pub use environment::BitcoinEnv;

use super::init::bitcoin::Bitcoin;
use super::JetEnvironment;
use simplicity_sys::c_jets::frame_ffi::CFrameItem;

impl<T: core::borrow::Borrow<bitcoin::Transaction>> JetEnvironment for BitcoinEnv<T> {
    type Jet = Bitcoin;
    type CJetEnvironment = ();

    fn c_jet_env(&self) -> &Self::CJetEnvironment {
        &()
    }

    fn c_jet_ptr(
        jet: &Self::Jet,
    ) -> fn(&mut CFrameItem, CFrameItem, &Self::CJetEnvironment) -> bool {
        super::init::bitcoin::c_jet_ptr(jet)
    }
}
