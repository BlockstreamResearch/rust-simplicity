// Simplicity Policy Compiler
// Written in 2020 by
//     Sanket Kanjalkar <sanket1729@gmail.com>
// To the extent possible under law, the author(s) have dedicated all
// copyright and related and neighboring rights to this software to
// the public domain worldwide. This software is distributed without
// any warranty.
//
// You should have received a copy of the CC0 Public Domain Dedication
// along with this software.
// If not, see <http://creativecommons.org/publicdomain/zero/1.0/>.
//

//! # Policy Compiler
//! Compile a policy to Simplicity Program
//! Currently the policy compilation is one to one mapping
//! between policy fragment and a simplicity program.

use super::ast::Policy;
use crate::core::commit::CommitNode;
use crate::core::{Context, Value};
use crate::jet;
use crate::jet::application::Bitcoin;
use crate::merkle::cmr::Cmr;
use crate::miniscript::MiniscriptKey;
use crate::policy::key::PublicKey32;
use crate::Error;
use std::rc::Rc;

/// Compile the given policy into a Simplicity program.
pub fn compile<Pk: MiniscriptKey + PublicKey32>(
    context: &mut Context<Bitcoin>,
    policy: &Policy<Pk>,
) -> Result<Rc<CommitNode<Bitcoin>>, Error> {
    match policy {
        // TODO: Choose specific Merkle roots for unsatisfiable policies
        Policy::Unsatisfiable => CommitNode::fail(context, Cmr::from([0; 32]), Cmr::from([0; 32])),
        Policy::Trivial => CommitNode::unit(context),
        Policy::Key(key) => {
            let key_value = Value::u256_from_slice(&key.to_32_bytes());
            let scribe_key = CommitNode::scribe(context, &key_value)?;
            let sighash_all = CommitNode::jet(context, &jet::bitcoin::SIGHASH_ALL)?;
            let pair_key_msg = CommitNode::pair(context, scribe_key, sighash_all)?;
            let witness = CommitNode::witness(context)?;
            let pair_key_msg_sig = CommitNode::pair(context, pair_key_msg, witness)?;
            let bip_0340_verify = CommitNode::jet(context, &jet::bitcoin::BIP_0340_VERIFY)?;

            CommitNode::comp(context, pair_key_msg_sig, bip_0340_verify)
        }
        Policy::After(n) => {
            let n_value = Value::u32(*n);
            let scribe_n = CommitNode::scribe(context, &n_value)?;
            let lock_time = CommitNode::jet(context, &jet::bitcoin::LOCK_TIME)?;
            let pair_n_locktime = CommitNode::pair(context, scribe_n, lock_time)?;
            let lt32_verify = CommitNode::jet(context, &jet::bitcoin::LT32_VERIFY)?;

            CommitNode::comp(context, pair_n_locktime, lt32_verify)
        }
        Policy::Older(n) => {
            let n_value = Value::u32(*n);
            let scribe_n = CommitNode::scribe(context, &n_value)?;
            let current_sequence = CommitNode::jet(context, &jet::bitcoin::CURRENT_SEQUENCE)?;
            let pair_n_sequence = CommitNode::pair(context, scribe_n, current_sequence)?;
            let lt32_verify = CommitNode::jet(context, &jet::bitcoin::LT32_VERIFY)?;

            CommitNode::comp(context, pair_n_sequence, lt32_verify)
        }
        Policy::Sha256(hash) => {
            let hash_value = Value::u256_from_slice(hash);
            let scribe_hash = CommitNode::scribe(context, &hash_value)?;
            let witness = CommitNode::witness(context)?;
            let sha256 = CommitNode::jet(context, &jet::bitcoin::SHA256)?;
            let computed_hash = CommitNode::comp(context, witness, sha256)?;
            let pair_hash_computed_hash = CommitNode::pair(context, scribe_hash, computed_hash)?;
            let eq256_verify = CommitNode::jet(context, &jet::bitcoin::EQ256_VERIFY)?;

            CommitNode::comp(context, pair_hash_computed_hash, eq256_verify)
        }
        Policy::And(sub_policies) => {
            assert_eq!(
                2,
                sub_policies.len(),
                "Conjunctions must have exactly two sub-policies"
            );

            if let Policy::Trivial = sub_policies[0] {
                return compile(context, &sub_policies[1]);
            }
            if let Policy::Trivial = sub_policies[1] {
                return compile(context, &sub_policies[0]);
            }

            let left = compile(context, &sub_policies[0])?;
            let right = compile(context, &sub_policies[1])?;
            CommitNode::comp(context, left, right)
        }
        Policy::Or(sub_policies) => {
            assert_eq!(
                2,
                sub_policies.len(),
                "Disjunctions must have exactly two sub-policies"
            );

            let left = compile(context, &sub_policies[0])?;
            let right = compile(context, &sub_policies[1])?;

            let cond_right_left = CommitNode::cond(context, right, left)?;
            let witness = CommitNode::witness(context)?;
            let unit = CommitNode::unit(context)?;
            let selector = CommitNode::pair(context, witness, unit)?;

            CommitNode::comp(context, selector, cond_right_left)
        }
        Policy::Threshold(k, sub_policies) => {
            assert!(
                sub_policies.len() >= 2,
                "Thresholds must have at least two sub-policies"
            );

            // 1 → 2^32
            let scribe_zero = CommitNode::scribe(context, &Value::u32(0))?;
            // 1 → 2^32
            let scribe_one = CommitNode::scribe(context, &Value::u32(1))?;

            // 1 → 2^32
            let get_summand = |policy: &Policy<Pk>, context: &mut Context<Bitcoin>| {
                // 1 → 1
                let child = compile(context, policy)?;
                // 1 → 2
                let witness = CommitNode::witness(context)?;
                // 1 → 1
                let unit = CommitNode::unit(context)?;
                // 1 → 2 x 1
                let selector = CommitNode::pair(context, witness, unit)?;
                // 1 → 2^32
                let child_one = CommitNode::comp(context, child, scribe_one.clone())?;
                // 2 x 1 → 2^32
                let child_one_or_zero = CommitNode::cond(context, child_one, scribe_zero.clone())?;
                // 1 → 2^32
                CommitNode::comp(context, selector, child_one_or_zero)
            };

            let mut sum = get_summand(&sub_policies[0], context)?;

            for policy in &sub_policies[1..] {
                // 1 → 2^32
                let summand = get_summand(policy, context)?;
                // 1 → 2^32 × 2^32
                let pair_sum_summand = CommitNode::pair(context, sum, summand)?;
                // 2^32 × 2^32 → 2 × 2^32
                let add32 = CommitNode::jet(context, &jet::bitcoin::ADD32)?;
                // 1 → 2 x 2^32
                let full_sum = CommitNode::comp(context, pair_sum_summand, add32)?;
                // 2^32 → 2^32
                let iden = CommitNode::iden(context)?;
                // 2 × 2^32 → 2^32
                let drop_iden = CommitNode::drop(context, iden)?;

                // Discard the overflow bit.
                // FIXME: enforce that sum of weights is less than 2^32
                // 1 → 2^32
                sum = CommitNode::comp(context, full_sum, drop_iden)?;
            }

            // 1 → 2^32
            let scribe_k = CommitNode::scribe(context, &Value::u32(*k as u32))?;
            // 1 → 2^32 × 2^32
            let pair_k_sum = CommitNode::pair(context, scribe_k, sum)?;
            // 2^32 × 2^32 → 1
            let eq32_verify = CommitNode::jet(context, &jet::bitcoin::EQ32_VERIFY)?;

            // 1 → 1
            CommitNode::comp(context, pair_k_sum, eq32_verify)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::exec::BitMachine;
    use crate::jet::application::BitcoinEnv;
    use bitcoin_hashes::{sha256, Hash};
    use miniscript::DummyKey;

    fn compile(policy: Policy<DummyKey>) -> (Rc<CommitNode<Bitcoin>>, BitcoinEnv) {
        let mut context = Context::new();
        let commit = super::compile(&mut context, &policy).expect("compile");
        let env = BitcoinEnv::default();

        (commit, env)
    }

    fn execute_successful(
        commit: &CommitNode<Bitcoin>,
        witness: Vec<Value>,
        env: &BitcoinEnv,
    ) -> bool {
        let finalized = commit.finalize(witness.into_iter()).expect("finalize");
        let mut mac = BitMachine::for_program(&finalized);

        let success = match mac.exec(&finalized, env) {
            Ok(output) => output == Value::Unit,
            Err(_) => false,
        };
        success
    }

    #[ignore]
    #[test]
    fn execute_unsatisfiable() {
        let (commit, env) = compile(Policy::Unsatisfiable);
        assert!(!execute_successful(&commit, vec![], &env));
    }

    #[ignore]
    #[test]
    fn execute_trivial() {
        let (commit, env) = compile(Policy::Trivial);
        assert!(execute_successful(&commit, vec![], &env));
    }

    // TODO: check execution once implemented
    #[ignore]
    #[test]
    fn compile_pk() {
        let _ = compile(Policy::Key(DummyKey));
    }

    #[ignore]
    #[test]
    fn execute_after() {
        let (commit, mut env) = compile(Policy::After(42));

        env.tx.lock_time = 42;
        assert!(!execute_successful(&commit, vec![], &env));

        env.tx.lock_time = 43;
        assert!(execute_successful(&commit, vec![], &env));
    }

    // TODO: check execution once implemented
    #[ignore]
    #[test]
    fn compile_older() {
        let _ = compile(Policy::Older(42));
    }

    #[ignore]
    #[test]
    fn execute_sha256() {
        let preimage = [1; 32];
        let image = sha256::Hash::hash(&preimage);
        let (commit, env) = compile(Policy::Sha256(image));

        let valid_witness = vec![Value::u256_from_slice(&preimage)];
        assert!(execute_successful(&commit, valid_witness, &env));

        let invalid_witness = vec![Value::u256_from_slice(&[0; 32])];
        assert!(!execute_successful(&commit, invalid_witness, &env));
    }

    #[ignore]
    #[test]
    fn execute_and() {
        let preimage0 = [1; 32];
        let image0 = sha256::Hash::hash(&preimage0);
        let preimage1 = [2; 32];
        let image1 = sha256::Hash::hash(&preimage1);

        let (commit, env) = compile(Policy::And(vec![
            Policy::Sha256(image0),
            Policy::Sha256(image1),
        ]));

        let valid_witness = vec![
            Value::u256_from_slice(&preimage0),
            Value::u256_from_slice(&preimage1),
        ];
        assert!(execute_successful(&commit, valid_witness, &env));

        let invalid_witness = vec![
            Value::u256_from_slice(&preimage0),
            Value::u256_from_slice(&[0; 32]),
        ];
        assert!(!execute_successful(&commit, invalid_witness, &env));

        let invalid_witness = vec![
            Value::u256_from_slice(&[0; 32]),
            Value::u256_from_slice(&preimage1),
        ];
        assert!(!execute_successful(&commit, invalid_witness, &env));
    }

    #[ignore]
    #[test]
    fn execute_and_true() {
        let preimage0 = [1; 32];
        let image0 = sha256::Hash::hash(&preimage0);

        let (commit, env) = compile(Policy::And(vec![Policy::Sha256(image0), Policy::Trivial]));

        let valid_witness = vec![Value::u256_from_slice(&preimage0)];
        assert!(execute_successful(&commit, valid_witness, &env));

        let invalid_witness = vec![Value::u256_from_slice(&[0; 32])];
        assert!(!execute_successful(&commit, invalid_witness, &env));
    }

    #[ignore]
    #[test]
    fn execute_or() {
        let preimage0 = [1; 32];
        let image0 = sha256::Hash::hash(&preimage0);
        let preimage1 = [2; 32];
        let image1 = sha256::Hash::hash(&preimage1);

        let (commit, env) = compile(Policy::Or(vec![
            Policy::Sha256(image0),
            Policy::Sha256(image1),
        ]));

        let valid_witness = vec![
            Value::u1(0),
            Value::u256_from_slice(&preimage0),
            Value::u256_from_slice(&[0; 32]),
        ];
        assert!(execute_successful(&commit, valid_witness, &env));
        let valid_witness = vec![
            Value::u1(1),
            Value::u256_from_slice(&[0; 32]),
            Value::u256_from_slice(&preimage1),
        ];
        assert!(execute_successful(&commit, valid_witness, &env));

        let invalid_witness = vec![
            Value::u1(0),
            Value::u256_from_slice(&[0; 32]),
            Value::u256_from_slice(&preimage1),
        ];
        assert!(!execute_successful(&commit, invalid_witness, &env));
        let invalid_witness = vec![
            Value::u1(1),
            Value::u256_from_slice(&preimage0),
            Value::u256_from_slice(&[0; 32]),
        ];
        assert!(!execute_successful(&commit, invalid_witness, &env));
    }

    #[ignore]
    #[test]
    fn execute_threshold() {
        let preimage0 = [1; 32];
        let image0 = sha256::Hash::hash(&preimage0);
        let preimage1 = [2; 32];
        let image1 = sha256::Hash::hash(&preimage1);
        let preimage2 = [3; 32];
        let image2 = sha256::Hash::hash(&preimage2);

        let (commit, env) = compile(Policy::Threshold(
            2,
            vec![
                Policy::Sha256(image0),
                Policy::Sha256(image1),
                Policy::Sha256(image2),
            ],
        ));

        let valid_witness = vec![
            Value::u1(1),
            Value::u256_from_slice(&preimage0),
            Value::u1(1),
            Value::u256_from_slice(&preimage1),
            Value::u1(0),
            Value::u256_from_slice(&[0; 32]),
        ];
        assert!(execute_successful(&commit, valid_witness, &env));

        let valid_witness = vec![
            Value::u1(1),
            Value::u256_from_slice(&preimage0),
            Value::u1(0),
            Value::u256_from_slice(&[0; 32]),
            Value::u1(1),
            Value::u256_from_slice(&preimage2),
        ];
        assert!(execute_successful(&commit, valid_witness, &env));

        let valid_witness = vec![
            Value::u1(0),
            Value::u256_from_slice(&[0; 32]),
            Value::u1(1),
            Value::u256_from_slice(&preimage1),
            Value::u1(1),
            Value::u256_from_slice(&preimage2),
        ];
        assert!(execute_successful(&commit, valid_witness, &env));

        let invalid_witness = vec![
            Value::u1(1),
            Value::u256_from_slice(&preimage0),
            Value::u1(1),
            Value::u256_from_slice(&preimage1),
            Value::u1(1),
            Value::u256_from_slice(&preimage2),
        ];
        assert!(!execute_successful(&commit, invalid_witness, &env));

        let invalid_witness = vec![
            Value::u1(1),
            Value::u256_from_slice(&preimage1),
            Value::u1(1),
            Value::u256_from_slice(&preimage0),
            Value::u1(0),
            Value::u256_from_slice(&[0; 32]),
        ];
        assert!(!execute_successful(&commit, invalid_witness, &env));

        let invalid_witness = vec![
            Value::u1(1),
            Value::u256_from_slice(&preimage0),
            Value::u1(0),
            Value::u256_from_slice(&[0; 32]),
            Value::u1(0),
            Value::u256_from_slice(&[0; 32]),
        ];
        assert!(!execute_successful(&commit, invalid_witness, &env));
    }
}
