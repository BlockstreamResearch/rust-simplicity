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

#[cfg(test)]
#[cfg(feature = "test-utils")]
mod tests {

    use simplicity_sys::c_jets::exec_ffi;

    use crate::bititer::BitIter;
    use crate::jet::Elements;
    use crate::merkle::common::MerkleRoot;
    use crate::RedeemNode;

    // // TODO: check AMR(if we implement it)
    // The new IMR also has type commitments, so we don't really need to check AMR
    fn check_merkle_roots(test: &exec_ffi::TestData) {
        let mut bits = BitIter::new(test.prog.iter().cloned());
        let prog = RedeemNode::<Elements>::decode(&mut bits).unwrap();
        assert_eq!(prog.cmr.into_inner(), test.cmr);
        // assert_eq!(redeem.amr.into_inner(), test.amr);
        assert_eq!(prog.imr.into_inner(), test.imr);
    }

    #[test]
    fn progs_cmr() {
        let hash_block = exec_ffi::hash_block_test_data();
        let schnorr0 = exec_ffi::schnorr0_test_data();
        let schnorr6 = exec_ffi::schnorr6_test_data();
        check_merkle_roots(&hash_block);
        check_merkle_roots(&schnorr0);
        check_merkle_roots(&schnorr6);
    }
}
