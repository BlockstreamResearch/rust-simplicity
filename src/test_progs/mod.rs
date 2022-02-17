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
mod hashblock;
mod schnorr0;
mod schnorr6;

#[cfg(test)]
mod tests {
    use super::hashblock::{HASHBLOCK, HASHBLOCK_AMR, HASHBLOCK_CMR};
    use super::schnorr0::{SCHNORR0, SCHNORR0_AMR, SCHNORR0_CMR};
    use super::schnorr6::{SCHNORR6, SCHNORR6_AMR, SCHNORR6_CMR};
    use crate::extension::dummy::{DummyNode, TxEnv};
    use crate::util::bitvec_to_bytevec;
    use crate::Value;

    fn check_merkle_roots(prog: &[u8], cmr: [u8; 32], amr: [u8; 32]) {
        let mut bits: crate::bititer::BitIter<_> = prog.iter().cloned().into();
        let program =
            crate::program::Program::<DummyNode>::decode(&mut bits).expect("decoding program");
        assert_eq!(program.root_node().cmr.into_inner(), cmr);
        assert_eq!(program.root_node().amr.into_inner(), amr);
    }

    fn exec_prog(prog: &[u8]) {
        let mut bits: crate::bititer::BitIter<_> = prog.iter().cloned().into();
        let program =
            crate::program::Program::<DummyNode>::decode(&mut bits).expect("decoding program");
        let txenv = TxEnv;
        //finally run the program
        let mut mac = crate::exec::BitMachine::for_program(&program);
        mac.exec(&program, &txenv).unwrap();
    }

    #[test]
    fn progs_cmr_amr() {
        check_merkle_roots(&HASHBLOCK, HASHBLOCK_CMR, HASHBLOCK_AMR);
        check_merkle_roots(&SCHNORR0, SCHNORR0_CMR, SCHNORR0_AMR);
        check_merkle_roots(&SCHNORR6, SCHNORR6_CMR, SCHNORR6_AMR);
    }

    #[test]
    fn exec_hashblock() {
        let mut bits: crate::bititer::BitIter<_> = HASHBLOCK.iter().cloned().into();
        let program =
            crate::program::Program::<DummyNode>::decode(&mut bits).expect("decoding program");
        let txenv = TxEnv;
        //finally run the program
        let mut mac = crate::exec::BitMachine::for_program(&program);

        let h = [
            0x6a, 0x09, 0xe6, 0x67, 0xbb, 0x67, 0xae, 0x85, 0x3c, 0x6e, 0xf3, 0x72, 0xa5, 0x4f,
            0xf5, 0x3a, 0x51, 0x0e, 0x52, 0x7f, 0x9b, 0x05, 0x68, 0x8c, 0x1f, 0x83, 0xd9, 0xab,
            0x5b, 0xe0, 0xcd, 0x19,
        ];
        let h = Value::u256_from_slice(&h);
        let mut block = [0u8; 64];
        block[0] = 0x61;
        block[1] = 0x62;
        block[2] = 0x63;
        block[3] = 0x80;
        block[63] = 0x18;
        let block = Value::u512_from_slice(&block);
        mac.input(&Value::prod(h, block));
        let expected: [u8; 32] = [
            0xba, 0x78, 0x16, 0xbf, 0x8f, 0x1, 0xcf, 0xea, 0x41, 0x41, 0x40, 0xde, 0x5d, 0xae,
            0x22, 0x23, 0xb0, 0x3, 0x61, 0xa3, 0x96, 0x17, 0x7a, 0x9c, 0xb4, 0x10, 0xff, 0x61,
            0xf2, 0x0, 0x15, 0xad,
        ];
        let output = mac.exec(&program, &txenv).unwrap();
        assert_eq!(bitvec_to_bytevec(&output.into_bits()), expected);
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
