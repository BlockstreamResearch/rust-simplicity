// SPDX-License-Identifier: CC0-1.0

#![cfg_attr(fuzzing, no_main)]

#[cfg(any(fuzzing, test))]
fn do_test(data: &[u8]) -> Option<()> {
    use simplicity::dag::{DagLike, NoSharing};
    use simplicity::types::{CompleteBound, Final};

    let mut extractor = simplicity_fuzz::Extractor::new(data);

    let val = extractor.extract_value_padded()?;
    let ty = val.ty();
    
    // Construct a smaller type
    let mut stack = vec![];
    for node in ty.post_order_iter::<NoSharing>() {
        match node.node.bound() {
            CompleteBound::Unit => stack.push(Final::unit()),
            CompleteBound::Sum(..) => {
                let right = stack.pop().unwrap();
                let left = stack.pop().unwrap();
                stack.push(Final::sum(left, right));
            }
            CompleteBound::Product(..) => {
                let mut right = stack.pop().unwrap();
                let mut left = stack.pop().unwrap();
                if extractor.extract_bit()? {
                    left = Final::unit();
                }
                if extractor.extract_bit()? {
                    right = Final::unit();
                }
                stack.push(Final::product(left, right));
            }
        }
    }
    let pruned_ty = stack.pop().unwrap();
    assert!(stack.is_empty());


    // Prune the value
    let pruned_val = match val.prune(&pruned_ty) {
        Some(val) => val,
        None => panic!("Failed to prune value {val} from {ty} to {pruned_ty}"),
    };

    /*
    // If you have a regression you likely want to uncomment these printlns.
    println!("Original Value Bits: {:?}", val.iter_padded().collect::<Vec<bool>>());
    println!("Original Value: {val}");
    println!(" Original Type: {ty}");
    println!("  Pruned Value: {pruned_val}");
    println!("   Pruned Type: {pruned_ty}");
    */

    // Check that pruning made sense by going through the compact bit iterator
    // and checking that the pruned value is obtained from the original by
    // just deleting bits.
    let mut orig_iter = val.iter_compact();
    let mut prune_iter = pruned_val.iter_compact();

    loop {
        match (orig_iter.next(), prune_iter.next()) {
            (Some(true), Some(true)) => {},
            (Some(false), Some(false)) => {},
            (Some(_), Some(prune_bit)) => {
                // We get here if the pruned and the original iterator disagree.
                // This should happen iff we deleted some bits from the pruned
                // value, meaning that we just need to ratchet forward the
                // original iterator until we're back on track.
                loop {
                    match orig_iter.next() {
                        Some(orig_bit) => {
                            if orig_bit == prune_bit { break }
                        },
                        None => panic!("original iterator ran out before pruned iterator did"),
                    }
                }
            }
            (None, Some(_)) => panic!("original iterator ran out before pruned iterator did"),
            (_, None) => break, // done once the pruned iterator runs out
        }
    }
    Some(())
}

#[cfg(fuzzing)]
libfuzzer_sys::fuzz_target!(|data| { let _ = do_test(data); });

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
        let _ = super::do_test(&data);
    }
}
