// SPDX-License-Identifier: CC0-1.0

use crate::analysis::Cost;
use crate::jet::Elements;
use crate::node::{RedeemNode, WitnessNode};
use crate::policy::ToXOnlyPubkey;
use crate::{Cmr, Error, Policy, Value};
use elements::bitcoin;

use elements::locktime::Height;
use elements::taproot::TapLeafHash;
use hashes::Hash;

use std::convert::TryFrom;
use std::sync::Arc;

/// Type alias for 32-byte preimage.
pub type Preimage32 = [u8; 32];

/// Lookup table for signatures, hash preimages, etc.
///
/// Every method has a default implementation that simply returns `None`
/// on every query. Users are expected to override the methods that they
/// have data for.
pub trait Satisfier<Pk: ToXOnlyPubkey> {
    /// Given a public key, look up a Schnorr signature with that key.
    fn lookup_tap_leaf_script_sig(&self, _: &Pk, _: &TapLeafHash) -> Option<elements::SchnorrSig> {
        None
    }

    /// Given a SHA256 hash, look up its preimage.
    fn lookup_sha256(&self, _: &Pk::Sha256) -> Option<Preimage32> {
        None
    }

    /// Assert that a relative locktime is satisfied.
    fn check_older(&self, _: elements::Sequence) -> bool {
        false
    }

    /// Assert that an absolute locktime is satisfied.
    fn check_after(&self, _: elements::LockTime) -> bool {
        false
    }

    /// Given a CMR, look up a matching satisfied Simplicity program.
    ///
    /// It is the **responsibility of the satisfier** to make sure that given **program is satisfied**.
    /// That is, each witness note is populated with a value of the correct type
    /// and the program successfully runs on the Bit Machine.
    ///
    /// If the satisfier provides an unsatisfied program,
    /// then this may **corrupt** the computation of an **overall satisfaction**.
    /// That is, the resulting "satisfaction" fails to satisfy the program.
    fn lookup_asm_program(&self, _: Cmr) -> Option<Arc<WitnessNode<Elements>>> {
        None
    }
}

impl<Pk: ToXOnlyPubkey> Satisfier<Pk> for elements::Sequence {
    fn check_older(&self, n: elements::Sequence) -> bool {
        use elements::bitcoin::locktime::relative::LockTime::*;

        let this = match bitcoin::Sequence(self.0).to_relative_lock_time() {
            Some(x) => x,
            None => return false,
        };
        let n = match bitcoin::Sequence(n.0).to_relative_lock_time() {
            Some(x) => x,
            None => return false,
        };

        match (n, this) {
            (Blocks(n), Blocks(lock_time)) => n <= lock_time,
            (Time(n), Time(lock_time)) => n <= lock_time,
            _ => false, // Not the same units
        }
    }
}

impl<Pk: ToXOnlyPubkey> Satisfier<Pk> for elements::LockTime {
    fn check_after(&self, n: elements::LockTime) -> bool {
        use elements::LockTime::*;

        match (n, *self) {
            (Blocks(n), Blocks(lock_time)) => n <= lock_time,
            (Seconds(n), Seconds(lock_time)) => n <= lock_time,
            _ => false, // Not the same units.
        }
    }
}

impl<Pk: ToXOnlyPubkey> Policy<Pk> {
    fn satisfy_internal<S: Satisfier<Pk>>(
        &self,
        satisfier: &S,
    ) -> Result<Arc<WitnessNode<Elements>>, Error> {
        let node = match *self {
            Policy::Unsatisfiable(entropy) => super::serialize::unsatisfiable(entropy),
            Policy::Trivial => super::serialize::trivial(),
            Policy::Key(ref key) => {
                let sig_wit = satisfier
                    .lookup_tap_leaf_script_sig(key, &TapLeafHash::all_zeros())
                    .map(|sig| Value::u512_from_slice(sig.sig.as_ref()));
                super::serialize::key(key, sig_wit)
            }
            Policy::After(n) => {
                let node = super::serialize::after::<Arc<_>>(n);
                let height = Height::from_consensus(n).expect("timelock is valid");
                if satisfier.check_after(elements::LockTime::Blocks(height)) {
                    node
                } else {
                    node.pruned()
                }
            }
            Policy::Older(n) => {
                let node = super::serialize::older::<Arc<_>>(n);
                if satisfier.check_older(elements::Sequence((n).into())) {
                    node
                } else {
                    node.pruned()
                }
            }
            Policy::Sha256(ref hash) => {
                let preimage_wit = satisfier
                    .lookup_sha256(hash)
                    .map(|preimage| Value::u256_from_slice(preimage.as_ref()));
                super::serialize::sha256::<Pk, _, _>(hash, preimage_wit)
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
                    super::serialize::or(&left.pruned(), &right, Some(Value::u1(1)))
                } else {
                    super::serialize::or(&left, &right.pruned(), Some(Value::u1(0)))
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
                let mut witness_bits = vec![Some(Value::u1(0)); subs.len()];

                for (cost, node) in costs.iter_mut().zip(nodes.iter()) {
                    if !node.must_prune() {
                        *cost = node.finalize()?.bounds().cost;
                    }
                }

                // Sort by witness cost and mark everything except the cheapest k as to-prune
                let mut sorted_costs: Vec<_> = costs.iter().copied().enumerate().collect();
                sorted_costs.sort_by_key(|pair| pair.1);
                let b1 = Value::u1(1);
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
            Policy::Assembly(cmr) => satisfier
                .lookup_asm_program(cmr)
                .ok_or(Error::IncompleteFinalization)?,
        };
        Ok(node)
    }

    pub fn satisfy<S: Satisfier<Pk>>(
        &self,
        satisfier: &S,
    ) -> Result<Arc<RedeemNode<Elements>>, Error> {
        let witnode = self.satisfy_internal(satisfier)?;
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
    use crate::node::{CoreConstructible, JetConstructible, SimpleFinalizer, WitnessConstructible};
    use crate::policy::serialize;
    use crate::{BitMachine, FailEntropy, SimplicityKey};
    use elements::bitcoin::key::{Keypair, XOnlyPublicKey};
    use elements::secp256k1_zkp;
    use hashes::{sha256, Hash};
    use std::collections::HashMap;
    use std::sync::Arc;

    pub struct PolicySatisfier<'a, Pk: SimplicityKey> {
        pub preimages: HashMap<Pk::Sha256, Preimage32>,
        pub signatures: HashMap<Pk, elements::SchnorrSig>,
        pub assembly: HashMap<Cmr, Arc<WitnessNode<Elements>>>,
        pub tx: &'a elements::Transaction,
        pub index: usize,
    }

    impl<'a, Pk: ToXOnlyPubkey> Satisfier<Pk> for PolicySatisfier<'a, Pk> {
        fn lookup_tap_leaf_script_sig(
            &self,
            pk: &Pk,
            _: &TapLeafHash,
        ) -> Option<elements::SchnorrSig> {
            self.signatures.get(pk).copied()
        }

        fn lookup_sha256(&self, hash: &Pk::Sha256) -> Option<Preimage32> {
            self.preimages.get(hash).copied()
        }

        fn check_older(&self, sequence: elements::Sequence) -> bool {
            let self_sequence = self.tx.input[self.index].sequence;
            <elements::Sequence as Satisfier<Pk>>::check_older(&self_sequence, sequence)
        }

        fn check_after(&self, locktime: elements::LockTime) -> bool {
            <elements::LockTime as Satisfier<Pk>>::check_after(&self.tx.lock_time, locktime)
        }

        fn lookup_asm_program(&self, cmr: Cmr) -> Option<Arc<WitnessNode<Elements>>> {
            self.assembly.get(&cmr).cloned()
        }
    }

    fn get_satisfier(
        env: &ElementsEnv<Arc<elements::Transaction>>,
    ) -> PolicySatisfier<XOnlyPublicKey> {
        let mut preimages = HashMap::new();

        for i in 0..3 {
            let preimage = [i; 32];
            preimages.insert(sha256::Hash::hash(&preimage), preimage);
        }

        let secp = secp256k1_zkp::Secp256k1::new();
        let mut rng = secp256k1_zkp::rand::rngs::ThreadRng::default();
        let mut signatures = HashMap::new();

        for _ in 0..3 {
            let keypair = Keypair::new(&secp, &mut rng);
            let xonly = keypair.x_only_public_key().0;

            let sighash = env.c_tx_env().sighash_all();
            let msg = secp256k1_zkp::Message::from(sighash);
            let sig = elements::SchnorrSig {
                sig: keypair.sign_schnorr(msg),
                hash_ty: elements::SchnorrSighashType::All,
            };

            signatures.insert(xonly, sig);
        }

        PolicySatisfier {
            preimages,
            signatures,
            assembly: HashMap::new(),
            tx: env.tx(),
            index: env.ix() as usize,
        }
    }

    fn execute_successful(
        program: Arc<RedeemNode<Elements>>,
        env: &ElementsEnv<Arc<elements::Transaction>>,
    ) {
        let mut mac = BitMachine::for_program(&program);
        assert!(mac.exec(&program, env).is_ok());
    }

    fn execute_unsuccessful(
        program: Arc<RedeemNode<Elements>>,
        env: &ElementsEnv<Arc<elements::Transaction>>,
    ) {
        let mut mac = BitMachine::for_program(&program);
        assert!(mac.exec(&program, env).is_err());
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
        let satisfier = get_satisfier(&env);
        let policy = Policy::Unsatisfiable(FailEntropy::ZERO);

        assert!(policy.satisfy(&satisfier).is_err());

        let commit = policy.commit().expect("no asm");
        let program = commit
            .finalize(&mut SimpleFinalizer::new(std::iter::empty()))
            .expect("finalize");

        execute_unsuccessful(program, &env);
    }

    #[test]
    fn satisfy_trivial() {
        let env = ElementsEnv::dummy();
        let satisfier = get_satisfier(&env);
        let policy = Policy::Trivial;

        let program = policy.satisfy(&satisfier).expect("satisfiable");
        let witness = to_witness(&program);
        assert!(witness.is_empty());

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
        let height = Height::from_consensus(42).unwrap();
        let env =
            ElementsEnv::dummy_with(elements::LockTime::Blocks(height), elements::Sequence::ZERO);
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
        assert!(policy2.satisfy(&satisfier).is_err(), "unsatisfiable");
    }

    #[test]
    fn satisfy_older() {
        let env = ElementsEnv::dummy_with(
            elements::LockTime::ZERO,
            elements::Sequence::from_consensus(42),
        );
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
        assert!(policy2.satisfy(&satisfier).is_err(), "unsatisfiable");
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
            left: Arc::new(Policy::Sha256(sha256::Hash::from_byte_array([0; 32]))),
            right: Arc::new(Policy::Sha256(images[1])),
        };
        assert!(policy1.satisfy(&satisfier).is_err());

        // Policy 2

        let policy2 = Policy::And {
            left: Arc::new(Policy::Sha256(images[0])),
            right: Arc::new(Policy::Sha256(sha256::Hash::from_byte_array([0; 32]))),
        };
        assert!(policy2.satisfy(&satisfier).is_err());
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

        let assert_branch = |policy: &Policy<XOnlyPublicKey>, bit: bool| {
            let program = policy.satisfy(&satisfier).expect("satisfiable");
            let witness = to_witness(&program);
            assert_eq!(2, witness.len());

            assert_eq!(Value::u1(bit as u8), *witness[0]);
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
            right: Arc::new(Policy::Sha256(sha256::Hash::from_byte_array([1; 32]))),
        };
        assert_branch(&policy1, false);

        // Policy 2

        let policy2 = Policy::Or {
            left: Arc::new(Policy::Sha256(sha256::Hash::from_byte_array([0; 32]))),
            right: Arc::new(Policy::Sha256(images[1])),
        };
        assert_branch(&policy2, true);

        // Policy 3

        let policy3 = Policy::Or {
            left: Arc::new(Policy::Sha256(sha256::Hash::from_byte_array([0; 32]))),
            right: Arc::new(Policy::Sha256(sha256::Hash::from_byte_array([1; 32]))),
        };
        assert!(policy3.satisfy(&satisfier).is_err());
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

        let assert_branches = |policy: &Policy<XOnlyPublicKey>, bits: &[bool]| {
            let program = policy.satisfy(&satisfier).expect("satisfiable");
            let witness = to_witness(&program);
            assert_eq!(
                witness.len(),
                bits.len() + bits.iter().filter(|b| **b).count(),
            );

            let mut witidx = 0;
            for (bit_n, bit) in bits.iter().copied().enumerate() {
                assert_eq!(*witness[witidx], Value::u1(bit.into()));
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
                sha256::Hash::from_byte_array([j; 32])
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

    #[test]
    fn satisfy_asm() {
        let env = ElementsEnv::dummy();
        let mut satisfier = get_satisfier(&env);

        let mut assert_branch = |witness0: Arc<Value>, witness1: Arc<Value>| {
            let asm_program = serialize::verify_bexp(
                &Arc::<WitnessNode<Elements>>::pair(
                    &Arc::<WitnessNode<Elements>>::witness(Some(witness0.clone())),
                    &Arc::<WitnessNode<Elements>>::witness(Some(witness1.clone())),
                )
                .expect("sound types"),
                &Arc::<WitnessNode<Elements>>::jet(Elements::Eq8),
            );
            let cmr = asm_program.cmr();
            satisfier.assembly.insert(cmr, asm_program);

            let policy = Policy::Assembly(cmr);
            let program = policy.satisfy(&satisfier).expect("satisfiable");
            let witness = to_witness(&program);

            assert_eq!(2, witness.len());
            assert_eq!(&witness0, witness[0]);
            assert_eq!(&witness1, witness[1]);

            if witness0 == witness1 {
                execute_successful(program, &env);
            } else {
                execute_unsuccessful(program, &env);
            }
        };

        for a in 0..2 {
            for b in 0..2 {
                assert_branch(Value::u8(a), Value::u8(b))
            }
        }
    }
}
