use std::str::FromStr;

use bitcoin_hashes::hex::FromHex;
use bitcoin_hashes::sha256;
use miniscript::expression;
use miniscript::Error as msError;
use miniscript::MiniscriptKey;

use crate::policy::ast::Policy;

impl<Pk> FromStr for Policy<Pk>
where
    Pk: MiniscriptKey + FromStr,
    <Pk as FromStr>::Err: ToString,
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
    Pk: MiniscriptKey + FromStr,
    <Pk as FromStr>::Err: ToString,
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
