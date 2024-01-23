// SPDX-License-Identifier: CC0-1.0

use bitcoin::absolute;

/// Environment for Bitcoin Simplicity
pub struct BitcoinEnv {
    pub tx: bitcoin::Transaction,
}

impl BitcoinEnv {
    pub fn new(tx: bitcoin::Transaction) -> Self {
        BitcoinEnv { tx }
    }
}

impl Default for BitcoinEnv {
    fn default() -> Self {
        // FIXME: Review and check if the defaults make sense
        BitcoinEnv::new(bitcoin::Transaction {
            version: bitcoin::transaction::Version::TWO,
            lock_time: absolute::LockTime::ZERO,
            input: vec![],
            output: vec![],
        })
    }
}
