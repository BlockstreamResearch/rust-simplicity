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
use crate::core::commit::{CommitNode, CommitNodeInner, UsedCaseBranch};
use crate::jet::Elements;
use crate::miniscript::MiniscriptKey;
use crate::policy::key::PublicKey32;
use crate::types::Error;
use crate::{Cmr, Context, Value};
use std::rc::Rc;

impl<Pk: MiniscriptKey + PublicKey32> Policy<Pk> {
    /// Compile the policy into a Simplicity program
    pub fn compile(
        &self,
        context: &mut Context<Elements>,
    ) -> Result<Rc<CommitNode<Elements>>, Error> {
        compile(context, self)
    }
}

fn compute_sha256(
    context: &mut Context<Elements>,
    witness256: Rc<CommitNode<Elements>>,
) -> Result<Rc<CommitNode<Elements>>, Error> {
    let ctx = CommitNode::jet(context, Elements::Sha256Ctx8Init);
    let pair_ctx_witness = CommitNode::pair(context, ctx, witness256)?;
    let add256 = CommitNode::jet(context, Elements::Sha256Ctx8Add32);
    let digest_ctx = CommitNode::comp(context, pair_ctx_witness, add256)?;
    let finalize = CommitNode::jet(context, Elements::Sha256Ctx8Finalize);
    CommitNode::comp(context, digest_ctx, finalize)
}

fn verify_bexp(
    context: &mut Context<Elements>,
    input: Rc<CommitNode<Elements>>,
    bexp: Rc<CommitNode<Elements>>,
) -> Result<Rc<CommitNode<Elements>>, Error> {
    let computed_bexp = CommitNode::comp(context, input, bexp)?;
    let verify = CommitNode::jet(context, Elements::Verify);
    CommitNode::comp(context, computed_bexp, verify)
}

/// Compile the given policy into a Simplicity program.
fn compile<Pk: MiniscriptKey + PublicKey32>(
    context: &mut Context<Elements>,
    policy: &Policy<Pk>,
) -> Result<Rc<CommitNode<Elements>>, Error> {
    match policy {
        // TODO: Choose specific Merkle roots for unsatisfiable policies
        Policy::Unsatisfiable => Ok(CommitNode::fail(
            context,
            Cmr::from([0; 32]),
            Cmr::from([0; 32]),
        )),
        Policy::Trivial => Ok(CommitNode::unit(context)),
        Policy::Key(key) => {
            let key_value = Value::u256_from_slice(&key.to_32_bytes());
            let const_key = CommitNode::const_word(context, key_value);
            let sighash_all = CommitNode::jet(context, Elements::SigAllHash);
            let pair_key_msg = CommitNode::pair(context, const_key, sighash_all)?;
            let witness = CommitNode::witness(context);
            let pair_key_msg_sig = CommitNode::pair(context, pair_key_msg, witness)?;
            let bip_0340_verify = CommitNode::jet(context, Elements::Bip0340Verify);

            CommitNode::comp(context, pair_key_msg_sig, bip_0340_verify)
        }
        Policy::After(n) => {
            let n_value = Value::u32(*n);
            let const_n = CommitNode::const_word(context, n_value);
            let check_lock_height = CommitNode::jet(context, Elements::CheckLockHeight);

            CommitNode::comp(context, const_n, check_lock_height)
        }
        Policy::Older(n) => {
            let n_value = Value::u16(*n);
            let const_n = CommitNode::const_word(context, n_value);
            let check_lock_distance = CommitNode::jet(context, Elements::CheckLockDistance);

            CommitNode::comp(context, const_n, check_lock_distance)
        }
        Policy::Sha256(hash) => {
            let hash_value = Value::u256_from_slice(&Pk::hash_to_32_bytes(hash));
            let const_hash = CommitNode::const_word(context, hash_value);
            let witness256 = CommitNode::witness(context);
            let computed_hash = compute_sha256(context, witness256)?;
            let pair_hash_computed_hash = CommitNode::pair(context, const_hash, computed_hash)?;
            let eq256 = CommitNode::jet(context, Elements::Eq256);

            verify_bexp(context, pair_hash_computed_hash, eq256)
        }
        Policy::And(sub_policies) => {
            assert_eq!(
                2,
                sub_policies.len(),
                "Conjunctions must have exactly two sub-policies"
            );

            let left = compile(context, &sub_policies[0])?;
            let right = compile(context, &sub_policies[1])?;

            and(context, left, right)
        }
        Policy::Or(sub_policies) => {
            assert_eq!(
                2,
                sub_policies.len(),
                "Disjunctions must have exactly two sub-policies"
            );

            let left = compile(context, &sub_policies[0])?;
            let right = compile(context, &sub_policies[1])?;

            or(context, left, right, UsedCaseBranch::Both)
        }
        Policy::Threshold(k, sub_policies) => {
            assert!(
                sub_policies.len() >= 2,
                "Thresholds must have at least two sub-policies"
            );

            let child = sub_policies[0].compile(context)?;
            let mut sum = thresh_summand(context, child, UsedCaseBranch::Both)?;

            for policy in &sub_policies[1..] {
                let child = policy.compile(context)?;
                let summand = thresh_summand(context, child, UsedCaseBranch::Both)?;
                sum = thresh_add(context, sum, summand)?;
            }

            thresh_verify(context, sum, *k as u32)
        }
    }
}

fn selector(context: &mut Context<Elements>) -> Result<Rc<CommitNode<Elements>>, Error> {
    let witness = CommitNode::witness(context);
    let unit = CommitNode::unit(context);
    CommitNode::pair(context, witness, unit)
}

pub(crate) fn and(
    context: &mut Context<Elements>,
    left: Rc<CommitNode<Elements>>,
    right: Rc<CommitNode<Elements>>,
) -> Result<Rc<CommitNode<Elements>>, Error> {
    if let CommitNodeInner::Unit = left.inner() {
        return Ok(right);
    }
    if let CommitNodeInner::Unit = right.inner() {
        return Ok(left);
    }

    CommitNode::comp(context, left, right)
}

pub(crate) fn or(
    context: &mut Context<Elements>,
    left: Rc<CommitNode<Elements>>,
    right: Rc<CommitNode<Elements>>,
    branch: UsedCaseBranch,
) -> Result<Rc<CommitNode<Elements>>, Error> {
    let drop_left = CommitNode::drop(context, left);
    let drop_right = CommitNode::drop(context, right);
    let case_left_right = CommitNode::case_branch(context, drop_left, drop_right, branch)?;
    let selector = selector(context)?;

    CommitNode::comp(context, selector, case_left_right)
}

/// child: 1 → 1
///
/// summand(child): 1 → 2^32
pub(crate) fn thresh_summand(
    context: &mut Context<Elements>,
    child: Rc<CommitNode<Elements>>,
    branch: UsedCaseBranch,
) -> Result<Rc<CommitNode<Elements>>, Error> {
    // 1 → 2 x 1
    let selector = selector(context)?;

    // 1 → 2^32
    let const_one = CommitNode::const_word(context, Value::u32(1));
    // 1 → 2^32
    let child_one = CommitNode::comp(context, child, const_one)?;
    // 1 → 2^32
    let const_zero = CommitNode::const_word(context, Value::u32(0));
    // 2 x 1 → 2^32
    let child_one_or_zero = CommitNode::cond_branch(context, child_one, const_zero, branch)?;

    // 1 → 2^32
    CommitNode::comp(context, selector, child_one_or_zero)
}

/// acc: 1 → 2^32, summand: 1 → 2^32
///
/// add(sum, summand): 1 → 2^32
pub(crate) fn thresh_add(
    context: &mut Context<Elements>,
    sum: Rc<CommitNode<Elements>>,
    summand: Rc<CommitNode<Elements>>,
) -> Result<Rc<CommitNode<Elements>>, Error> {
    // 1 → 2^32 × 2^32
    let pair_sum_summand = CommitNode::pair(context, sum, summand)?;
    // 2^32 × 2^32 → 2 × 2^32
    let add32 = CommitNode::jet(context, Elements::Add32);
    // 1 → 2 x 2^32
    let full_sum = CommitNode::comp(context, pair_sum_summand, add32)?;
    // 2^32 → 2^32
    let iden = CommitNode::iden(context);
    // 2 × 2^32 → 2^32
    let drop_iden = CommitNode::drop(context, iden);

    // Discard the overflow bit.
    // FIXME: enforce that sum of weights is less than 2^32
    // 1 → 2^32
    CommitNode::comp(context, full_sum, drop_iden)
}

/// verify(sum): 1 → 1
pub(crate) fn thresh_verify(
    context: &mut Context<Elements>,
    sum: Rc<CommitNode<Elements>>,
    k: u32,
) -> Result<Rc<CommitNode<Elements>>, Error> {
    // 1 → 2^32
    let const_k = CommitNode::const_word(context, Value::u32(k));
    // 1 → 2^32 × 2^32
    let pair_k_sum = CommitNode::pair(context, const_k, sum)?;
    // 2^32 × 2^32 → 2
    let eq32 = CommitNode::jet(context, Elements::Eq32);

    // 1 → 1
    verify_bexp(context, pair_k_sum, eq32)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::exec::BitMachine;
    use crate::jet::elements::ElementsEnv;
    use bitcoin_hashes::{sha256, Hash};
    use elements::{bitcoin, secp256k1_zkp};

    fn compile(policy: Policy<bitcoin::XOnlyPublicKey>) -> (Rc<CommitNode<Elements>>, ElementsEnv) {
        let mut context = Context::new();
        let commit = super::compile(&mut context, &policy).expect("compile");
        let env = ElementsEnv::dummy();

        (commit, env)
    }

    fn execute_successful(
        commit: &CommitNode<Elements>,
        witness: Vec<Value>,
        env: &ElementsEnv,
    ) -> bool {
        let finalized = commit
            .finalize(witness.into_iter(), true)
            .expect("finalize");
        let mut mac = BitMachine::for_program(&finalized);

        match mac.exec(&finalized, env) {
            Ok(output) => output == Value::Unit,
            Err(_) => false,
        }
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

    #[test]
    fn execute_pk() {
        let env = ElementsEnv::dummy();

        let sighash = env.c_tx_env().sighash_all();
        let secp = secp256k1_zkp::Secp256k1::new();
        let keypair = secp256k1_zkp::KeyPair::new(&secp, &mut secp256k1_zkp::rand::rngs::OsRng);
        let message = secp256k1_zkp::Message::from(sighash);
        let signature = keypair.sign_schnorr(message);

        let (xonly, _) = keypair.x_only_public_key();
        let mut context = Context::new();
        let commit = Policy::Key(xonly).compile(&mut context).expect("compile");

        assert!(execute_successful(
            &commit,
            vec![Value::u512_from_slice(signature.as_ref())],
            &env
        ));
    }

    #[test]
    fn execute_after() {
        let env = ElementsEnv::dummy_with(elements::PackedLockTime(42), elements::Sequence::ZERO);

        let mut context = Context::new();
        let commit = Policy::<bitcoin::XOnlyPublicKey>::After(41)
            .compile(&mut context)
            .expect("compile");
        assert!(execute_successful(&commit, vec![], &env));

        let commit = Policy::<bitcoin::XOnlyPublicKey>::After(42)
            .compile(&mut context)
            .expect("compile");
        assert!(execute_successful(&commit, vec![], &env));

        let commit = Policy::<bitcoin::XOnlyPublicKey>::After(43)
            .compile(&mut context)
            .expect("compile");
        assert!(!execute_successful(&commit, vec![], &env));
    }

    #[test]
    fn execute_older() {
        let env = ElementsEnv::dummy_with(
            elements::PackedLockTime::ZERO,
            elements::Sequence::from_consensus(42),
        );

        let mut context = Context::new();
        let commit = Policy::<bitcoin::XOnlyPublicKey>::Older(41)
            .compile(&mut context)
            .expect("compile");
        assert!(execute_successful(&commit, vec![], &env));

        let commit = Policy::<bitcoin::XOnlyPublicKey>::Older(42)
            .compile(&mut context)
            .expect("compile");
        assert!(execute_successful(&commit, vec![], &env));

        let commit = Policy::<bitcoin::XOnlyPublicKey>::Older(43)
            .compile(&mut context)
            .expect("compile");
        assert!(!execute_successful(&commit, vec![], &env));
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
