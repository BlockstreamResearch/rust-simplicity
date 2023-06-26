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

use crate::jet::Elements;
use crate::miniscript::ToPublicKey;
use crate::node::ConstructNode;
use std::sync::Arc;

impl<Pk: ToPublicKey> Policy<Pk> {
    /// Compile the policy into a Simplicity program
    pub fn compile(&self) -> Arc<ConstructNode<Elements>> {
        self.serialize_no_witness()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::jet::elements::ElementsEnv;
    use crate::node::SimpleFinalizer;
    use crate::{BitMachine, FailEntropy, Value};
    use bitcoin_hashes::{sha256, Hash};
    use elements::{bitcoin, secp256k1_zkp};
    use std::sync::Arc;

    fn compile(
        policy: Policy<bitcoin::XOnlyPublicKey>,
    ) -> (Arc<ConstructNode<Elements>>, ElementsEnv) {
        let commit = policy.compile();
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
