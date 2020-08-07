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
use crate::bititer::BitIter;
use crate::extension;
use crate::miniscript::MiniscriptKey;
use crate::program::scribe;
use crate::Error;
use crate::Node;
use crate::To32BytePubKey;
use crate::Value;

use crate::core::types::pow2_types;

pub fn compile<Pk: MiniscriptKey + To32BytePubKey, Ext: extension::Node>(
    pol: &Policy<Pk>,
) -> Result<Vec<Node<(), Ext>>, Error> {
    let pk_ty = pow2_types()[9].clone();
    let frag = match pol {
        Policy::Unsatisfiable => unimplemented!(),
        Policy::Trivial => unimplemented!(),
        Policy::Key(ref pk) => {
            let mut nodes = vec![];
            let pk_value = Value::from_bits_and_type(
                &mut BitIter::from(pk.to_32_byte_pubkey().to_vec().into_iter()),
                &pk_ty,
            )?;
            nodes.extend(scribe(pk_value));
            nodes.push(Node::Witness(()));
            nodes.push(Node::Pair(2, 1));
            nodes.push(Node::Jet(extension::jets::Node::SchnorrAssert));
            nodes.push(Node::Comp(2, 1));
            nodes
        }
        Policy::Sha256(ref _h) => {
            unimplemented!()
            // use bitcoin_hashes::Hash;
            // use bitcoin_hashes::HashEngine;
            // let x = sha256::HashEngine::default();
            // let midstate = x.midstate().0;
            // let midstate_value =
            //     Value::from_bits_and_type(&mut BitIter::from(h.to_vec().into_iter()), &pk_ty)?;
            // let mut nodes = scribe(midstate_value);
            // nodes.push(Node::Witness(()));
            // nodes.push(Node::Pair(2, 1));
            // nodes.push(Node::Jet(extension::jets::Node::Sha256HashBlock));
            // nodes.push(Node::Comp(2, 1));
            // // Add the equality test here
            // let hash_value = Value::from_bits_and_type(
            //     &mut BitIter::from(h.into_inner().to_vec().into_iter()),
            //     &pk_ty,
            // )?;
            // let nodes2 = scribe(hash_value);
            // let li = nodes2.len() + 1;
            // // Check eq256 here
            // nodes
        }
        Policy::After(_n) => unimplemented!(),
        Policy::Older(_n) => unimplemented!(),
        Policy::Threshold(_k, ref _subs) => unimplemented!(),
        Policy::And(ref subs) => {
            assert!(subs.len() == 2);
            let mut nodes = compile(&subs[0])?;
            let r_nodes = compile(&subs[1])?;
            let li = r_nodes.len() + 1;
            nodes.extend(r_nodes);
            nodes.push(Node::Comp(li, 1));
            nodes
        }
        Policy::Or(ref subs) => {
            assert!(subs.len() == 2);
            let mut nodes = compile(&subs[0])?;
            let r_nodes = compile(&subs[1])?;
            let li = r_nodes.len() + 1;
            nodes.extend(r_nodes);
            nodes.push(Node::Case(li, 1));
            nodes.push(Node::Witness(()));
            nodes.push(Node::Comp(1, 2));
            nodes
        }
    };
    Ok(frag)
}
