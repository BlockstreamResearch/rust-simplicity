use std::convert::{TryFrom, TryInto};
use std::str::FromStr;

use miniscript::Error as msError;
use miniscript::MiniscriptKey;
use miniscript::{expression, Miniscript, ScriptContext, Terminal};

use crate::policy::ast::Policy;
use crate::Error;

serde_string_impl_pk!(Policy, "a Simplicity policy");

impl<Pk> FromStr for Policy<Pk>
where
    Pk: MiniscriptKey + FromStr,
    Pk::Sha256: FromStr,
    <Pk as FromStr>::Err: ToString,
    <Pk::Sha256 as FromStr>::Err: ToString,
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
    Pk::Sha256: FromStr,
    <Pk as FromStr>::Err: ToString,
    <Pk::Sha256 as FromStr>::Err: ToString,
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
                Pk::Sha256::from_str(x).map(Policy::Sha256)
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
                if nsubs < 3 {
                    return Err(msError::Unexpected(
                        "thresh must have a threshold value and at least 2 children".to_owned(),
                    ));
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

impl<'a, Pk: MiniscriptKey, Ctx: ScriptContext> TryFrom<&'a Miniscript<Pk, Ctx>> for Policy<Pk> {
    type Error = Error;

    fn try_from(top: &Miniscript<Pk, Ctx>) -> Result<Self, Self::Error> {
        match &top.node {
            Terminal::True => Ok(Policy::Trivial),
            Terminal::False => Ok(Policy::Unsatisfiable),
            Terminal::PkK(key) => Ok(Policy::Key(key.clone())),
            Terminal::PkH(_) | Terminal::RawPkH(_) => {
                Err(Error::ParseError("Public key hashes are not supported"))
            }
            Terminal::After(n) => Ok(Policy::After(n.0)),
            Terminal::Older(n) => Ok(Policy::Older(n.0)),
            Terminal::Sha256(h) => Ok(Policy::Sha256(h.clone())),
            Terminal::Hash256(_h) => Err(Error::ParseError("SHA256d is not supported")),
            Terminal::Ripemd160(_) | Terminal::Hash160(_) => {
                Err(Error::ParseError("RIPEMD160 is not supported"))
            }
            Terminal::AndV(x, y) | Terminal::AndB(x, y) => {
                let inner = vec![x.as_ref().try_into()?, y.as_ref().try_into()?];
                Ok(Policy::And(inner))
            }
            Terminal::AndOr(x, y, z) => {
                let inner_and = vec![x.as_ref().try_into()?, y.as_ref().try_into()?];
                let and = Policy::And(inner_and);
                let inner_or = vec![and, z.as_ref().try_into()?];
                Ok(Policy::Or(inner_or))
            }
            Terminal::OrB(x, y)
            | Terminal::OrD(x, y)
            | Terminal::OrC(x, y)
            | Terminal::OrI(x, y) => {
                let inner = vec![x.as_ref().try_into()?, y.as_ref().try_into()?];
                Ok(Policy::Or(inner))
            }
            Terminal::Thresh(k, sub_policies) => {
                let mut translated_sub_policies = Vec::with_capacity(sub_policies.len());

                for sub in sub_policies {
                    translated_sub_policies.push(sub.as_ref().try_into()?);
                }

                Ok(Policy::Threshold(*k, translated_sub_policies))
            }
            Terminal::Alt(x)
            | Terminal::Swap(x)
            | Terminal::Check(x)
            | Terminal::DupIf(x)
            | Terminal::Verify(x)
            | Terminal::NonZero(x)
            | Terminal::ZeroNotEqual(x) => x.as_ref().try_into(),
            Terminal::Multi(_, _) | Terminal::MultiA(_, _) => {
                Err(Error::ParseError("Multisig is not supported"))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_bad_thresh() {
        assert_eq!(
            Policy::<String>::from_str("thresh()"),
            Err(msError::Unexpected(
                "thresh must have a threshold value and at least 2 children".to_string()
            )),
        );

        assert_eq!(
            Policy::<String>::from_str("thresh"),
            Err(msError::Unexpected("thresh without args".to_string())),
        );

        assert_eq!(
            Policy::<String>::from_str("thresh(0)"),
            Err(msError::Unexpected(
                "thresh must have a threshold value and at least 2 children".to_string()
            )),
        );

        assert_eq!(
            Policy::<String>::from_str("thresh(0,TRIVIAL)"),
            Err(msError::Unexpected(
                "thresh must have a threshold value and at least 2 children".to_string()
            )),
        );

        assert!(Policy::<String>::from_str("thresh(0,TRIVIAL,TRIVIAL)").is_ok());
        assert!(Policy::<String>::from_str("thresh(2,TRIVIAL,TRIVIAL)").is_ok());

        assert_eq!(
            Policy::<String>::from_str("thresh(3,TRIVIAL,TRIVIAL)"),
            Err(msError::Unexpected("3".to_string())),
        );
    }
}
