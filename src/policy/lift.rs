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

use crate::core::bitvec_to_bytevec;
use crate::core::term::DagTerm;
use crate::extension::bitcoin::BtcNode;
use crate::PubkeyKey32;
use crate::Value;
use bitcoin_hashes::{sha256, Hash};
use miniscript::policy::Liftable;
use miniscript::policy::Semantic;
use miniscript::{DummyKey, MiniscriptKey};

use crate::extension::jets::JetsNode::{EqV256, LessThanV32, SchnorrAssert, Sha256};

use std::rc::Rc;

/// Functional opposite of scribe. Read the scribed value
/// by interpretting that as constant function and return
/// a value corresponding to it.
pub fn read_scribed_value<Witness, Ext>(dag: Rc<DagTerm<Witness, Ext>>) -> Value {
    match dag.as_ref() {
        DagTerm::Unit => Value::Unit,
        DagTerm::InjL(l) => Value::sum_l(read_scribed_value(Rc::clone(l))),
        DagTerm::InjR(r) => Value::sum_r(read_scribed_value(Rc::clone(r))),
        DagTerm::Pair(l, r) => Value::prod(
            read_scribed_value(Rc::clone(l)),
            read_scribed_value(Rc::clone(r)),
        ),
        // Fixme: Change to errors
        _ => unreachable!(),
    }
}

// FIXME: Wait for 32 byte pubkeys to be added to rust-bitcoin.
// Then, we can add implementations that depend on bitcoin::PublicKey
impl<Witness> Liftable<DummyKey> for DagTerm<Witness, BtcNode>
where
    Witness: Eq,
{
    // Lift a simplicity program into a semantic policy
    fn lift(&self) -> Semantic<DummyKey> {
        match self {
            DagTerm::Unit => Semantic::Trivial,
            DagTerm::Comp(l, r) => {
                // check for Key
                match (&**l, &**r) {
                    (DagTerm::Pair(key, w), DagTerm::Jet(SchnorrAssert)) => {
                        let key_value = read_scribed_value(Rc::clone(&Rc::clone(key)));
                        let key_bytes = bitvec_to_bytevec(&key_value.into_bits());
                        let k = DummyKey::from_32_byte_pubkey(&key_bytes);
                        match &**w {
                            DagTerm::Witness(..) => Semantic::KeyHash(k.to_pubkeyhash()),
                            _ => unimplemented!(),
                        }
                    }
                    (DagTerm::Pair(scribed_hash, computed_hash), DagTerm::Jet(EqV256)) => {
                        let hash_value = read_scribed_value(Rc::clone(&Rc::clone(scribed_hash)));
                        let hash_bytes = bitvec_to_bytevec(&hash_value.into_bits());
                        let h = sha256::Hash::from_slice(&hash_bytes).unwrap();
                        match &**computed_hash {
                            DagTerm::Pair(w, sha_jet) => match (&**w, &**sha_jet) {
                                (DagTerm::Witness(..), DagTerm::Jet(Sha256)) => Semantic::Sha256(h),
                                _ => unimplemented!(),
                            },
                            _ => unimplemented!(),
                        }
                    }
                    (DagTerm::Pair(scibe_t, computed_t), DagTerm::Jet(LessThanV32)) => {
                        let timelock_value = read_scribed_value(Rc::clone(&Rc::clone(scibe_t)));
                        let timelock_bytes = bitvec_to_bytevec(&timelock_value.into_bits());
                        let t = u32_from_be_bytes(&timelock_bytes);
                        match &**computed_t {
                            DagTerm::Ext(BtcNode::LockTime) => Semantic::After(t),
                            DagTerm::Ext(BtcNode::CurrentSequence) => Semantic::After(t),
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

// utility function to convert 4 bytes [u8;4] to u32
// panics if bytes.len() !=4
fn u32_from_be_bytes(bytes: &[u8]) -> u32 {
    assert!(bytes.len() == 4);
    let mut ret: u32 = 0;
    for (i, byte) in bytes.iter().enumerate() {
        ret += (*byte as u32) * (1 << (24 - 8 * i));
    }
    ret
}
