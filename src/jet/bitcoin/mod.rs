// SPDX-License-Identifier: CC0-1.0

mod environment;
pub mod execute;

pub use environment::BitcoinEnv;

use super::init::bitcoin::Bitcoin;
use super::JetEnvironment;
use crate::jet::Jet;
use simplicity_sys::c_jets::frame_ffi::CFrameItem;

impl JetEnvironment for BitcoinEnv {
    type Jet = Bitcoin;
    type CJetEnvironment = BitcoinEnv;

    fn c_jet_env(&self) -> &Self::CJetEnvironment {
        self
    }

    fn c_jet_ptr(
        jet: &Self::Jet,
    ) -> &dyn Fn(&mut CFrameItem, CFrameItem, &Self::CJetEnvironment) -> bool {
        jet.c_jet_ptr()
    }
}
