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

use std::{cmp, fmt, ptr};

use {Node, Program, Value};
use bititer::BitIter;
use types::FinalTypeInner;
use extension;

/// A frame used internally by the Bit Machine to keep track of
/// where we are reading or writing to
struct Frame {
    data: *mut u8,
    abs_pos: isize,
    start: isize,
    len: isize,
}

impl fmt::Debug for Frame {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        unsafe {
            for i in 0..self.len {
                if i == self.abs_pos - self.start {
                    f.write_str("^")?;
                }

                let p = self.data.offset((self.start + i) / 8);
                if *p & (1 << ((self.start + i) % 8)) != 0 {
                    f.write_str("1")?;
                } else {
                    f.write_str("0")?;
                }
            }
        }
        Ok(())
    }
}

impl Iterator for Frame {
    type Item = bool;
    fn next(&mut self) -> Option<bool> {
        if self.abs_pos < self.start + self.len {
            let bit = self.read();
            self.abs_pos += 1;
            Some(bit)
        } else {
            None
        }
    }
}

impl Frame {
    fn read_at_rel(&self, n: isize) -> bool {
        unsafe {
            let p = self.data.offset((self.abs_pos + n) / 8);
            *p & (1 << ((self.abs_pos + n) % 8)) != 0
        }
    }

    fn read(&self) -> bool {
        unsafe {
            let p = self.data.offset(self.abs_pos / 8);
            *p & (1 << (self.abs_pos % 8)) != 0
        }
    }

    fn write(&mut self, b: bool) {
        let mask = 1 << (self.abs_pos % 8);
        unsafe {
            let p = self.data.offset(self.abs_pos / 8);
            if b {
                *p |= mask;
            } else {
                *p &= !mask;
            }
        }
        self.abs_pos += 1;
    }

    fn fwd(&mut self, n: usize) {
        self.abs_pos += n as isize;
    }

    fn back(&mut self, n: usize) {
        self.abs_pos -= n as isize;
    }

    fn copy_from(&mut self, other: &Frame, n: usize) {
        if self.abs_pos % 8 == 0 && other.abs_pos % 8 == 0 {
            unsafe {
                ptr::copy_nonoverlapping(
                    other.data.offset(other.abs_pos / 8),
                    self.data.offset(self.abs_pos / 8),
                    (n + 7) / 8,
                );
                self.abs_pos += n as isize;
            }
        } else {
            for i in 0..n as isize {
                let bit = unsafe {
                    let p = other.data.offset((other.abs_pos + i) / 8);
                    *p & (1 << ((other.abs_pos + i) % 8)) != 0
                };
                self.write(bit);
            }
        }
    }
}

/// An execution context for a Simplicity program
pub struct BitMachine {
    data: Vec<u8>,
    next_pos: isize,
    read: Vec<Frame>,
    write: Vec<Frame>,
}

impl BitMachine {
    /// Construct a Bit Machine with enough space to execute
    /// the given program
    pub fn for_program<Ext: extension::Node>(program: &Program<Ext>) -> BitMachine {
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
        assert!(self.next_pos as usize + len < self.data.len() * 8);
        assert!(self.write.len() + self.read.len() < self.read.capacity());

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
    fn write(&mut self, bit: bool) {
        let idx = self.write.len() - 1;
        self.write[idx].write(bit);
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
        for idx in 0..64 {
            self.write(data & (1 << (63 - idx)) != 0);
        }
    }

    /// Write a big-endian u32 value to the current write frame
    pub(crate) fn write_u32(&mut self, data: u32) {
        for idx in 0..32 {
            self.write(data & (1 << (31 - idx)) != 0);
        }
    }

    /// Write a buch of bytes to the current write frame
    pub(crate) fn write_bytes(&mut self, data: &[u8]) {
        for bit in BitIter::new(data.iter().cloned()) {
            self.write(bit);
        }
    }

    /// Write a value to the current write frame
    fn write_value(&mut self, val: &Value) {
        // FIXME don't recurse
        match *val {
            Value::Unit => {}
            Value::SumL(ref a) => {
                self.write(false);
                self.write_value(a);
            }
            Value::SumR(ref a) => {
                self.write(true);
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
    pub fn exec<Ext: extension::Node>(&mut self, program: &Program<Ext>, txenv: &Ext::TxEnv) -> Value {
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

        let input_width = ip.target_ty.bit_width();
        if input_width > 0 && self.read.is_empty() {
            panic!("Pleas call `Program::input` to add an input value for this program {}", ip);
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
                Node::Unit => {},
                Node::Iden => self.copy(ip.source_ty.bit_width()),
                Node::InjL(t) => {
                    self.write(false);
                    if let FinalTypeInner::Sum(ref a, _) = ip.target_ty.ty {
                        let aw = a.bit_width();
                        self.skip(ip.target_ty.bit_width() - aw - 1);
                        call_stack.push(CallStack::Goto(t));
                    } else {
                        panic!("type error")
                    }
                },
                Node::InjR(t) => {
                    self.write(true);
                    if let FinalTypeInner::Sum(_, ref b) = ip.target_ty.ty {
                        let bw = b.bit_width();
                        self.skip(ip.target_ty.bit_width() - bw - 1);
                        call_stack.push(CallStack::Goto(t));
                    } else {
                        panic!("type error")
                    }
                },
                Node::Pair(s, t) => {
                    call_stack.push(CallStack::Goto(t));
                    call_stack.push(CallStack::Goto(s));
                },
                Node::Comp(s, t) => {
                    let size = program.nodes[s].target_ty.bit_width();
                    self.new_frame(size);

                    call_stack.push(CallStack::DropFrame);
                    call_stack.push(CallStack::Goto(t));
                    call_stack.push(CallStack::MoveFrame);
                    call_stack.push(CallStack::Goto(s));
                },
                Node::Disconnect(s, t) => {
                    // Write `t`'s CMR followed by `s` input to a new read frame
                    let size = program.nodes[s].source_ty.bit_width();
                    assert!(size >= 256);
                    self.new_frame(size);
                    self.write_bytes(&program.nodes[t].cmr);
                    self.copy(size - 256);
                    self.move_frame();

                    let s_target_size = program.nodes[s].target_ty.bit_width();
                    self.new_frame(s_target_size);
                    // Then recurse. Remembering that call stack pushes are executed
                    // in reverse order:

                    // 3. Delete the two frames we created, which have both moved to the read stack
                    call_stack.push(CallStack::DropFrame);
                    call_stack.push(CallStack::DropFrame);
                    // 2. Copy the first half of `s`s output directly then execute `t` on the second half
                    call_stack.push(CallStack::Goto(t));
                    let b_size = s_target_size - program.nodes[t].source_ty.bit_width();
                    call_stack.push(CallStack::CopyFwd(b_size));
                    // 1. Execute `s` then move the write frame to the read frame for `t`
                    call_stack.push(CallStack::MoveFrame);
                    call_stack.push(CallStack::Goto(s));
                },
                Node::Take(t) => call_stack.push(CallStack::Goto(t)),
                Node::Drop(t) => {
                    if let FinalTypeInner::Product(ref a, _) = ip.source_ty.ty {
                        let aw = a.bit_width();
                        self.fwd(aw);
                        call_stack.push(CallStack::Back(aw));
                        call_stack.push(CallStack::Goto(t));
                    } else {
                        panic!("type error")
                    }
                },
                Node::Case(s, t) => {
                    let sw = self.read[self.read.len() - 1].read();
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
                        call_stack.push(CallStack::Goto(t));
                    } else {
                        self.fwd(1 + cmp::max(aw, bw) - aw);
                        call_stack.push(CallStack::Back(1 + cmp::max(aw, bw) - aw));
                        call_stack.push(CallStack::Goto(s));
                    }
                },
                Node::Witness(ref value) => self.write_value(value),
                Node::Hidden(ref h) => panic!("Hit hidden node {} at iter {}: {}", ip, iters, h),
                Node::Ext(ref e) => e.exec(self, txenv),
    /*
    */
                Node::Jet(ref j) => unimplemented!("jet {}", j),
                Node::Fail(..) => panic!("encountered fail node while executing"),
            }

            ip = loop {
                match call_stack.pop() {
                    Some(CallStack::Goto(next)) => break &program.nodes[next],
                    Some(CallStack::MoveFrame) => self.move_frame(),
                    Some(CallStack::DropFrame) => self.drop_frame(),
                    Some(CallStack::CopyFwd(n)) => {
                        self.copy(n);
                        self.fwd(n);
                    },
                    Some(CallStack::Back(n)) => self.back(n),
                    None => break 'main_loop,
                };
            };
        }

        if output_width > 0 {
            let out_frame = self.write.last_mut().unwrap();
            out_frame.abs_pos -= out_frame.len;
            Value::from_witness(out_frame, &program.root_node().target_ty)
                .expect("unwrapping output value")
        } else {
            Value::Unit
        }
    }
}

