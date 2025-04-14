// SPDX-License-Identifier: CC0-1.0

#![cfg_attr(fuzzing, no_main)]

#[cfg(any(fuzzing, test))]
fn do_test(data: &[u8]) {
    use simplicity::{jet::Core, BitIter, CommitNode};

    let mut extractor = simplicity_fuzz::Extractor::new(data);

    let construct =
        match extractor.extract_core_construct_node(Some(simplicity_fuzz::ProgramControl {
            enable_type_bomb: false,
            enable_disconnect: false,
            enable_witness: false,
            enable_fail: false,
            enable_asserts: true,
            max_nodes: Some(25),
        })) {
            Some(x) => x,
            None => return,
        };
    //println!("constructed {construct}");
    let finalized = match construct.finalize_types() {
        Ok(x) => x,
        Err(_) => return,
    };
    //println!("finalized {finalized}");
    let prog = finalized.encode_to_vec();
    //println!("{}", simplicity::bitcoin::hex::DisplayHex::as_hex(&prog));
    let prog = BitIter::from(prog);
    let decode = CommitNode::<Core>::decode(prog).unwrap();
    assert_eq!(
        finalized, decode,
        "Constructed committed LHS; encoded and decoded to get RHS",
    );
}

#[cfg(fuzzing)]
libfuzzer_sys::fuzz_target!(|data| do_test(data));

#[cfg(not(fuzzing))]
fn main() {}

#[cfg(test)]
mod tests {
    use base64::Engine;

    #[test]
    fn duplicate_crash() {
        let data = base64::prelude::BASE64_STANDARD
            .decode("Cg==")
            .expect("base64 should be valid");
        super::do_test(&data);
    }
}
