use simplicity::ffi as simplicity_sys;

use crate::JetParams;
use rand::rngs::ThreadRng;
use simplicity::core::types::Type;
use simplicity_sys::c_jets::frame_ffi::{c_readBit, c_writeBit};
use simplicity_sys::c_jets::round_u_word;
use simplicity_sys::CFrameItem;

/// A jet buffer with pre-computed and pre-stored pointers to holding
/// source and destination frames.
/// The logic is exactly the same as the one used in simplicity crate
/// but a special support to add "padding" to frames to make them
/// mis-aligned for benchmarking purposes. The padding can be considered
/// a special type of desired len that is added to source_type of jet
/// and to the target_type of jet.
pub struct JetBuffer {
    #[allow(dead_code)]
    cells: Vec<usize>,
    src_bit_width: usize,
    dst_bit_width: usize,
    src_ptr_begin: *mut usize,
    src_ptr_end: *mut usize,
    #[allow(dead_code)]
    dst_ptr_begin: *mut usize,
    dst_ptr_end: *mut usize,
}

impl JetBuffer {
    /// Creates a new [`JetBuffer`].
    ///
    /// # Panics
    ///
    /// Panics if the source and target types are both unit types.
    pub fn new(src_ty: &Type, tgt_type: &Type, params: &JetParams) -> Self {
        // Note here that we are adding the alignment to the bit width
        let src_bit_width = src_ty.bit_width + params.src_align;
        let dst_bit_width = tgt_type.bit_width + params.tgt_align;
        let src_frame_size = round_u_word(src_bit_width);
        let dst_frame_size = round_u_word(dst_bit_width);

        if src_frame_size == 0 && dst_frame_size == 0 {
            panic!("No unit to unit jets in our benchmarks");
        }

        let mut cells = vec![0usize; src_frame_size + dst_frame_size];

        Self {
            src_bit_width,
            dst_bit_width,
            src_ptr_begin: cells.as_mut_ptr(),
            src_ptr_end: unsafe { cells.as_mut_ptr().add(src_frame_size) },
            dst_ptr_begin: unsafe { cells.as_mut_ptr().add(src_frame_size) },
            dst_ptr_end: unsafe { cells.as_mut_ptr().add(src_frame_size + dst_frame_size) },
            cells,
        }
    }

    pub fn write(
        &mut self,
        src_ty: &Type,
        params: &JetParams,
        rng: &mut ThreadRng,
    ) -> (CFrameItem, CFrameItem) {
        unsafe {
            // Source frame:
            // 1. Write dummy bits for alignment and then write input bits via write frame
            let mut src_write_frame = CFrameItem::new_write(self.src_bit_width, self.src_ptr_end);
            for _ in 0..params.src_align {
                c_writeBit(&mut src_write_frame, false);
            }
            params.input.write_sample(&mut src_write_frame, src_ty, rng);

            // 2. Read dummy bits for alignment and return read frame
            let mut src_read_frame = CFrameItem::new_read(self.src_bit_width, self.src_ptr_begin);
            for _ in 0..params.src_align {
                c_readBit(&mut src_read_frame);
            }

            // Destination frame:
            // Write dummy bits for alignment and return write frame
            let mut dst_write_frame = CFrameItem::new_write(self.dst_bit_width, self.dst_ptr_end);
            for _ in 0..params.tgt_align {
                c_writeBit(&mut dst_write_frame, false);
            }
        }

        (src_read_frame, dst_write_frame)
    }
}
