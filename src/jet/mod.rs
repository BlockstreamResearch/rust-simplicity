// Rust Simplicity Library
// Written in 2022 by
//   Christian Lewe <clewe@blockstream.com>
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

//! # Simplicity jets
//!
//! Jets are special nodes that read a value,
//! process it internally, and write an output value.
//! This evaluation happens in a black-box manner:
//! In terms of the Bit Machine, it is a one-step process.
//!
//! In practice, jets call foreign C code that is equivalent to some Simplicity DAG.
//! This speeds up evaluation tremendously.
//! Equivalence of C and Simplicity is proved using the _Verified Software Toolchain_.
//! Programs are also smaller in size because jets replace large, equivalent Simplicity DAGs.

#[cfg(feature = "bitcoin")]
pub mod bitcoin;
pub mod core;
#[cfg(feature = "elements")]
pub mod elements;
mod init;
pub mod type_name;

#[cfg(feature = "bitcoin")]
pub use init::bitcoin::Bitcoin;
pub use init::core::Core;
#[cfg(feature = "elements")]
pub use init::elements::Elements;
use simplicity_sys::c_jets::c_env::CTxEnv;
use simplicity_sys::c_jets::c_frame::ffi_bytes_size;
use simplicity_sys::c_jets::frame_ffi::{c_readBit, c_writeBit, CFrameItem};

use crate::bititer::BitIter;
use crate::bitwriter::BitWriter;
use crate::core::types::Type;
use crate::exec::BitMachine;
use crate::jet::type_name::TypeName;
use crate::merkle::cmr::Cmr;
use crate::merkle::common::MerkleRoot;
use crate::merkle::imr::Imr;
use crate::Error;
use std::hash::Hash;
use std::io::Write;

/// Generic error that a jet failed during its execution.
///
/// Failure could be due to a failed assertion, an illegal input, etc.
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Debug, Hash)]
pub struct JetFailed;

impl std::fmt::Display for JetFailed {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str("Jet failed during execution")
    }
}

impl std::error::Error for JetFailed {}

/// Family of jets that share an encoding scheme and execution environment.
///
/// Jets are single nodes that read an input,
/// process it internally using foreign C code _(black box)_,
/// and produce an output.
/// Jets may read values from their _environment_.
///
/// Jets are **always** leaves in a Simplicity DAG.
pub trait Jet: Copy + Eq + Ord + Hash + std::fmt::Debug + std::fmt::Display {
    /// Environment for jet to read from
    type Environment: JetEnvironment;

    /// Return the CMR of the jet.
    fn cmr(&self) -> Cmr;

    /// Return the IMR of the jet.
    fn imr(&self) -> Imr {
        self.cmr().into_inner().into()
    }

    /// Return the source type of the jet.
    fn source_ty(&self) -> TypeName;

    /// Return the target type of the jet.
    fn target_ty(&self) -> TypeName;

    /// Encode the jet to bits.
    fn encode<W: Write>(&self, w: &mut BitWriter<W>) -> std::io::Result<usize>;

    /// Decode a jet from bits.
    fn decode<I: Iterator<Item = u8>>(bits: &mut BitIter<I>) -> Result<Self, Error>;

    /// Obtain the FFI C pointer for the jet.
    fn c_jet_ptr(&self) -> &'static dyn Fn(&mut CFrameItem, CFrameItem, *const CTxEnv) -> bool;

    /// Execute the jet on the Bit Machine, using the given environment.
    fn exec(&self, mac: &mut BitMachine, env: &Self::Environment) -> Result<(), JetFailed> {
        // Sanity Check: This should never really fail, but still good to do
        if !simplicity_sys::c_jets::sanity_checks() {
            return Err(JetFailed);
        }
        let pows_of_two = Type::powers_of_two(); // TODO: make this compile time static
        let src_ty = self.source_ty().to_type(&pows_of_two);
        let target_ty = self.target_ty().to_type(&pows_of_two);

        let a_frame_size = ffi_bytes_size(src_ty.bit_width);
        let b_frame_size = ffi_bytes_size(target_ty.bit_width);
        // a_frame_size + b_frame_size must be non-zero unless it is a unit to unit jet
        if a_frame_size == 0 && b_frame_size == 0 {
            return Ok(());
        }
        let mut src_buf = vec![0u8; a_frame_size + b_frame_size];
        let src_ptr_end = unsafe { src_buf.as_mut_ptr().add(a_frame_size) }; // A frame write
        let src_ptr = src_buf.as_mut_ptr(); // A read frame at ptr start
        let dst_ptr_begin = unsafe { src_buf.as_mut_ptr().add(a_frame_size) }; // B read frame at ptr begin
        let dst_ptr_end = unsafe { src_buf.as_mut_ptr().add(a_frame_size + b_frame_size) }; // B write frame at ptr end

        // For jet from type A -> B
        // 1. Write into C bitmachine using A write frame
        let mut a_frame = CFrameItem::new_write(src_ty.bit_width, src_ptr_end);
        for _ in 0..src_ty.bit_width {
            let bit = mac.read_bit();
            unsafe {
                c_writeBit(&mut a_frame, bit);
            }
        }

        // 2. Execute the jet. src = A read frame, dst = B write frame
        let src_frame = CFrameItem::new_read(src_ty.bit_width, src_ptr);
        let mut dst_frame = CFrameItem::new_write(target_ty.bit_width, dst_ptr_end);
        let jet_fn = self.c_jet_ptr();
        let c_env = env.c_tx_env();
        let res = match c_env {
            Some(env) => jet_fn(&mut dst_frame, src_frame, env),
            None => jet_fn(&mut dst_frame, src_frame, std::ptr::null()),
        };
        if !res {
            return Err(JetFailed);
        }

        // 3. Read the result from B read frame
        let mut b_frame = CFrameItem::new_read(target_ty.bit_width, dst_ptr_begin);
        // Read the value from b_frame
        for _ in 0..target_ty.bit_width {
            let bit = unsafe { c_readBit(&mut b_frame) };
            mac.write_bit(bit);
        }
        Ok(())
    }
}

pub trait JetEnvironment {
    fn c_tx_env(&self) -> Option<&CTxEnv> {
        None
    }
}

impl JetEnvironment for () {}

#[cfg(test)]
mod tests {
    use crate::{exec::BitMachine, jet::Jet};

    #[test]
    fn test_ffi_jet() {
        // Indirectly create bit machine
        let mut mac = BitMachine {
            data: vec![0u8; 1024],
            next_frame_start: 0,
            read: vec![],
            write: vec![],
        };
        mac.new_frame_unchecked(64);
        mac.write_u32(2);
        mac.write_u32(16);
        mac.move_frame();

        mac.new_frame_unchecked(33);
        let env = ();
        let jet = super::init::core::Core::Add32;
        jet.exec(&mut mac, &env).unwrap();

        mac.move_frame();
        let carry = mac.read_bit();
        let res = mac.read_u32();
        assert_eq!(carry, false);
        assert_eq!(res, 2 + 16);
    }

    #[test]
    fn test_simple() {
        let mut mac = BitMachine {
            data: vec![0u8; 1024],
            next_frame_start: 0,
            read: vec![],
            write: vec![],
        };

        mac.new_frame_unchecked(64);
        mac.write_u32(2);
        mac.write_u32(16);
        mac.move_frame();
        let x = mac.read_u32();
        let y = mac.read_u32();
        assert_eq!(x, 2);
        assert_eq!(y, 16);
    }
}
