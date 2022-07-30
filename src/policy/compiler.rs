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
use crate::core::TermDag;
use crate::core::Value;
use crate::jet;
use crate::jet::application::Bitcoin;
use crate::merkle::cmr::Cmr;
use crate::miniscript::MiniscriptKey;
use crate::policy::key::PublicKey32;
use crate::Error;
use std::rc::Rc;

/// Compile the given policy into a Simplicity DAG.
pub fn compile<Pk: MiniscriptKey + PublicKey32>(
    policy: &Policy<Pk>,
) -> Result<Rc<TermDag<(), Bitcoin>>, Error> {
    let dag = match policy {
        // TODO: Choose specific Merkle roots for unsatisfiable policies
        Policy::Unsatisfiable => TermDag::fail(Cmr::from([0; 32]), Cmr::from([0; 32])),
        Policy::Trivial => TermDag::unit(),
        Policy::Key(key) => {
            let key_value = Value::u256_from_slice(&key.to_32_bytes());
            let pair_key_msg = TermDag::pair(
                TermDag::scribe(&key_value),
                TermDag::jet(&jet::bitcoin::SIGHASH_ALL),
            );
            let pair_key_msg_sig = TermDag::pair(pair_key_msg, TermDag::witness(()));
            TermDag::comp(
                pair_key_msg_sig,
                TermDag::jet(&jet::bitcoin::BIP_0340_VERIFY),
            )
        }
        Policy::After(n) => {
            let n_value = Value::u32(*n);
            let pair_n_locktime = TermDag::pair(
                TermDag::scribe(&n_value),
                TermDag::jet(&jet::bitcoin::LOCK_TIME),
            );
            TermDag::comp(pair_n_locktime, TermDag::jet(&jet::bitcoin::LT32_VERIFY))
        }
        Policy::Older(n) => {
            let n_value = Value::u32(*n);
            let pair_n_sequence = TermDag::pair(
                TermDag::scribe(&n_value),
                TermDag::jet(&jet::bitcoin::CURRENT_SEQUENCE),
            );
            TermDag::comp(pair_n_sequence, TermDag::jet(&jet::bitcoin::LT32_VERIFY))
        }
        Policy::Sha256(hash) => {
            let hash_value = Value::u256_from_slice(hash);
            let computed_hash =
                TermDag::comp(TermDag::witness(()), TermDag::jet(&jet::bitcoin::SHA256));
            let pair_hash_computed_hash =
                TermDag::pair(TermDag::scribe(&hash_value), computed_hash);
            TermDag::comp(
                pair_hash_computed_hash,
                TermDag::jet(&jet::bitcoin::EQ256_VERIFY),
            )
        }
        Policy::And(sub_policies) => {
            assert_eq!(2, sub_policies.len());

            let left = compile(&sub_policies[0])?;
            let right = compile(&sub_policies[1])?;
            TermDag::comp(left, right)
        }
        Policy::Or(sub_policies) => {
            assert_eq!(2, sub_policies.len());

            let left = compile(&sub_policies[0])?;
            let right = compile(&sub_policies[1])?;
            // Cannot use TermDag::cond lest the witness is in reverse order
            let cond_left_right = TermDag::case(TermDag::drop(left), TermDag::drop(right));
            let selector = TermDag::pair(TermDag::witness(()), TermDag::unit());
            TermDag::comp(selector, cond_left_right)
        }
        Policy::Threshold(k, sub_policies) => {
            assert!(
                sub_policies.len() >= 2,
                "Thresholds must have at least two sub-policies"
            );

            // 1 -> 1
            let child = compile(&sub_policies[0])?;
            // 1 -> 2 x 1
            let selector = TermDag::pair(TermDag::witness(()), TermDag::unit());
            // 1 -> 2^32
            let child_one = TermDag::comp(child, TermDag::scribe(&Value::u32(1)));
            // 2 x 1 -> 2^32
            let child_one_or_zero = TermDag::cond(child_one, TermDag::scribe(&Value::u32(0)));
            // 1 -> 2^32
            let mut sum = TermDag::comp(selector, child_one_or_zero);

            for sub in &sub_policies[1..] {
                // 1 -> 1
                let child = compile(sub)?;
                // 1 -> 2 x 1
                let selector = TermDag::pair(TermDag::witness(()), TermDag::unit());
                // 1 -> 2^32
                let child_one = TermDag::comp(child, TermDag::scribe(&Value::u32(1)));
                // 2 x 1 -> 2^32
                let child_one_or_zero = TermDag::cond(child_one, TermDag::scribe(&Value::u32(0)));
                // 1 -> 2^32
                let local_summand = TermDag::comp(selector, child_one_or_zero);
                // 1 -> 2 x 2^32
                let full_sum = TermDag::comp(
                    TermDag::pair(sum, local_summand),
                    TermDag::jet(&jet::bitcoin::ADD32),
                );
                // Discard the overflow bit.
                // FIXME: enforce that sum of weights is less than 2^32
                // 1 -> 2^32
                sum = TermDag::comp(full_sum, TermDag::drop(TermDag::iden()));
            }

            // 1 -> 2^32
            let scribe_k = TermDag::scribe(&Value::u32(*k as u32));
            // 1 -> 1
            TermDag::comp(
                TermDag::pair(scribe_k, sum),
                TermDag::jet(&jet::bitcoin::EQ32_VERIFY),
            )
        }
    };

    Ok(dag)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::TypedProgram;
    use crate::exec::BitMachine;
    use crate::jet::application::BitcoinEnv;
    use bitcoin_hashes::{sha256, Hash};
    use miniscript::DummyKey;

    fn compile(policy: Policy<DummyKey>) -> (TypedProgram<(), Bitcoin>, BitcoinEnv) {
        let typed = policy
            .compile()
            .expect("compile")
            .type_check()
            .expect("type check");
        let env = BitcoinEnv::default();

        (typed, env)
    }

    fn execute_successful(
        typed: TypedProgram<(), Bitcoin>,
        witness: Vec<Value>,
        env: &BitcoinEnv,
    ) -> bool {
        let program = typed
            .add_witness(witness)
            .expect("witness")
            .finalize_insane()
            .maximize_sharing();
        let mut mac = BitMachine::for_program(&program);

        let success = match mac.exec(&program, &env) {
            Ok(output) => output == Value::Unit,
            Err(_) => false,
        };
        success
    }

    #[test]
    fn execute_unsatisfiable() {
        let (typed, env) = compile(Policy::Unsatisfiable);
        assert!(!execute_successful(typed, vec![], &env));
    }

    #[test]
    fn execute_trivial() {
        let (typed, env) = compile(Policy::Trivial);
        assert!(execute_successful(typed, vec![], &env));
    }

    // TODO: check execution once implemented
    #[test]
    fn compile_pk() {
        let _ = compile(Policy::Key(DummyKey));
    }

    #[test]
    fn execute_after() {
        let (typed, mut env) = compile(Policy::After(42));

        env.tx.lock_time = 42;
        assert!(!execute_successful(typed.clone(), vec![], &env));

        env.tx.lock_time = 43;
        assert!(execute_successful(typed, vec![], &env));
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
        let (typed, env) = compile(Policy::Sha256(image));

        let valid_witness = vec![Value::u256_from_slice(&preimage)];
        assert!(execute_successful(typed.clone(), valid_witness, &env));

        let invalid_witness = vec![Value::u256_from_slice(&[0; 32])];
        assert!(!execute_successful(typed, invalid_witness, &env));
    }

    #[test]
    fn execute_and() {
        let preimage0 = [1; 32];
        let image0 = sha256::Hash::hash(&preimage0);
        let preimage1 = [2; 32];
        let image1 = sha256::Hash::hash(&preimage1);

        let (typed, env) = compile(Policy::And(vec![
            Policy::Sha256(image0),
            Policy::Sha256(image1),
        ]));

        let valid_witness = vec![
            Value::u256_from_slice(&preimage0),
            Value::u256_from_slice(&preimage1),
        ];
        assert!(execute_successful(typed.clone(), valid_witness, &env));

        let invalid_witness = vec![
            Value::u256_from_slice(&preimage0),
            Value::u256_from_slice(&[0; 32]),
        ];
        assert!(!execute_successful(typed.clone(), invalid_witness, &env));

        let invalid_witness = vec![
            Value::u256_from_slice(&[0; 32]),
            Value::u256_from_slice(&preimage1),
        ];
        assert!(!execute_successful(typed, invalid_witness, &env));
    }

    #[test]
    fn execute_or() {
        let preimage0 = [1; 32];
        let image0 = sha256::Hash::hash(&preimage0);
        let preimage1 = [2; 32];
        let image1 = sha256::Hash::hash(&preimage1);

        let (typed, env) = compile(Policy::Or(vec![
            Policy::Sha256(image0),
            Policy::Sha256(image1),
        ]));

        let valid_witness = vec![
            Value::u1(0),
            Value::u256_from_slice(&preimage0),
            Value::u256_from_slice(&[0; 32]),
        ];
        assert!(execute_successful(typed.clone(), valid_witness, &env));
        let valid_witness = vec![
            Value::u1(1),
            Value::u256_from_slice(&[0; 32]),
            Value::u256_from_slice(&preimage1),
        ];
        assert!(execute_successful(typed.clone(), valid_witness, &env));

        let invalid_witness = vec![
            Value::u1(0),
            Value::u256_from_slice(&[0; 32]),
            Value::u256_from_slice(&preimage1),
        ];
        assert!(!execute_successful(typed.clone(), invalid_witness, &env));
        let invalid_witness = vec![
            Value::u1(1),
            Value::u256_from_slice(&preimage0),
            Value::u256_from_slice(&[0; 32]),
        ];
        assert!(!execute_successful(typed, invalid_witness, &env));
    }

    #[test]
    fn execute_threshold() {
        let preimage0 = [1; 32];
        let image0 = sha256::Hash::hash(&preimage0);
        let preimage1 = [2; 32];
        let image1 = sha256::Hash::hash(&preimage1);
        let preimage2 = [3; 32];
        let image2 = sha256::Hash::hash(&preimage2);

        let (typed, env) = compile(Policy::Threshold(
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
        assert!(execute_successful(typed.clone(), valid_witness, &env));

        let valid_witness = vec![
            Value::u1(1),
            Value::u256_from_slice(&preimage0),
            Value::u1(0),
            Value::u256_from_slice(&[0; 32]),
            Value::u1(1),
            Value::u256_from_slice(&preimage2),
        ];
        assert!(execute_successful(typed.clone(), valid_witness, &env));

        let valid_witness = vec![
            Value::u1(0),
            Value::u256_from_slice(&[0; 32]),
            Value::u1(1),
            Value::u256_from_slice(&preimage1),
            Value::u1(1),
            Value::u256_from_slice(&preimage2),
        ];
        assert!(execute_successful(typed.clone(), valid_witness, &env));

        let invalid_witness = vec![
            Value::u1(1),
            Value::u256_from_slice(&preimage0),
            Value::u1(1),
            Value::u256_from_slice(&preimage1),
            Value::u1(1),
            Value::u256_from_slice(&preimage2),
        ];
        assert!(!execute_successful(typed.clone(), invalid_witness, &env));

        let invalid_witness = vec![
            Value::u1(1),
            Value::u256_from_slice(&preimage1),
            Value::u1(1),
            Value::u256_from_slice(&preimage0),
            Value::u1(0),
            Value::u256_from_slice(&[0; 32]),
        ];
        assert!(!execute_successful(typed.clone(), invalid_witness, &env));

        let invalid_witness = vec![
            Value::u1(1),
            Value::u256_from_slice(&preimage0),
            Value::u1(0),
            Value::u256_from_slice(&[0; 32]),
            Value::u1(0),
            Value::u256_from_slice(&[0; 32]),
        ];
        assert!(!execute_successful(typed, invalid_witness, &env));
    }
}
