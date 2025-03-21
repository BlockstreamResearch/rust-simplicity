// SPDX-License-Identifier: CC0-1.0

use simplicity::types::Final as FinalTy;
use std::sync::Arc;

/// A wrapper around a buffer which has utilities for extracting various
/// Simplicity types.
pub struct Extractor<'f> {
    data: &'f [u8],
    bit_cache: u8,
    bit_len: usize,
}

impl<'f> Extractor<'f> {
    /// Wrap the buffer in an extractor.
    pub fn new(data: &'f [u8]) -> Self {
        Self {
            data,
            bit_cache: 0,
            bit_len: 0,
        }
    }

    /// Attempt to yield a u8 from the fuzzer.
    pub fn extract_u8(&mut self) -> Option<u8> {
        if self.data.is_empty() {
            None
        } else {
            let ret = self.data[0];
            self.data = &self.data[1..];
            Some(ret)
        }
    }

    /// Attempt to yield a single bit from the fuzzer.
    pub fn extract_bit(&mut self) -> Option<bool> {
        if self.bit_len == 0 {
            self.bit_cache = self.extract_u8()?;
            self.bit_len = 8;
        }

        let ret = self.bit_cache & 1 == 1;
        self.bit_len -= 1;
        self.bit_cache >>= 1;
        Some(ret)
    }

    /// Attempt to yield a type from the fuzzer.
    pub fn extract_final_type(&mut self) -> Option<Arc<FinalTy>> {
        // We can costruct extremely large types by duplicating Arcs; there
        // is no need to have an exponential blowup in the number of tasks.
        const MAX_N_TASKS: usize = 300;

        enum StackElem {
            NeedType,
            Binary { is_sum: bool, dupe: bool },
        }

        let mut task_stack = vec![StackElem::NeedType];
        let mut result_stack = vec![];

        while let Some(task) = task_stack.pop() {
            match task {
                StackElem::NeedType => {
                    if self.extract_bit()? {
                        result_stack.push(FinalTy::unit());
                    } else {
                        let is_sum = self.extract_bit()?;
                        let dupe = task_stack.len() >= MAX_N_TASKS || self.extract_bit()?;
                        task_stack.push(StackElem::Binary { is_sum, dupe });
                        if !dupe {
                            task_stack.push(StackElem::NeedType)
                        }
                        task_stack.push(StackElem::NeedType);
                    }
                }
                StackElem::Binary { is_sum, dupe } => {
                    let right = result_stack.pop().unwrap();
                    let left = if dupe {
                        Arc::clone(&right)
                    } else {
                        result_stack.pop().unwrap()
                    };
                    if is_sum {
                        result_stack.push(FinalTy::sum(left, right));
                    } else {
                        result_stack.push(FinalTy::product(left, right));
                    }
                }
            }
        }
        assert_eq!(result_stack.len(), 1);
        result_stack.pop()
    }
}
