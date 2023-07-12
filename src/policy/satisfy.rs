use crate::analysis::Cost;
use crate::jet::Elements;
use crate::node::{RedeemNode, WitnessNode};
use crate::{miniscript, Error, Policy, Value};

use bitcoin_hashes::Hash;
use elements::locktime::Height;
use elements::taproot::TapLeafHash;
use elements::{LockTime, Sequence};
use miniscript::{Satisfier, ToPublicKey};

use std::convert::TryFrom;
use std::sync::Arc;

impl<Pk: ToPublicKey> Policy<Pk> {
    pub fn satisfy_internal<S: Satisfier<Pk>>(
        &self,
        satisfier: &S,
    ) -> Result<Arc<WitnessNode<Elements>>, Error> {
        let node = match *self {
            Policy::Unsatisfiable(entropy) => super::serialize::unsatisfiable(entropy),
            Policy::Trivial => super::serialize::trivial(),
            Policy::Key(ref key) => {
                let sig_wit = satisfier
                    .lookup_tap_leaf_script_sig(key, &TapLeafHash::all_zeros())
                    .map(|sig| Arc::new(Value::u512_from_slice(sig.sig.as_ref())));
                super::serialize::key(key, sig_wit)
            }
            Policy::After(n) => {
                let node = super::serialize::after(n);
                let height = Height::from_consensus(n).expect("timelock is valid");
                if satisfier.check_after(LockTime::Blocks(height)) {
                    node
                } else {
                    node.pruned()
                }
            }
            Policy::Older(n) => {
                let node = super::serialize::older(n);
                if satisfier.check_older(Sequence((n).into())) {
                    node
                } else {
                    node.pruned()
                }
            }
            Policy::Sha256(ref hash) => {
                let preimage_wit = satisfier
                    .lookup_sha256(hash)
                    .map(|preimage| Arc::new(Value::u256_from_slice(preimage.as_ref())));
                super::serialize::sha256::<_, Pk>(hash, preimage_wit)
            }
            Policy::And {
                ref left,
                ref right,
            } => {
                let left = left.satisfy_internal(satisfier)?;
                let right = right.satisfy_internal(satisfier)?;
                super::serialize::and(&left, &right)
            }
            Policy::Or {
                ref left,
                ref right,
            } => {
                let left = left.satisfy_internal(satisfier)?;
                let right = right.satisfy_internal(satisfier)?;

                let take_right = match (left.must_prune(), right.must_prune()) {
                    (false, false) => {
                        let left_cost = left.finalize()?.bounds().cost;
                        let right_cost = right.finalize()?.bounds().cost;
                        left_cost > right_cost
                    }
                    (false, true) => false,
                    (true, false) => true,
                    // If both children are must_prune the choice doesn't matter,
                    // the case node itself will be marked must_prune.
                    (true, true) => false,
                };

                if take_right {
                    super::serialize::or(&left.pruned(), &right, Some(Arc::new(Value::u1(1))))
                } else {
                    super::serialize::or(&left, &right.pruned(), Some(Arc::new(Value::u1(0))))
                }
                .prune_and_retype()
            }
            Policy::Threshold(k, ref subs) => {
                let nodes: Result<Vec<Arc<WitnessNode<_>>>, Error> = subs
                    .iter()
                    .map(|sub| sub.satisfy_internal(satisfier))
                    .collect();
                let mut nodes = nodes?;
                let mut costs = vec![Cost::CONSENSUS_MAX; subs.len()];
                // 0 means skip, 1 means don't skip
                let mut witness_bits = vec![Some(Arc::new(Value::u1(0))); subs.len()];

                for (cost, node) in costs.iter_mut().zip(nodes.iter()) {
                    if !node.must_prune() {
                        *cost = node.finalize()?.bounds().cost;
                    }
                }

                // Sort by witness cost and mark everything except the cheapest k as to-prune
                let mut sorted_costs: Vec<_> = costs.iter().copied().enumerate().collect();
                sorted_costs.sort_by_key(|pair| pair.1);
                let b1 = Arc::new(Value::u1(1));
                let mut threshold_failed = false;
                for &(idx, _) in &sorted_costs[..k] {
                    if nodes[idx].must_prune() {
                        // Unlike in the `or` case, where two pruned branches will automatically
                        // cause the `or` itself to be pruned, with thresholds we have to track
                        // this manually.
                        threshold_failed = true;
                    }
                    witness_bits[idx] = Some(Arc::clone(&b1));
                }
                for &(idx, _) in &sorted_costs[k..] {
                    nodes[idx] = nodes[idx].pruned();
                }

                let k = u32::try_from(k).expect("k less than 2^32");
                if threshold_failed {
                    super::serialize::threshold(k, &nodes, &witness_bits).pruned()
                } else {
                    super::serialize::threshold(k, &nodes, &witness_bits)
                }
                .prune_and_retype()
            }
        };
        Ok(node)
    }

    pub fn satisfy<S: Satisfier<Pk>>(
        &self,
        satisfier: S,
    ) -> Result<Arc<RedeemNode<Elements>>, Error> {
        let witnode = self.satisfy_internal(&satisfier)?;
        if witnode.must_prune() {
            Err(Error::IncompleteFinalization)
        } else {
            WitnessNode::finalize(&witnode.prune_and_retype())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::dag::{DagLike, NoSharing};
    use crate::jet::elements::ElementsEnv;
    use crate::node::SimpleFinalizer;
    use crate::{BitMachine, FailEntropy};
    use bitcoin_hashes::sha256;
    use elements::{bitcoin, secp256k1_zkp, PackedLockTime, SchnorrSig, SchnorrSigHashType};
    use miniscript::{MiniscriptKey, Preimage32};
    use std::collections::HashMap;
    use std::sync::Arc;

    struct HashSatisfier<Pk: MiniscriptKey>(HashMap<Pk::Sha256, Preimage32>);

    impl<Pk: ToPublicKey> Satisfier<Pk> for HashSatisfier<Pk> {
        fn lookup_sha256(&self, image: &Pk::Sha256) -> Option<Preimage32> {
            self.0.get(image).copied()
        }
    }

    fn get_hash_satisfier() -> HashSatisfier<bitcoin::XOnlyPublicKey> {
        let mut preimages = HashMap::new();

        for i in 0..3 {
            let preimage = [i; 32];
            preimages.insert(sha256::Hash::hash(&preimage), preimage);
        }

        HashSatisfier(preimages)
    }

    struct SigSatisfier<Pk: MiniscriptKey>(HashMap<Pk, SchnorrSig>);

    impl<Pk: ToPublicKey> Satisfier<Pk> for SigSatisfier<Pk> {
        fn lookup_tap_leaf_script_sig(&self, key: &Pk, _: &TapLeafHash) -> Option<SchnorrSig> {
            self.0.get(key).copied()
        }
    }

    fn get_sig_satisfier(env: &ElementsEnv) -> SigSatisfier<bitcoin::XOnlyPublicKey> {
        let mut signatures = HashMap::new();

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

        SigSatisfier(signatures)
    }

    fn execute_successful(program: Arc<RedeemNode<Elements>>, env: &ElementsEnv) {
        let mut mac = BitMachine::for_program(&program);
        assert!(mac.exec(&program, env).is_ok());
    }

    fn to_witness(program: &RedeemNode<Elements>) -> Vec<&Arc<Value>> {
        program
            .post_order_iter::<NoSharing>()
            .into_witnesses()
            .collect()
    }

    #[test]
    fn satisfy_unsatisfiable() {
        let env = ElementsEnv::dummy();
        let satisfier = get_sig_satisfier(&env);
        let policy = Policy::Unsatisfiable(FailEntropy::ZERO);

        assert!(policy.satisfy(&satisfier).is_err());

        let commit = policy.serialize_no_witness();
        let program = commit
            .finalize_types()
            .expect("finalize types")
            .finalize(&mut SimpleFinalizer::new(std::iter::empty()))
            .expect("finalize");
        let mut mac = BitMachine::for_program(&program);

        assert!(mac.exec(&program, &env).is_err());
    }

    #[test]
    fn satisfy_trivial() {
        let env = ElementsEnv::dummy();
        let satisfier = get_sig_satisfier(&env);
        let policy = Policy::Trivial;

        let program = policy.satisfy(&satisfier).expect("satisfiable");
        let witness = to_witness(&program);
        assert!(witness.is_empty());

        execute_successful(program, &env);
    }

    #[test]
    fn satisfy_pk() {
        let env = ElementsEnv::dummy();
        let satisfier = get_sig_satisfier(&env);
        let xonly = satisfier.0.keys().next().expect("satisfier has keys");
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
        let satisfier = get_hash_satisfier();
        let image = *satisfier.0.keys().next().expect("satisfier has image");
        let policy = Policy::Sha256(image);

        let program = policy.satisfy(&satisfier).expect("satisfiable");
        let witness = to_witness(&program);
        assert_eq!(1, witness.len());

        let witness_bytes = witness[0].try_to_bytes().expect("to bytes");
        let witness_preimage = Preimage32::try_from(witness_bytes.as_slice()).expect("to array");
        let preimage = *satisfier.0.get(&image).unwrap();
        assert_eq!(preimage, witness_preimage);

        execute_successful(program, &env);
    }

    #[test]
    fn satisfy_after() {
        let locktime = 42;
        let env = ElementsEnv::dummy_with(PackedLockTime(locktime), Sequence::ZERO);
        let satisfier = LockTime::Blocks(Height::from_consensus(locktime).expect("valid height"));

        let policy0: Policy<bitcoin::XOnlyPublicKey> = Policy::After(41);
        let program = policy0.satisfy(&satisfier).expect("satisfiable");
        let witness = to_witness(&program);
        assert!(witness.is_empty());
        execute_successful(program, &env);

        let policy1: Policy<bitcoin::XOnlyPublicKey> = Policy::After(42);
        let program = policy1.satisfy(&satisfier).expect("satisfiable");
        let witness = to_witness(&program);
        assert!(witness.is_empty());
        execute_successful(program, &env);

        let policy2: Policy<bitcoin::XOnlyPublicKey> = Policy::After(43);
        assert!(policy2.satisfy(&satisfier).is_err(), "unsatisfiable");
    }

    #[test]
    fn satisfy_older() {
        let sequence = Sequence::from_consensus(42);
        let env = ElementsEnv::dummy_with(PackedLockTime::ZERO, sequence);
        let satisfier = sequence;

        let policy0: Policy<bitcoin::XOnlyPublicKey> = Policy::Older(41);
        let program = policy0.satisfy(&satisfier).expect("satisfiable");
        let witness = to_witness(&program);
        assert!(witness.is_empty());
        execute_successful(program, &env);

        let policy1: Policy<bitcoin::XOnlyPublicKey> = Policy::Older(42);
        let program = policy1.satisfy(&satisfier).expect("satisfiable");
        let witness = to_witness(&program);
        assert!(witness.is_empty());
        execute_successful(program, &env);

        let policy2: Policy<bitcoin::XOnlyPublicKey> = Policy::Older(43);
        assert!(policy2.satisfy(&satisfier).is_err(), "unsatisfiable");
    }

    #[test]
    fn satisfy_and() {
        let env = ElementsEnv::dummy();
        let satisfier = get_hash_satisfier();
        let images: Vec<_> = satisfier.0.keys().copied().collect();
        let preimages: Vec<_> = images.iter().map(|x| satisfier.0.get(x).unwrap()).collect();

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
        assert!(policy1.satisfy(&satisfier).is_err());

        // Policy 2

        let policy2 = Policy::And {
            left: Arc::new(Policy::Sha256(images[0])),
            right: Arc::new(Policy::Sha256(sha256::Hash::from_inner([0; 32]))),
        };
        assert!(policy2.satisfy(&satisfier).is_err());
    }

    #[test]
    fn satisfy_or() {
        let env = ElementsEnv::dummy();
        let satisfier = get_hash_satisfier();
        let images: Vec<_> = satisfier.0.keys().copied().collect();
        let preimages: Vec<_> = images.iter().map(|x| satisfier.0.get(x).unwrap()).collect();

        let assert_branch = |policy: &Policy<bitcoin::XOnlyPublicKey>, bit: bool| {
            let program = policy.satisfy(&satisfier).expect("satisfiable");
            let witness = to_witness(&program);
            assert_eq!(2, witness.len());

            assert_eq!(Value::u1(bit as u8), **witness[0]);
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
        assert!(policy3.satisfy(&satisfier).is_err());
    }

    #[test]
    fn satisfy_thresh() {
        let env = ElementsEnv::dummy();
        let satisfier = get_hash_satisfier();
        let images: Vec<_> = satisfier.0.keys().copied().collect();
        let preimages: Vec<_> = images.iter().map(|x| satisfier.0.get(x).unwrap()).collect();

        let assert_branches = |policy: &Policy<bitcoin::XOnlyPublicKey>, bits: &[bool]| {
            let program = policy.satisfy(&satisfier).expect("satisfiable");
            let witness = to_witness(&program);
            assert_eq!(
                witness.len(),
                bits.len() + bits.iter().filter(|b| **b).count(),
            );

            let mut witidx = 0;
            for (bit_n, bit) in bits.iter().copied().enumerate() {
                assert_eq!(**witness[witidx], Value::u1(bit.into()));
                witidx += 1;
                if bit {
                    let preimage_bytes = witness[witidx].try_to_bytes().expect("to bytes");
                    let witness_preimage =
                        Preimage32::try_from(preimage_bytes.as_slice()).expect("to array");
                    assert_eq!(preimages[bit_n], &witness_preimage);
                    witidx += 1;
                }
            }

            execute_successful(program, &env);
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
                        _ => assert!(policy.satisfy(&satisfier).is_err()),
                    }
                }
            }
        }
    }
}
