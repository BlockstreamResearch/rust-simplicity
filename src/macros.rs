macro_rules! matches {
    ($expression:expr, $($pattern:tt)+) => {
        match $expression {
            $($pattern)+ => true,
            ref _e => false,
        }
    }
}

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
