use bitcoin_miniscript::{MiniscriptKey, ToPublicKey};
use elements::bitcoin::key::XOnlyPublicKey;
use hashes::sha256;
use std::fmt::{Debug, Display};

/// Public key which can be converted to a hash type.
pub trait SimplicityKey: Clone + Eq + Ord + Debug + Display + std::hash::Hash {
    /// SHA 256 hash associated with this key, used in the sha256 fragment.
    type Sha256: Clone + Eq + Ord + Display + Debug + std::hash::Hash;
}

impl<Pk: MiniscriptKey> SimplicityKey for Pk {
    type Sha256 = <Pk as MiniscriptKey>::Sha256;
}

/// Public key which can be converted to a (x-only) public key which can be used in Simplicity.
pub trait ToXOnlyPubkey: SimplicityKey {
    /// Convert the key to an x-only public key.
    fn to_x_only_pubkey(&self) -> XOnlyPublicKey;

    /// Convert the generic associated [`SimplicityKey::Sha256`] to [`sha256::Hash`].
    fn to_sha256(hash: &Self::Sha256) -> sha256::Hash;
}

impl<Pk: ToPublicKey> ToXOnlyPubkey for Pk {
    fn to_x_only_pubkey(&self) -> XOnlyPublicKey {
        <Pk as ToPublicKey>::to_x_only_pubkey(self)
    }

    fn to_sha256(hash: &Self::Sha256) -> sha256::Hash {
        <Pk as ToPublicKey>::to_sha256(hash)
    }
}

/// Object which can translate one key type to another, including all associated hashes.
pub trait Translator<P, Q, E>
where
    P: SimplicityKey,
    Q: SimplicityKey,
{
    /// Translates public keys [`P`] → [`Q`].
    fn pk(&mut self, pk: &P) -> Result<Q, E>;

    /// Translates SHA 256 hashes [`P::Sha256`] → [`Q::Sha256`].
    fn sha256(&mut self, sha256: &P::Sha256) -> Result<Q::Sha256, E>;
}
