// SPDX-License-Identifier: CC0-1.0

#![cfg_attr(fuzzing, no_main)]

#[cfg(any(fuzzing, test))]
use std::sync::Arc;

#[cfg(any(fuzzing, test))]
use old_simplicity::{types::Final as OldFinal, Value as OldValue};
#[cfg(any(fuzzing, test))]
use simplicity::types::Final;

#[cfg(any(fuzzing, test))]
fn convert_ty(new: &Final) -> Option<Arc<OldFinal>> {
    /// Our stack of tasks describing “what we need to do next.”
    enum Task<'a> {
        /// Convert this `Final` into an `OldFinal`.
        NeedType(&'a Final),
        Binary {
            is_sum: bool,
            dupe: bool,
        },
    }

    // We'll push tasks onto this stack until everything is converted.
    let mut task_stack = vec![Task::NeedType(new)];
    // As we finish conversion of subtrees, we store them here along with
    // a count of units. Because the released version of 0.3.0 does not
    // have any typeskip optimization we need to bail out if there are
    // too many units, since otherwise we will OOM in from_compact_bits.
    let mut result_stack: Vec<(usize, Arc<OldFinal>)> = vec![];
    const MAX_UNITS: usize = 1024 * 1024;

    // Process tasks in LIFO order
    while let Some(task) = task_stack.pop() {
        match task {
            Task::NeedType(final_ty) => {
                if final_ty.is_unit() {
                    result_stack.push((1, OldFinal::unit()));
                } else if let Some((left, right)) = final_ty.as_sum() {
                    let dupe = Arc::ptr_eq(left, right);
                    task_stack.push(Task::Binary { is_sum: true, dupe });
                    if !dupe {
                        task_stack.push(Task::NeedType(right));
                    }
                    task_stack.push(Task::NeedType(left));
                } else if let Some((left, right)) = final_ty.as_product() {
                    let dupe = Arc::ptr_eq(left, right);
                    task_stack.push(Task::Binary {
                        is_sum: false,
                        dupe,
                    });
                    if !dupe {
                        task_stack.push(Task::NeedType(right));
                    }
                    task_stack.push(Task::NeedType(left));
                } else {
                    unreachable!();
                }
            }
            Task::Binary { is_sum, dupe } => {
                let right = result_stack.pop().expect("right type missing");
                let left = if dupe {
                    (right.0, Arc::clone(&right.1))
                } else {
                    result_stack.pop().expect("left type missing")
                };
                let new_total = left.0 + right.0;
                if new_total > MAX_UNITS {
                    return None;
                }
                if is_sum {
                    result_stack.push((new_total, OldFinal::sum(left.1, right.1)));
                } else {
                    result_stack.push((new_total, OldFinal::product(left.1, right.1)));
                }
            }
        }
    }

    // At the end, we should have exactly one final type.
    assert_eq!(result_stack.len(), 1, "Internal conversion error");
    let (_, res) = result_stack.pop().unwrap();
    Some(res)
}

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
        (Some(val), None) => panic!("Could extract new value but not old."),
        (None, Some(val)) => panic!("Could extract old value but not new."),
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
