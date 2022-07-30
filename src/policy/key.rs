use bitcoin::XOnlyPublicKey;
use miniscript::{DummyKey, MiniscriptKey};

/// Public key that can be serialized in 32 bytes.
pub trait PublicKey32: MiniscriptKey {
    /// Encode the public key to 32 bytes.
    fn to_32_bytes(&self) -> [u8; 32];

    /// Decode the public key from 32 bytes.
    fn from_32_bytes(bytes: &[u8]) -> Self;
}

impl PublicKey32 for XOnlyPublicKey {
    fn to_32_bytes(&self) -> [u8; 32] {
        self.serialize()
    }

    fn from_32_bytes(bytes: &[u8]) -> Self {
        XOnlyPublicKey::from_slice(bytes).expect("Parse x-only public key from bytes")
    }
}

impl PublicKey32 for DummyKey {
    fn to_32_bytes(&self) -> [u8; 32] {
        [0xab; 32]
    }

    fn from_32_bytes(bytes: &[u8]) -> Self {
        if *bytes != [0xab; 32] {
            panic!("Unable to parse DummyKey from bytes")
        }
        DummyKey
    }
}
