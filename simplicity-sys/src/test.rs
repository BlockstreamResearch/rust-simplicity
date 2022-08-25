// Simplicity Bindings
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

use crate::util;

extern "C" {
    static schnorr0: [u8; 257];
    static schnorr0_cmr: [u32; 8];
    static schnorr0_imr: [u32; 8];

    static schnorr6: [u8; 405];
    static schnorr6_cmr: [u32; 8];
    static schnorr6_imr: [u32; 8];

    static hashBlock: [u8; 3259];
    static hashBlock_cmr: [u32; 8];
    static hashBlock_imr: [u32; 8];

    static elementsCheckSigHashAllTx1: [u8; 1151];
    static elementsCheckSigHashAllTx1_cmr: [u32; 8];
    static elementsCheckSigHashAllTx1_imr: [u32; 8];
}

/// Simplicity program with bytes, CMR and IMR for testing.
pub struct TestProgram {
    pub bytes: &'static [u8],
    pub(crate) cmr: &'static [u32; 8],
    pub(crate) imr: &'static [u32; 8],
}

impl TestProgram {
    /// Return the CMR of the given program.
    pub fn cmr(&self) -> [u8; 32] {
        util::into_u8_merkle_root(self.cmr)
    }

    /// Return the IMR of the given program.
    pub fn imr(&self) -> [u8; 32] {
        util::into_u8_merkle_root(self.imr)
    }
}

pub static SCHNORR0: TestProgram = unsafe {
    TestProgram {
        bytes: &schnorr0,
        cmr: &schnorr0_cmr,
        imr: &schnorr0_imr,
    }
};

pub static SCHNORR6: TestProgram = unsafe {
    TestProgram {
        bytes: &schnorr6,
        cmr: &schnorr6_cmr,
        imr: &schnorr6_imr,
    }
};

pub static HASHBLOCK: TestProgram = unsafe {
    TestProgram {
        bytes: &hashBlock,
        cmr: &hashBlock_cmr,
        imr: &hashBlock_imr,
    }
};

pub static ELEMENTS_CHECK_SIGHASH_ALL_TX1: TestProgram = unsafe {
    TestProgram {
        bytes: &elementsCheckSigHashAllTx1,
        cmr: &elementsCheckSigHashAllTx1_cmr,
        imr: &elementsCheckSigHashAllTx1_imr,
    }
};
