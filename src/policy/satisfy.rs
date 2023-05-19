use crate::core::commit::UsedCaseBranch;
use crate::core::Value;
use crate::jet::elements::ElementsEnv;
use crate::jet::Elements;
use crate::policy::compiler;
use crate::policy::key::PublicKey32;
use crate::{CommitNode, Context, Policy};
use bitcoin_hashes::Hash;
use elements::bitcoin;
use elements::locktime::Height;
use elements::taproot::TapLeafHash;
use elements::{secp256k1_zkp, LockTime, SchnorrSigHashType, Sequence};
use elements_miniscript::{MiniscriptKey, Preimage32, Satisfier, ToPublicKey};
use std::collections::{HashMap, VecDeque};
use std::iter::FromIterator;
use std::rc::Rc;

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

struct SatisfierExtData {
    witness: VecDeque<Value>,
    cost: u64,
    pruned: Rc<CommitNode<Elements>>,
}

impl<Pk: MiniscriptKey + PublicKey32 + ToPublicKey> Policy<Pk> {
    pub fn satisfy<S: Satisfier<Pk>>(
        &self,
        satisfier: &S,
    ) -> Option<(Vec<Value>, Rc<CommitNode<Elements>>)> {
        let mut context = Context::default();
        let helper = self.satisfy_helper(satisfier, &mut context)?;
        Some((helper.witness.into(), helper.pruned))
    }

    fn compile_no_witness(&self, context: &mut Context<Elements>) -> SatisfierExtData {
        SatisfierExtData {
            witness: VecDeque::new(),
            cost: 0,
            pruned: self.compile(context).unwrap(),
        }
    }

    fn satisfy_helper<S: Satisfier<Pk>>(
        &self,
        satisfier: &S,
        context: &mut Context<Elements>,
    ) -> Option<SatisfierExtData> {
        match self {
            Policy::Unsatisfiable => None,
            Policy::Trivial => Some(self.compile_no_witness(context)),
            Policy::Key(pk) => satisfier
                .lookup_tap_leaf_script_sig(pk, &TapLeafHash::all_zeros())
                .map(|sig| {
                    let value = Value::u512_from_slice(sig.sig.as_ref());
                    let witness_data = VecDeque::from_iter(std::iter::once(value));

                    SatisfierExtData {
                        witness: witness_data,
                        cost: 512,
                        pruned: self.compile(context).unwrap(),
                    }
                }),
            Policy::After(n) => {
                let height = Height::from_consensus(*n).ok()?;
                if satisfier.check_after(LockTime::Blocks(height)) {
                    Some(self.compile_no_witness(context))
                } else {
                    None
                }
            }
            Policy::Older(n) => {
                if satisfier.check_older(Sequence((*n).into())) {
                    Some(self.compile_no_witness(context))
                } else {
                    None
                }
            }
            Policy::Sha256(hash) => satisfier.lookup_sha256(hash).map(|preimage| {
                let value = Value::u256_from_slice(preimage.as_ref());
                let witness_data = VecDeque::from_iter(std::iter::once(value));

                SatisfierExtData {
                    witness: witness_data,
                    cost: 256,
                    pruned: self.compile(context).unwrap(),
                }
            }),
            Policy::And(sub_policies) => {
                assert_eq!(
                    2,
                    sub_policies.len(),
                    "Conjunctions must have exactly two sub-policies"
                );

                let mut left = sub_policies[0].satisfy_helper(satisfier, context)?;
                let right = sub_policies[1].satisfy_helper(satisfier, context)?;
                left.witness.extend(right.witness);
                left.cost += right.cost;
                left.pruned = compiler::and(context, left.pruned, right.pruned).ok()?;

                Some(left)
            }
            Policy::Or(sub_policies) => {
                assert_eq!(
                    2,
                    sub_policies.len(),
                    "Disjunctions must have exactly two sub-policies"
                );

                let maybe_left = sub_policies[0].satisfy_helper(satisfier, context);
                let maybe_right = sub_policies[1].satisfy_helper(satisfier, context);

                match (maybe_left, maybe_right) {
                    (Some(mut left), Some(mut right)) => {
                        if left.cost <= right.cost {
                            left.witness.push_front(Value::u1(0));
                            left.cost += 1;
                            left.pruned = compiler::or(
                                context,
                                left.pruned,
                                right.pruned,
                                UsedCaseBranch::Left,
                            )
                            .ok()?;
                            Some(left)
                        } else {
                            right.witness.push_front(Value::u1(1));
                            right.cost += 1;
                            right.pruned = compiler::or(
                                context,
                                left.pruned,
                                right.pruned,
                                UsedCaseBranch::Right,
                            )
                            .ok()?;
                            Some(right)
                        }
                    }
                    (Some(mut left), None) => {
                        left.witness.push_front(Value::u1(0));
                        left.cost += 1;
                        let right = sub_policies[1].compile(context).ok()?;
                        left.pruned =
                            compiler::or(context, left.pruned, right, UsedCaseBranch::Left).ok()?;
                        Some(left)
                    }
                    (None, Some(mut right)) => {
                        right.witness.push_front(Value::u1(1));
                        right.cost += 1;
                        let left = sub_policies[0].compile(context).ok()?;
                        right.pruned =
                            compiler::or(context, left, right.pruned, UsedCaseBranch::Right)
                                .ok()?;
                        Some(right)
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
    use crate::exec::BitMachine;
    use bitcoin_hashes::{sha256, Hash};
    use std::convert::TryFrom;

    fn get_satisfier() -> PolicySatisfier<bitcoin::XOnlyPublicKey> {
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
            let (xonly, _) = keypair.x_only_public_key();
            keys.insert(xonly, keypair);
        }

        PolicySatisfier {
            preimages,
            keys,
            env: ElementsEnv::dummy(),
        }
    }

    fn execute_successful(
        commit: Rc<CommitNode<Elements>>,
        env: &ElementsEnv,
        witness: Vec<Value>,
    ) {
        let program = commit.finalize(witness.into_iter()).expect("finalize");
        let mut mac = BitMachine::for_program(&program);
        assert!(mac.exec(&program, &env).is_ok());
    }

    #[test]
    fn satisfy_unsatisfiable() {
        let satisfier = get_satisfier();
        let policy = Policy::Unsatisfiable;

        assert!(policy.satisfy(&satisfier).is_none());

        let mut context = Context::default();
        let commit = policy.compile(&mut context).expect("compile");
        let program = commit.finalize(std::iter::empty()).expect("finalize");
        let mut mac = BitMachine::for_program(&program);

        assert!(mac.exec(&program, &satisfier.env).is_err());
    }

    #[test]
    fn satisfy_trivial() {
        let satisfier = get_satisfier();
        let policy = Policy::Trivial;

        let (witness, commit) = policy.satisfy(&satisfier).expect("satisfiable");
        assert_eq!(Vec::<Value>::new(), witness);

        execute_successful(commit, &satisfier.env, witness);
    }

    #[test]
    fn satisfy_pk() {
        let satisfier = get_satisfier();
        let mut it = satisfier.keys.keys();
        let xonly = it.next().unwrap();
        let policy = Policy::Key(*xonly);

        let (witness, commit) = policy.satisfy(&satisfier).expect("satisfiable");
        assert_eq!(1, witness.len());

        let sighash = satisfier.env.c_tx_env().sighash_all();
        let message = secp256k1_zkp::Message::from(sighash);
        let signature_bytes = witness[0].try_to_bytes().expect("to bytes");
        let signature =
            secp256k1_zkp::schnorr::Signature::from_slice(&signature_bytes).expect("to signature");
        assert!(signature.verify(&message, &xonly).is_ok());

        execute_successful(commit, &satisfier.env, witness);
    }

    #[test]
    fn satisfy_sha256() {
        let satisfier = get_satisfier();
        let mut it = satisfier.preimages.keys();
        let image = *it.next().unwrap();
        let policy = Policy::Sha256(image);

        let (witness, commit) = policy.satisfy(&satisfier).expect("satisfiable");
        assert_eq!(1, witness.len());

        let witness_bytes = witness[0].try_to_bytes().expect("to bytes");
        let witness_preimage = Preimage32::try_from(witness_bytes.as_slice()).expect("to array");
        let preimage = *satisfier.preimages.get(&image).unwrap();
        assert_eq!(preimage, witness_preimage);

        execute_successful(commit, &satisfier.env, witness);
    }

    #[test]
    fn satisfy_and() {
        let satisfier = get_satisfier();
        let images: Vec<_> = satisfier.preimages.keys().map(|x| *x).collect();
        let preimages: Vec<_> = images
            .iter()
            .map(|x| satisfier.preimages.get(x).unwrap())
            .collect();

        // Policy 0

        let policy0 = Policy::And(vec![Policy::Sha256(images[0]), Policy::Sha256(images[1])]);
        let (witness, commit) = policy0.satisfy(&satisfier).expect("satisfiable");
        assert_eq!(2, witness.len());

        for i in 0..2 {
            let preimage_bytes = witness[i].try_to_bytes().expect("to bytes");
            let witness_preimage =
                Preimage32::try_from(preimage_bytes.as_slice()).expect("to array");
            assert_eq!(preimages[i], &witness_preimage);
        }

        execute_successful(commit, &satisfier.env, witness);

        // Policy 1

        let policy1 = Policy::And(vec![
            Policy::Sha256(sha256::Hash::from_inner([0; 32])),
            Policy::Sha256(images[1]),
        ]);
        assert!(policy1.satisfy(&satisfier).is_none());

        // Policy 2

        let policy2 = Policy::And(vec![
            Policy::Sha256(images[0]),
            Policy::Sha256(sha256::Hash::from_inner([0; 32])),
        ]);
        assert!(policy2.satisfy(&satisfier).is_none());
    }

    #[test]
    fn satisfy_or() {
        let satisfier = get_satisfier();
        let images: Vec<_> = satisfier.preimages.keys().map(|x| *x).collect();
        let preimages: Vec<_> = images
            .iter()
            .map(|x| satisfier.preimages.get(x).unwrap())
            .collect();

        let assert_branch = |policy: &Policy<bitcoin::XOnlyPublicKey>, branch: u8| {
            let (witness, commit) = policy.satisfy(&satisfier).expect("satisfiable");
            assert_eq!(2, witness.len());

            assert_eq!(Value::u1(branch), witness[0]);
            let preimage_bytes = witness[1].try_to_bytes().expect("to bytes");
            let witness_preimage =
                Preimage32::try_from(preimage_bytes.as_slice()).expect("to array");
            assert_eq!(preimages[branch as usize], &witness_preimage);

            execute_successful(commit, &satisfier.env, witness);
        };

        // Policy 0

        let policy0 = Policy::Or(vec![Policy::Sha256(images[0]), Policy::Sha256(images[1])]);
        assert_branch(&policy0, 0);

        // Policy 1

        let policy1 = Policy::Or(vec![
            Policy::Sha256(images[0]),
            Policy::Sha256(sha256::Hash::from_inner([1; 32])),
        ]);
        assert_branch(&policy1, 0);

        // Policy 2

        let policy2 = Policy::Or(vec![
            Policy::Sha256(sha256::Hash::from_inner([0; 32])),
            Policy::Sha256(images[1]),
        ]);
        assert_branch(&policy2, 1);

        // Policy 3

        let policy3 = Policy::Or(vec![
            Policy::Sha256(sha256::Hash::from_inner([0; 32])),
            Policy::Sha256(sha256::Hash::from_inner([1; 32])),
        ]);
        assert!(policy3.satisfy(&satisfier).is_none());
    }
}
