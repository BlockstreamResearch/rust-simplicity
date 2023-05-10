use crate::core::Value;
use crate::jet::elements::ElementsEnv;
use crate::Policy;
use bitcoin_hashes::Hash;
use elements::bitcoin;
use elements::locktime::Height;
use elements::taproot::TapLeafHash;
use elements::{secp256k1_zkp, LockTime, SchnorrSigHashType, Sequence};
use elements_miniscript::{MiniscriptKey, Preimage32, Satisfier, ToPublicKey};
use std::collections::{HashMap, VecDeque};
use std::iter::FromIterator;

pub struct PolicySatisfier<Pk: MiniscriptKey> {
    pub preimages: HashMap<Pk::Sha256, Preimage32>,
    pub keys: HashMap<Pk, bitcoin::KeyPair>,
    pub env: ElementsEnv,
}

impl<Pk: MiniscriptKey + ToPublicKey> Satisfier<Pk> for PolicySatisfier<Pk> {
    fn lookup_tap_leaf_script_sig(&self, pk: &Pk, _: &TapLeafHash) -> Option<elements::SchnorrSig> {
        self.keys.get(pk).map(|keypair| {
            let sighash = self.env.c_tx_env().sighash_all();
            let msg = secp256k1_zkp::Message::from_slice(&sighash).unwrap();
            let sig = keypair.sign_schnorr(msg);

            elements::SchnorrSig {
                sig,
                hash_ty: SchnorrSigHashType::All,
            }
        })
    }

    fn lookup_sha256(&self, hash: &Pk::Sha256) -> Option<Preimage32> {
        self.preimages.get(hash).copied()
    }

    fn check_older(&self, sequence: Sequence) -> bool {
        let self_sequence = self.env.tx().input[self.env.ix() as usize].sequence;
        <Sequence as Satisfier<Pk>>::check_older(&self_sequence, sequence)
    }

    fn check_after(&self, locktime: LockTime) -> bool {
        let self_locktime = LockTime::from(self.env.tx().lock_time);
        <LockTime as Satisfier<Pk>>::check_after(&self_locktime, locktime)
    }
}

impl<Pk: MiniscriptKey + ToPublicKey> Policy<Pk> {
    pub fn satisfy<S: Satisfier<Pk>>(&self, satisfier: &S) -> Option<Vec<Value>> {
        let (wit, _) = self.satisfy_helper(satisfier)?;
        Some(wit.into())
    }

    fn satisfy_helper<S: Satisfier<Pk>>(&self, satisfier: &S) -> Option<(VecDeque<Value>, u64)> {
        match self {
            Policy::Unsatisfiable => None,
            Policy::Trivial => Some((VecDeque::new(), 0)),
            Policy::Key(pk) => satisfier
                .lookup_tap_leaf_script_sig(pk, &TapLeafHash::all_zeros())
                .map(|sig| {
                    let value = Value::u512_from_slice(sig.sig.as_ref());
                    let wit = VecDeque::from_iter(std::iter::once(value));
                    (wit, 512)
                }),
            Policy::After(n) => {
                let height = Height::from_consensus(*n).ok()?;
                if satisfier.check_after(LockTime::Blocks(height)) {
                    Some((VecDeque::new(), 0))
                } else {
                    None
                }
            }
            Policy::Older(n) => {
                if satisfier.check_older(Sequence((*n).into())) {
                    Some((VecDeque::new(), 0))
                } else {
                    None
                }
            }
            Policy::Sha256(hash) => satisfier.lookup_sha256(hash).map(|preimage| {
                let value = Value::u256_from_slice(preimage.as_ref());
                let wit = VecDeque::from_iter(std::iter::once(value));
                (wit, 256)
            }),
            Policy::And(sub_policies) => {
                assert_eq!(
                    2,
                    sub_policies.len(),
                    "Conjunctions must have exactly two sub-policies"
                );

                sub_policies[0]
                    .satisfy_helper(satisfier)
                    .and_then(|(mut wit1, size1)| {
                        sub_policies[1]
                            .satisfy_helper(satisfier)
                            .map(|(wit2, size2)| {
                                wit1.extend(wit2.into_iter());
                                (wit1, size1 + size2)
                            })
                    })
            }
            Policy::Or(sub_policies) => {
                assert_eq!(
                    2,
                    sub_policies.len(),
                    "Disjunctions must have exactly two sub-policies"
                );

                let maybe_sat1 = sub_policies[0].satisfy_helper(satisfier);
                let maybe_sat2 = sub_policies[1].satisfy_helper(satisfier);

                match (maybe_sat1, maybe_sat2) {
                    (Some((mut wit1, size1)), Some((mut wit2, size2))) => {
                        if size1 <= size2 {
                            wit1.push_front(Value::u1(0));
                            Some((wit1, size1 + 1))
                        } else {
                            wit2.push_front(Value::u1(1));
                            Some((wit2, size2 + 1))
                        }
                    }
                    (Some((mut wit1, size1)), None) => {
                        wit1.push_front(Value::u1(0));
                        Some((wit1, size1 + 1))
                    }
                    (None, Some((mut wit2, size2))) => {
                        wit2.push_front(Value::u1(1));
                        Some((wit2, size2 + 1))
                    }
                    _ => None,
                }
            }
            Policy::Threshold(_, _) => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use bitcoin_hashes::{sha256, Hash};

    fn get_satisfier() -> PolicySatisfier<bitcoin::PublicKey> {
        let mut preimages = HashMap::new();
        let mut keys = HashMap::new();

        for i in 0..3 {
            let preimage = [i; 32];
            preimages.insert(sha256::Hash::hash(&preimage), preimage);
        }

        let secp = secp256k1_zkp::Secp256k1::new();
        let mut rng = secp256k1_zkp::rand::rngs::ThreadRng::default();

        for _ in 0..3 {
            let keypair = bitcoin::KeyPair::new(&secp, &mut rng);
            keys.insert(keypair.public_key().to_public_key(), keypair);
        }

        PolicySatisfier {
            preimages,
            keys,
            env: ElementsEnv::dummy(),
        }
    }

    #[test]
    fn satisfy_unsatisfiable() {
        let satisfier = get_satisfier();
        let policy = Policy::Unsatisfiable;
        assert_eq!(None, policy.satisfy(&satisfier));
    }

    #[test]
    fn satisfy_trivial() {
        let satisfier = get_satisfier();
        let policy = Policy::Trivial;
        assert_eq!(
            Vec::<Value>::new(),
            policy.satisfy(&satisfier).expect("satisfiable")
        );
    }

    #[test]
    fn satisfy_pk() {
        let satisfier = get_satisfier();
        let mut keys = satisfier.keys.keys();
        let policy = Policy::Key(*keys.next().unwrap());

        let witness = policy.satisfy(&satisfier).expect("sat");
        assert_eq!(1, witness.len());
        let signature = witness[0].try_to_bytes().expect("bytes");
        assert_eq!(64, signature.len());
    }

    #[test]
    fn satisfy_sha256() {
        let satisfier = get_satisfier();
        let mut images = satisfier.preimages.keys();
        let image0 = *images.next().unwrap();
        let policy = Policy::Sha256(image0);

        let preimage0 = satisfier.preimages.get(&image0).unwrap();

        assert_eq!(
            vec![Value::u256_from_slice(preimage0)],
            policy.satisfy(&satisfier).expect("sat")
        );
    }

    #[test]
    fn satisfy_and() {
        let satisfier = get_satisfier();
        let mut images = satisfier.preimages.keys();
        let image0 = *images.next().unwrap();
        let image1 = *images.next().unwrap();
        let preimage0 = satisfier.preimages.get(&image0).unwrap();
        let preimage1 = satisfier.preimages.get(&image1).unwrap();

        let policy0 = Policy::And(vec![Policy::Sha256(image0), Policy::Sha256(image1)]);
        assert_eq!(
            vec![
                Value::u256_from_slice(preimage0),
                Value::u256_from_slice(preimage1)
            ],
            policy0.satisfy(&satisfier).expect("sat")
        );

        let policy1 = Policy::And(vec![
            Policy::Sha256(sha256::Hash::from_inner([0; 32])),
            Policy::Sha256(image1),
        ]);
        assert_eq!(None, policy1.satisfy(&satisfier));

        let policy2 = Policy::And(vec![
            Policy::Sha256(image0),
            Policy::Sha256(sha256::Hash::from_inner([1; 32])),
        ]);
        assert_eq!(None, policy2.satisfy(&satisfier));
    }

    #[test]
    fn satisfy_or() {
        let satisfier = get_satisfier();
        let mut images = satisfier.preimages.keys();
        let image0 = *images.next().unwrap();
        let image1 = *images.next().unwrap();
        let preimage0 = satisfier.preimages.get(&image0).unwrap();
        let preimage1 = satisfier.preimages.get(&image1).unwrap();

        let policy0 = Policy::Or(vec![
            Policy::Sha256(image0),
            Policy::Sha256(sha256::Hash::from_inner([1; 32])),
        ]);
        assert_eq!(
            vec![Value::u1(0), Value::u256_from_slice(preimage0)],
            policy0.satisfy(&satisfier).expect("sat")
        );

        let policy1 = Policy::Or(vec![
            Policy::Sha256(sha256::Hash::from_inner([0; 32])),
            Policy::Sha256(image1),
        ]);
        assert_eq!(
            vec![Value::u1(1), Value::u256_from_slice(preimage1)],
            policy1.satisfy(&satisfier).expect("sat")
        );

        let policy2 = Policy::Or(vec![
            Policy::Sha256(sha256::Hash::from_inner([0; 32])),
            Policy::Sha256(sha256::Hash::from_inner([1; 32])),
        ]);
        assert_eq!(None, policy2.satisfy(&satisfier));
    }
}
