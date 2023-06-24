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
#[cfg(feature = "elements")]
pub mod elements;
mod init;
pub mod type_name;

#[cfg(feature = "bitcoin")]
pub use init::bitcoin::Bitcoin;
pub use init::core::Core;
#[cfg(feature = "elements")]
pub use init::elements::Elements;
use simplicity_sys::c_jets::frame_ffi::{c_readBit, c_writeBit, CFrameItem};
use simplicity_sys::c_jets::round_u_word;

use crate::decode;
use crate::exec::BitMachine;
use crate::jet::type_name::TypeName;
use crate::merkle::cmr::Cmr;
use crate::{BitIter, BitWriter};
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
    type Environment;
    /// CJetEnvironment to interact with C FFI.
    type CJetEnvironment;

    /// Return the CMR of the jet.
    fn cmr(&self) -> Cmr;

    /// Return the source type of the jet.
    fn source_ty(&self) -> TypeName;

    /// Return the target type of the jet.
    fn target_ty(&self) -> TypeName;

    /// Encode the jet to bits.
    fn encode<W: Write>(&self, w: &mut BitWriter<W>) -> std::io::Result<usize>;

    /// Decode a jet from bits.
    fn decode<I: Iterator<Item = u8>>(bits: &mut BitIter<I>) -> Result<Self, decode::Error>;

    /// Obtains a C FFI compatible environment for the jet.
    fn c_jet_env<'env>(&self, env: &'env Self::Environment) -> &'env Self::CJetEnvironment;

    /// Obtain the FFI C pointer for the jet.
    fn c_jet_ptr(&self) -> &dyn Fn(&mut CFrameItem, CFrameItem, &Self::CJetEnvironment) -> bool;

    /// Execute the jet on the Bit Machine, using the given environment.
    fn exec<'env>(
        &self,
        mac: &mut BitMachine,
        env: &'env Self::Environment,
    ) -> Result<(), JetFailed>
    where
        Self::CJetEnvironment: 'env,
    {
        // Sanity Check: This should never really fail, but still good to do
        if !simplicity_sys::c_jets::sanity_checks() {
            return Err(JetFailed);
        }
        let src_ty_bit_width = self.source_ty().to_bit_width();
        let target_ty_bit_width = self.target_ty().to_bit_width();

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
            let bit = mac.read_bit();
            unsafe {
                c_writeBit(&mut a_frame, bit);
            }
        }
        mac.back(src_ty_bit_width);

        // 2. Execute the jet. src = A read frame, dst = B write frame
        // Precondition satisfied: src_ptr is the start of slice of UWORDs of A.
        let src_frame = unsafe { CFrameItem::new_read(src_ty_bit_width, src_ptr) };
        // Precondition satisfied: dst_ptr_end is one past the end of slice of UWORDs of B.
        let mut dst_frame = unsafe { CFrameItem::new_write(target_ty_bit_width, dst_ptr_end) };
        let jet_fn = self.c_jet_ptr();
        let c_env = self.c_jet_env(env);
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
            mac.write_bit(bit);
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::node::{ConstructNode, CoreConstructible, JetConstructible};
    use crate::Value;
    use crate::{exec::BitMachine, jet::Core};
    use std::sync::Arc;

    #[test]
    fn test_ffi_jet() {
        let two_words = Arc::<ConstructNode<_>>::comp(
            &Arc::<ConstructNode<_>>::pair(
                &Arc::<ConstructNode<_>>::const_word(Arc::new(Value::u32(2))),
                &Arc::<ConstructNode<_>>::const_word(Arc::new(Value::u32(16))),
            )
            .unwrap(),
            &Arc::<ConstructNode<_>>::jet(Core::Add32),
        )
        .unwrap();
        assert_eq!(
            BitMachine::test_exec(two_words, &()).expect("executing"),
            Value::Prod(
                Box::new(Value::u1(0)),       // carry bit
                Box::new(Value::u32(2 + 16)), // result
            ),
        );
    }

    #[test]
    fn test_simple() {
        let two_words = Arc::<ConstructNode<Core>>::pair(
            &Arc::<ConstructNode<_>>::const_word(Arc::new(Value::u32(2))),
            &Arc::<ConstructNode<_>>::const_word(Arc::new(Value::u16(16))),
        )
        .unwrap();
        assert_eq!(
            BitMachine::test_exec(two_words, &()).expect("executing"),
            Value::Prod(Box::new(Value::u32(2)), Box::new(Value::u16(16))),
        );
    }
}
