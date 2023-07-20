#[macro_export]
macro_rules! decode_bits {
    ($bits:ident, {}) => {
        Err($crate::decode::Error::InvalidJet.into())
    };
    ($bits:ident, {$jet:path}) => {
        Ok($jet)
    };
    ($bits:ident, { 0 => $false_branch:tt, 1 => $true_branch:tt }) => {
        match $bits.next() {
            None => Err($crate::decode::Error::EndOfStream.into()),
            Some(false) => decode_bits!($bits, $false_branch),
            Some(true) => decode_bits!($bits, $true_branch),
        }
    };
}
