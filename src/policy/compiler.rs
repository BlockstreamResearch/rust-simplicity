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
