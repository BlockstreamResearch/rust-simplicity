// Rust Simplicity Library
// Written in 2020 by
//   Andrew Poelstra <apoelstra@blockstream.com>
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

use honggfuzz::fuzz;

use simplicity::bitcoin_hashes::sha256::Midstate;
use simplicity::ffi::{tests, ffi::SimplicityErr};
use simplicity::jet::Elements;
use simplicity::{BitIter, RedeemNode};

fn do_test(data: &[u8]) {
    let c_result = tests::run_program(data, tests::TestUpTo::CheckOneOne);

    let mut iter = BitIter::from(data);
    let rust_result = RedeemNode::<Elements>::decode(&mut iter);

    match (c_result, rust_result) {
        (Ok(_), Err(e)) => panic!("C accepted code that Rust rejected: {}", e),
        (Err(SimplicityErr::FailCode), Ok(_)) => {} // fine, we parse FAIL but C doesn't
        (Err(e), Ok(_)) => panic!("Rust accepted code that C rejected: {}", e),
        (Err(_), Err(_)) => {} // whatever, just skip
        (Ok(c_data), Ok(rust_data)) => {
            /*
            use simplicity::dag::{DagLike, InternalSharing};
            for data in rust_data.as_ref().post_order_iter::<InternalSharing>() {
                println!("{}: {} [{:?} {:?}]", data.index, data.node.inner, data.left_index, data.right_index);
            }
            println!("{}", rust_data.cmr);
            println!("{}", rust_data.amr);
            println!("{}", rust_data.imr);
            */
            assert_eq!(&Midstate::from(c_data.cmr)[..], rust_data.cmr.as_ref(),);
            assert_eq!(&Midstate::from(c_data.amr)[..], rust_data.amr.as_ref(),);
            assert_eq!(&Midstate::from(c_data.imr)[..], rust_data.imr.as_ref(),);
        }
    }
}

fn main() {
    loop {
        fuzz!(|data| {
            do_test(data);
        });
    }
}

#[cfg(test)]
mod unit_tests {
    fn extend_vec_from_hex(hex: &str, out: &mut Vec<u8>) {
        let mut b = 0;
        for (idx, c) in hex.as_bytes().iter().enumerate() {
            b <<= 4;
            match *c {
                b'A'..=b'F' => b |= c - b'A' + 10,
                b'a'..=b'f' => b |= c - b'a' + 10,
                b'0'..=b'9' => b |= c - b'0',
                _ => panic!("Bad hex"),
            }
            if (idx & 1) == 1 {
                out.push(b);
                b = 0;
            }
        }
    }

    #[test]
    fn duplicate_crash() {
        let mut a = Vec::new();
        extend_vec_from_hex("7b88f500", &mut a);
        super::do_test(&a);
    }
}
