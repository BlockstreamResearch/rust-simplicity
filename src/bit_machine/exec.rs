// Rust Simplicity Library
// Written in 2020 by
//   Andrew Poelstra <apoelstra@blockstream.com>
//
// To the extent possible under law, the author(s) have dedicated all
// copyright and related and neighboring rights to this software to
// the public domain worldwide. This software is distributed without
// any warranty.
//
// You should have received a copy of the CC0 Public Domain Dedication
// along with this software.
// If not, see <http://creativecommons.org/publicdomain/zero/1.0/>.
//

//! # Simplicity Execution
//!
//! Implementation of the Bit Machine, without TCO, as TCO precludes some
//! frame management optimizations which can be used to great benefit.
//!

use super::frame::Frame;
use crate::core::redeem::RedeemNodeInner;
use crate::core::types::TypeInner;
use crate::core::{RedeemNode, Value};
use crate::decode;
use crate::jet::{AppError, Application};
use std::fmt;
use std::{cmp, error};

/// An execution context for a Simplicity program
pub struct BitMachine {
    /// Space for bytes that read and write frames point to.
    /// (De)allocation happens LIFO from left to right
    pub(crate) data: Vec<u8>,
    /// Top of data stack; index of first unused bit
    pub(crate) next_frame_start: usize,
    /// Read frame stack
    pub(crate) read: Vec<Frame>,
    /// Write frame stack
    pub(crate) write: Vec<Frame>,
}

impl BitMachine {
    /// Construct a Bit Machine with enough space to execute the given program.
    pub fn for_program<App: Application>(program: &RedeemNode<App>) -> Self {
        let io_width = program.ty.source.bit_width + program.ty.target.bit_width;

        Self {
            data: vec![0; (io_width + program.bounds.extra_cells + 7) / 8],
            next_frame_start: 0,
            read: Vec::with_capacity(program.bounds.extra_frames + 2),
            write: Vec::with_capacity(program.bounds.extra_frames + 2),
        }
    }

    /// Push a new frame of given size onto the write frame stack
    fn new_frame(&mut self, len: usize) {
        debug_assert!(
            self.next_frame_start + len <= self.data.len() * 8,
            "Data out of bounds: number of cells"
        );
        debug_assert!(
            self.write.len() + self.read.len() < self.read.capacity(),
            "Stacks out of bounds: number of frames"
        );

        self.write.push(Frame::new(self.next_frame_start, len));
        self.next_frame_start += len;
    }

    /// Move the active write frame to the read frame stack
    fn move_frame(&mut self) {
        let mut _active_write_frame = self.write.pop().unwrap();
        _active_write_frame.reset_cursor();
        self.read.push(_active_write_frame);
    }

    /// Drop the active read frame
    fn drop_frame(&mut self) {
        let active_read_frame = self.read.pop().unwrap();
        self.next_frame_start -= active_read_frame.len;
        assert_eq!(self.next_frame_start, active_read_frame.start);
    }

    /// Write a single bit to the active write frame
    pub(crate) fn write_bit(&mut self, bit: bool) {
        self.write
            .last_mut()
            .expect("Empty write frame stack")
            .write_bit(bit, &mut self.data);
    }

    /// Move the cursor of the active write frame forward by
    /// a specified number of bits
    pub(crate) fn skip(&mut self, n: usize) {
        // short circuit n = 0
        if n == 0 {
            return;
        }
        let idx = self.write.len() - 1;
        self.write[idx].move_cursor_forward(n);
    }

    /// Copy the given number of bits from the active read frame
    /// to the active write frame
    fn copy(&mut self, n: usize) {
        // short circuit n = 0
        if n == 0 {
            return;
        }
        let widx = self.write.len() - 1;
        let ridx = self.read.len() - 1;
        self.write[widx].copy_from(&self.read[ridx], n, &mut self.data);
    }

    /// Move the cursor of the active read frame forward
    /// by the given number of bits
    fn fwd(&mut self, n: usize) {
        // short circuit n = 0
        if n == 0 {
            return;
        }
        let idx = self.read.len() - 1;
        self.read[idx].move_cursor_forward(n);
    }

    /// Move the cursor of the active read frame back
    /// by the given number of bits
    fn back(&mut self, n: usize) {
        // short circuit n = 0
        if n == 0 {
            return;
        }
        let idx = self.read.len() - 1;
        self.read[idx].move_cursor_backward(n);
    }

    /// Write a big-endian u64 value to the active write frame
    pub(crate) fn write_u64(&mut self, value: u64) {
        self.write
            .last_mut()
            .expect("Empty write frame stack")
            .write_u64(value, &mut self.data);
    }

    /// Write a big-endian u32 value to the active write frame
    pub(crate) fn write_u32(&mut self, value: u32) {
        self.write
            .last_mut()
            .expect("Empty write frame stack")
            .write_u32(value, &mut self.data);
    }

    /// Write a big-endian u16 value to the active write frame
    pub(crate) fn write_u16(&mut self, value: u16) {
        self.write
            .last_mut()
            .expect("Empty write frame stack")
            .write_u16(value, &mut self.data);
    }

    /// Write a big-endian u8 value to the active write frame
    pub(crate) fn write_u8(&mut self, value: u8) {
        self.write
            .last_mut()
            .expect("Empty write frame stack")
            .write_u8(value, &mut self.data);
    }

    /// Read a big-endian u64 value from the active read frame
    pub(crate) fn read_u64(&mut self) -> u64 {
        self.read
            .last_mut()
            .expect("Empty read frame stack")
            .read_u64(&self.data)
    }

    /// Read a big-endian u32 value from the active read frame
    pub(crate) fn read_u32(&mut self) -> u32 {
        self.read
            .last_mut()
            .expect("Empty read frame stack")
            .read_u32(&self.data)
    }

    /// Read a big-endian u16 value from the active read frame
    pub(crate) fn read_u16(&mut self) -> u16 {
        self.read
            .last_mut()
            .expect("Empty read frame stack")
            .read_u16(&self.data)
    }

    /// Read a big-endian u8 value from the active read frame
    pub(crate) fn read_u8(&mut self) -> u8 {
        self.read
            .last_mut()
            .expect("Empty read frame stack")
            .read_u8(&self.data)
    }

    /// Read a bit from the active read frame
    pub(crate) fn read_bit(&mut self) -> bool {
        self.read
            .last_mut()
            .expect("Empty read frame stack")
            .read_bit(&self.data)
    }

    /// Read 32 bytes from the active read frame
    pub(crate) fn read_32bytes(&mut self) -> [u8; 32] {
        let mut ret = [0u8; 32];
        for byte in &mut ret {
            *byte = self
                .read
                .last_mut()
                .expect("Empty read frame stack")
                .read_u8(&self.data);
        }
        ret
    }

    /// Read the given number of bytes from the active read frame
    pub(crate) fn read_bytes(&mut self, n: usize) -> Vec<u8> {
        let mut ret = Vec::with_capacity(n);
        for _i in 0..n {
            ret.push(
                self.read
                    .last_mut()
                    .expect("Empty read frame stack")
                    .read_u8(&self.data),
            );
        }
        ret
    }

    /// Write a bit string to the active write frame
    pub(crate) fn write_bytes(&mut self, bytes: &[u8]) {
        for bit in bytes {
            self.write_u8(*bit);
        }
    }

    /// Write a value to the current write frame
    fn write_value(&mut self, val: &Value) {
        // FIXME don't recurse
        match *val {
            Value::Unit => {}
            Value::SumL(ref a) => {
                self.write_bit(false);
                self.write_value(a);
            }
            Value::SumR(ref a) => {
                self.write_bit(true);
                self.write_value(a);
            }
            Value::Prod(ref a, ref b) => {
                self.write_value(a);
                self.write_value(b);
            }
        }
    }

    /// Add a read frame with some given value in it, as input to the
    /// program
    pub fn input(&mut self, input: &Value) {
        // FIXME typecheck this
        self.new_frame(input.len());
        self.write_value(input);
        self.move_frame();
    }

    /// Execute the given program on the Bit Machine, using the given environment.
    ///
    /// Make sure the Bit Machine has enough space by constructing it via [`Self::for_program()`].
    pub fn exec<'a, App: Application + std::fmt::Debug>(
        &mut self,
        program: &'a RedeemNode<App>,
        env: &App::Environment,
    ) -> Result<Value, ExecutionError<'a>> {
        // Rust cannot use `App` from parent function
        enum CallStack<'a, App: Application> {
            Goto(&'a RedeemNode<App>),
            MoveFrame,
            DropFrame,
            CopyFwd(usize),
            Back(usize),
        }

        let mut ip = program;
        let mut call_stack = vec![];
        let mut iterations = 0u64;

        let input_width = ip.ty.source.bit_width;
        // TODO: convert into crate::Error
        if input_width > 0 && self.read.is_empty() {
            panic!("Program requires a non-empty input to execute");
        }
        let output_width = ip.ty.target.bit_width;
        if output_width > 0 {
            self.new_frame(output_width);
        }

        'main_loop: loop {
            iterations += 1;
            if iterations % 1_000_000_000 == 0 {
                println!("({:5} M) exec {:?}", iterations / 1_000_000, ip);
            }

            match &ip.inner {
                RedeemNodeInner::Unit => {}
                RedeemNodeInner::Iden => {
                    let size_a = ip.ty.source.bit_width;
                    self.copy(size_a);
                }
                RedeemNodeInner::InjL(left) => {
                    let padl_b_c = if let TypeInner::Sum(b, _) = &ip.ty.target.inner {
                        ip.ty.target.bit_width - b.bit_width - 1
                    } else {
                        unreachable!()
                    };

                    self.write_bit(false);
                    self.skip(padl_b_c);
                    call_stack.push(CallStack::Goto(left));
                }
                RedeemNodeInner::InjR(left) => {
                    let padr_b_c = if let TypeInner::Sum(_, c) = &ip.ty.target.inner {
                        ip.ty.target.bit_width - c.bit_width - 1
                    } else {
                        unreachable!()
                    };

                    self.write_bit(true);
                    self.skip(padr_b_c);
                    call_stack.push(CallStack::Goto(left));
                }
                RedeemNodeInner::Pair(left, right) => {
                    call_stack.push(CallStack::Goto(right));
                    call_stack.push(CallStack::Goto(left));
                }
                RedeemNodeInner::Comp(left, right) => {
                    let size_b = left.ty.target.bit_width;

                    self.new_frame(size_b);
                    call_stack.push(CallStack::DropFrame);
                    call_stack.push(CallStack::Goto(right));
                    call_stack.push(CallStack::MoveFrame);
                    call_stack.push(CallStack::Goto(left));
                }
                RedeemNodeInner::Disconnect(left, right) => {
                    let size_prod_256_a = left.ty.source.bit_width;
                    let size_a = size_prod_256_a - 256;
                    let size_prod_b_c = left.ty.target.bit_width;
                    let size_b = size_prod_b_c - right.ty.source.bit_width;

                    self.new_frame(size_prod_256_a);
                    self.write_bytes(right.cmr.as_ref());
                    self.copy(size_a);
                    self.move_frame();
                    self.new_frame(size_prod_b_c);

                    // Remember that call stack pushes are executed in reverse order
                    call_stack.push(CallStack::DropFrame);
                    call_stack.push(CallStack::DropFrame);
                    call_stack.push(CallStack::Goto(right));
                    call_stack.push(CallStack::CopyFwd(size_b));
                    call_stack.push(CallStack::MoveFrame);
                    call_stack.push(CallStack::Goto(left));
                }
                RedeemNodeInner::Take(left) => call_stack.push(CallStack::Goto(left)),
                RedeemNodeInner::Drop(left) => {
                    let size_a = if let TypeInner::Product(a, _) = &ip.ty.source.inner {
                        a.bit_width
                    } else {
                        unreachable!()
                    };

                    self.fwd(size_a);
                    call_stack.push(CallStack::Back(size_a));
                    call_stack.push(CallStack::Goto(left));
                }
                RedeemNodeInner::Case(left, right)
                | RedeemNodeInner::AssertL(left, right)
                | RedeemNodeInner::AssertR(left, right) => {
                    let choice_bit = self.read[self.read.len() - 1].peek_bit(&self.data);

                    let (size_a, size_b) =
                        if let TypeInner::Product(sum_a_b, _c) = &ip.ty.source.inner {
                            if let TypeInner::Sum(a, b) = &sum_a_b.inner {
                                (a.bit_width, b.bit_width)
                            } else {
                                unreachable!()
                            }
                        } else {
                            unreachable!()
                        };

                    if choice_bit {
                        let padr_a_b = cmp::max(size_a, size_b) - size_b;
                        self.fwd(1 + padr_a_b);
                        call_stack.push(CallStack::Back(1 + padr_a_b));
                        call_stack.push(CallStack::Goto(right));
                    } else {
                        let padl_a_b = cmp::max(size_a, size_b) - size_a;
                        self.fwd(1 + padl_a_b);
                        call_stack.push(CallStack::Back(1 + padl_a_b));
                        call_stack.push(CallStack::Goto(left));
                    }
                }
                RedeemNodeInner::Witness(value) => self.write_value(value),
                RedeemNodeInner::Hidden(..) => return Err(ExecutionError::ReachedPrunedBranch),
                RedeemNodeInner::Jet(j) => App::exec_jet(j, self, env)
                    .map_err(|x| ExecutionError::AppError(Box::new(x)))?,
                RedeemNodeInner::Fail(..) => return Err(ExecutionError::ReachedFailNode),
            }

            ip = loop {
                match call_stack.pop() {
                    Some(CallStack::Goto(next)) => break next,
                    Some(CallStack::MoveFrame) => self.move_frame(),
                    Some(CallStack::DropFrame) => self.drop_frame(),
                    Some(CallStack::CopyFwd(n)) => {
                        self.copy(n);
                        self.fwd(n);
                    }
                    Some(CallStack::Back(n)) => self.back(n),
                    None => break 'main_loop,
                };
            };
        }

        if output_width > 0 {
            let out_frame = self.write.last_mut().unwrap();
            out_frame.reset_cursor();
            let value =
                decode::decode_value(&program.ty.target, &mut out_frame.to_frame_data(&self.data))
                    .expect("Decode value of output frame");

            Ok(value)
        } else {
            Ok(Value::Unit)
        }
    }
}

/// Errors related to simplicity Execution
#[derive(Debug)]
pub enum ExecutionError<'a> {
    /// Reached a fail node
    ReachedFailNode,
    /// Reached a pruned branch
    ReachedPrunedBranch,
    /// Application-related error
    AppError(Box<dyn AppError + 'a>),
}

impl<'a> fmt::Display for ExecutionError<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ExecutionError::ReachedFailNode => f.write_str("Execution reached a fail node"),
            ExecutionError::ReachedPrunedBranch => f.write_str("Execution reached a pruned branch"),
            ExecutionError::AppError(e) => e.fmt(f),
        }
    }
}

impl<'a> error::Error for ExecutionError<'a> {}

impl<'a, E: AppError + 'a> From<E> for ExecutionError<'a> {
    fn from(error: E) -> Self {
        ExecutionError::AppError(Box::new(error))
    }
}
