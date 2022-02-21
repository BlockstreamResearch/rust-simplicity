// Simplicity lifting to miniscript semantic representation
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

//! # Lift simplicity programs into miniscript semantic policies
//! Not all simplicity programs can be lifted to semantic langauge.
//! Currently the policy compilation is one to one mapping
//! between policy fragment and a simplicity program.

use crate::core::term_dag::TermDag;
use crate::extension::bitcoin::BtcNode;
use crate::extension::jets::JetsNode::{EqV256, LessThanV32, SchnorrAssert, Sha256};
use crate::util::slice_to_u32_be;
use crate::PubkeyKey32;
use crate::Value;

use bitcoin_hashes::{sha256, Hash};
use miniscript::policy::Liftable;
use miniscript::policy::Semantic;
use miniscript::{DummyKey, MiniscriptKey};
use std::rc::Rc;

/// Functional opposite of scribe. Read the scribed value
/// by interpretting that as constant function and return
/// a value corresponding to it.
pub fn read_scribed_value<Witness, Ext>(dag: Rc<TermDag<Witness, Ext>>) -> Value {
    match dag.as_ref() {
        TermDag::Unit => Value::Unit,
        TermDag::InjL(l) => Value::sum_l(read_scribed_value(Rc::clone(l))),
        TermDag::InjR(r) => Value::sum_r(read_scribed_value(Rc::clone(r))),
        TermDag::Pair(l, r) => Value::prod(
            read_scribed_value(Rc::clone(l)),
            read_scribed_value(Rc::clone(r)),
        ),
        // Fixme: Change to errors
        _ => unreachable!(),
    }
}

// FIXME: Wait for 32 byte pubkeys to be added to rust-bitcoin.
// Then, we can add implementations that depend on bitcoin::PublicKey
impl<Witness> Liftable<DummyKey> for TermDag<Witness, BtcNode>
where
    Witness: Eq,
{
    // Lift a simplicity program into a semantic policy
    fn lift(&self) -> Semantic<DummyKey> {
        match self {
            TermDag::Unit => Semantic::Trivial,
            TermDag::Comp(l, r) => {
                // check for Key
                match (&**l, &**r) {
                    (TermDag::Pair(key, w), TermDag::Jet(SchnorrAssert)) => {
                        let key_value = read_scribed_value(Rc::clone(&Rc::clone(key)));
                        let key_bytes = key_value.try_to_bytes().unwrap();
                        let k = DummyKey::from_32_byte_pubkey(&key_bytes);
                        match &**w {
                            TermDag::Witness(..) => Semantic::KeyHash(k.to_pubkeyhash()),
                            _ => unimplemented!(),
                        }
                    }
                    (TermDag::Pair(scribed_hash, computed_hash), TermDag::Jet(EqV256)) => {
                        let hash_value = read_scribed_value(Rc::clone(&Rc::clone(scribed_hash)));
                        let hash_bytes = hash_value.try_to_bytes().unwrap();
                        let h = sha256::Hash::from_slice(&hash_bytes).unwrap();
                        match &**computed_hash {
                            TermDag::Pair(w, sha_jet) => match (&**w, &**sha_jet) {
                                (TermDag::Witness(..), TermDag::Jet(Sha256)) => Semantic::Sha256(h),
                                _ => unimplemented!(),
                            },
                            _ => unimplemented!(),
                        }
                    }
                    (TermDag::Pair(scibe_t, computed_t), TermDag::Jet(LessThanV32)) => {
                        let timelock_value = read_scribed_value(Rc::clone(&Rc::clone(scibe_t)));
                        let timelock_bytes = timelock_value.try_to_bytes().unwrap();
                        let t = slice_to_u32_be(&timelock_bytes);
                        match &**computed_t {
                            TermDag::Ext(BtcNode::LockTime) => Semantic::After(t),
                            TermDag::Ext(BtcNode::CurrentSequence) => Semantic::After(t),
                            _ => unimplemented!(),
                        }
                    }
                    _ => unimplemented!(),
                }
            }
            _ => unimplemented!(),
        }
    }
}
