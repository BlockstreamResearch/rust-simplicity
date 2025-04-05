// SPDX-License-Identifier: CC0-1.0

#![cfg_attr(fuzzing, no_main)]

#[cfg(any(fuzzing, test))]
fn do_test(data: &[u8]) {
    let mut extractor_1 = simplicity_fuzz::Extractor::new(data);
    let mut extractor_2 = simplicity_fuzz::Extractor::new(data);

    let (val, old_val) = match (
        extractor_1.extract_value_direct(),
        extractor_2.extract_old_value_direct(),
    ) {
        (Some(val), Some(old_val)) => (val, old_val),
        (None, None) => return,
        (Some(_), None) => panic!("Could extract new value but not old."),
        (None, Some(_)) => panic!("Could extract old value but not new."),
    };

    assert!(val.iter_compact().eq(old_val.iter_compact()));
    assert!(val.iter_padded().eq(old_val.iter_padded()));
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
