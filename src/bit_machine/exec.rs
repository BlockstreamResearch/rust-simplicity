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

use std::cmp;

use crate::bititer::BitIter;
use crate::core::types::FinalTypeInner;
use crate::extension;
use crate::Program;
use crate::Term;
use crate::Value;

use crate::extension::Jet as JetNode;

use super::frame::Frame;

/// An execution context for a Simplicity program
pub struct BitMachine {
    /// Data corresponding to bitMachine
    pub(crate) data: Vec<u8>,
    /// Current position of the cursor
    pub(crate) next_pos: isize,
    /// Read frame stack
    pub(crate) read: Vec<Frame>,
    /// Write frame stack
    pub(crate) write: Vec<Frame>,
}

impl BitMachine {
    /// Construct a Bit Machine with enough space to execute
    /// the given program
    pub fn for_program<Ext: extension::Jet>(program: &Program<Ext>) -> BitMachine {
        let prog = program.root_node();
        let io_width = prog.source_ty.bit_width() + prog.target_ty.bit_width();
        BitMachine {
            data: vec![0; (io_width + prog.extra_cells_bound + 7) / 8],
            next_pos: 0,
            // +1's for input and output; these are used only for nontrivial
            read: Vec::with_capacity(prog.frame_count_bound + 1),
            write: Vec::with_capacity(prog.frame_count_bound + 1),
        }
    }

    /// Push a new frame of given size onto the write stack
    fn new_frame(&mut self, len: usize) {
        // assert!(self.next_pos as usize + len < self.data.len() * 8);
        // assert!(self.write.len() + self.read.len() < self.read.capacity());

        self.write.push(Frame {
            data: self.data.as_mut_ptr(),
            abs_pos: self.next_pos,
            start: self.next_pos,
            len: len as isize,
        });
        self.next_pos += len as isize;
    }

    /// Move the topmost write frame to the read stack
    fn move_frame(&mut self) {
        let mut pop = self.write.pop().unwrap();
        pop.abs_pos = pop.start;
        self.read.push(pop);
    }

    /// Drop the topmost read frame
    fn drop_frame(&mut self) {
        let d = self.read.pop().unwrap();
        self.next_pos -= d.len;
        assert_eq!(self.next_pos, d.start);
    }

    /// Write a single bit to the current write frame
    pub(crate) fn write_bit(&mut self, bit: bool) {
        self.write
            .last_mut()
            .expect("Empty write frame")
            .write_bit(bit);
    }

    /// Move the cursor of the current write frame forward by
    /// a specified number of bits
    fn skip(&mut self, n: usize) {
        let idx = self.write.len() - 1;
        self.write[idx].fwd(n);
    }

    /// Copy a specified number of bits from the current read
    /// frame to the current write frame
    fn copy(&mut self, n: usize) {
        let widx = self.write.len() - 1;
        let ridx = self.read.len() - 1;
        self.write[widx].copy_from(&self.read[ridx], n);
    }

    /// Move the cursor of the current read frame forward a number of bits
    fn fwd(&mut self, n: usize) {
        let idx = self.read.len() - 1;
        self.read[idx].fwd(n);
    }

    /// Move the cursor of the current read frame back a number of bits
    fn back(&mut self, n: usize) {
        let idx = self.read.len() - 1;
        self.read[idx].back(n);
    }

    /// Write a big-endian u64 value to the current write frame
    pub(crate) fn write_u64(&mut self, data: u64) {
        self.write
            .last_mut()
            .expect("Empty write frame")
            .write_u64(data);
    }

    /// Write a big-endian u32 value to the current write frame
    pub(crate) fn write_u32(&mut self, data: u32) {
        self.write
            .last_mut()
            .expect("Empty write frame")
            .write_u32(data);
    }

    /// Write a big-endian u16 value to the current write frame
    pub(crate) fn write_u16(&mut self, data: u16) {
        self.write
            .last_mut()
            .expect("Empty write frame")
            .write_u16(data);
    }

    /// Write a big-endian u8 value to the current write frame
    pub(crate) fn write_u8(&mut self, data: u8) {
        self.write
            .last_mut()
            .expect("Empty write frame")
            .write_u8(data);
    }

    /// Read a big-endian u64 value to the current read frame
    pub(crate) fn read_u64(&mut self) -> u64 {
        self.read.last_mut().expect("Empty read frame").read_u64()
    }

    /// Read a big-endian u32 value to the current read frame
    pub(crate) fn read_u32(&mut self) -> u32 {
        self.read.last_mut().expect("Empty read frame").read_u32()
    }

    /// Read a big-endian u16 value to the current read frame
    pub(crate) fn read_u16(&mut self) -> u16 {
        self.read.last_mut().expect("Empty read frame").read_u16()
    }

    /// Read a big-endian u8 value to the current read frame
    pub(crate) fn read_u8(&mut self) -> u8 {
        self.read.last_mut().expect("Empty read frame").read_u8()
    }

    /// Read a bit value to the current read frame
    pub(crate) fn read_bit(&mut self) -> bool {
        self.read.last_mut().expect("Empty read frame").read_bit()
    }

    /// Read bytes 32 `u8` bytes to the current read frame
    pub(crate) fn read_32bytes(&mut self) -> [u8; 32] {
        let mut ret = [0u8; 32];
        for byte in &mut ret {
            *byte = self.read.last_mut().expect("Empty read frame").read_u8();
        }
        ret
    }

    /// Read bytes n `u8` bytes to the current read frame
    pub(crate) fn read_bytes(&mut self, n: usize) -> Vec<u8> {
        let mut ret = Vec::with_capacity(n);
        for _i in 0..n {
            ret.push(self.read.last_mut().expect("Empty read frame").read_u8());
        }
        ret
    }

    /// Write a buch of bytes to the current write frame
    pub(crate) fn write_bytes(&mut self, data: &[u8]) {
        for bit in BitIter::new(data.iter().cloned()) {
            self.write_bit(bit);
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

    /// Execute a program in the Bit Machine
    pub fn exec<Ext: extension::Jet>(
        &mut self,
        program: &Program<Ext>,
        txenv: &Ext::TxEnv,
    ) -> Value {
        enum CallStack {
            Goto(usize),
            MoveFrame,
            DropFrame,
            CopyFwd(usize),
            Back(usize),
        };

        let mut ip = program.root_node();
        let mut call_stack = vec![];
        let mut iters = 0u64;

        let input_width = ip.source_ty.bit_width();
        if input_width > 0 && self.read.is_empty() {
            panic!(
                "Pleas call `Program::input` to add an input value for this program {}",
                ip
            );
        }
        let output_width = ip.target_ty.bit_width();
        if output_width > 0 {
            self.new_frame(output_width);
        }

        'main_loop: loop {
            iters += 1;
            if iters % 1_000_000_000 == 0 {
                println!("({:5} M) exec {}", iters / 1_000_000, ip);
            }

            match ip.node {
                Term::Unit => {}
                Term::Iden => self.copy(ip.source_ty.bit_width()),
                Term::InjL(t) => {
                    self.write_bit(false);
                    if let FinalTypeInner::Sum(ref a, _) = ip.target_ty.ty {
                        let aw = a.bit_width();
                        self.skip(ip.target_ty.bit_width() - aw - 1);
                        call_stack.push(CallStack::Goto(ip.index - t));
                    } else {
                        panic!("type error")
                    }
                }
                Term::InjR(t) => {
                    self.write_bit(true);
                    if let FinalTypeInner::Sum(_, ref b) = ip.target_ty.ty {
                        let bw = b.bit_width();
                        self.skip(ip.target_ty.bit_width() - bw - 1);
                        call_stack.push(CallStack::Goto(ip.index - t));
                    } else {
                        panic!("type error")
                    }
                }
                Term::Pair(s, t) => {
                    call_stack.push(CallStack::Goto(ip.index - t));
                    call_stack.push(CallStack::Goto(ip.index - s));
                }
                Term::Comp(s, t) => {
                    let size = program.nodes[ip.index - s].target_ty.bit_width();
                    self.new_frame(size);

                    call_stack.push(CallStack::DropFrame);
                    call_stack.push(CallStack::Goto(ip.index - t));
                    call_stack.push(CallStack::MoveFrame);
                    call_stack.push(CallStack::Goto(ip.index - s));
                }
                Term::Disconnect(s, t) => {
                    // Write `t`'s CMR followed by `s` input to a new read frame
                    let size = program.nodes[ip.index - s].source_ty.bit_width();
                    assert!(size >= 256);
                    self.new_frame(size);
                    self.write_bytes(&program.nodes[ip.index - t].cmr);
                    self.copy(size - 256);
                    self.move_frame();

                    let s_target_size = program.nodes[ip.index - s].target_ty.bit_width();
                    self.new_frame(s_target_size);
                    // Then recurse. Remembering that call stack pushes are executed
                    // in reverse order:

                    // 3. Delete the two frames we created, which have both moved to the read stack
                    call_stack.push(CallStack::DropFrame);
                    call_stack.push(CallStack::DropFrame);
                    // 2. Copy the first half of `s`s output directly then execute `t` on the second half
                    call_stack.push(CallStack::Goto(ip.index - t));
                    let b_size = s_target_size - program.nodes[ip.index - t].source_ty.bit_width();
                    call_stack.push(CallStack::CopyFwd(b_size));
                    // 1. Execute `s` then move the write frame to the read frame for `t`
                    call_stack.push(CallStack::MoveFrame);
                    call_stack.push(CallStack::Goto(ip.index - s));
                }
                Term::Take(t) => call_stack.push(CallStack::Goto(ip.index - t)),
                Term::Drop(t) => {
                    if let FinalTypeInner::Product(ref a, _) = ip.source_ty.ty {
                        let aw = a.bit_width();
                        self.fwd(aw);
                        call_stack.push(CallStack::Back(aw));
                        call_stack.push(CallStack::Goto(ip.index - t));
                    } else {
                        panic!("type error")
                    }
                }
                Term::Case(s, t) => {
                    let sw = self.read[self.read.len() - 1].peek_bit();
                    let aw;
                    let bw;
                    if let FinalTypeInner::Product(ref a, _) = ip.source_ty.ty {
                        if let FinalTypeInner::Sum(ref a, ref b) = a.ty {
                            aw = a.bit_width();
                            bw = b.bit_width();
                        } else {
                            panic!("type error");
                        }
                    } else {
                        panic!("type error");
                    }

                    if sw {
                        self.fwd(1 + cmp::max(aw, bw) - bw);
                        call_stack.push(CallStack::Back(1 + cmp::max(aw, bw) - bw));
                        call_stack.push(CallStack::Goto(ip.index - t));
                    } else {
                        self.fwd(1 + cmp::max(aw, bw) - aw);
                        call_stack.push(CallStack::Back(1 + cmp::max(aw, bw) - aw));
                        call_stack.push(CallStack::Goto(ip.index - s));
                    }
                }
                Term::Witness(ref value) => self.write_value(value),
                Term::Hidden(ref h) => panic!("Hit hidden node {} at iter {}: {}", ip, iters, h),
                Term::Ext(ref e) => e.exec(self, txenv),
                /*
                 */
                Term::Jet(ref j) => j.exec(self, &()),
                Term::Fail(..) => panic!("encountered fail node while executing"),
            }

            ip = loop {
                match call_stack.pop() {
                    Some(CallStack::Goto(next)) => break &program.nodes[next],
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
            out_frame.abs_pos -= out_frame.len;
            Value::from_bits_and_type(out_frame, &program.root_node().target_ty)
                .expect("unwrapping output value")
        } else {
            Value::Unit
        }
    }
}
