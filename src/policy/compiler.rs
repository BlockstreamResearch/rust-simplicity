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
use crate::core::Value;
use crate::jet;
use crate::jet::application::Bitcoin;
use crate::merkle::cmr::Cmr;
use crate::miniscript::MiniscriptKey;
use crate::policy::key::PublicKey32;
use crate::Error;
use std::rc::Rc;

/// Compile the given policy into a Simplicity program.
pub fn compile<Pk: MiniscriptKey + PublicKey32>(
    policy: &Policy<Pk>,
) -> Result<Rc<CommitNode<Bitcoin>>, Error> {
    let dag = match policy {
        // TODO: Choose specific Merkle roots for unsatisfiable policies
        Policy::Unsatisfiable => CommitNode::fail(Cmr::from([0; 32]), Cmr::from([0; 32])),
        Policy::Trivial => CommitNode::unit(),
        Policy::Key(key) => {
            let key_value = Value::u256_from_slice(&key.to_32_bytes());
            let pair_key_msg = CommitNode::pair(
                CommitNode::scribe(&key_value),
                CommitNode::jet(&jet::bitcoin::SIGHASH_ALL),
            );
            let pair_key_msg_sig = CommitNode::pair(pair_key_msg, CommitNode::witness());
            CommitNode::comp(
                pair_key_msg_sig,
                CommitNode::jet(&jet::bitcoin::BIP_0340_VERIFY),
            )
        }
        Policy::After(n) => {
            let n_value = Value::u32(*n);
            let pair_n_locktime = CommitNode::pair(
                CommitNode::scribe(&n_value),
                CommitNode::jet(&jet::bitcoin::LOCK_TIME),
            );
            CommitNode::comp(pair_n_locktime, CommitNode::jet(&jet::bitcoin::LT32_VERIFY))
        }
        Policy::Older(n) => {
            let n_value = Value::u32(*n);
            let pair_n_sequence = CommitNode::pair(
                CommitNode::scribe(&n_value),
                CommitNode::jet(&jet::bitcoin::CURRENT_SEQUENCE),
            );
            CommitNode::comp(pair_n_sequence, CommitNode::jet(&jet::bitcoin::LT32_VERIFY))
        }
        Policy::Sha256(hash) => {
            let hash_value = Value::u256_from_slice(hash);
            let computed_hash = CommitNode::comp(
                CommitNode::witness(),
                CommitNode::jet(&jet::bitcoin::SHA256),
            );
            let pair_hash_computed_hash =
                CommitNode::pair(CommitNode::scribe(&hash_value), computed_hash);
            CommitNode::comp(
                pair_hash_computed_hash,
                CommitNode::jet(&jet::bitcoin::EQ256_VERIFY),
            )
        }
        Policy::And(sub_policies) => {
            assert_eq!(
                2,
                sub_policies.len(),
                "Conjunctions must have exactly two sub-policies"
            );

            if let Policy::Trivial = sub_policies[0] {
                return compile(&sub_policies[1]);
            }
            if let Policy::Trivial = sub_policies[1] {
                return compile(&sub_policies[0]);
            }

            let left = compile(&sub_policies[0])?;
            let right = compile(&sub_policies[1])?;
            CommitNode::comp(left, right)
        }
        Policy::Or(sub_policies) => {
            assert_eq!(
                2,
                sub_policies.len(),
                "Disjunctions must have exactly two sub-policies"
            );

            let left = compile(&sub_policies[0])?;
            let right = compile(&sub_policies[1])?;

            let cond_right_left = CommitNode::cond(right, left);
            let selector = CommitNode::pair(CommitNode::witness(), CommitNode::unit());
            CommitNode::comp(selector, cond_right_left)
        }
        Policy::Threshold(k, sub_policies) => {
            assert!(
                sub_policies.len() >= 2,
                "Thresholds must have at least two sub-policies"
            );

            // 1 -> 1
            let child = compile(&sub_policies[0])?;
            // 1 -> 2 x 1
            let selector = CommitNode::pair(CommitNode::witness(), CommitNode::unit());
            // 1 -> 2^32
            let child_one = CommitNode::comp(child, CommitNode::scribe(&Value::u32(1)));
            // 2 x 1 -> 2^32
            let child_one_or_zero = CommitNode::cond(child_one, CommitNode::scribe(&Value::u32(0)));
            // 1 -> 2^32
            let mut sum = CommitNode::comp(selector, child_one_or_zero);

            for sub in &sub_policies[1..] {
                // 1 -> 1
                let child = compile(sub)?;
                // 1 -> 2 x 1
                let selector = CommitNode::pair(CommitNode::witness(), CommitNode::unit());
                // 1 -> 2^32
                let child_one = CommitNode::comp(child, CommitNode::scribe(&Value::u32(1)));
                // 2 x 1 -> 2^32
                let child_one_or_zero =
                    CommitNode::cond(child_one, CommitNode::scribe(&Value::u32(0)));
                // 1 -> 2^32
                let local_summand = CommitNode::comp(selector, child_one_or_zero);
                // 1 -> 2 x 2^32
                let full_sum = CommitNode::comp(
                    CommitNode::pair(sum, local_summand),
                    CommitNode::jet(&jet::bitcoin::ADD32),
                );
                // Discard the overflow bit.
                // FIXME: enforce that sum of weights is less than 2^32
                // 1 -> 2^32
                sum = CommitNode::comp(full_sum, CommitNode::drop(CommitNode::iden()));
            }

            // 1 -> 2^32
            let scribe_k = CommitNode::scribe(&Value::u32(*k as u32));
            // 1 -> 1
            CommitNode::comp(
                CommitNode::pair(scribe_k, sum),
                CommitNode::jet(&jet::bitcoin::EQ32_VERIFY),
            )
        }
    };

    Ok(dag)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::exec::BitMachine;
    use crate::jet::application::BitcoinEnv;
    use bitcoin_hashes::{sha256, Hash};
    use miniscript::DummyKey;

    fn compile(policy: Policy<DummyKey>) -> (Rc<CommitNode<Bitcoin>>, BitcoinEnv) {
        let commit = super::compile(&policy).expect("compile");
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

    #[test]
    fn execute_unsatisfiable() {
        let (commit, env) = compile(Policy::Unsatisfiable);
        assert!(!execute_successful(&commit, vec![], &env));
    }

    #[test]
    fn execute_trivial() {
        let (commit, env) = compile(Policy::Trivial);
        assert!(execute_successful(&commit, vec![], &env));
    }

    // TODO: check execution once implemented
    #[test]
    fn compile_pk() {
        let _ = compile(Policy::Key(DummyKey));
    }

    #[test]
    fn execute_after() {
        let (commit, mut env) = compile(Policy::After(42));

        env.tx.lock_time = 42;
        assert!(!execute_successful(&commit, vec![], &env));

        env.tx.lock_time = 43;
        assert!(execute_successful(&commit, vec![], &env));
    }

    // TODO: check execution once implemented
    #[test]
    fn compile_older() {
        let _ = compile(Policy::Older(42));
    }

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
