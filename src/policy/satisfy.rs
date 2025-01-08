// SPDX-License-Identifier: CC0-1.0

use crate::analysis::Cost;
use crate::jet::Elements;
use crate::node::{RedeemNode, WitnessNode};
use crate::policy::ToXOnlyPubkey;
use crate::types;
use crate::{Cmr, Policy, Value};

use elements::bitcoin;
use elements::locktime::Height;
use elements::taproot::TapLeafHash;
use hashes::Hash;

use crate::jet::elements::ElementsEnv;
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

    /// Assert that a relative lock time is satisfied.
    fn check_older(&self, _: elements::Sequence) -> bool {
        false
    }

    /// Assert that an absolute lock time is satisfied.
    fn check_after(&self, _: elements::LockTime) -> bool {
        false
    }

    /// Given a CMR, look up a matching assembly program.
    ///
    /// ## Successful execution
    ///
    /// It is the **responsibility of the implementor** to ensure that the returned assembly
    /// program has sufficient witness data to successfully run on the Bit Machine.
    ///
    /// The execution of a program depends on the transaction environment,
    /// so implementations should compute witness data based on that.
    ///
    /// If the assembly program fails to run for the current transaction environment,
    /// then implementations should return `None`.
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

#[derive(Debug)]
pub enum SatisfierError {
    /// A satisfying witness for the policy could not be produced.
    Unsatisfiable,
    /// An assembly program was missing.
    ///
    /// The satisfier needs to know all assembly programs, **even those that are not executed**!
    MissingAssembly(Cmr),
    /// An assembly program failed to run on the Bit Machine.
    ///
    /// This can happen when the [`Satisfier::lookup_asm_program`] method is incorrectly implemented.
    AssemblyFailed(crate::bit_machine::ExecutionError),
}

#[derive(Clone, Debug)]
struct SatResult {
    serialization: Arc<WitnessNode<Elements>>,
    is_satisfied: bool,
}

impl<Pk: ToXOnlyPubkey> Policy<Pk> {
    fn satisfy_internal<S: Satisfier<Pk>>(
        &self,
        inference_context: &types::Context,
        satisfier: &S,
    ) -> Result<SatResult, SatisfierError> {
        let node = match *self {
            Policy::Unsatisfiable(entropy) => SatResult {
                serialization: super::serialize::unsatisfiable(inference_context, entropy),
                is_satisfied: false,
            },
            Policy::Trivial => SatResult {
                serialization: super::serialize::trivial(inference_context),
                is_satisfied: true,
            },
            Policy::Key(ref key) => {
                let signature = satisfier
                    .lookup_tap_leaf_script_sig(key, &TapLeafHash::all_zeros())
                    .map(|sig| sig.sig.serialize())
                    .map(Value::u512);
                SatResult {
                    is_satisfied: signature.is_some(),
                    serialization: super::serialize::key(inference_context, key, signature),
                }
            }
            Policy::After(n) => {
                let height = Height::from_consensus(n).expect("timelock is valid");
                SatResult {
                    serialization: super::serialize::after::<Arc<_>>(inference_context, n),
                    is_satisfied: satisfier.check_after(elements::LockTime::Blocks(height)),
                }
            }
            Policy::Older(n) => SatResult {
                serialization: super::serialize::older::<Arc<_>>(inference_context, n),
                is_satisfied: satisfier.check_older(elements::Sequence(n.into())),
            },
            Policy::Sha256(ref hash) => {
                let preimage = satisfier.lookup_sha256(hash).map(Value::u256);
                SatResult {
                    is_satisfied: preimage.is_some(),
                    serialization: super::serialize::sha256::<Pk, _, _>(
                        inference_context,
                        hash,
                        preimage,
                    ),
                }
            }
            Policy::And {
                ref left,
                ref right,
            } => {
                let SatResult {
                    serialization: left,
                    is_satisfied: left_ok,
                } = left.satisfy_internal(inference_context, satisfier)?;
                let SatResult {
                    serialization: right,
                    is_satisfied: right_ok,
                } = right.satisfy_internal(inference_context, satisfier)?;
                SatResult {
                    serialization: super::serialize::and(&left, &right),
                    is_satisfied: left_ok && right_ok,
                }
            }
            Policy::Or {
                ref left,
                ref right,
            } => {
                let SatResult {
                    serialization: left,
                    is_satisfied: left_ok,
                } = left.satisfy_internal(inference_context, satisfier)?;
                let SatResult {
                    serialization: right,
                    is_satisfied: right_ok,
                } = right.satisfy_internal(inference_context, satisfier)?;
                let take_right = match (left_ok, right_ok) {
                    (false, false) => {
                        let left_cost = left
                            .finalize_unpruned()
                            .expect("serialization should be sound")
                            .bounds()
                            .cost;
                        let right_cost = right
                            .finalize_unpruned()
                            .expect("serialization should be sound")
                            .bounds()
                            .cost;
                        right_cost < left_cost
                    }
                    (false, true) => true,
                    (true, false) => false,
                    // If both children are unsatisfiable, then the choice doesn't matter.
                    // The entire expression will be pruned out.
                    (true, true) => false,
                };

                let serialization = if take_right {
                    super::serialize::or(&left, &right, Some(Value::u1(1)))
                } else {
                    super::serialize::or(&left, &right, Some(Value::u1(0)))
                };
                SatResult {
                    serialization,
                    is_satisfied: left_ok || right_ok,
                }
            }
            Policy::Threshold(k, ref subs) => {
                let node_results: Vec<SatResult> = subs
                    .iter()
                    .map(|sub| sub.satisfy_internal(inference_context, satisfier))
                    .collect::<Result<_, SatisfierError>>()?;
                let costs: Vec<Cost> = node_results
                    .iter()
                    .map(|result| match result.is_satisfied {
                        true => result
                            .serialization
                            .finalize_unpruned()
                            .map(|redeem| redeem.bounds().cost)
                            .unwrap_or(Cost::CONSENSUS_MAX),
                        false => Cost::CONSENSUS_MAX,
                    })
                    .collect();
                let selected_node_indices = {
                    let mut indices: Vec<usize> = (0..costs.len()).collect();
                    indices.sort_by_key(|&i| costs[i]);
                    indices.truncate(k);
                    indices
                };
                let all_selected_ok = selected_node_indices
                    .iter()
                    .all(|&i| node_results[i].is_satisfied);
                let witness_bits: Vec<Option<Value>> = (0..node_results.len())
                    .map(|i| Some(Value::u1(u8::from(selected_node_indices.contains(&i)))))
                    .collect();

                let k = u32::try_from(k).expect("k should be less than 2^32");
                let nodes: Vec<_> = node_results
                    .into_iter()
                    .map(|result| result.serialization)
                    .collect();
                SatResult {
                    serialization: super::serialize::threshold(k, &nodes, &witness_bits),
                    is_satisfied: all_selected_ok,
                }
            }
            // FIXME: Allow assembly fragments to be missing if they are not needed
            // Because the CMR of the satisfied program must match the CMR of the program commitment,
            // the satisfier needs to produce a Simplicity expression with the given `cmr`.
            // This is the case even if the assembly program is not executed in the final program!
            // Because we cannot construct hidden nodes, the satisfier needs to know the assembly
            // program, even if this program is not needed in the final execution.
            //
            // This feels like a huge footgun, especially when the unnecessary assembly program
            // is lost but the UTXO remains spendable via other spending paths.
            //
            // By reintroducing hidden nodes, the satisfier could insert a placeholder for an
            // un-executed assembly program.
            Policy::Assembly(cmr) => match satisfier.lookup_asm_program(cmr) {
                Some(serialization) => SatResult {
                    serialization,
                    is_satisfied: true,
                },
                _ => return Err(SatisfierError::MissingAssembly(cmr)),
            },
        };
        Ok(node)
    }

    /// Return the policy program with satisfying witness data.
    ///
    /// The program is run on the Bit Machine for pruning,
    /// so the transaction environment needs to be provided.
    pub fn satisfy<S: Satisfier<Pk>>(
        &self,
        satisfier: &S,
        env: &ElementsEnv<Arc<elements::Transaction>>,
    ) -> Result<Arc<RedeemNode<Elements>>, SatisfierError> {
        let SatResult {
            serialization: witness_program,
            is_satisfied,
        } = self.satisfy_internal(&types::Context::new(), satisfier)?;
        if !is_satisfied {
            return Err(SatisfierError::Unsatisfiable);
        };
        let unpruned_program = witness_program
            .finalize_unpruned()
            .expect("serialization should be sound");
        unpruned_program
            .prune(env)
            .map_err(SatisfierError::AssemblyFailed)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::bit_encoding::BitCollector;
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

    impl<Pk: ToXOnlyPubkey> Satisfier<Pk> for PolicySatisfier<'_, Pk> {
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
            let msg = secp256k1_zkp::Message::from_digest(sighash.to_byte_array());
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

        assert!(policy.satisfy(&satisfier, &env).is_err());

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

        let program = policy.satisfy(&satisfier, &env).expect("satisfiable");
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

        let program = policy.satisfy(&satisfier, &env).expect("satisfiable");
        let witness = to_witness(&program);
        assert_eq!(1, witness.len());

        let sighash = env.c_tx_env().sighash_all();
        let message = secp256k1_zkp::Message::from_digest(sighash.to_byte_array());
        let signature_bytes = witness[0]
            .iter_padded()
            .try_collect_bytes()
            .expect("to bytes");
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

        let program = policy.satisfy(&satisfier, &env).expect("satisfiable");
        let witness = to_witness(&program);
        assert_eq!(1, witness.len());

        let witness_bytes = witness[0]
            .iter_padded()
            .try_collect_bytes()
            .expect("to bytes");
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
        let program = policy0.satisfy(&satisfier, &env).expect("satisfiable");
        let witness = to_witness(&program);
        assert!(witness.is_empty());
        execute_successful(program, &env);

        let policy1 = Policy::After(42);
        let program = policy1.satisfy(&satisfier, &env).expect("satisfiable");
        let witness = to_witness(&program);
        assert!(witness.is_empty());
        execute_successful(program, &env);

        let policy2 = Policy::After(43);
        assert!(policy2.satisfy(&satisfier, &env).is_err(), "unsatisfiable");
    }

    #[test]
    fn satisfy_older() {
        let env = ElementsEnv::dummy_with(
            elements::LockTime::ZERO,
            elements::Sequence::from_consensus(42),
        );
        let satisfier = get_satisfier(&env);

        let policy0 = Policy::Older(41);
        let program = policy0.satisfy(&satisfier, &env).expect("satisfiable");
        let witness = to_witness(&program);
        assert!(witness.is_empty());
        execute_successful(program, &env);

        let policy1 = Policy::Older(42);
        let program = policy1.satisfy(&satisfier, &env).expect("satisfiable");
        let witness = to_witness(&program);
        assert!(witness.is_empty());
        execute_successful(program, &env);

        let policy2 = Policy::Older(43);
        assert!(policy2.satisfy(&satisfier, &env).is_err(), "unsatisfiable");
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
        let program = policy0.satisfy(&satisfier, &env).expect("satisfiable");
        let witness = to_witness(&program);
        assert_eq!(2, witness.len());

        for i in 0..2 {
            let preimage_bytes = witness[i]
                .iter_padded()
                .try_collect_bytes()
                .expect("to bytes");
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
        assert!(policy1.satisfy(&satisfier, &env).is_err());

        // Policy 2

        let policy2 = Policy::And {
            left: Arc::new(Policy::Sha256(images[0])),
            right: Arc::new(Policy::Sha256(sha256::Hash::from_byte_array([0; 32]))),
        };
        assert!(policy2.satisfy(&satisfier, &env).is_err());
    }

    #[test]
    fn satisfy_or() {
        let env = ElementsEnv::dummy();
        let satisfier = get_satisfier(&env);
        let images: Vec<_> = satisfier.preimages.keys().copied().collect();
        let preimages: Vec<_> = images.iter().map(|x| satisfier.preimages[x]).collect();

        let assert_branch = |policy: &Policy<XOnlyPublicKey>, bit: bool| {
            let program = policy.satisfy(&satisfier, &env).expect("satisfiable");
            let witness = to_witness(&program);
            assert_eq!(2, witness.len());

            assert_eq!(Value::u1(bit as u8), *witness[0]);
            let preimage_bytes = witness[1]
                .iter_padded()
                .try_collect_bytes()
                .expect("to bytes");
            let witness_preimage =
                Preimage32::try_from(preimage_bytes.as_slice()).expect("to array");
            assert_eq!(preimages[bit as usize], witness_preimage);

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
        assert!(policy3.satisfy(&satisfier, &env).is_err());
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
            let program = policy.satisfy(&satisfier, &env).expect("satisfiable");
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
                    let preimage_bytes = witness[witidx]
                        .iter_padded()
                        .try_collect_bytes()
                        .expect("to bytes");
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
                        _ => assert!(policy.satisfy(&satisfier, &env).is_err()),
                    }
                }
            }
        }
    }

    #[test]
    fn satisfy_asm() {
        let ctx = types::Context::new();
        let env = ElementsEnv::dummy();
        let mut satisfier = get_satisfier(&env);

        let mut assert_branch = |witness0: Value, witness1: Value| {
            let asm_program = serialize::verify_bexp(
                &Arc::<WitnessNode<Elements>>::pair(
                    &Arc::<WitnessNode<Elements>>::witness(&ctx, Some(witness0.clone())),
                    &Arc::<WitnessNode<Elements>>::witness(&ctx, Some(witness1.clone())),
                )
                .expect("sound types"),
                &Arc::<WitnessNode<Elements>>::jet(&ctx, Elements::Eq8),
            );
            let cmr = asm_program.cmr();
            satisfier.assembly.insert(cmr, asm_program);

            let policy = Policy::Assembly(cmr);
            let result = policy.satisfy(&satisfier, &env);

            if witness0 == witness1 {
                let program = result.expect("policy should be satisfiable");
                let witness = to_witness(&program);

                assert_eq!(2, witness.len());
                assert_eq!(&witness0, witness[0]);
                assert_eq!(&witness1, witness[1]);

                execute_successful(program, &env);
            } else {
                assert!(matches!(result, Err(SatisfierError::AssemblyFailed(..))));
            }
        };

        for a in 0..2 {
            for b in 0..2 {
                assert_branch(Value::u8(a), Value::u8(b))
            }
        }
    }
}
