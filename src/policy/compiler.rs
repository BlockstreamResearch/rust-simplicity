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
use crate::core::TermDag;
use crate::core::Value;
use crate::jet;
use crate::jet::application::Bitcoin;
use crate::jet::Application;
use crate::miniscript::MiniscriptKey;
use crate::policy::key::PublicKey32;
use crate::Error;
use std::rc::Rc;

/// Convert a single bit into u2 by pre-padding zeros
fn u1_to_u2<App: Application>(s: Rc<TermDag<(), App>>) -> Rc<TermDag<(), App>> {
    TermDag::pair(TermDag::scribe(&Value::u1(0)), s)
}

/// Convert a single bit into u4 by pre-padding zeros
fn u1_to_u4<App: Application>(s: Rc<TermDag<(), App>>) -> Rc<TermDag<(), App>> {
    TermDag::pair(TermDag::scribe(&Value::u2(0)), u1_to_u2(s))
}

/// Convert a single bit into u8 by pre-padding zeros
fn u1_to_u8<App: Application>(s: Rc<TermDag<(), App>>) -> Rc<TermDag<(), App>> {
    TermDag::pair(TermDag::scribe(&Value::u4(0)), u1_to_u4(s))
}

/// Convert a single bit into u16 by pre-padding zeros
fn u1_to_u16<App: Application>(s: Rc<TermDag<(), App>>) -> Rc<TermDag<(), App>> {
    TermDag::pair(TermDag::scribe(&Value::u8(0)), u1_to_u8(s))
}

/// Convert a single bit into u32 by pre-padding zeros
fn u1_to_u32<App: Application>(s: Rc<TermDag<(), App>>) -> Rc<TermDag<(), App>> {
    TermDag::pair(TermDag::scribe(&Value::u16(0)), u1_to_u16(s))
}

/// Compile the desired policy into a bitcoin simplicity program
pub fn compile<Pk: MiniscriptKey + PublicKey32>(
    pol: &Policy<Pk>,
) -> Result<Rc<TermDag<(), Bitcoin>>, Error> {
    let frag = match pol {
        Policy::Unsatisfiable => unimplemented!(), //lookup  fail
        Policy::Trivial => TermDag::unit(),
        Policy::Key(ref pk) => {
            let pk_value = Value::u256_from_slice(&pk.to_32_bytes());
            let scribe_pk = TermDag::scribe(&pk_value);
            let pk_sig_pair = TermDag::pair(scribe_pk, TermDag::witness(()));
            TermDag::comp(pk_sig_pair, TermDag::jet(&jet::bitcoin::BIP_0340_VERIFY))
        }
        Policy::Sha256(ref h) => {
            let hash_value = Value::u256_from_slice(h);
            let scribe_hash = TermDag::scribe(&hash_value);
            let computed_hash =
                TermDag::comp(TermDag::witness(()), TermDag::jet(&jet::bitcoin::SHA256));
            let pair = TermDag::pair(scribe_hash, computed_hash);
            TermDag::comp(pair, TermDag::jet(&jet::bitcoin::EQ256_VERIFY))
        }
        Policy::After(n) => {
            let n_value = Value::u32(*n);
            let scribe_n = TermDag::scribe(&n_value);
            let cltv = TermDag::jet(&jet::bitcoin::LOCK_TIME);
            let pair = TermDag::pair(scribe_n, cltv);
            TermDag::comp(pair, TermDag::jet(&jet::bitcoin::LT32_VERIFY))
        }
        Policy::Older(n) => {
            let n_value = Value::u32(*n);
            let scribe_n = TermDag::scribe(&n_value);
            let csv = TermDag::jet(&jet::bitcoin::CURRENT_SEQUENCE);
            let pair = TermDag::pair(scribe_n, csv);
            TermDag::comp(pair, TermDag::jet(&jet::bitcoin::LT32_VERIFY))
        }
        Policy::Threshold(k, ref subs) => {
            assert!(subs.len() >= 2, "Threshold must have numbre of subs >=2");
            let child = compile(&subs[0])?;
            // selector denotes a bit that specifies whether the first child should be executed.
            let selector = TermDag::witness(());
            // The case condition that for the current child
            let case_term = TermDag::cond(child, TermDag::unit());
            let mut acc = TermDag::comp(selector.clone(), case_term);
            let mut sum = u1_to_u32(selector);
            for sub in &subs[1..] {
                let child = compile(sub)?;
                let selector = TermDag::witness(());
                let case_term = TermDag::cond(child, TermDag::unit());

                let curr_term = TermDag::comp(selector.clone(), case_term);
                let selector_u32 = u1_to_u32(selector);

                acc = TermDag::comp(acc, curr_term);
                let full_sum = TermDag::comp(
                    TermDag::pair(sum, selector_u32),
                    TermDag::jet(&jet::bitcoin::ADD32),
                );
                // Discard the overflow bit.
                // NOTE: This *assumes* that the threshold would be have 2**32 branches.
                // FIXME: enforce this in policy specification.
                sum = TermDag::drop(full_sum);
            }
            let scribe_k = TermDag::scribe(&Value::u32(*k as u32));
            TermDag::comp(
                TermDag::pair(scribe_k, sum),
                TermDag::jet(&jet::bitcoin::EQ32_VERIFY),
            )
        }
        Policy::And(ref subs) => {
            assert!(subs.len() == 2);
            let l = compile(&subs[0])?;
            let r = compile(&subs[1])?;
            TermDag::comp(l, r)
        }
        Policy::Or(ref subs) => {
            assert!(subs.len() == 2);
            let l = compile(&subs[0])?;
            let r = compile(&subs[1])?;
            let case_term = TermDag::cond(l, r);
            TermDag::comp(TermDag::witness(()), case_term)
        }
    };
    Ok(frag)
}
