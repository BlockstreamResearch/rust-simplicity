// Simplcity Policy language
// Written in 2023 by
//     Andrew Poelstra <apoelstra@blockstream.com>
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

//! Serialization of Policy as Simplicity

use crate::jet::Elements;
use crate::miniscript::ToPublicKey;
use crate::node::{CoreConstructible, JetConstructible, WitnessConstructible};
use crate::node::{Node, NodeData};
use crate::{FailEntropy, Value};

use std::convert::TryFrom;
use std::sync::Arc;

pub type ArcNode<N> = Arc<Node<N, Elements>>;

pub fn unsatisfiable<N>(entropy: FailEntropy) -> ArcNode<N>
where
    N: NodeData<Elements>,
    ArcNode<N>: CoreConstructible,
{
    ArcNode::fail(entropy)
}

pub fn trivial<N>() -> ArcNode<N>
where
    N: NodeData<Elements>,
    ArcNode<N>: CoreConstructible,
{
    ArcNode::unit()
}

pub fn key<N, Pk>(key: &Pk, witness: N::Witness) -> ArcNode<N>
where
    Pk: ToPublicKey,
    N: NodeData<Elements>,
    ArcNode<N>: CoreConstructible + JetConstructible<Elements> + WitnessConstructible<N::Witness>,
{
    let key_value = Arc::new(Value::u256_from_slice(&key.to_x_only_pubkey().serialize()));
    let const_key = ArcNode::const_word(key_value);
    let sighash_all = ArcNode::jet(Elements::SigAllHash);
    let pair_key_msg = ArcNode::pair(&const_key, &sighash_all).expect("consistent types");
    let witness = ArcNode::witness(witness);
    let pair_key_msg_sig = ArcNode::pair(&pair_key_msg, &witness).expect("consistent types");
    let bip_0340_verify = ArcNode::jet(Elements::Bip0340Verify);

    ArcNode::comp(&pair_key_msg_sig, &bip_0340_verify).expect("consistent types")
}

pub fn after<N>(n: u32) -> ArcNode<N>
where
    N: NodeData<Elements>,
    ArcNode<N>: CoreConstructible + JetConstructible<Elements>,
{
    let n_value = Arc::new(Value::u32(n));
    let const_n = ArcNode::const_word(n_value);
    let check_lock_height = ArcNode::jet(Elements::CheckLockHeight);

    ArcNode::comp(&const_n, &check_lock_height).expect("consistent types")
}

pub fn older<N>(n: u16) -> ArcNode<N>
where
    N: NodeData<Elements>,
    ArcNode<N>: CoreConstructible + JetConstructible<Elements>,
{
    let n_value = Arc::new(Value::u16(n));
    let const_n = ArcNode::const_word(n_value);
    let check_lock_distance = ArcNode::jet(Elements::CheckLockDistance);

    ArcNode::comp(&const_n, &check_lock_distance).expect("consistent types")
}

fn compute_sha256<N>(witness256: &ArcNode<N>) -> ArcNode<N>
where
    N: NodeData<Elements>,
    ArcNode<N>: CoreConstructible + JetConstructible<Elements>,
{
    let ctx = ArcNode::jet(Elements::Sha256Ctx8Init);
    let pair_ctx_witness = ArcNode::pair(&ctx, witness256).expect("consistent types");
    let add256 = ArcNode::jet(Elements::Sha256Ctx8Add32);
    let digest_ctx = ArcNode::comp(&pair_ctx_witness, &add256).expect("consistent types");
    let finalize = ArcNode::jet(Elements::Sha256Ctx8Finalize);
    ArcNode::comp(&digest_ctx, &finalize).expect("consistent types")
}

fn verify_bexp<N>(input: &ArcNode<N>, bexp: &ArcNode<N>) -> ArcNode<N>
where
    N: NodeData<Elements>,
    ArcNode<N>: CoreConstructible + JetConstructible<Elements>,
{
    let computed_bexp = ArcNode::comp(input, bexp).expect("consistent types");
    let verify = ArcNode::jet(Elements::Verify);
    ArcNode::comp(&computed_bexp, &verify).expect("consistent types")
}

pub fn sha256<N, Pk>(hash: &Pk::Sha256, witness: N::Witness) -> ArcNode<N>
where
    Pk: ToPublicKey,
    N: NodeData<Elements>,
    ArcNode<N>: CoreConstructible + JetConstructible<Elements> + WitnessConstructible<N::Witness>,
{
    let hash_value = Arc::new(Value::u256_from_slice(Pk::to_sha256(hash).as_ref()));
    let const_hash = ArcNode::const_word(hash_value);
    let witness256 = ArcNode::witness(witness);
    let computed_hash = compute_sha256(&witness256);
    let pair_hash_computed_hash =
        ArcNode::pair(&const_hash, &computed_hash).expect("consistent types");
    let eq256 = ArcNode::jet(Elements::Eq256);

    verify_bexp(&pair_hash_computed_hash, &eq256)
}

pub fn and<N>(left: &ArcNode<N>, right: &ArcNode<N>) -> ArcNode<N>
where
    N: NodeData<Elements>,
    ArcNode<N>: CoreConstructible,
{
    ArcNode::comp(left, right).expect("consistent types")
}

fn selector<N>(witness_bit: N::Witness) -> ArcNode<N>
where
    N: NodeData<Elements>,
    ArcNode<N>: CoreConstructible + WitnessConstructible<N::Witness>,
{
    let witness = ArcNode::witness(witness_bit);
    let unit = ArcNode::unit();
    ArcNode::pair(&witness, &unit).expect("consistent types")
}

pub fn or<N>(left: &ArcNode<N>, right: &ArcNode<N>, witness_bit: N::Witness) -> ArcNode<N>
where
    N: NodeData<Elements>,
    ArcNode<N>: CoreConstructible + WitnessConstructible<N::Witness>,
{
    let drop_left = ArcNode::drop_(left);
    let drop_right = ArcNode::drop_(right);
    let case_left_right = ArcNode::case(&drop_left, &drop_right).expect("consistent types");
    let selector = selector(witness_bit);

    ArcNode::comp(&selector, &case_left_right).expect("consistent types")
}

/// child: 1 → 1
///
/// summand(child): 1 → 2^32
fn thresh_summand<N>(child: &ArcNode<N>, witness_bit: N::Witness) -> ArcNode<N>
where
    N: NodeData<Elements>,
    ArcNode<N>: CoreConstructible + WitnessConstructible<N::Witness>,
{
    // 1 → 2 x 1
    let selector = selector(witness_bit);

    // 1 → 2^32
    let const_one = ArcNode::const_word(Arc::new(Value::u32(1)));
    // 1 → 2^32
    let child_one = ArcNode::comp(child, &const_one).expect("consistent types");
    // 1 → 2^32
    let const_zero = ArcNode::const_word(Arc::new(Value::u32(0)));

    // 1 × 1 → 2^32
    let drop_left = ArcNode::drop_(&const_zero);
    // 1 × 1 → 2^32
    let drop_right = ArcNode::drop_(&child_one);
    // 2 x 1 → 2^32
    let child_one_or_zero = ArcNode::case(&drop_left, &drop_right).expect("consistent types");

    // 1 → 2^32
    ArcNode::comp(&selector, &child_one_or_zero).expect("consistent types")
}

/// acc: 1 → 2^32, summand: 1 → 2^32
///
/// add(sum, summand): 1 → 2^32
fn thresh_add<N>(sum: &ArcNode<N>, summand: &ArcNode<N>) -> ArcNode<N>
where
    N: NodeData<Elements>,
    ArcNode<N>: CoreConstructible + JetConstructible<Elements> + WitnessConstructible<N::Witness>,
{
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
fn thresh_verify<N>(sum: &ArcNode<N>, k: u32) -> ArcNode<N>
where
    N: NodeData<Elements>,
    ArcNode<N>: CoreConstructible + JetConstructible<Elements> + WitnessConstructible<N::Witness>,
{
    // 1 → 2^32
    let const_k = ArcNode::const_word(Arc::new(Value::u32(k)));
    // 1 → 2^32 × 2^32
    let pair_k_sum = ArcNode::pair(&const_k, sum).expect("consistent types");
    // 2^32 × 2^32 → 2
    let eq32 = ArcNode::jet(Elements::Eq32);

    // 1 → 1
    verify_bexp(&pair_k_sum, &eq32)
}

pub fn threshold<N>(k: u32, subs: &[ArcNode<N>], witness_bits: &[N::Witness]) -> ArcNode<N>
where
    N: NodeData<Elements>,
    ArcNode<N>: CoreConstructible + JetConstructible<Elements> + WitnessConstructible<N::Witness>,
{
    let n = u32::try_from(subs.len()).expect("can have at most 2^32 children in a threshold");
    assert!(k <= n, "threshold must be <= number of children");
    assert!(!subs.is_empty(), "cannot have empty threshold");

    let mut sum = thresh_summand(&subs[0], witness_bits[0].clone());
    for (sub, witness_bit) in subs[1..].iter().zip(witness_bits[1..].iter()) {
        let summand = thresh_summand(sub, witness_bit.clone());
        sum = thresh_add(&sum, &summand);
    }

    thresh_verify(&sum, k)
}
