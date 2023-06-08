use std::mem::size_of;

use crate::input::InputSampling;

pub struct JetParams {
    /// The alignment of the source frame
    pub src_align: usize,
    /// The alignment of the destination frame
    pub tgt_align: usize,
    /// Sampling of inputs
    pub input: InputSampling,
}

impl JetParams {
    #[allow(dead_code)]
    pub fn new(src_align: usize, tgt_align: usize, input: InputSampling) -> Self {
        Self {
            src_align,
            tgt_align,
            input,
        }
    }

    pub fn with_rand_aligns(input: InputSampling) -> Self {
        Self {
            src_align: rand::random::<usize>() % (8 * size_of::<usize>()), // Assuming 8*sizeof(usize) < 2^16
            tgt_align: rand::random::<usize>() % (8 * size_of::<usize>()),
            input,
        }
    }
}
