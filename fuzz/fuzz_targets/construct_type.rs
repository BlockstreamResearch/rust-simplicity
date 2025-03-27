// SPDX-License-Identifier: CC0-1.0

#![cfg_attr(fuzzing, no_main)]

#[cfg(any(fuzzing, test))]
fn do_test(data: &[u8]) {
    let mut extractor = simplicity_fuzz::Extractor::new(data);
    let _ = extractor.extract_final_type();
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
