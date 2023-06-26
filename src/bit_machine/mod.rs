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

mod frame;

use crate::dag::{DagLike, NoSharing};
use crate::jet::{Jet, JetFailed};
use crate::node::{self, RedeemNode};
use crate::{analysis, types};
use crate::{Cmr, FailEntropy, Value};
use frame::Frame;
use std::fmt;
use std::{cmp, error};

/// An execution context for a Simplicity program
pub struct BitMachine {
    /// Space for bytes that read and write frames point to.
    /// (De)allocation happens LIFO from left to right
    data: Vec<u8>,
    /// Top of data stack; index of first unused bit
    next_frame_start: usize,
    /// Read frame stack
    read: Vec<Frame>,
    /// Write frame stack
    write: Vec<Frame>,
}

impl BitMachine {
    /// Construct a Bit Machine with enough space to execute the given program.
    pub fn for_program<J: Jet>(program: &RedeemNode<J>) -> Self {
        let io_width = program.arrow().source.bit_width() + program.arrow().target.bit_width();

        Self {
            data: vec![0; (io_width + program.bounds().extra_cells + 7) / 8],
            next_frame_start: 0,
            read: Vec::with_capacity(program.bounds().extra_frames + analysis::IO_EXTRA_FRAMES),
            write: Vec::with_capacity(program.bounds().extra_frames + analysis::IO_EXTRA_FRAMES),
        }
    }

    #[cfg(test)]
    pub fn test_exec<J: Jet>(
        program: std::sync::Arc<crate::node::ConstructNode<J>>,
        env: &J::Environment,
    ) -> Result<Value, ExecutionError> {
        use crate::node::SimpleFinalizer;

        let prog = program
            .finalize_types_non_program()
            .expect("finalizing types")
            .finalize(&mut SimpleFinalizer::new(None.into_iter()))
            .expect("finalizing");
        let mut mac = BitMachine::for_program(&prog);
        mac.exec(&prog, env)
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
    fn write_bit(&mut self, bit: bool) {
        self.write
            .last_mut()
            .expect("Empty write frame stack")
            .write_bit(bit, &mut self.data);
    }

    /// Move the cursor of the active write frame forward by
    /// a specified number of bits
    fn skip(&mut self, n: usize) {
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

    /// Write a big-endian u8 value to the active write frame
    fn write_u8(&mut self, value: u8) {
        self.write
            .last_mut()
            .expect("Empty write frame stack")
            .write_u8(value, &mut self.data);
    }

    /// Read a bit from the active read frame
    fn read_bit(&mut self) -> bool {
        self.read
            .last_mut()
            .expect("Empty read frame stack")
            .read_bit(&self.data)
    }

    /// Write a bit string to the active write frame
    fn write_bytes(&mut self, bytes: &[u8]) {
        for bit in bytes {
            self.write_u8(*bit);
        }
    }

    /// Write a value to the current write frame
    fn write_value(&mut self, val: &Value) {
        for val in val.pre_order_iter::<NoSharing>() {
            match val {
                Value::Unit => {}
                Value::SumL(..) => self.write_bit(false),
                Value::SumR(..) => self.write_bit(true),
                Value::Prod(..) => {}
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
    pub fn exec<J: Jet + std::fmt::Debug>(
        &mut self,
        program: &RedeemNode<J>,
        env: &J::Environment,
    ) -> Result<Value, ExecutionError> {
        enum CallStack<'a, J: Jet> {
            Goto(&'a RedeemNode<J>),
            MoveFrame,
            DropFrame,
            CopyFwd(usize),
            Back(usize),
        }

        // Not used, but useful for debugging, so keep it around
        impl<'a, J: Jet> fmt::Debug for CallStack<'a, J> {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                match self {
                    CallStack::Goto(ins) => write!(f, "goto {}", ins.inner()),
                    CallStack::MoveFrame => f.write_str("move frame"),
                    CallStack::DropFrame => f.write_str("drop frame"),
                    CallStack::CopyFwd(n) => write!(f, "copy/fwd {}", n),
                    CallStack::Back(n) => write!(f, "back {}", n),
                }
            }
        }

        let mut ip = program;
        let mut call_stack = vec![];
        let mut iterations = 0u64;

        let input_width = ip.arrow().source.bit_width();
        // TODO: convert into crate::Error
        assert!(
            self.read.is_empty() || input_width > 0,
            "Program requires a non-empty input to execute",
        );
        let output_width = ip.arrow().target.bit_width();
        if output_width > 0 {
            self.new_frame(output_width);
        }

        'main_loop: loop {
            iterations += 1;
            if iterations % 1_000_000_000 == 0 {
                println!("({:5} M) exec {:?}", iterations / 1_000_000, ip);
            }

            match ip.inner() {
                node::Inner::Unit => {}
                node::Inner::Iden => {
                    let size_a = ip.arrow().source.bit_width();
                    self.copy(size_a);
                }
                node::Inner::InjL(left) => {
                    let padl_b_c =
                        if let types::CompleteBound::Sum(b, _) = &ip.arrow().target.bound() {
                            ip.arrow().target.bit_width() - b.bit_width() - 1
                        } else {
                            unreachable!()
                        };

                    self.write_bit(false);
                    self.skip(padl_b_c);
                    call_stack.push(CallStack::Goto(left));
                }
                node::Inner::InjR(left) => {
                    let padr_b_c =
                        if let types::CompleteBound::Sum(_, c) = &ip.arrow().target.bound() {
                            ip.arrow().target.bit_width() - c.bit_width() - 1
                        } else {
                            unreachable!()
                        };

                    self.write_bit(true);
                    self.skip(padr_b_c);
                    call_stack.push(CallStack::Goto(left));
                }
                node::Inner::Pair(left, right) => {
                    call_stack.push(CallStack::Goto(right));
                    call_stack.push(CallStack::Goto(left));
                }
                node::Inner::Comp(left, right) => {
                    let size_b = left.arrow().target.bit_width();

                    self.new_frame(size_b);
                    call_stack.push(CallStack::DropFrame);
                    call_stack.push(CallStack::Goto(right));
                    call_stack.push(CallStack::MoveFrame);
                    call_stack.push(CallStack::Goto(left));
                }
                node::Inner::Disconnect(left, right) => {
                    let size_prod_256_a = left.arrow().source.bit_width();
                    let size_a = size_prod_256_a - 256;
                    let size_prod_b_c = left.arrow().target.bit_width();
                    let size_b = size_prod_b_c - right.arrow().source.bit_width();

                    self.new_frame(size_prod_256_a);
                    self.write_bytes(right.cmr().as_ref());
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
                node::Inner::Take(left) => call_stack.push(CallStack::Goto(left)),
                node::Inner::Drop(left) => {
                    let size_a =
                        if let types::CompleteBound::Product(a, _) = &ip.arrow().source.bound() {
                            a.bit_width()
                        } else {
                            unreachable!()
                        };

                    self.fwd(size_a);
                    call_stack.push(CallStack::Back(size_a));
                    call_stack.push(CallStack::Goto(left));
                }
                node::Inner::Case(..) | node::Inner::AssertL(..) | node::Inner::AssertR(..) => {
                    let choice_bit = self.read[self.read.len() - 1].peek_bit(&self.data);

                    let (size_a, size_b) = if let types::CompleteBound::Product(sum_a_b, _c) =
                        &ip.arrow().source.bound()
                    {
                        if let types::CompleteBound::Sum(a, b) = &sum_a_b.bound() {
                            (a.bit_width(), b.bit_width())
                        } else {
                            unreachable!()
                        }
                    } else {
                        unreachable!()
                    };

                    match (ip.inner(), choice_bit) {
                        (node::Inner::Case(_, right), true)
                        | (node::Inner::AssertR(_, right), true) => {
                            let padr_a_b = cmp::max(size_a, size_b) - size_b;
                            self.fwd(1 + padr_a_b);
                            call_stack.push(CallStack::Back(1 + padr_a_b));
                            call_stack.push(CallStack::Goto(right));
                        }
                        (node::Inner::Case(left, _), false)
                        | (node::Inner::AssertL(left, _), false) => {
                            let padl_a_b = cmp::max(size_a, size_b) - size_a;
                            self.fwd(1 + padl_a_b);
                            call_stack.push(CallStack::Back(1 + padl_a_b));
                            call_stack.push(CallStack::Goto(left));
                        }
                        (node::Inner::AssertL(_, r_cmr), true) => {
                            return Err(ExecutionError::ReachedPrunedBranch(*r_cmr))
                        }
                        (node::Inner::AssertR(l_cmr, _), false) => {
                            return Err(ExecutionError::ReachedPrunedBranch(*l_cmr))
                        }
                        _ => unreachable!(),
                    }
                }
                node::Inner::Witness(value) => self.write_value(value),
                node::Inner::Jet(jet) => self.exec_jet(*jet, env)?,
                node::Inner::Word(value) => self.write_value(value),
                node::Inner::Fail(entropy) => {
                    return Err(ExecutionError::ReachedFailNode(*entropy))
                }
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
            let value = out_frame
                .as_bit_iter(&self.data)
                .read_value(&program.arrow().target)
                .expect("Decode value of output frame");

            Ok(value)
        } else {
            Ok(Value::Unit)
        }
    }

    fn exec_jet<J: Jet>(&mut self, jet: J, env: &J::Environment) -> Result<(), JetFailed> {
        use simplicity_sys::c_jets::frame_ffi::{c_readBit, c_writeBit, CFrameItem};
        use simplicity_sys::c_jets::round_u_word;

        // Sanity Check: This should never really fail, but still good to do
        if !simplicity_sys::c_jets::sanity_checks() {
            return Err(JetFailed);
        }
        let src_ty_bit_width = jet.source_ty().to_bit_width();
        let target_ty_bit_width = jet.target_ty().to_bit_width();

        let a_frame_size = round_u_word(src_ty_bit_width);
        let b_frame_size = round_u_word(target_ty_bit_width);
        // a_frame_size + b_frame_size must be non-zero unless it is a unit to unit jet
        if a_frame_size == 0 && b_frame_size == 0 {
            return Ok(());
        }
        let mut src_buf = vec![0usize; a_frame_size + b_frame_size];
        let src_ptr_end = unsafe { src_buf.as_mut_ptr().add(a_frame_size) }; // A frame write
        let src_ptr = src_buf.as_mut_ptr(); // A read frame at ptr start
        let dst_ptr_begin = unsafe { src_buf.as_mut_ptr().add(a_frame_size) }; // B read frame at ptr begin
        let dst_ptr_end = unsafe { src_buf.as_mut_ptr().add(a_frame_size + b_frame_size) }; // B write frame at ptr end

        // For jet from type A -> B
        // Jets execution: There is single buffer with a_frame_size + b_frame_size UWORDs
        // ------[ A read frame     ][    B write frame  ]---
        //       ^ src_ptr         ^src_ptr_end(dst_ptr_begin)      ^ dst_ptr_end
        // 1. Write into C bitmachine using A write frame(= src_ptr_end)
        // Precondition satisfied: src_ptr_end is one past the end of slice of UWORDs for A.
        let mut a_frame = unsafe { CFrameItem::new_write(src_ty_bit_width, src_ptr_end) };
        for _ in 0..src_ty_bit_width {
            let bit = self.read_bit();
            unsafe {
                c_writeBit(&mut a_frame, bit);
            }
        }
        self.back(src_ty_bit_width);

        // 2. Execute the jet. src = A read frame, dst = B write frame
        // Precondition satisfied: src_ptr is the start of slice of UWORDs of A.
        let src_frame = unsafe { CFrameItem::new_read(src_ty_bit_width, src_ptr) };
        // Precondition satisfied: dst_ptr_end is one past the end of slice of UWORDs of B.
        let mut dst_frame = unsafe { CFrameItem::new_write(target_ty_bit_width, dst_ptr_end) };
        let jet_fn = jet.c_jet_ptr();
        let c_env = jet.c_jet_env(env);
        let res = jet_fn(&mut dst_frame, src_frame, c_env);

        if !res {
            return Err(JetFailed);
        }

        // 3. Read the result from B read frame
        // Precondition satisfied: dst_ptr_begin is the start of slice of UWORDs of B.
        let mut b_frame = unsafe { CFrameItem::new_read(target_ty_bit_width, dst_ptr_begin) };
        // Read the value from b_frame
        for _ in 0..target_ty_bit_width {
            let bit = unsafe { c_readBit(&mut b_frame) };
            self.write_bit(bit);
        }
        Ok(())
    }
}

/// Errors related to simplicity Execution
#[derive(Debug)]
pub enum ExecutionError {
    /// Reached a fail node
    ReachedFailNode(FailEntropy),
    /// Reached a pruned branch
    ReachedPrunedBranch(Cmr),
    /// Jet failed during execution
    JetFailed(JetFailed),
}

impl fmt::Display for ExecutionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ExecutionError::ReachedFailNode(entropy) => {
                write!(f, "Execution reached a fail node: {}", entropy)
            }
            ExecutionError::ReachedPrunedBranch(hash) => {
                write!(f, "Execution reached a pruned branch: {}", hash)
            }
            ExecutionError::JetFailed(jet_failed) => fmt::Display::fmt(jet_failed, f),
        }
    }
}

impl error::Error for ExecutionError {}

impl From<JetFailed> for ExecutionError {
    fn from(jet_failed: JetFailed) -> Self {
        ExecutionError::JetFailed(jet_failed)
    }
}

#[cfg(test)]
mod tests {
    #[cfg(feature = "elements")]
    use super::*;

    #[cfg(feature = "elements")]
    use crate::bitcoin_hashes::hex::ToHex;
    #[cfg(feature = "elements")]
    use crate::jet::{elements::ElementsEnv, Elements};
    #[cfg(feature = "elements")]
    use crate::{node::RedeemNode, BitIter};

    #[cfg(feature = "elements")]
    fn run_program_elements(
        prog_bytes: &[u8],
        cmr_str: &str,
        amr_str: &str,
        imr_str: &str,
    ) -> Result<Value, ExecutionError> {
        let prog_hex = prog_bytes.to_hex();

        let mut iter = BitIter::from(prog_bytes);
        let prog = match RedeemNode::<Elements>::decode(&mut iter) {
            Ok(prog) => prog,
            Err(e) => panic!("program {} failed: {}", prog_hex, e),
        };

        // Check Merkle roots; check AMR last because the AMR is a more complicated
        // calculation and historically has been much more likely to be wrong.
        assert_eq!(
            prog.cmr().to_string(),
            cmr_str,
            "CMR mismatch (got {} expected {}) for program {}",
            prog.cmr().to_string(),
            cmr_str,
            prog_hex,
        );
        assert_eq!(
            prog.imr().to_string(),
            imr_str,
            "IMR mismatch (got {} expected {}) for program {}",
            prog.imr().to_string(),
            imr_str,
            prog_hex,
        );
        assert_eq!(
            prog.amr().to_string(),
            amr_str,
            "AMR mismatch (got {} expected {}) for program {}",
            prog.amr().to_string(),
            amr_str,
            prog_hex,
        );

        // Try to run it on the bit machine and return the result
        let env = ElementsEnv::dummy();
        BitMachine::for_program(&prog).exec(&prog, &env)
    }

    #[test]
    #[cfg(feature = "elements")]
    fn crash_regression1() {
        // cfe18fb44028870400
        // This program caused an array OOB.
        // # Witnesses
        // wit1 = witness :: (((1 + (1 + 2^256)) * (1 + (1 + 2^256))) + 1) -> 1
        //
        // # Program code
        // jt1 = jet_num_outputs    :: 1 -> 2^32                                          # cmr 447165a3...
        // jt2 = jet_issuance_token :: 2^32 -> (1 + (1 + 2^256))                          # cmr 85e9591c...
        // pr3 = pair jt2 jt2       :: 2^32 -> ((1 + (1 + 2^256)) * (1 + (1 + 2^256)))    # cmr 45d40848...
        // cp4 = comp jt1 pr3       :: 1 -> ((1 + (1 + 2^256)) * (1 + (1 + 2^256)))       # cmr 7bb1824f...
        // jl5 = injl cp4           :: 1 -> (((1 + (1 + 2^256)) * (1 + (1 + 2^256))) + 1) # cmr 277ee32c...
        //
        // main = comp jl5 wit1     :: 1 -> 1                                             # cmr 7050c4a6...

        let res = run_program_elements(
            &[0xcf, 0xe1, 0x8f, 0xb4, 0x40, 0x28, 0x87, 0x04, 0x00],
            "7050c4a60140e0ded5d6205d8471b645580f883918f508a9ee3cff590c633286",
            "50fecf371c06d1aeaf599d267cb305d81d00e7412ffb9172d4098394b3a20533",
            "2539210870790f5a74f636a5582c03be95428dfe96619b69d103804b089ceef8",
        );
        assert_eq!(res.unwrap(), Value::Unit);
    }
}
