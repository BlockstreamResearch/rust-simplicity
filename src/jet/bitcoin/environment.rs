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
            version: 2,
            lock_time: absolute::LockTime::ZERO,
            input: vec![],
            output: vec![],
        })
    }
}
