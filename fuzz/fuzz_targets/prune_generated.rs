// SPDX-License-Identifier: CC0-1.0

#![cfg_attr(fuzzing, no_main)]

#[cfg(fuzzing)]
libfuzzer_sys::fuzz_target!(|data| {
    simplicity_fuzz::check_prune(data);
});

#[cfg(not(fuzzing))]
fn main() {}
