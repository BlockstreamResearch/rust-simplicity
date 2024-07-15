// SPDX-License-Identifier: CC0-1.0

use honggfuzz::fuzz;

use simplicity::ffi::tests::{ffi::SimplicityErr, run_program, TestUpTo};
use simplicity::hashes::sha256::Midstate;
use simplicity::jet::Elements;
use simplicity::{BitIter, RedeemNode};

fn do_test(data: &[u8]) {
    // To decode the program length, we first try decoding the program using
    // `decode_expression` which will not error on a length check. Alternately
    // we could decode using RedeemNode::decode and then extract the length
    // from the error return.
    //
    // If the program doesn't decode, just use the first byte as a "length".
    let prog_len = {
        let mut iter = BitIter::from(data);
        match simplicity::decode::decode_expression::<_, Elements>(&mut iter) {
            Ok(_) => (iter.n_total_read() + 7) / 8,
            Err(_) => match data.first() {
                Some(&n) => core::cmp::min(data.len(), n.into()),
                None => return,
            },
        }
    };

    let (program, witness) = data.split_at(prog_len);
    let c_result = run_program(program, witness, TestUpTo::CheckOneOne);

    let prog_iter = BitIter::from(program);
    let wit_iter = BitIter::from(witness);
    let rust_result = RedeemNode::<Elements>::decode(prog_iter, wit_iter);

    match (c_result, rust_result) {
        (Ok(_), Err(e)) => panic!("C accepted code that Rust rejected: {}", e),
        (Err(SimplicityErr::FailCode), Ok(_)) => {} // fine, we parse FAIL but C doesn't
        (Err(e), Ok(_)) => panic!("Rust accepted code that C rejected: {}", e),
        (Err(_), Err(_)) => {} // whatever, just skip
        (Ok(c_data), Ok(rust_data)) => {
            /*
            use simplicity::dag::{DagLike, InternalSharing};
            for data in rust_data.as_ref().post_order_iter::<InternalSharing>() {
                println!("{}: {} [{:?} {:?}]", data.index, data.node.inner(), data.left_index, data.right_index);
            }
            println!("{}", rust_data.cmr());
            println!("{}", rust_data.amr());
            println!("{}", rust_data.imr());
            */
            assert_eq!(&Midstate::from(c_data.cmr)[..], rust_data.cmr().as_ref(),);
            assert_eq!(&Midstate::from(c_data.amr)[..], rust_data.amr().as_ref(),);
            assert_eq!(&Midstate::from(c_data.imr)[..], rust_data.imr().as_ref(),);
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
mod tests {
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
        extend_vec_from_hex("ffffff0000010080800000000000380000001adfc7040000000000000000000007fffffffffffffe1000000000000000000001555600000000000000000000000000000000000000000000000000000000000000000000ffffffffffffff0300000000000000000000000000000000000000000000000000008000000000000000ffffffffffffff7f00000000000000ffffff151515111515155555555555d6eeffffff00", &mut a);
        super::do_test(&a);
    }
}
