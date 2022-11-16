/*
macro_rules! matches {
    ($expression:expr, $($pattern:tt)+) => {
        match $expression {
            $($pattern)+ => true,
            ref _e => false,
        }
    }
}
*/

// Code from rust 1.45.0 for assert_matches
// Handy for testing Results or other enums
#[cfg(test)]
macro_rules! assert_matches {
    ($expression:expr, $($pattern:tt)+) => {
        match $expression {
            $($pattern)+ => (),
            ref e => panic!("assertion failed: `{:?}` does not match `{}`", e, stringify!($($pattern)+)),
        }
    }
}

#[macro_export]
macro_rules! decode_bits {
    ($bits:ident, {}) => {
        Err(Error::ParseError("Illegal jet"))
    };
    ($bits:ident, {$jet:path}) => {
        Ok($jet)
    };
    ($bits:ident, { 0 => $false_branch:tt, 1 => $true_branch:tt }) => {
        match $bits.next() {
            None => Err(Error::EndOfStream),
            Some(false) => decode_bits!($bits, $false_branch),
            Some(true) => decode_bits!($bits, $true_branch),
        }
    };
}
