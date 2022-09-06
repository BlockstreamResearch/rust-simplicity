// Rust Simplicity Library
// Written in 2020 by
//   Andrew Poelstra <apoelstra@blockstream.com>
//   Sanket K <sanket1729@blockstream.com>
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

/// Test programs for simplicity.
pub(crate) mod hashblock;
pub(crate) mod schnorr0;
pub(crate) mod schnorr6;
pub(crate) mod sighash_all;

#[cfg(test)]
mod tests {
    use super::hashblock::{HASHBLOCK, HASHBLOCK_CMR};
    use crate::bititer::BitIter;
    use crate::core::{CommitNode, RedeemNode, Value};
    use crate::exec::BitMachine;
    use crate::jet::application::Core;
    use crate::merkle::common::MerkleRoot;
    use crate::test_progs::schnorr0::{SCHNORR0, SCHNORR0_CMR};
    use crate::test_progs::schnorr6::{SCHNORR6, SCHNORR6_CMR};

    // TODO: check IMR
    fn check_merkle_roots(bytes: &[u8], cmr: [u8; 32]) {
        let mut bits = BitIter::new(bytes.iter().cloned());
        let commit = CommitNode::<_, Core>::decode(&mut bits).expect("decoding program");
        assert_eq!(commit.cmr.into_inner(), cmr);
    }

    fn exec_prog(bytes: &[u8]) {
        let mut bits = BitIter::new(bytes.iter().cloned());
        let program = RedeemNode::<_, Core>::decode(&mut bits).expect("decode");

        let mut mac = BitMachine::for_program(&program);
        mac.exec(&program, &()).unwrap();
    }

    #[test]
    fn progs_cmr() {
        check_merkle_roots(&HASHBLOCK, HASHBLOCK_CMR);
        check_merkle_roots(&SCHNORR0, SCHNORR0_CMR);
        check_merkle_roots(&SCHNORR6, SCHNORR6_CMR);
    }

    #[test]
    fn exec_hashblock() {
        let mut bits = BitIter::new(HASHBLOCK.iter().cloned());
        let program = RedeemNode::<_, Core>::decode(&mut bits).expect("decoding program");

        let state = Value::u256_from_slice(&[
            0x6a, 0x09, 0xe6, 0x67, 0xbb, 0x67, 0xae, 0x85, 0x3c, 0x6e, 0xf3, 0x72, 0xa5, 0x4f,
            0xf5, 0x3a, 0x51, 0x0e, 0x52, 0x7f, 0x9b, 0x05, 0x68, 0x8c, 0x1f, 0x83, 0xd9, 0xab,
            0x5b, 0xe0, 0xcd, 0x19,
        ]);
        let mut block = [0u8; 64];
        block[0] = 0x61;
        block[1] = 0x62;
        block[2] = 0x63;
        block[3] = 0x80;
        block[63] = 0x18;
        let block = Value::u512_from_slice(&block);

        let hash: [u8; 32] = [
            0xba, 0x78, 0x16, 0xbf, 0x8f, 0x1, 0xcf, 0xea, 0x41, 0x41, 0x40, 0xde, 0x5d, 0xae,
            0x22, 0x23, 0xb0, 0x3, 0x61, 0xa3, 0x96, 0x17, 0x7a, 0x9c, 0xb4, 0x10, 0xff, 0x61,
            0xf2, 0x0, 0x15, 0xad,
        ];

        let mut mac = BitMachine::for_program(&program);
        mac.input(&Value::prod(state, block));
        let output = mac.exec(&program, &()).unwrap();

        assert_eq!(output.try_to_bytes().unwrap(), hash);
    }

    #[test]
    #[ignore] // too expensive to run. Run with -- --ignored and --release to run this
    #[should_panic]
    fn schnorr6() {
        exec_prog(&SCHNORR6);
    }

    #[test]
    #[ignore] // too expensive to run. Run with -- --ignored and --release to run this
    fn schnorr0() {
        exec_prog(&SCHNORR0);
    }
}
