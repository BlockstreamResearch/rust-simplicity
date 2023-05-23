use std::fmt;

pub enum Error {
    InvalidSequence,
    PublicKeyHash,
    Sha256d,
    Ripemd160,
    Multisig,
    Extensions,
}

impl fmt::Debug for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::InvalidSequence => writeln!(f, "Sequence does not encode relative locktime"),
            Error::PublicKeyHash => writeln!(f, "Public key hashes are not suppored"),
            Error::Sha256d => writeln!(f, "Sha256d is not supported"),
            Error::Ripemd160 => writeln!(f, "Ripemd160 is not supported"),
            Error::Multisig => writeln!(f, "Multisig is not supported"),
            Error::Extensions => writeln!(f, "Extensions are not supported"),
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Debug::fmt(self, f)
    }
}

impl std::error::Error for Error {}
