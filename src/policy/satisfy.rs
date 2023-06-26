use crate::core::commit::UsedCaseBranch;
use crate::jet::Elements;
use crate::policy::compiler;
use crate::{CommitNode, Policy, RedeemNode, Value};
use bitcoin_hashes::Hash;
use elements::locktime::Height;
use elements::taproot::TapLeafHash;
use elements::{LockTime, Sequence};
use elements_miniscript::{MiniscriptKey, Preimage32, Satisfier, ToPublicKey};
use std::collections::{HashMap, VecDeque};
use std::iter::FromIterator;
use std::rc::Rc;

pub struct PolicySatisfier<'a, Pk: MiniscriptKey> {
    pub preimages: HashMap<Pk::Sha256, Preimage32>,
    pub signatures: HashMap<Pk, elements::SchnorrSig>,
    pub tx: &'a elements::Transaction,
    pub index: usize,
}

impl<'a, Pk: ToPublicKey> Satisfier<Pk> for PolicySatisfier<'a, Pk> {
    fn lookup_tap_leaf_script_sig(&self, pk: &Pk, _: &TapLeafHash) -> Option<elements::SchnorrSig> {
        self.signatures.get(pk).copied()
    }

    fn lookup_sha256(&self, hash: &Pk::Sha256) -> Option<Preimage32> {
        self.preimages.get(hash).copied()
    }

    fn check_older(&self, sequence: Sequence) -> bool {
        let self_sequence = self.tx.input[self.index].sequence;
        <Sequence as Satisfier<Pk>>::check_older(&self_sequence, sequence)
    }

    fn check_after(&self, locktime: LockTime) -> bool {
        let self_locktime = LockTime::from(self.tx.lock_time);
        <LockTime as Satisfier<Pk>>::check_after(&self_locktime, locktime)
    }
}

struct SatisfierExtData {
    witness: VecDeque<Value>,
    witness_cost: u64,
    program: Rc<CommitNode<Elements>>,
    program_cost: u64,
}

impl SatisfierExtData {
    fn cost(&self) -> u64 {
        self.witness_cost + self.program_cost * 8
    }
}

impl<Pk: ToPublicKey> Policy<Pk> {
    pub fn satisfy<S: Satisfier<Pk>>(&self, satisfier: &S) -> Option<Rc<RedeemNode<Elements>>> {
        let ext = self.satisfy_helper(satisfier)?;
        let program = ext.program.finalize(ext.witness.into_iter(), true).unwrap();
        Some(program)
    }

    fn compile_no_witness(&self) -> SatisfierExtData {
        SatisfierExtData {
            witness: VecDeque::new(),
            witness_cost: 0,
            program: self.compile().unwrap(),
            program_cost: 0,
        }
    }

    fn compile_witness_bytes(&self, witness: &[u8]) -> SatisfierExtData {
        let value = match witness.len() {
            32 => Value::u256_from_slice(witness),
            64 => Value::u512_from_slice(witness),
            _ => unimplemented!(),
        };
        let witness_data = VecDeque::from_iter(std::iter::once(value));
        let program = self.compile().unwrap();
        let program_cost = program.len() as u64;

        SatisfierExtData {
            witness: witness_data,
            witness_cost: witness.len() as u64 * 8,
            program,
            program_cost,
        }
    }

    fn satisfy_helper<S: Satisfier<Pk>>(&self, satisfier: &S) -> Option<SatisfierExtData> {
        match self {
            Policy::Unsatisfiable(..) => None,
            Policy::Trivial => Some(self.compile_no_witness()),
            Policy::Key(pk) => satisfier
                .lookup_tap_leaf_script_sig(pk, &TapLeafHash::all_zeros())
                .map(|sig| self.compile_witness_bytes(sig.sig.as_ref())),
            Policy::After(n) => {
                let height = Height::from_consensus(*n).ok()?;
                if satisfier.check_after(LockTime::Blocks(height)) {
                    Some(self.compile_no_witness())
                } else {
                    None
                }
            }
            Policy::Older(n) => {
                if satisfier.check_older(Sequence((*n).into())) {
                    Some(self.compile_no_witness())
                } else {
                    None
                }
            }
            Policy::Sha256(hash) => satisfier
                .lookup_sha256(hash)
                .map(|preimage| self.compile_witness_bytes(preimage.as_ref())),
            Policy::And { left, right } => {
                let mut left = left.satisfy_helper(satisfier)?;
                let right = right.satisfy_helper(satisfier)?;
                left.witness.extend(right.witness);
                left.witness_cost += right.witness_cost;
                left.program = compiler::and(left.program, right.program).ok()?;
                left.program_cost += right.program_cost + 1;

                Some(left)
            }
            Policy::Or { left, right } => {
                // TODO: Replace std::u64::MAX in MSRV >=1.43.0
                let mut left = left.satisfy_helper(satisfier).unwrap_or(SatisfierExtData {
                    witness: VecDeque::new(),
                    witness_cost: std::u64::MAX,
                    program: left.compile().unwrap(),
                    program_cost: 0,
                });
                let mut right = right.satisfy_helper(satisfier).unwrap_or(SatisfierExtData {
                    witness: VecDeque::new(),
                    witness_cost: std::u64::MAX,
                    program: right.compile().unwrap(),
                    program_cost: 0,
                });

                if left.cost() <= right.cost() {
                    // Both left and right path are unsatisfiable
                    // TODO: Replace std::u64::MAX in MSRV >=1.43.0
                    if left.cost() == std::u64::MAX {
                        return None;
                    }

                    left.witness.push_front(Value::u1(0));
                    left.witness_cost += 1;
                    left.program =
                        compiler::or(left.program, right.program, UsedCaseBranch::Left).ok()?;
                    left.program_cost += 6;
                    Some(left)
                } else {
                    right.witness.push_front(Value::u1(1));
                    right.witness_cost += 1;
                    right.program =
                        compiler::or(left.program, right.program, UsedCaseBranch::Right).ok()?;
                    right.program_cost += 6;
                    Some(right)
                }
            }
            Policy::Threshold(k, sub_policies) => {
                assert!(
                    sub_policies.len() >= 2,
                    "Thresholds must have at least two sub-policies"
                );

                let mut satisfiable_children = Vec::new();

                for (index, policy) in sub_policies.iter().enumerate() {
                    if let Some(ext) = policy.satisfy_helper(satisfier) {
                        satisfiable_children.push((ext, index));
                    }
                }

                if satisfiable_children.len() < *k {
                    return None;
                }

                // 1) Sort by witness cost
                // 2) Select k best satisfactions
                // 3) Sort by index in decreasing order
                satisfiable_children.sort_by_key(|(sat, _)| sat.cost());
                satisfiable_children.truncate(*k);
                satisfiable_children.sort_by(|(_, i), (_, j)| j.cmp(i));

                /// Return satisfaction for `i`th child, if it exists, or return dummy instead
                fn get_sat_or_default<Pk: ToPublicKey>(
                    i: usize,
                    satisfiable_children: &mut Vec<(SatisfierExtData, usize)>,
                    sub_policies: &[Policy<Pk>],
                ) -> SatisfierExtData {
                    if let Some(x) = satisfiable_children.last() {
                        if x.1 == i {
                            return satisfiable_children.pop().unwrap().0;
                        }
                    }

                    SatisfierExtData {
                        witness: VecDeque::new(),
                        witness_cost: 0,
                        program: sub_policies[i].compile().unwrap(),
                        program_cost: 0,
                    }
                }

                /// Return summand program for `i`th child
                fn get_summand<Pk: ToPublicKey>(
                    i: usize,
                    satisfiable_children: &mut Vec<(SatisfierExtData, usize)>,
                    sub_policies: &[Policy<Pk>],
                ) -> SatisfierExtData {
                    let mut sat = get_sat_or_default(i, satisfiable_children, sub_policies);
                    // Satisfactions always have non-zero program cost
                    let branch = if sat.program_cost == 0 {
                        sat.witness.push_front(Value::u1(0));
                        UsedCaseBranch::Right
                    } else {
                        sat.witness.push_front(Value::u1(1));
                        UsedCaseBranch::Left
                    };
                    sat.witness_cost += 1;
                    sat.program = compiler::thresh_summand(sat.program, branch).unwrap();
                    sat.program_cost += 9;

                    sat
                }

                let mut sat = get_summand(0, &mut satisfiable_children, sub_policies);

                for i in 1..sub_policies.len() {
                    let child_sat = get_summand(i, &mut satisfiable_children, sub_policies);

                    sat.witness.extend(child_sat.witness);
                    sat.witness_cost += child_sat.witness_cost;
                    sat.program = compiler::thresh_add(sat.program, child_sat.program).unwrap();
                    sat.program_cost += child_sat.program_cost + 6;
                }

                sat.program = compiler::thresh_verify(sat.program, *k as u32).unwrap();
                sat.program_cost += 4;

                Some(sat)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::dag::{DagLike, NoSharing};
    use crate::jet::elements::ElementsEnv;
    use crate::{BitMachine, FailEntropy};
    use bitcoin_hashes::{sha256, Hash};
    use elements::{bitcoin, secp256k1_zkp, PackedLockTime, SchnorrSigHashType};
    use std::convert::TryFrom;
    use std::sync::Arc;

    fn get_satisfier(env: &ElementsEnv) -> PolicySatisfier<bitcoin::XOnlyPublicKey> {
        let mut preimages = HashMap::new();
        let mut signatures = HashMap::new();

        for i in 0..3 {
            let preimage = [i; 32];
            preimages.insert(sha256::Hash::hash(&preimage), preimage);
        }

        let secp = secp256k1_zkp::Secp256k1::new();
        let mut rng = secp256k1_zkp::rand::rngs::ThreadRng::default();

        for _ in 0..3 {
            let keypair = bitcoin::KeyPair::new(&secp, &mut rng);
            let xonly = keypair.x_only_public_key().0;

            let sighash = env.c_tx_env().sighash_all();
            let msg = secp256k1_zkp::Message::from_slice(&sighash).expect("constant sighash");
            let sig = elements::SchnorrSig {
                sig: keypair.sign_schnorr(msg),
                hash_ty: SchnorrSigHashType::All,
            };

            signatures.insert(xonly, sig);
        }

        PolicySatisfier {
            preimages,
            signatures,
            tx: env.tx(),
            index: 0,
        }
    }

    fn execute_successful(program: Rc<RedeemNode<Elements>>, env: &ElementsEnv) {
        let program = program.to_node();
        let mut mac = BitMachine::for_program(&program);
        assert!(mac.exec(&program, env).is_ok());
    }

    fn to_witness(program: &RedeemNode<Elements>) -> Vec<&Value> {
        program
            .post_order_iter::<NoSharing>()
            .into_witnesses()
            .collect()
    }

    #[test]
    fn satisfy_unsatisfiable() {
        let env = ElementsEnv::dummy();
        let satisfier = get_satisfier(&env);
        let policy = Policy::Unsatisfiable(FailEntropy::ZERO);

        assert!(policy.satisfy(&satisfier).is_none());

        let commit = policy.compile().expect("compile");
        let program = commit.finalize(std::iter::empty(), true).expect("finalize");
        let program = program.to_node();
        let mut mac = BitMachine::for_program(&program);

        assert!(mac.exec(&program, &env).is_err());
    }

    #[test]
    fn satisfy_trivial() {
        let env = ElementsEnv::dummy();
        let satisfier = get_satisfier(&env);
        let policy = Policy::Trivial;

        let program = policy.satisfy(&satisfier).expect("satisfiable");
        let witness = to_witness(&program);
        assert_eq!(Vec::<&Value>::new(), witness);

        execute_successful(program, &env);
    }

    #[test]
    fn satisfy_pk() {
        let env = ElementsEnv::dummy();
        let satisfier = get_satisfier(&env);
        let mut it = satisfier.signatures.keys();
        let xonly = it.next().unwrap();
        let policy = Policy::Key(*xonly);

        let program = policy.satisfy(&satisfier).expect("satisfiable");
        let witness = to_witness(&program);
        assert_eq!(1, witness.len());

        let sighash = env.c_tx_env().sighash_all();
        let message = secp256k1_zkp::Message::from(sighash);
        let signature_bytes = witness[0].try_to_bytes().expect("to bytes");
        let signature =
            secp256k1_zkp::schnorr::Signature::from_slice(&signature_bytes).expect("to signature");
        assert!(signature.verify(&message, xonly).is_ok());

        execute_successful(program, &env);
    }

    #[test]
    fn satisfy_sha256() {
        let env = ElementsEnv::dummy();
        let satisfier = get_satisfier(&env);
        let mut it = satisfier.preimages.keys();
        let image = *it.next().unwrap();
        let policy = Policy::Sha256(image);

        let program = policy.satisfy(&satisfier).expect("satisfiable");
        let witness = to_witness(&program);
        assert_eq!(1, witness.len());

        let witness_bytes = witness[0].try_to_bytes().expect("to bytes");
        let witness_preimage = Preimage32::try_from(witness_bytes.as_slice()).expect("to array");
        let preimage = *satisfier.preimages.get(&image).unwrap();
        assert_eq!(preimage, witness_preimage);

        execute_successful(program, &env);
    }

    #[test]
    fn satisfy_after() {
        let env = ElementsEnv::dummy_with(PackedLockTime(42), Sequence::ZERO);
        let satisfier = get_satisfier(&env);

        let policy0 = Policy::After(41);
        let program = policy0.satisfy(&satisfier).expect("satisfiable");
        let witness = to_witness(&program);
        assert!(witness.is_empty());
        execute_successful(program, &env);

        let policy1 = Policy::After(42);
        let program = policy1.satisfy(&satisfier).expect("satisfiable");
        let witness = to_witness(&program);
        assert!(witness.is_empty());
        execute_successful(program, &env);

        let policy2 = Policy::After(43);
        assert!(policy2.satisfy(&satisfier).is_none(), "unsatisfiable");
    }

    #[test]
    fn satisfy_older() {
        let env = ElementsEnv::dummy_with(PackedLockTime::ZERO, Sequence::from_consensus(42));
        let satisfier = get_satisfier(&env);

        let policy0 = Policy::Older(41);
        let program = policy0.satisfy(&satisfier).expect("satisfiable");
        let witness = to_witness(&program);
        assert!(witness.is_empty());
        execute_successful(program, &env);

        let policy1 = Policy::Older(42);
        let program = policy1.satisfy(&satisfier).expect("satisfiable");
        let witness = to_witness(&program);
        assert!(witness.is_empty());
        execute_successful(program, &env);

        let policy2 = Policy::Older(43);
        assert!(policy2.satisfy(&satisfier).is_none(), "unsatisfiable");
    }

    #[test]
    fn satisfy_and() {
        let env = ElementsEnv::dummy();
        let satisfier = get_satisfier(&env);
        let images: Vec<_> = satisfier.preimages.keys().copied().collect();
        let preimages: Vec<_> = images
            .iter()
            .map(|x| satisfier.preimages.get(x).unwrap())
            .collect();

        // Policy 0

        let policy0 = Policy::And {
            left: Arc::new(Policy::Sha256(images[0])),
            right: Arc::new(Policy::Sha256(images[1])),
        };
        let program = policy0.satisfy(&satisfier).expect("satisfiable");
        let witness = to_witness(&program);
        assert_eq!(2, witness.len());

        for i in 0..2 {
            let preimage_bytes = witness[i].try_to_bytes().expect("to bytes");
            let witness_preimage =
                Preimage32::try_from(preimage_bytes.as_slice()).expect("to array");
            assert_eq!(preimages[i], &witness_preimage);
        }

        execute_successful(program, &env);

        // Policy 1

        let policy1 = Policy::And {
            left: Arc::new(Policy::Sha256(sha256::Hash::from_inner([0; 32]))),
            right: Arc::new(Policy::Sha256(images[1])),
        };
        assert!(policy1.satisfy(&satisfier).is_none());

        // Policy 2

        let policy2 = Policy::And {
            left: Arc::new(Policy::Sha256(images[0])),
            right: Arc::new(Policy::Sha256(sha256::Hash::from_inner([0; 32]))),
        };
        assert!(policy2.satisfy(&satisfier).is_none());
    }

    #[test]
    fn satisfy_or() {
        let env = ElementsEnv::dummy();
        let satisfier = get_satisfier(&env);
        let images: Vec<_> = satisfier.preimages.keys().copied().collect();
        let preimages: Vec<_> = images
            .iter()
            .map(|x| satisfier.preimages.get(x).unwrap())
            .collect();

        let assert_branch = |policy: &Policy<bitcoin::XOnlyPublicKey>, bit: bool| {
            let program = policy.satisfy(&satisfier).expect("satisfiable");
            let witness = to_witness(&program);
            assert_eq!(2, witness.len());

            assert_eq!(&Value::u1(bit as u8), witness[0]);
            let preimage_bytes = witness[1].try_to_bytes().expect("to bytes");
            let witness_preimage =
                Preimage32::try_from(preimage_bytes.as_slice()).expect("to array");
            assert_eq!(preimages[bit as usize], &witness_preimage);

            execute_successful(program, &env);
        };

        // Policy 0

        let policy0 = Policy::Or {
            left: Arc::new(Policy::Sha256(images[0])),
            right: Arc::new(Policy::Sha256(images[1])),
        };
        assert_branch(&policy0, false);

        // Policy 1

        let policy1 = Policy::Or {
            left: Arc::new(Policy::Sha256(images[0])),
            right: Arc::new(Policy::Sha256(sha256::Hash::from_inner([1; 32]))),
        };
        assert_branch(&policy1, false);

        // Policy 2

        let policy2 = Policy::Or {
            left: Arc::new(Policy::Sha256(sha256::Hash::from_inner([0; 32]))),
            right: Arc::new(Policy::Sha256(images[1])),
        };
        assert_branch(&policy2, true);

        // Policy 3

        let policy3 = Policy::Or {
            left: Arc::new(Policy::Sha256(sha256::Hash::from_inner([0; 32]))),
            right: Arc::new(Policy::Sha256(sha256::Hash::from_inner([1; 32]))),
        };
        assert!(policy3.satisfy(&satisfier).is_none());
    }

    #[test]
    fn satisfy_thresh() {
        let env = ElementsEnv::dummy();
        let satisfier = get_satisfier(&env);
        let images: Vec<_> = satisfier.preimages.keys().copied().collect();
        let preimages: Vec<_> = images
            .iter()
            .map(|x| satisfier.preimages.get(x).unwrap())
            .collect();

        let assert_branches = |policy: &Policy<bitcoin::XOnlyPublicKey>, bits: &[bool]| {
            let program = policy.satisfy(&satisfier).expect("satisfiable");
            let witness = to_witness(&program);
            assert_eq!(
                witness.len(),
                bits.len() + bits.iter().filter(|b| **b).count(),
            );

            let mut witidx = 0;
            for (bit_n, bit) in bits.iter().copied().enumerate() {
                assert_eq!(witness[witidx], &Value::u1(bit.into()));
                witidx += 1;
                if bit {
                    let preimage_bytes = witness[witidx].try_to_bytes().expect("to bytes");
                    let witness_preimage =
                        Preimage32::try_from(preimage_bytes.as_slice()).expect("to array");
                    assert_eq!(preimages[bit_n], &witness_preimage);
                    witidx += 1;
                }
            }
        };

        let image_from_bit = |bit: bool, j: u8| {
            if bit {
                images[j as usize]
            } else {
                sha256::Hash::from_inner([j; 32])
            }
        };

        for &bit0 in &[true, false] {
            let image0 = image_from_bit(bit0, 0);

            for &bit1 in &[true, false] {
                let image1 = image_from_bit(bit1, 1);

                for &bit2 in &[true, false] {
                    let image2 = image_from_bit(bit2, 2);

                    let policy = Policy::Threshold(
                        2,
                        vec![
                            Policy::Sha256(image0),
                            Policy::Sha256(image1),
                            Policy::Sha256(image2),
                        ],
                    );

                    match bit0 as u8 + bit1 as u8 + bit2 as u8 {
                        3 => assert_branches(&policy, &[bit0, bit1, false]),
                        2 => assert_branches(&policy, &[bit0, bit1, bit2]),
                        _ => assert!(policy.satisfy(&satisfier).is_none()),
                    }
                }
            }
        }
    }
}
