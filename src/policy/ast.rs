// Simplicity Policy
// Written in 2020 by
//     Andrew Poelstra <apoelstra@wpsoftware.net>
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

//! # Abstract Policies
//!
//! These policies represent spending conditions in the most simplest form
//! Policies are combination of `and`, `or` and `thresh` fragments. For example,
//! or(pk(A),pk(B)) represents a policy that can be spend with either pk(A) or pk(B).
//! These policies can be compiled to Simplicity and also be lifted back up from
//! Simplicity expressions to policy.

use std::hash::Hash;
use std::{fmt, str};

use bitcoin_hashes::hex::FromHex;
use bitcoin_hashes::sha256;

use miniscript::expression;
use miniscript::Error as msError;
use miniscript::MiniscriptKey;

use crate::extension::Jet;

use crate::core::term::UnTypedProg;
use crate::Error;
use crate::To32BytePubKey;

use super::compiler;

/// Abstract policy which corresponds to the semantics of a Simplicity
/// and which allows complex forms of analysis.

// FIXME: Which hashes to support?
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Policy<Pk: MiniscriptKey> {
    /// Unsatisfiable
    Unsatisfiable,
    /// Trivially satisfiable
    Trivial,
    /// Signature and public key matching a given hash is required
    Key(Pk),
    /// A relative locktime restriction
    After(u32),
    /// An absolute locktime restriction
    Older(u32),
    /// A SHA256 whose preimage must be provided to satisfy the descriptor
    Sha256(sha256::Hash),
    /// A list of sub-policies, all of which must be satisfied
    And(Vec<Policy<Pk>>),
    /// A list of sub-policies, one of which must be satisfied
    Or(Vec<Policy<Pk>>),
    /// A set of sub-policies, satisfactions must be provided for `k` of them
    Threshold(usize, Vec<Policy<Pk>>),
}
impl<Pk: MiniscriptKey + To32BytePubKey> Policy<Pk> {
    /// Compile a policy into a simplicity frgament
    pub fn compile<Ext: Jet + Hash + Clone>(&self) -> Result<UnTypedProg<(), Ext>, Error> {
        let dag = compiler::compile(&self)?;
        Ok(dag.into_untyped_prog())
    }
}

impl<Pk: MiniscriptKey> Policy<Pk> {
    /// Convert a policy using one kind of public key to another
    /// type of public key
    pub fn translate<Fpk, Q, E>(&self, mut translatefpk: Fpk) -> Result<Policy<Q>, E>
    where
        Fpk: FnMut(&Pk) -> Result<Q, E>,
        Q: MiniscriptKey,
    {
        match *self {
            Policy::Unsatisfiable => Ok(Policy::Unsatisfiable),
            Policy::Trivial => Ok(Policy::Trivial),
            Policy::Key(ref pk) => translatefpk(pk).map(Policy::Key),
            Policy::Sha256(ref h) => Ok(Policy::Sha256(*h)),
            Policy::After(n) => Ok(Policy::After(n)),
            Policy::Older(n) => Ok(Policy::Older(n)),
            Policy::Threshold(k, ref subs) => {
                let new_subs: Result<Vec<Policy<Q>>, _> = subs
                    .iter()
                    .map(|sub| sub.translate(&mut translatefpk))
                    .collect();
                new_subs.map(|ok| Policy::Threshold(k, ok))
            }
            Policy::And(ref subs) => Ok(Policy::And(
                subs.iter()
                    .map(|sub| sub.translate(&mut translatefpk))
                    .collect::<Result<Vec<Policy<Q>>, E>>()?,
            )),
            Policy::Or(ref subs) => Ok(Policy::Or(
                subs.iter()
                    .map(|sub| sub.translate(&mut translatefpk))
                    .collect::<Result<Vec<Policy<Q>>, E>>()?,
            )),
        }
    }
}

impl<Pk: MiniscriptKey> fmt::Debug for Policy<Pk> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Policy::Unsatisfiable => f.write_str("UNSATISFIABLE()"),
            Policy::Trivial => f.write_str("TRIVIAL()"),
            Policy::Key(ref pk) => write!(f, "pk({:?})", pk),
            Policy::After(n) => write!(f, "after({})", n),
            Policy::Older(n) => write!(f, "older({})", n),
            Policy::Sha256(h) => write!(f, "sha256({})", h),
            Policy::And(ref subs) => {
                f.write_str("and(")?;
                if !subs.is_empty() {
                    write!(f, "{:?}", subs[0])?;
                    for sub in &subs[1..] {
                        write!(f, ",{:?}", sub)?;
                    }
                }
                f.write_str(")")
            }
            Policy::Or(ref subs) => {
                f.write_str("or(")?;
                if !subs.is_empty() {
                    write!(f, "{:?}", subs[0])?;
                    for sub in &subs[1..] {
                        write!(f, ",{:?}", sub)?;
                    }
                }
                f.write_str(")")
            }
            Policy::Threshold(k, ref subs) => {
                write!(f, "thresh({}", k)?;
                for sub in subs {
                    write!(f, ",{:?}", sub)?;
                }
                f.write_str(")")
            }
        }
    }
}

impl<Pk: MiniscriptKey> fmt::Display for Policy<Pk> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Policy::Unsatisfiable => f.write_str("UNSATISFIABLE"),
            Policy::Trivial => f.write_str("TRIVIAL"),
            Policy::Key(ref pk) => write!(f, "pk({})", pk),
            Policy::After(n) => write!(f, "after({})", n),
            Policy::Older(n) => write!(f, "older({})", n),
            Policy::Sha256(h) => write!(f, "sha256({})", h),
            Policy::And(ref subs) => {
                f.write_str("and(")?;
                if !subs.is_empty() {
                    write!(f, "{}", subs[0])?;
                    for sub in &subs[1..] {
                        write!(f, ",{}", sub)?;
                    }
                }
                f.write_str(")")
            }
            Policy::Or(ref subs) => {
                f.write_str("or(")?;
                if !subs.is_empty() {
                    write!(f, "{}", subs[0])?;
                    for sub in &subs[1..] {
                        write!(f, ",{}", sub)?;
                    }
                }
                f.write_str(")")
            }
            Policy::Threshold(k, ref subs) => {
                write!(f, "thresh({}", k)?;
                for sub in subs {
                    write!(f, ",{}", sub)?;
                }
                f.write_str(")")
            }
        }
    }
}

impl<Pk> str::FromStr for Policy<Pk>
where
    Pk: MiniscriptKey,
    <Pk as str::FromStr>::Err: ToString,
    <<Pk as MiniscriptKey>::Hash as str::FromStr>::Err: ToString,
{
    type Err = miniscript::Error;

    fn from_str(s: &str) -> Result<Policy<Pk>, miniscript::Error> {
        for ch in s.as_bytes() {
            if *ch < 20 || *ch > 127 {
                return Err(miniscript::Error::Unprintable(*ch));
            }
        }

        let tree = expression::Tree::from_str(s)?;
        miniscript::expression::FromTree::from_tree(&tree)
    }
}

// FIXME: export macro from miniscript, avoid code repeatation
// miniscript::serde_string_impl_pk!(Policy, "a simplicity abstract policy");

// FIXME: Make a generic module for parsing recusrive structure with it's own error type.
impl<Pk> expression::FromTree for Policy<Pk>
where
    Pk: MiniscriptKey,
    <Pk as str::FromStr>::Err: ToString,
{
    fn from_tree(top: &expression::Tree) -> Result<Policy<Pk>, msError> {
        use miniscript::policy::concrete::PolicyError as MsPolicyError;
        match (top.name, top.args.len() as u32) {
            ("UNSATISFIABLE", 0) => Ok(Policy::Unsatisfiable),
            ("TRIVIAL", 0) => Ok(Policy::Trivial),
            ("pk", 1) => expression::terminal(&top.args[0], |pk| Pk::from_str(pk).map(Policy::Key)),
            ("after", 1) => expression::terminal(&top.args[0], |x| {
                expression::parse_num(x).map(Policy::After)
            }),
            ("older", 1) => expression::terminal(&top.args[0], |x| {
                expression::parse_num(x).map(Policy::Older)
            }),
            ("sha256", 1) => expression::terminal(&top.args[0], |x| {
                sha256::Hash::from_hex(x).map(Policy::Sha256)
            }),
            ("and", _) => {
                if top.args.len() != 2 {
                    return Err(msError::PolicyError(MsPolicyError::NonBinaryArgAnd));
                }
                let mut subs = Vec::with_capacity(top.args.len());
                for arg in &top.args {
                    subs.push(Policy::from_tree(arg)?);
                }
                Ok(Policy::And(subs))
            }
            ("or", _) => {
                if top.args.len() != 2 {
                    return Err(msError::PolicyError(MsPolicyError::NonBinaryArgOr));
                }
                let mut subs = Vec::with_capacity(top.args.len());
                for arg in &top.args {
                    subs.push(Policy::from_tree(arg)?);
                }
                Ok(Policy::Or(subs))
            }
            ("thresh", nsubs) => {
                if nsubs == 0 {
                    return Err(msError::Unexpected("thresh without args".to_owned()));
                }
                if !top.args[0].args.is_empty() {
                    return Err(msError::Unexpected(top.args[0].args[0].name.to_owned()));
                }

                let thresh = expression::parse_num(top.args[0].name)?;
                if thresh >= nsubs {
                    return Err(msError::Unexpected(top.args[0].name.to_owned()));
                }

                let mut subs = Vec::with_capacity(top.args.len() - 1);
                for arg in &top.args[1..] {
                    subs.push(Policy::from_tree(arg)?);
                }
                Ok(Policy::Threshold(thresh as usize, subs))
            }
            _ => Err(msError::Unexpected(top.name.to_owned())),
        }
    }
}

impl<Pk: MiniscriptKey> Policy<Pk> {
    /// Flatten out trees of `And`s and `Or`s; eliminate `Trivial` and
    /// `Unsatisfiable`s. Does not reorder any branches; use `.sort`.
    pub fn normalized(self) -> Policy<Pk> {
        match self {
            Policy::And(subs) => {
                let mut ret_subs = Vec::with_capacity(subs.len());
                for sub in subs {
                    match sub.normalized() {
                        Policy::Trivial => {}
                        Policy::Unsatisfiable => return Policy::Unsatisfiable,
                        Policy::And(and_subs) => ret_subs.extend(and_subs),
                        x => ret_subs.push(x),
                    }
                }
                match ret_subs.len() {
                    0 => Policy::Trivial,
                    1 => ret_subs.pop().unwrap(),
                    _ => Policy::And(ret_subs),
                }
            }
            Policy::Or(subs) => {
                let mut ret_subs = Vec::with_capacity(subs.len());
                for sub in subs {
                    match sub {
                        Policy::Trivial => return Policy::Trivial,
                        Policy::Unsatisfiable => {}
                        Policy::Or(or_subs) => ret_subs.extend(or_subs),
                        x => ret_subs.push(x),
                    }
                }
                match ret_subs.len() {
                    0 => Policy::Trivial,
                    1 => ret_subs.pop().unwrap(),
                    _ => Policy::Or(ret_subs),
                }
            }
            x => x,
        }
    }
}

impl<Pk: MiniscriptKey> Policy<Pk> {
    /// "Sort" a policy to bring it into a canonical form to allow comparisons.
    /// Does **not** allow policies to be compared for functional equivalence;
    /// in general this appears to require Gröbner basis techniques that are not
    /// implemented.
    pub fn sorted(self) -> Policy<Pk> {
        match self {
            Policy::And(subs) => {
                let mut new_subs: Vec<_> = subs.into_iter().map(Policy::sorted).collect();
                new_subs.sort();
                Policy::And(new_subs)
            }
            Policy::Or(subs) => {
                let mut new_subs: Vec<_> = subs.into_iter().map(Policy::sorted).collect();
                new_subs.sort();
                Policy::Or(new_subs)
            }
            Policy::Threshold(k, subs) => {
                let mut new_subs: Vec<_> = subs.into_iter().map(Policy::sorted).collect();
                new_subs.sort();
                Policy::Threshold(k, new_subs)
            }
            x => x,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::bititer::BitIter;
    use crate::exec;
    use crate::extension::dummy::{DummyNode, TxEnv};
    use crate::program::Program;
    use crate::DummyKey;
    use crate::Value;
    use std::str::FromStr;

    fn compile_and_exec(pol: &str, witness: Vec<u8>) {
        // A single pk compilation
        let pol = Policy::<DummyKey>::from_str(pol).unwrap();
        let prog: UnTypedProg<_, DummyNode> = pol.compile().unwrap();

        let prog =
            Program::from_untyped_nodes(prog, &mut BitIter::from(witness.into_iter())).unwrap();
        // prog.graph_print();

        let mut mac = exec::BitMachine::for_program(&prog);
        let output = mac.exec(&prog, &TxEnv);

        assert!(output == Value::Unit);
    }

    #[test]
    fn basic_compile() {
        // A single pk compilation
        let mut witness = vec![0x80];
        // Partial witness is consumed as sig. Since all sigs verify as of now
        // we don't worry about the exact parsing of witness
        witness.extend(vec![0x34; 64]);
        compile_and_exec("pk()", witness);

        // and compilation. We need to set the witness len here, but since it is not
        // used in the current code, we are temparorily abusing it.
        let mut witness = vec![0x80];
        witness.extend(vec![0x34; 128]);
        compile_and_exec("and(pk(),pk())", witness);

        let mut witness = vec![0x80];
        witness.extend(vec![0x34; 128]);
        compile_and_exec("or(pk(),pk())", witness);

        let mut witness = vec![0x80];
        witness.extend(vec![0x34; 640]);
        compile_and_exec(
            "or(and(or(pk(),pk()),or(pk(),pk())),and(or(pk(),pk()),or(pk(),pk())))",
            witness,
        );
    }
}
