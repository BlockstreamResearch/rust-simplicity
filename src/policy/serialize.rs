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
use crate::node::{self, Node};
use crate::node::{CoreConstructible, JetConstructible, WitnessConstructible};
use crate::{FailEntropy, Value};

use std::convert::TryFrom;
use std::sync::Arc;

pub type ArcNode<N> = Arc<Node<N>>;

pub fn unsatisfiable<N>(entropy: FailEntropy) -> ArcNode<N>
where
    N: node::Marker,
    ArcNode<N>: CoreConstructible,
{
    ArcNode::fail(entropy)
}

pub fn trivial<N>() -> ArcNode<N>
where
    N: node::Marker,
    ArcNode<N>: CoreConstructible,
{
    ArcNode::unit()
}

pub fn key<N, Pk>(key: &Pk, witness: N::Witness) -> ArcNode<N>
where
    Pk: ToPublicKey,
    N: node::Marker,
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
    N: node::Marker,
    ArcNode<N>: CoreConstructible + JetConstructible<Elements>,
{
    let n_value = Arc::new(Value::u32(n));
    let const_n = ArcNode::const_word(n_value);
    let check_lock_height = ArcNode::jet(Elements::CheckLockHeight);

    ArcNode::comp(&const_n, &check_lock_height).expect("consistent types")
}

pub fn older<N>(n: u16) -> ArcNode<N>
where
    N: node::Marker,
    ArcNode<N>: CoreConstructible + JetConstructible<Elements>,
{
    let n_value = Arc::new(Value::u16(n));
    let const_n = ArcNode::const_word(n_value);
    let check_lock_distance = ArcNode::jet(Elements::CheckLockDistance);

    ArcNode::comp(&const_n, &check_lock_distance).expect("consistent types")
}

fn compute_sha256<N>(witness256: &ArcNode<N>) -> ArcNode<N>
where
    N: node::Marker,
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
    N: node::Marker,
    ArcNode<N>: CoreConstructible + JetConstructible<Elements>,
{
    let computed_bexp = ArcNode::comp(input, bexp).expect("consistent types");
    let verify = ArcNode::jet(Elements::Verify);
    ArcNode::comp(&computed_bexp, &verify).expect("consistent types")
}

pub fn sha256<N, Pk>(hash: &Pk::Sha256, witness: N::Witness) -> ArcNode<N>
where
    Pk: ToPublicKey,
    N: node::Marker,
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
    N: node::Marker,
    ArcNode<N>: CoreConstructible,
{
    ArcNode::comp(left, right).expect("consistent types")
}

fn selector<N>(witness_bit: N::Witness) -> ArcNode<N>
where
    N: node::Marker,
    ArcNode<N>: CoreConstructible + WitnessConstructible<N::Witness>,
{
    let witness = ArcNode::witness(witness_bit);
    let unit = ArcNode::unit();
    ArcNode::pair(&witness, &unit).expect("consistent types")
}

pub fn or<N>(left: &ArcNode<N>, right: &ArcNode<N>, witness_bit: N::Witness) -> ArcNode<N>
where
    N: node::Marker,
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
    N: node::Marker,
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
    N: node::Marker,
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
    N: node::Marker,
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
    N: node::Marker,
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::jet::elements::ElementsEnv;
    use crate::node::{ConstructNode, SimpleFinalizer};
    use crate::policy::Policy;
    use crate::{BitMachine, FailEntropy, Value};
    use bitcoin_hashes::{sha256, Hash};
    use elements::bitcoin::key::XOnlyPublicKey;
    use elements::locktime::Height;
    use elements::secp256k1_zkp;
    use std::sync::Arc;

    fn compile(policy: Policy<XOnlyPublicKey>) -> (Arc<ConstructNode<Elements>>, ElementsEnv) {
        let commit = policy.serialize_no_witness();
        let env = ElementsEnv::dummy();

        (commit, env)
    }

    fn execute_successful(
        commit: &ConstructNode<Elements>,
        witness: Vec<Value>,
        env: &ElementsEnv,
    ) -> bool {
        let finalized = commit
            .finalize_types()
            .expect("finalize types")
            .finalize(&mut SimpleFinalizer::new(witness.into_iter().map(Arc::new)))
            .expect("finalize");
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
        let commit = Policy::Key(xonly).serialize_no_witness();

        assert!(execute_successful(
            &commit,
            vec![Value::u512_from_slice(signature.as_ref())],
            &env
        ));
    }

    #[test]
    fn execute_after() {
        let height = Height::from_consensus(42).unwrap();
        let env =
            ElementsEnv::dummy_with(elements::LockTime::Blocks(height), elements::Sequence::ZERO);

        let commit = Policy::<XOnlyPublicKey>::After(41).serialize_no_witness();
        assert!(execute_successful(&commit, vec![], &env));

        let commit = Policy::<XOnlyPublicKey>::After(42).serialize_no_witness();
        assert!(execute_successful(&commit, vec![], &env));

        let commit = Policy::<XOnlyPublicKey>::After(43).serialize_no_witness();
        assert!(!execute_successful(&commit, vec![], &env));
    }

    #[test]
    fn execute_older() {
        let env = ElementsEnv::dummy_with(
            elements::LockTime::ZERO,
            elements::Sequence::from_consensus(42),
        );

        let commit = Policy::<XOnlyPublicKey>::Older(41).serialize_no_witness();
        assert!(execute_successful(&commit, vec![], &env));

        let commit = Policy::<XOnlyPublicKey>::Older(42).serialize_no_witness();
        assert!(execute_successful(&commit, vec![], &env));

        let commit = Policy::<XOnlyPublicKey>::Older(43).serialize_no_witness();
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
