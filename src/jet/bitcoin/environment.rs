// SPDX-License-Identifier: CC0-1.0

use bitcoin::absolute;

/// Environment for Bitcoin Simplicity
pub struct BitcoinEnv {
    /// The transaction being executed.
    pub tx: crate::bitcoin::Transaction,
    /// The outputs being spent by the transaction inputs.
    pub spent_outputs: Vec<crate::bitcoin::TxOut>,
    /// The index of the input currently being executed.
    pub ix: u32,
}

impl BitcoinEnv {
    /// Construct a new Bitcoin environment.
    pub fn new(tx: crate::bitcoin::Transaction, spent_outputs: Vec<crate::bitcoin::TxOut>, ix: u32) -> Self {
        BitcoinEnv {
            tx,
            spent_outputs,
            ix,
        }
    }

    /// Create a dummy environment for testing.
    pub fn dummy() -> Self {
        Self::default()
    }
}

impl Default for BitcoinEnv {
    fn default() -> Self {
        BitcoinEnv::new(
            crate::bitcoin::Transaction {
                version: crate::bitcoin::transaction::Version::TWO,
                lock_time: absolute::LockTime::ZERO,
                input: vec![],
                output: vec![],
            },
            vec![],
            0,
        )
    }
}
