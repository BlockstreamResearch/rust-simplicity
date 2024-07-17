
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
            src_align: rand::random::<usize>() % usize::BITS as usize, // Assuming usize::BITS < 2^16
            tgt_align: rand::random::<usize>() % usize::BITS as usize,
            input,
        }
    }
}
