// Simplcity Policy language
// Written in 2020 by
//     Sanket Kanjalkar <sanket1729@gmail.com>
//
// To the extent possible under law, the author(s) have dedicated all
// copyright and related and neighboring rights to this software to
// the public domain worldwide. This software is distributed without
// any warranty.
//
// You should have received a copy of the CC0 Public Domain Dedication
// along with this software.
// If not, see <http://creativecommons.org/publicdomain/zero/1.0/>.
//

//! # Function-like Expression Language
//! This language is similar to policy language in miniscript, but
//! whose target is simplicity instead of bitcoin Script. Policy
//!
//! # Simplicity Script Policies
//!
//! Tools for representing simplicity programs as spending policies.
//! These may be compiled to Simplicity programs.
//!
//! The format represents public keys abstractly to allow wallets to replace
//! these with BIP32 paths, pay-to-contract instructions, etc.
//!

mod ast;
mod compiler;
pub mod descriptor;
mod embed;
mod error;
//pub mod satisfy;

use crate::jet::Elements;
use crate::miniscript::{ToPublicKey, Translator};
use crate::node::{CommitNode, ConstructNode, NoWitness};
use crate::node::{CoreConstructible, JetConstructible, WitnessConstructible};
use crate::{FailEntropy, Value};

use std::convert::TryInto;
use std::fmt;
use std::sync::Arc;

pub use descriptor::Descriptor;
pub use error::Error;

type ArcNode = Arc<ConstructNode<Elements>>;

/// Policy that expresses spending conditions for Simplicity.
///
/// The policy can be encoded directly as a Simplicity program which can be committed to the
/// blockchain; when finalized, it can executed on the Bit Machine.
///
/// Policies can be normalized and are amenable to semantic analysis.
#[derive(Clone)]
pub struct Policy<Pk: ToPublicKey> {
    fragment: ast::Fragment<Pk>,
    node: ArcNode,
}

impl<Pk: ToPublicKey> PartialEq for Policy<Pk> {
    fn eq(&self, other: &Self) -> bool {
        self.node.cmr() == other.node.cmr()
    }
}
impl<Pk: ToPublicKey> Eq for Policy<Pk> {}

impl<Pk: ToPublicKey> Policy<Pk> {
    /// Create an unsatisfiable policy.
    pub fn unsatisfiable(entropy: FailEntropy) -> Self {
        Policy {
            fragment: ast::Fragment::Unsatisfiable(entropy),
            node: ArcNode::fail(entropy),
        }
    }

    /// Create a trivial policy.
    pub fn trivial() -> Self {
        Policy {
            fragment: ast::Fragment::Trivial,
            node: ArcNode::unit(),
        }
    }

    /// Create a signature check.
    pub fn key(key: Pk) -> Self {
        let key_value = Arc::new(Value::u256_from_slice(&key.to_x_only_pubkey().serialize()));
        let const_key = ArcNode::const_word(key_value);
        let sighash_all = ArcNode::jet(Elements::SigAllHash);
        let pair_key_msg = ArcNode::pair(&const_key, &sighash_all).expect("consistent types");
        let witness = ArcNode::witness(NoWitness);
        let pair_key_msg_sig = ArcNode::pair(&pair_key_msg, &witness).expect("consistent types");
        let bip_0340_verify = ArcNode::jet(Elements::Bip0340Verify);

        Policy {
            fragment: ast::Fragment::Key(key),
            node: ArcNode::comp(&pair_key_msg_sig, &bip_0340_verify).expect("consistent types"),
        }
    }

    /// Create an absolute locktime check.
    pub fn after(n: u32) -> Self {
        let n_value = Arc::new(Value::u32(n));
        let const_n = ArcNode::const_word(n_value);
        let check_lock_height = ArcNode::jet(Elements::CheckLockHeight);
        Policy {
            fragment: ast::Fragment::After(n),
            node: ArcNode::comp(&const_n, &check_lock_height).expect("consistent types"),
        }
    }

    /// Create a relative locktime check.
    pub fn older(n: u16) -> Self {
        let n_value = Arc::new(Value::u16(n));
        let const_n = ArcNode::const_word(n_value);
        let check_lock_distance = ArcNode::jet(Elements::CheckLockDistance);
        Policy {
            fragment: ast::Fragment::Older(n),
            node: ArcNode::comp(&const_n, &check_lock_distance).expect("consistent types"),
        }
    }

    /// Create a SHA256 preimage check.
    pub fn sha256(hash: Pk::Sha256) -> Self {
        let hash_bytes = Pk::to_sha256(&hash);
        let hash_value = Arc::new(Value::u256_from_slice(hash_bytes.as_ref()));
        let const_hash = ArcNode::const_word(hash_value);
        let witness256 = ArcNode::witness(NoWitness);
        let computed_hash = fragment_aux::compute_sha256(&witness256);
        let pair_hash_computed_hash =
            ArcNode::pair(&const_hash, &computed_hash).expect("consistent types");
        let eq256 = ArcNode::jet(Elements::Eq256);

        Policy {
            fragment: ast::Fragment::Sha256(hash),
            node: fragment_aux::verify_bexp(&pair_hash_computed_hash, &eq256),
        }
    }

    /// Create a conjunction of two policies.
    pub fn and(left: Arc<Self>, right: Arc<Self>) -> Self {
        Policy {
            node: ArcNode::comp(&left.node, &right.node).expect("consistent types"),
            fragment: ast::Fragment::And { left, right },
        }
    }

    /// Create a disjunction of two policies.
    pub fn or(left: Arc<Self>, right: Arc<Self>) -> Self {
        Policy {
            node: fragment_aux::or(&left.node, &right.node),
            fragment: ast::Fragment::Or { left, right },
        }
    }

    /// Create a disjunction of two policies.
    pub fn threshold(k: usize, subs: Vec<Self>) -> Self {
        assert!(!subs.is_empty());
        assert!(k <= subs.len());
        let k32: u32 = k.try_into().expect("k < 2^32");

        let mut sum = fragment_aux::thresh_summand(Arc::clone(&subs[0].node));
        for sub in &subs[1..] {
            let summand = fragment_aux::thresh_summand(Arc::clone(&sub.node));
            sum = fragment_aux::thresh_add(&sum, &summand);
        }

        Policy {
            fragment: ast::Fragment::Threshold(k, subs),
            node: fragment_aux::thresh_verify(sum, k32),
        }
    }

    /// Serializes the policy as a Simplicity program
    pub fn serialize(&self) -> Arc<CommitNode<Elements>> {
        self.node.finalize_types().expect("consistent types")
    }

    /// Convert a fragment using one kind of public key to another type of public key
    pub fn translate<T, Q, E>(&self, translator: &mut T) -> Result<Policy<Q>, E>
    where
        T: Translator<Pk, Q, E>,
        Q: ToPublicKey,
    {
        Ok(Policy {
            fragment: self.fragment.translate(translator)?,
            node: Arc::clone(&self.node),
        })
    }
}

impl<Pk: ToPublicKey> fmt::Debug for Policy<Pk> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Debug::fmt(&self.fragment, f)
    }
}

impl<Pk: ToPublicKey> fmt::Display for Policy<Pk> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(&self.fragment, f)
    }
}

/// Some helper methods for constructing fragments
mod fragment_aux {
    use super::*;

    pub fn compute_sha256(witness256: &ArcNode) -> ArcNode {
        let ctx = ArcNode::jet(Elements::Sha256Ctx8Init);
        let pair_ctx_witness = ArcNode::pair(&ctx, witness256).expect("consistent types");
        let add256 = ArcNode::jet(Elements::Sha256Ctx8Add32);
        let digest_ctx = ArcNode::comp(&pair_ctx_witness, &add256).expect("consistent types");
        let finalize = ArcNode::jet(Elements::Sha256Ctx8Finalize);
        ArcNode::comp(&digest_ctx, &finalize).expect("consistent types")
    }

    pub fn verify_bexp(input: &ArcNode, bexp: &ArcNode) -> ArcNode {
        let computed_bexp = ArcNode::comp(input, bexp).expect("consistent types");
        let verify = ArcNode::jet(Elements::Verify);
        ArcNode::comp(&computed_bexp, &verify).expect("consistent types")
    }

    pub fn selector() -> ArcNode {
        let witness = ArcNode::witness(NoWitness);
        let unit = ArcNode::unit();
        ArcNode::pair(&witness, &unit).expect("consistent types")
    }

    pub fn or(left: &ArcNode, right: &ArcNode) -> ArcNode {
        let drop_left = ArcNode::drop_(left);
        let drop_right = ArcNode::drop_(right);
        let case_left_right = ArcNode::case(&drop_left, &drop_right).expect("consistent types");
        let selector = selector();

        ArcNode::comp(&selector, &case_left_right).expect("consistent types")
    }

    /// child: 1 → 1
    ///
    /// summand(child): 1 → 2^32
    pub fn thresh_summand(child: ArcNode) -> ArcNode {
        // 1 → 2 x 1
        let selector = selector();

        // 1 → 2^32
        let const_one = ArcNode::const_word(Arc::new(Value::u32(1)));
        // 1 → 2^32
        let child_one = ArcNode::comp(&child, &const_one).expect("consistent types");
        // 1 → 2^32
        let const_zero = ArcNode::const_word(Arc::new(Value::u32(0)));
        // 2 x 1 → 2^32
        let drop_left = ArcNode::drop_(&const_zero);
        let drop_right = ArcNode::drop_(&child_one);
        let child_one_or_zero = ArcNode::case(&drop_left, &drop_right).expect("consistent types");

        // 1 → 2^32
        ArcNode::comp(&selector, &child_one_or_zero).expect("consistent types")
    }

    /// acc: 1 → 2^32, summand: 1 → 2^32
    ///
    /// add(sum, summand): 1 → 2^32
    pub fn thresh_add(sum: &ArcNode, summand: &ArcNode) -> ArcNode {
        // 1 → 2^32 × 2^32
        let pair_sum_summand = ArcNode::pair(sum, summand).expect("consistent types");
        // 2^32 × 2^32 → 2 × 2^32
        let add32 = ArcNode::jet(Elements::Add32);
        // 1 → 2 x 2^32
        let full_sum = ArcNode::comp(&pair_sum_summand, &add32).expect("consistent types");
        // 2^32 → 2^32
        let iden = ArcNode::iden();
        // 2 × 2^32 → 2^32
        let drop_iden = ArcNode::drop_(&iden);

        // Discard the overflow bit.
        // FIXME: enforce that sum of weights is less than 2^32
        // 1 → 2^32
        ArcNode::comp(&full_sum, &drop_iden).expect("consistent types")
    }

    /// verify(sum): 1 → 1
    pub fn thresh_verify(sum: ArcNode, k: u32) -> ArcNode {
        // 1 → 2^32
        let const_k = ArcNode::const_word(Arc::new(Value::u32(k)));
        // 1 → 2^32 × 2^32
        let pair_k_sum = ArcNode::pair(&const_k, &sum).expect("consistent types");
        // 2^32 × 2^32 → 2
        let eq32 = ArcNode::jet(Elements::Eq32);

        // 1 → 1
        verify_bexp(&pair_k_sum, &eq32)
    }
}
