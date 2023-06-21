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

//! Abstract Policies
//!
//! These policies represent spending conditions in the most simplest form
//! Policies are combination of `and`, `or` and `thresh` fragments. For example,
//! or(pk(A),pk(B)) represents a policy that can be spend with either pk(A) or pk(B).
//! These policies can be compiled to Simplicity and also be lifted back up from

use crate::dag::PostOrderIterItem;
use crate::jet::Elements;
use crate::node::{self, Commit, CommitNode, Converter, Hide, NoWitness, Redeem, RedeemData, RedeemNode};
use crate::{Error, Value};

use super::ast::Fragment;
use super::{Policy, RtlPolicyIterator};

use bitcoin_hashes::Hash;
use elements::locktime::Height;
use elements::taproot::TapLeafHash;
use elements::{LockTime, Sequence};
use elements_miniscript::{MiniscriptKey, Preimage32, Satisfier, ToPublicKey};

use std::collections::HashMap;
use std::marker::PhantomData;
use std::sync::Arc;


impl<Pk: ToPublicKey> Policy<Pk> {
    pub fn satisfy<S: Satisfier<Pk>>(&self, satisfier: S) -> Result<Arc<RedeemNode<Elements>>, Error> {
        let mut finalizer = PolicyFinalizer::new(satisfier, &self);
        let commit = self.serialize();
        commit.rtl_finalize(&mut finalizer)
    }
}

/// Simple struct containing a transaction environment and maps of wallet
/// data (including secret keys!) that can be used to satisfy a transaction.
pub struct PolicySatisfier<'a, Pk: MiniscriptKey> {
    pub preimages: HashMap<Pk::Sha256, Preimage32>,
    pub signatures: HashMap<Pk, elements::SchnorrSig>,
    pub tx: &'a elements::Transaction,
    pub index: usize,
}

impl<'a, Pk: MiniscriptKey + ToPublicKey> Satisfier<Pk> for PolicySatisfier<'a, Pk> {
    fn lookup_tap_leaf_script_sig(&self, pk: &Pk, _: &TapLeafHash) -> Option<elements::SchnorrSig> {
        self.signatures.get(pk).copied()
    }

    fn lookup_sha256(&self, hash: &Pk::Sha256) -> Option<Preimage32> {
        self.preimages.get(hash).copied()
    }

    fn check_older(&self, sequence: Sequence) -> bool {
        let self_sequence = self.tx.input[self.index].sequence;
        <Sequence as Satisfier<Pk>>::check_older(&self_sequence, sequence)
    }

    fn check_after(&self, locktime: LockTime) -> bool {
        let self_locktime = LockTime::from(self.tx.lock_time);
        <LockTime as Satisfier<Pk>>::check_after(&self_locktime, locktime)
    }
}

pub struct PolicyFinalizer<'pol, Pk: ToPublicKey, S: Satisfier<Pk>> {
    satisfier: S,
    policy_iter: RtlPolicyIterator<'pol, Pk>,
    current: Option<&'pol Policy<Pk>>,
    last_or_branch: bool,
    last_thresh_weights: Vec<Option<usize>>,
    // Idk why this is necessary
    phantom: PhantomData<Pk>,
}

impl<'pol, Pk: ToPublicKey, S: Satisfier<Pk>> PolicyFinalizer<'pol, Pk, S> {
    pub fn new(satisfier: S, policy: &'pol Policy<Pk>) -> Self {
        let mut policy_iter = RtlPolicyIterator::new(policy);
        let current = policy_iter.next();
        PolicyFinalizer {
            satisfier,
            policy_iter,
            current,
            last_or_branch: false,
            last_thresh_weights: vec![],
            phantom: PhantomData,
        }

    }

    /// Move finalization state to the next policy fragment
    fn ratchet_policy_iter(&mut self) {
        self.current = self.policy_iter.next();
        if self.current.is_some() {
            println!("Ratcheted iterator to {}", self.current.as_ref().unwrap());
        }
    }

    fn current_fragment(&self) -> &Fragment<Pk> {
        match self.current {
            Some(pol) => &pol.fragment,
            None => panic!("`current_fragment` should not be called at the end of iteration")
        }
    }
}

impl<'pol, Pk: ToPublicKey, S: Satisfier<Pk>> Finalizer<Commit, Redeem, Elements> for PolicyFinalizer<'pol, Pk, S> {
    type Error = Error;

    fn visit_node(&mut self, data: &PostOrderIterItem<&CommitNode<Elements>>) {
        println!("Visiting {}", data.node.inner());
        assert!( 
            self.current.is_some(),
            "a current fragment should be available until the very last iteration",
        );
        if Some(data.node.cmr()) == self.current.map(|pol| pol.node.cmr()) {
            self.ratchet_policy_iter();

            // For thresholds specifically we have to do lookahead to figure out the
            // cost/feasibility of every child.
            //
            // What we do is spawn a new satisfier for each child, record its weight
            if let Some(pol) = self.current {
                if let Fragment::Threshold(k, ref subs) = pol.fragment {
                    self.last_thresh_weights = vec![None; subs.len()];
                    for (weight, sub) in self.last_thresh_weights.iter_mut().zip(subs.iter()) {
                        /*
                        //TODO
                        if sub.satisfy(self.satisfier).is_ok() {
                            *weight = Some(1); // FIXME
                        }
                        */
                        println!("Entered threshold {}/{}; looking at sub {}", k, subs.len(), sub);
                    }
                }
            }
        }
    }

    fn convert_witness(
        &mut self,
        _: &PostOrderIterItem<&CommitNode<Elements>>,
        _: &NoWitness,
    ) -> Result<Option<Arc<Value>>, Self::Error> {
        Ok(match self.current_fragment() {
            Fragment::Trivial
            | Fragment::Unsatisfiable(..) => unreachable!(),
            Fragment::Key(pk) => self
                .satisfier
                .lookup_tap_leaf_script_sig(pk, &TapLeafHash::all_zeros())
                .map(|sig| Arc::new(Value::u512_from_slice(sig.sig.as_ref()))),
            Fragment::Sha256(hash) => self
                .satisfier
                .lookup_sha256(hash)
                .map(|preimage| Arc::new(Value::u256_from_slice(&preimage))),
            Fragment::After(..)
            | Fragment::Older(..)
            | Fragment::And { .. } => unreachable!(),
            Fragment::Or { .. } => Some(Arc::new(Value::u1(self.last_or_branch.into()))),
            Fragment::Threshold { .. } => unreachable!(),
        })
    }

    fn prune_case(
        &mut self,
        _: &PostOrderIterItem<&CommitNode<Elements>>,
        left: Option<&Arc<RedeemNode<Elements>>>,
        right: Option<&Arc<RedeemNode<Elements>>>,
    ) -> Result<Hide, Self::Error> {
        let ret = match (left.is_some(), right.is_some()) {
            (true, true) => Hide::Left, // FIXME look at weights and make an optimal decision
            (false, true) => Hide::Left,
            (true, false) => Hide::Right,
            (false, false) => Hide::Both,
        };
        self.last_or_branch = ret == Hide::Left;
        Ok(ret)
    }

    fn convert_data(
        &mut self,
        data: &PostOrderIterItem<&CommitNode<Elements>>,
        inner: node::Inner<&Arc<RedeemNode<Elements>>, Elements, Arc<Value>>,
    ) -> Result<Option<Arc<RedeemData<Elements>>>, Self::Error> {
        // Handle inner/after nodes
        if let node::Inner::Jet(..) = inner {
            match self.current_fragment() {
                Fragment::After(n) => {
                    let height = Height::from_consensus(*n).expect("valid locktime");
                    if !self.satisfier.check_after(LockTime::Blocks(height)) {
                        return Ok(None);
                    }
                }
                Fragment::Older(n) => {
                    if !self.satisfier.check_older(Sequence((*n).into())) {
                        return Ok(None)
                    }
                }
                _ => {}
            }
        }

        // Copied from `SimpleFinalizer`
        let converted_data = inner.map(|node| node.cached_data());
        Ok(Some(Arc::new(RedeemData::new(
            data.node.arrow().shallow_clone(),
            converted_data,
        ))))
    }
}


/*
            Policy::Threshold(k, sub_policies) => {
                assert!(
                    sub_policies.len() >= 2,
                    "Thresholds must have at least two sub-policies"
                );

                let mut satisfiable_children = Vec::new();

                for (index, policy) in sub_policies.iter().enumerate() {
                    if let Some(ext) = policy.satisfy_helper(satisfier) {
                        satisfiable_children.push((ext, index));
                    }
                }

                if satisfiable_children.len() < *k {
                    return None;
                }

                // 1) Sort by witness cost
                // 2) Select k best satisfactions
                // 3) Sort by index in decreasing order
                satisfiable_children.sort_by_key(|(sat, _)| sat.cost());
                satisfiable_children.truncate(*k);
                satisfiable_children.sort_by(|(_, i), (_, j)| j.cmp(i));

                /// Return satisfaction for `i`th child, if it exists, or return dummy instead
                fn get_sat_or_default<Pk: MiniscriptKey>(
                    i: usize,
                    satisfiable_children: &mut Vec<(SatisfierExtData, usize)>,
                    sub_policies: &[Policy<Pk>],
                ) -> SatisfierExtData {
                    if let Some(x) = satisfiable_children.last() {
                        if x.1 == i {
                            return satisfiable_children.pop().unwrap().0;
                        }
                    }

                    SatisfierExtData {
                        witness: VecDeque::new(),
                        witness_cost: 0,
 //                       program: sub_policies[i].compile(context).unwrap(),
                        program_cost: 0,
                    }
                }

                /// Return summand program for `i`th child
                fn get_summand<Pk: MiniscriptKey>(
                    i: usize,
                    satisfiable_children: &mut Vec<(SatisfierExtData, usize)>,
                    sub_policies: &[Policy<Pk>],
                ) -> SatisfierExtData {
                    let mut sat = get_sat_or_default(i, satisfiable_children, sub_policies);
                    // Satisfactions always have non-zero program cost
                    let branch = if sat.program_cost == 0 {
                        sat.witness.push_front(Value::u1(0));
                    } else {
                        sat.witness.push_front(Value::u1(1));
                    };
                    sat.witness_cost += 1;
                    sat.program_cost += 9;

                    sat
                }

                let mut sat = get_summand(0, &mut satisfiable_children, sub_policies);

                for i in 1..sub_policies.len() {
                    let child_sat = get_summand(i, &mut satisfiable_children, sub_policies);

                    sat.witness.extend(child_sat.witness);
                    sat.witness_cost += child_sat.witness_cost;
//                    sat.program =
//                        compiler::thresh_add(context, sat.program, child_sat.program).unwrap();
                    sat.program_cost += child_sat.program_cost + 6;
                }

//                sat.program = compiler::thresh_verify(context, sat.program, *k as u32).unwrap();
                sat.program_cost += 4;

                Some(sat)
            }
        }
    }
}
*/

#[cfg(test)]
mod tests {
    use super::*;
    use crate::dag::{DagLike, NoSharing};
    use crate::exec::BitMachine;
    use crate::jet::elements::ElementsEnv;
    use crate::FailEntropy;
    use bitcoin_hashes::{sha256, Hash};
    use elements::{bitcoin, secp256k1_zkp, PackedLockTime, SchnorrSigHashType};
    use std::convert::TryFrom;

    type PubKey = bitcoin::XOnlyPublicKey;

    fn get_satisfier(env: &ElementsEnv) -> PolicySatisfier<PubKey> {
        let mut preimages = HashMap::new();
        let mut signatures = HashMap::new();

        for i in 0..3 {
            let preimage = [i; 32];
            preimages.insert(sha256::Hash::hash(&preimage), preimage);
        }

        let secp = secp256k1_zkp::Secp256k1::new();
        let mut rng = secp256k1_zkp::rand::rngs::ThreadRng::default();

        for _ in 0..3 {
            let keypair = bitcoin::KeyPair::new(&secp, &mut rng);
            let xonly = keypair.x_only_public_key().0;

            let sighash = env.c_tx_env().sighash_all();
            let msg = secp256k1_zkp::Message::from_slice(&sighash).expect("constant sighash");
            let sig = elements::SchnorrSig {
                sig: keypair.sign_schnorr(msg),
                hash_ty: SchnorrSigHashType::All,
            };

            signatures.insert(xonly, sig);
        }

        PolicySatisfier {
            preimages,
            signatures,
            tx: env.tx(),
            index: 0,
        }
    }

    fn execute_successful(program: &RedeemNode<Elements>, env: &ElementsEnv) {
        let mut mac = BitMachine::for_program(program);
        assert!(mac.exec(program, env).is_ok());
    }

    fn to_witness(program: &RedeemNode<Elements>) -> Vec<&Arc<Value>> {
        program
            .post_order_iter::<NoSharing>()
            .into_witnesses()
            .collect()
    }

    #[test]
    fn satisfy_unsatisfiable() {
        let env = ElementsEnv::dummy();
        let satisfier = get_satisfier(&env);

        let policy = Policy::unsatisfiable(FailEntropy::ZERO);
        assert!(policy.satisfy(&satisfier).is_err());
    }

    #[test]
    fn satisfy_trivial() {
        let env = ElementsEnv::dummy();
        let satisfier = get_satisfier(&env);

        let policy = Policy::trivial();
        let program = policy.satisfy(&satisfier).expect("satisfiable");

        let witness = to_witness(&program);
        assert!(witness.is_empty());

        execute_successful(&program, &env);
    }

    #[test]
    fn satisfy_pk() {
        let env = ElementsEnv::dummy();
        let satisfier = get_satisfier(&env);

        let mut it = satisfier.signatures.keys();
        let xonly = *it.next().unwrap();
        let policy = Policy::key(xonly);

        let program = policy.satisfy(&satisfier).expect("satisfiable");

        let witness = to_witness(&program);
        assert_eq!(1, witness.len());

        let sighash = env.c_tx_env().sighash_all();
        let message = secp256k1_zkp::Message::from(sighash);
        let signature_bytes = witness[0].try_to_bytes().expect("to bytes");
        let signature =
            secp256k1_zkp::schnorr::Signature::from_slice(&signature_bytes).expect("to signature");
        assert!(signature.verify(&message, &xonly).is_ok());

        execute_successful(&program, &env);
    }

    #[test]
    fn satisfy_sha256() {
        let env = ElementsEnv::dummy();
        let satisfier = get_satisfier(&env);

        let mut it = satisfier.preimages.keys();
        let image = *it.next().unwrap();
        let policy = Policy::sha256(image);

        let program = policy.satisfy(&satisfier).expect("satisfiable");

        let witness = to_witness(&program);
        assert_eq!(1, witness.len());

        let witness_bytes = witness[0].try_to_bytes().expect("to bytes");
        let witness_preimage = Preimage32::try_from(witness_bytes.as_slice()).expect("to array");
        let preimage = *satisfier.preimages.get(&image).unwrap();
        assert_eq!(preimage, witness_preimage);

        execute_successful(&program, &env);
    }

    #[test]
    fn satisfy_after() {
        let env = ElementsEnv::dummy_with(PackedLockTime(42), Sequence::ZERO);
        let satisfier = get_satisfier(&env);

        let policy0 = Policy::after(41);
        let program = policy0.satisfy(&satisfier).expect("satisfiable");
        let witness = to_witness(&program);
        assert!(witness.is_empty());
        execute_successful(&program, &env);

        let policy1 = Policy::after(42);
        let program = policy1.satisfy(&satisfier).expect("satisfiable");
        let witness = to_witness(&program);
        assert!(witness.is_empty());
        execute_successful(&program, &env);

        let policy2 = Policy::after(43);
        assert!(policy2.satisfy(&satisfier).is_err(), "unsatisfiable");
    }

    #[test]
    fn satisfy_older() {
        let env = ElementsEnv::dummy_with(PackedLockTime::ZERO, Sequence::from_consensus(42));
        let satisfier = get_satisfier(&env);

        let policy0 = Policy::older(41);
        let program = policy0.satisfy(&satisfier).expect("satisfiable");
        let witness = to_witness(&program);
        assert!(witness.is_empty());
        execute_successful(&program, &env);

        let policy1 = Policy::older(42);
        let program = policy1.satisfy(&satisfier).expect("satisfiable");
        let witness = to_witness(&program);
        assert!(witness.is_empty());
        execute_successful(&program, &env);

        let policy2 = Policy::older(43);
        assert!(policy2.satisfy(&satisfier).is_err(), "unsatisfiable");
    }

    #[test]
    fn satisfy_and() {
        let env = ElementsEnv::dummy_with(PackedLockTime::ZERO, Sequence::from_consensus(42));
        let satisfier = get_satisfier(&env);

        let images: Vec<_> = satisfier.preimages.keys().copied().collect();
        let preimages: Vec<_> = images
            .iter()
            .map(|x| satisfier.preimages.get(x).unwrap())
            .collect();

        // Policy 0
        let bad = Arc::new(Policy::sha256(sha256::Hash::from_inner([0; 32])));
        let left = Arc::new(Policy::sha256(images[0]));
        let right = Arc::new(Policy::sha256(images[1]));
        let policy0 = Policy::and(Arc::clone(&left), Arc::clone(&right));
        let program = policy0.satisfy(&satisfier).expect("satisfiable");
        let witness = to_witness(&program);
        assert_eq!(2, witness.len());

        for i in 0..2 {
            let preimage_bytes = witness[i].try_to_bytes().expect("to bytes");
            let witness_preimage =
                Preimage32::try_from(preimage_bytes.as_slice()).expect("to array");
            assert_eq!(preimages[i], &witness_preimage);
        }

        execute_successful(&program, &env);

        // Policy 1
        let policy1 = Policy::and(Arc::clone(&bad), Arc::clone(&right));
        assert!(policy1.satisfy(&satisfier).is_err());

        // Policy 2

        let policy2 = Policy::and(Arc::clone(&left), Arc::clone(&bad));
        assert!(policy2.satisfy(&satisfier).is_err());
    }

    #[test]
    fn satisfy_or() {
        let env = ElementsEnv::dummy_with(PackedLockTime::ZERO, Sequence::from_consensus(42));
        let satisfier = get_satisfier(&env);

        let images: Vec<_> = satisfier.preimages.keys().copied().collect();
        let preimages: Vec<_> = images
            .iter()
            .map(|x| satisfier.preimages.get(x).unwrap())
            .collect();

        let assert_branch = |policy: &Policy<PubKey>, bit: bool| {
            let program = policy.satisfy(&satisfier).expect("satisfiable");
            let witness = to_witness(&program);
            assert_eq!(2, witness.len());

            assert_eq!(Value::u1(bit.into()), **witness[0]);
            let preimage_bytes = witness[1].try_to_bytes().expect("to bytes");
            let witness_preimage =
                Preimage32::try_from(preimage_bytes.as_slice()).expect("to array");
            assert_eq!(preimages[usize::from(bit)], &witness_preimage);

            execute_successful(&program, &env);
        };

        // Policy 0
        let bad = Arc::new(Policy::sha256(sha256::Hash::from_inner([0; 32])));
        let left = Arc::new(Policy::sha256(images[0]));
        let right = Arc::new(Policy::sha256(images[1]));

        let policy0 = Policy::or(Arc::clone(&left), Arc::clone(&right));
        assert_branch(&policy0, true);

        // Policy 1
        let policy1 = Policy::or(Arc::clone(&bad), Arc::clone(&right));
        assert_branch(&policy1, true);

        // Policy 2
        let policy2 = Policy::or(Arc::clone(&left), Arc::clone(&bad));
        assert_branch(&policy2, false);

        // Policy 3
        let policy3 = Policy::or(Arc::clone(&bad), Arc::clone(&bad));
        assert!(policy3.satisfy(&satisfier).is_err());
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

        let assert_branches = |policy: &Policy<PubKey>, bits: &[bool]| {
            let program = policy.satisfy(&satisfier).expect("satisfiable");
            let witness = to_witness(&program);
            assert_eq!(
                witness.len(),
                bits.len() + bits.iter().filter(|b| **b).count(),
            );

            let mut witidx = 0;
            for (bit_n, bit) in bits.iter().copied().enumerate() {
                assert_eq!(**witness[witidx], Value::u1(bit.into()));
                witidx += 1;
                if bit {
                    let preimage_bytes = witness[witidx].try_to_bytes().expect("to bytes");
                    let witness_preimage =
                        Preimage32::try_from(preimage_bytes.as_slice()).expect("to array");
                    assert_eq!(preimages[bit_n], &witness_preimage);
                    witidx += 1;
                }
            }
        };

        let image_from_bit = |bit: bool, j: u8| {
            if bit {
                images[j as usize]
            } else {
                sha256::Hash::from_inner([j; 32])
            }
        };

        for &bit0 in &[true, false] {
            let image0 = image_from_bit(bit0, 0);

            for &bit1 in &[true, false] {
                let image1 = image_from_bit(bit1, 1);

                for &bit2 in &[true, false] {
                    let image2 = image_from_bit(bit2, 2);

                    let policy = Policy::threshold(
                        2,
                        vec![
                            Policy::sha256(image0),
                            Policy::sha256(image1),
                            Policy::sha256(image2),
                        ],
                    );

                    match bit0 as u8 + bit1 as u8 + bit2 as u8 {
                        3 => assert_branches(&policy, &[bit0, bit1, false]),
                        2 => assert_branches(&policy, &[bit0, bit1, bit2]),
                        _ => assert!(policy.satisfy(&satisfier).is_err()),
                    }
                }
            }
        }
    }
}
