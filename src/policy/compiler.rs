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

use super::Policy;

use crate::core::commit::{CommitNode, CommitNodeInner, UsedCaseBranch};
use crate::jet::Elements;
use crate::miniscript::ToPublicKey;
use crate::types::Error;
use crate::Value;
use std::rc::Rc;

impl<Pk: ToPublicKey> Policy<Pk> {
    /// Compile the policy into a Simplicity program
    pub fn compile(&self) -> Rc<CommitNode<Elements>> {
        compile(self)
    }
}

fn verify_bexp(
    input: Rc<CommitNode<Elements>>,
    bexp: Rc<CommitNode<Elements>>,
) -> Result<Rc<CommitNode<Elements>>, Error> {
    let computed_bexp = CommitNode::comp(input, bexp)?;
    let verify = CommitNode::jet(Elements::Verify);
    CommitNode::comp(computed_bexp, verify)
}

/// Compile the given policy into a Simplicity program.
fn compile<Pk: ToPublicKey>(policy: &Policy<Pk>) -> Rc<CommitNode<Elements>> {
    CommitNode::from_node(&policy.serialize_no_witness())
}

fn selector() -> Result<Rc<CommitNode<Elements>>, Error> {
    let witness = CommitNode::witness();
    let unit = CommitNode::unit();
    CommitNode::pair(witness, unit)
}

pub(crate) fn and(
    left: Rc<CommitNode<Elements>>,
    right: Rc<CommitNode<Elements>>,
) -> Result<Rc<CommitNode<Elements>>, Error> {
    if let CommitNodeInner::Unit = left.inner() {
        return Ok(right);
    }
    if let CommitNodeInner::Unit = right.inner() {
        return Ok(left);
    }

    CommitNode::comp(left, right)
}

pub(crate) fn or(
    left: Rc<CommitNode<Elements>>,
    right: Rc<CommitNode<Elements>>,
    branch: UsedCaseBranch,
) -> Result<Rc<CommitNode<Elements>>, Error> {
    let drop_left = CommitNode::drop(left);
    let drop_right = CommitNode::drop(right);
    let case_left_right = CommitNode::case_branch(drop_left, drop_right, branch)?;
    let selector = selector()?;

    CommitNode::comp(selector, case_left_right)
}

/// child: 1 → 1
///
/// summand(child): 1 → 2^32
pub(crate) fn thresh_summand(
    child: Rc<CommitNode<Elements>>,
    branch: UsedCaseBranch,
) -> Result<Rc<CommitNode<Elements>>, Error> {
    // 1 → 2 x 1
    let selector = selector()?;

    // 1 → 2^32
    let const_one = CommitNode::const_word(Value::u32(1));
    // 1 → 2^32
    let child_one = CommitNode::comp(child, const_one)?;
    // 1 → 2^32
    let const_zero = CommitNode::const_word(Value::u32(0));
    // 2 x 1 → 2^32
    let child_one_or_zero = CommitNode::cond_branch(child_one, const_zero, branch)?;

    // 1 → 2^32
    CommitNode::comp(selector, child_one_or_zero)
}

/// acc: 1 → 2^32, summand: 1 → 2^32
///
/// add(sum, summand): 1 → 2^32
pub(crate) fn thresh_add(
    sum: Rc<CommitNode<Elements>>,
    summand: Rc<CommitNode<Elements>>,
) -> Result<Rc<CommitNode<Elements>>, Error> {
    // 1 → 2^32 × 2^32
    let pair_sum_summand = CommitNode::pair(sum, summand)?;
    // 2^32 × 2^32 → 2 × 2^32
    let add32 = CommitNode::jet(Elements::Add32);
    // 1 → 2 x 2^32
    let full_sum = CommitNode::comp(pair_sum_summand, add32)?;
    // 2^32 → 2^32
    let iden = CommitNode::iden();
    // 2 × 2^32 → 2^32
    let drop_iden = CommitNode::drop(iden);

    // Discard the overflow bit.
    // FIXME: enforce that sum of weights is less than 2^32
    // 1 → 2^32
    CommitNode::comp(full_sum, drop_iden)
}

/// verify(sum): 1 → 1
pub(crate) fn thresh_verify(
    sum: Rc<CommitNode<Elements>>,
    k: u32,
) -> Result<Rc<CommitNode<Elements>>, Error> {
    // 1 → 2^32
    let const_k = CommitNode::const_word(Value::u32(k));
    // 1 → 2^32 × 2^32
    let pair_k_sum = CommitNode::pair(const_k, sum)?;
    // 2^32 × 2^32 → 2
    let eq32 = CommitNode::jet(Elements::Eq32);

    // 1 → 1
    verify_bexp(pair_k_sum, eq32)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::jet::elements::ElementsEnv;
    use crate::{BitMachine, FailEntropy};
    use bitcoin_hashes::{sha256, Hash};
    use elements::{bitcoin, secp256k1_zkp};
    use std::sync::Arc;

    fn compile(policy: Policy<bitcoin::XOnlyPublicKey>) -> (Rc<CommitNode<Elements>>, ElementsEnv) {
        let commit = super::compile(&policy);
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
        let finalized = finalized.to_node();
        let mut mac = BitMachine::for_program(&finalized);

        match mac.exec(&finalized, env) {
            Ok(output) => output == Value::Unit,
            Err(_) => false,
        }
    }

    #[test]
    fn execute_unsatisfiable() {
        let (commit, env) = compile(Policy::Unsatisfiable(FailEntropy::ZERO));
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
        let commit = Policy::Key(xonly).compile();

        assert!(execute_successful(
            &commit,
            vec![Value::u512_from_slice(signature.as_ref())],
            &env
        ));
    }

    #[test]
    fn execute_after() {
        let env = ElementsEnv::dummy_with(elements::PackedLockTime(42), elements::Sequence::ZERO);

        let commit = Policy::<bitcoin::XOnlyPublicKey>::After(41).compile();
        assert!(execute_successful(&commit, vec![], &env));

        let commit = Policy::<bitcoin::XOnlyPublicKey>::After(42).compile();
        assert!(execute_successful(&commit, vec![], &env));

        let commit = Policy::<bitcoin::XOnlyPublicKey>::After(43).compile();
        assert!(!execute_successful(&commit, vec![], &env));
    }

    #[test]
    fn execute_older() {
        let env = ElementsEnv::dummy_with(
            elements::PackedLockTime::ZERO,
            elements::Sequence::from_consensus(42),
        );

        let commit = Policy::<bitcoin::XOnlyPublicKey>::Older(41).compile();
        assert!(execute_successful(&commit, vec![], &env));

        let commit = Policy::<bitcoin::XOnlyPublicKey>::Older(42).compile();
        assert!(execute_successful(&commit, vec![], &env));

        let commit = Policy::<bitcoin::XOnlyPublicKey>::Older(43).compile();
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

        let (commit, env) = compile(Policy::And {
            left: Arc::new(Policy::Sha256(image0)),
            right: Arc::new(Policy::Sha256(image1)),
        });

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

        let (commit, env) = compile(Policy::And {
            left: Arc::new(Policy::Sha256(image0)),
            right: Arc::new(Policy::Trivial),
        });

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

        let (commit, env) = compile(Policy::Or {
            left: Arc::new(Policy::Sha256(image0)),
            right: Arc::new(Policy::Sha256(image1)),
        });

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
