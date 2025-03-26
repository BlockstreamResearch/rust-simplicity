// SPDX-License-Identifier: CC0-1.0

#![cfg_attr(fuzzing, no_main)]

#[cfg(any(fuzzing, test))]
fn do_test(data: &[u8]) -> Option<()> {
    let mut extractor = simplicity_fuzz::Extractor::new(data);
    if extractor.extract_bit()? {
        let _ = extractor.extract_value_direct();
    } else {
        if extractor.extract_bit()? {
            let _ = extractor.extract_value_compact();
        } else {
            let _ = extractor.extract_value_padded();
        }
    }

    Some(())
}

#[cfg(fuzzing)]
libfuzzer_sys::fuzz_target!(|data| {
    let _ = do_test(data);
});

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
