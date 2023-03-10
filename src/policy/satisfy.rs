use crate::core::Value;
use crate::jet::elements::ElementsEnv;
use crate::policy::ast::Policy;
use bitcoin_hashes::{sha256, Hash};
use elements::locktime::Height;
use elements::schnorr::{KeyPair, XOnlyPublicKey};
use elements::secp256k1_zkp::Message;
use elements::taproot::TapLeafHash;
use elements::SchnorrSig;
use elements::{LockTime, SchnorrSigHashType, Sequence};
use elements_miniscript::{Preimage32, Satisfier};
use std::cmp::Ordering;
use std::collections::{HashMap, VecDeque};
use std::iter::FromIterator;

pub struct PolicySatisfier {
    pub preimages: HashMap<sha256::Hash, Preimage32>,
    pub keys: HashMap<XOnlyPublicKey, KeyPair>,
    pub env: ElementsEnv,
}

impl Satisfier<XOnlyPublicKey> for PolicySatisfier {
    fn lookup_tap_leaf_script_sig(
        &self,
        pk: &XOnlyPublicKey,
        _: &TapLeafHash,
    ) -> Option<SchnorrSig> {
        self.keys.get(pk).map(|keypair| {
            let sighash = self.env.c_tx_env().sighash_all();
            let msg = Message::from_hashed_data::<sha256::Hash>(&sighash);
            let sig = keypair.sign_schnorr(msg);

            SchnorrSig {
                sig,
                hash_ty: SchnorrSigHashType::All,
            }
        })
    }

    fn lookup_sha256(&self, hash: &sha256::Hash) -> Option<Preimage32> {
        self.preimages.get(hash).copied()
    }

    fn check_older(&self, n: Sequence) -> bool {
        self.env.tx().input[self.env.ix() as usize].sequence >= n
    }

    fn check_after(&self, n: LockTime) -> bool {
        let m = LockTime::from(self.env.tx().lock_time);
        match m.partial_cmp(&n) {
            Some(Ordering::Less) | Some(Ordering::Equal) => true,
            _ => false,
        }
    }
}

impl Policy<XOnlyPublicKey> {
    pub fn satisfy<S: Satisfier<XOnlyPublicKey>>(&self, satisfier: &S) -> Option<Vec<Value>> {
        let (wit, _) = self.satisfy_helper(satisfier)?;
        Some(wit.into())
    }

    fn satisfy_helper<S: Satisfier<XOnlyPublicKey>>(
        &self,
        satisfier: &S,
    ) -> Option<(VecDeque<Value>, u64)> {
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
                if satisfier.check_older(Sequence(*n)) {
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
