// SPDX-License-Identifier: CC0-1.0

use old_simplicity::types::Final as OldFinalTy;
use old_simplicity::Value as OldValue;

use simplicity::types::Final as FinalTy;
use simplicity::{BitIter, Value};
use std::sync::Arc;

/// A wrapper around a buffer which has utilities for extracting various
/// Simplicity types.
#[derive(Clone)]
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

    /// Attempt to yield a value from the fuzzer by constructing a type and then
    /// reading a bitstring of that type, in the padded value encoding.
    pub fn extract_value_padded(&mut self) -> Option<Value> {
        let ty = self.extract_final_type()?;
        if ty.bit_width() > 64 * 1024 * 1024 {
            // little fuzzing value in producing massive values
            return None;
        }

        let mut iter = BitIter::new(self.data.iter().copied());
        let ret = Value::from_padded_bits(&mut iter, &ty).ok()?;
        self.data = &self.data[iter.n_total_read().div_ceil(8)..];
        Some(ret)
    }

    /// Attempt to yield a value from the fuzzer by constructing a type and then
    /// reading a bitstring of that type, in the compact value encoding.
    pub fn extract_value_compact(&mut self) -> Option<Value> {
        let ty = self.extract_final_type()?;
        if ty.bit_width() > 64 * 1024 * 1024 {
            // little fuzzing value in producing massive values
            return None;
        }

        let mut iter = BitIter::new(self.data.iter().copied());
        let ret = Value::from_compact_bits(&mut iter, &ty).ok()?;
        self.data = &self.data[iter.n_total_read().div_ceil(8)..];
        Some(ret)
    }

    /// Attempt to yield a value from the fuzzer by constructing it directly.
    pub fn extract_value_direct(&mut self) -> Option<Value> {
        const MAX_N_TASKS: usize = 300;
        const MAX_TY_WIDTH: usize = 10240;

        enum StackElem {
            NeedValue,
            Left,
            Right,
            Product,
        }

        let mut task_stack = vec![StackElem::NeedValue];
        let mut result_stack = vec![];

        while let Some(task) = task_stack.pop() {
            match task {
                StackElem::NeedValue => match (self.extract_bit()?, self.extract_bit()?) {
                    (false, false) => result_stack.push(Value::unit()),
                    (false, true) => {
                        if task_stack.len() <= MAX_N_TASKS {
                            task_stack.push(StackElem::Product);
                            task_stack.push(StackElem::NeedValue);
                            task_stack.push(StackElem::NeedValue);
                        } else {
                            task_stack.push(StackElem::NeedValue);
                        }
                    }
                    (true, false) => {
                        task_stack.push(StackElem::Left);
                        task_stack.push(StackElem::NeedValue);
                    }
                    (true, true) => {
                        task_stack.push(StackElem::Right);
                        task_stack.push(StackElem::NeedValue);
                    }
                },
                StackElem::Product => {
                    let right = result_stack.pop().unwrap();
                    let left = result_stack.pop().unwrap();
                    result_stack.push(Value::product(left, right));
                }
                StackElem::Left => {
                    let child = result_stack.pop().unwrap();
                    let ty = self.extract_final_type()?;
                    if ty.bit_width() > MAX_TY_WIDTH {
                        return None;
                    }
                    result_stack.push(Value::left(child, ty));
                }
                StackElem::Right => {
                    let child = result_stack.pop().unwrap();
                    let ty = self.extract_final_type()?;
                    if ty.bit_width() > MAX_TY_WIDTH {
                        return None;
                    }
                    result_stack.push(Value::right(ty, child));
                }
            }
        }
        assert_eq!(result_stack.len(), 1);
        result_stack.pop()
    }

    /// Attempt to yield a type from the fuzzer.
    pub fn extract_old_final_type(&mut self) -> Option<Arc<OldFinalTy>> {
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
                        result_stack.push(OldFinalTy::unit());
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
                        result_stack.push(OldFinalTy::sum(left, right));
                    } else {
                        result_stack.push(OldFinalTy::product(left, right));
                    }
                }
            }
        }
        assert_eq!(result_stack.len(), 1);
        result_stack.pop()
    }

    /// Attempt to yield a value from the fuzzer by constructing a type and then

    /// Attempt to yield a value from the fuzzer by constructing it directly.
    pub fn extract_old_value_direct(&mut self) -> Option<OldValue> {
        const MAX_N_TASKS: usize = 300;
        const MAX_TY_WIDTH: usize = 10240;

        enum StackElem {
            NeedValue,
            Left,
            Right,
            Product,
        }

        let mut task_stack = vec![StackElem::NeedValue];
        let mut result_stack = vec![];

        while let Some(task) = task_stack.pop() {
            match task {
                StackElem::NeedValue => match (self.extract_bit()?, self.extract_bit()?) {
                    (false, false) => result_stack.push(OldValue::unit()),
                    (false, true) => {
                        if task_stack.len() <= MAX_N_TASKS {
                            task_stack.push(StackElem::Product);
                            task_stack.push(StackElem::NeedValue);
                            task_stack.push(StackElem::NeedValue);
                        } else {
                            task_stack.push(StackElem::NeedValue);
                        }
                    }
                    (true, false) => {
                        task_stack.push(StackElem::Left);
                        task_stack.push(StackElem::NeedValue);
                    }
                    (true, true) => {
                        task_stack.push(StackElem::Right);
                        task_stack.push(StackElem::NeedValue);
                    }
                },
                StackElem::Product => {
                    let right = result_stack.pop().unwrap();
                    let left = result_stack.pop().unwrap();
                    result_stack.push(OldValue::product(left, right));
                }
                StackElem::Left => {
                    let child = result_stack.pop().unwrap();
                    let ty = self.extract_old_final_type()?;
                    if ty.bit_width() > MAX_TY_WIDTH {
                        return None;
                    }
                    result_stack.push(OldValue::left(child, ty));
                }
                StackElem::Right => {
                    let child = result_stack.pop().unwrap();
                    let ty = self.extract_old_final_type()?;
                    if ty.bit_width() > MAX_TY_WIDTH {
                        return None;
                    }
                    result_stack.push(OldValue::right(ty, child));
                }
            }
        }
        assert_eq!(result_stack.len(), 1);
        result_stack.pop()
    }
}
