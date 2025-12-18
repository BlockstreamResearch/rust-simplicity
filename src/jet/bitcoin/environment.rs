// SPDX-License-Identifier: CC0-1.0

use simplicity_sys::c_jets::c_env::bitcoin as c_bitcoin;

/// Environment for Bitcoin Simplicity
// In later commit, when we update Jet trait, will remove default type.
pub struct BitcoinEnv<T = bitcoin::Transaction> {
    pub tx: T,
}

impl<T> BitcoinEnv<T>
where
    T: core::borrow::Borrow<bitcoin::Transaction>,
{
    pub fn new(tx: T) -> Self {
        BitcoinEnv { tx }
    }

    pub fn c_tx_env(&self) -> &c_bitcoin::CTxEnv {
        unimplemented!()
    }
}
