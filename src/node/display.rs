use std::fmt;
use std::sync::OnceLock;

use crate::dag::{Dag, DagLike, InternalSharing, MaxSharing, NoSharing};
use crate::encode;
use crate::node::{Inner, Marker, Node};
use crate::BitWriter;

/// Convenience structure for displaying a Simplictiy expression with its
/// witness.
pub struct Display<'n, M: Marker> {
    node: &'n Node<M>,
    #[cfg_attr(not(feature = "base64"), allow(dead_code))]
    prog_bytes: OnceLock<Vec<u8>>,
    wit_bytes: OnceLock<Vec<u8>>,
}

impl<'n, M: Marker> From<&'n Node<M>> for Display<'n, M> {
    fn from(node: &'n Node<M>) -> Self {
        // Because of Rust's lack of specialization we cannot cache the witness data
        // until we're in a function which is gated on `M: Marker<Witness = Value>`.
        // So we use `OnceLock` for that.
        //
        // While we're at it, use `OnceLock` for the program bytes, since maybe the
        // user doesn't want the program data (or can't use it due to lack of base64).
        Self {
            node,
            prog_bytes: OnceLock::new(),
            wit_bytes: OnceLock::new(),
        }
    }
}

impl<'n, M: Marker> Display<'n, M> {
    /// Display the program in base64.
    #[cfg(feature = "base64")]
    pub fn program(&self) -> impl fmt::Display + '_ {
        use crate::base64::display::Base64Display;
        use crate::base64::engine::general_purpose;

        let prog_bytes = self
            .prog_bytes
            .get_or_init(|| self.node.to_vec_without_witness());
        Base64Display::new(prog_bytes, &general_purpose::STANDARD)
    }
}

impl<'n, M: Marker<Witness = crate::Value>> Display<'n, M> {
    /// Display the witness data in hex.
    pub fn witness(&self) -> impl fmt::Display + '_ {
        use crate::hex::DisplayHex;

        let wit_bytes = self.wit_bytes.get_or_init(|| {
            let mut wit_v = vec![];
            let mut witness = BitWriter::new(&mut wit_v);
            let sharing_iter = self.node.post_order_iter::<MaxSharing<M>>();

            encode::encode_witness(sharing_iter.into_witnesses(), &mut witness)
                .expect("Vec::write is infallible");
            witness.flush_all().expect("Vec::write is infallible");

            wit_v
        });

        wit_bytes.as_hex()
    }
}

/// Display a Simplicity expression as a linear string.
///
/// The linear string has no sharing and may be **exponentially larger**
/// than the underlying shared expression.
///
/// There are some basic transformations to increase readability:
///
/// ## Infix notation
///
/// `pair s t` → `s & t`
///
/// `comp s t` → `s; t`
///
/// ## Booleans
///
/// `injl unit` → `false`
///
/// `injr unit` → `true`
///
/// ## Selectors
///
/// `take drop iden` → `OIH`
///
/// Sequences of `take` and `drop` that end in `iden` are transformed as follows:
///
/// `take` → `O` (looks like zero)
///
/// `drop` → `I` (looks like one)
///
/// `iden` → `H`
pub struct DisplayExpr<'a, M: Marker>(&'a Node<M>);

impl<'a, M: Marker> From<&'a Node<M>> for DisplayExpr<'a, M> {
    fn from(node: &'a Node<M>) -> Self {
        Self(node)
    }
}

impl<'a, M: Marker> fmt::Display for DisplayExpr<'a, M>
where
    &'a Node<M>: DagLike,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for data in self.0.verbose_pre_order_iter::<NoSharing>(None) {
            match data.n_children_yielded {
                1 => match data.node.inner() {
                    Inner::Comp(..) => f.write_str("; ")?,
                    Inner::Pair(..) => f.write_str(" & ")?,
                    Inner::Case(..) => f.write_str(") (")?,
                    x => debug_assert!(matches!(x.as_dag(), Dag::Unary(..))),
                },
                2 => {
                    debug_assert!(matches!(data.node.inner().as_dag(), Dag::Binary(..)));
                    if data.parent.is_some() {
                        f.write_str(")")?;
                    } else {
                        // Omit parentheses for root node
                    }
                }
                n => {
                    debug_assert!(n == 0, "Combinators are nullary, unary or binary");
                    match data.node.inner() {
                        Inner::Iden
                            if matches!(
                                data.parent.map(Node::inner),
                                Some(Inner::Take(..) | Inner::Drop(..))
                            ) =>
                        {
                            f.write_str("H")?
                        }
                        Inner::Take(child) if is_take_drop_iden(child) => f.write_str("O")?,
                        Inner::Drop(child) if is_take_drop_iden(child) => f.write_str("I")?,
                        Inner::Iden => f.write_str("iden")?,
                        Inner::Take(..) => f.write_str("take ")?,
                        Inner::Drop(..) => f.write_str("drop ")?,
                        Inner::Unit
                            if matches!(
                                data.parent.map(Node::inner),
                                Some(Inner::InjL(..) | Inner::InjR(..))
                            ) => {} // skip unit inside false | true
                        Inner::InjL(child) if matches!(child.inner(), Inner::Unit) => {
                            f.write_str("false")?
                        }
                        Inner::InjR(child) if matches!(child.inner(), Inner::Unit) => {
                            f.write_str("true")?
                        }
                        Inner::Unit => f.write_str("unit")?,
                        Inner::InjL(..) => f.write_str("injl ")?,
                        Inner::InjR(..) => f.write_str("injr ")?,
                        Inner::Comp(..) | Inner::Pair(..) => {} // display comp and pair as infix
                        Inner::Case(..) => f.write_str("case ")?,
                        Inner::AssertL(..) => f.write_str("assertl ")?,
                        Inner::AssertR(..) => f.write_str("assertr ")?,
                        Inner::Disconnect(..) => f.write_str("disconnect ")?,
                        Inner::Witness(..) => f.write_str("witness ")?,
                        Inner::Fail(..) => f.write_str("fail")?,
                        Inner::Jet(jet) => write!(f, "jet_{jet} ")?,
                        Inner::Word(value) => write!(f, "const {value} ")?,
                    }

                    match data.node.inner().as_dag() {
                        Dag::Binary(..) if data.parent.is_some() => f.write_str("(")?,
                        _ => {} // Omit parentheses for root node
                    }
                }
            };
        }

        Ok(())
    }
}

fn is_take_drop_iden<M: Marker>(node: &Node<M>) -> bool {
    for node in node.pre_order_iter::<InternalSharing>() {
        match node.inner() {
            Inner::Take(..) | Inner::Drop(..) | Inner::Iden => {}
            _ => return false,
        }
    }
    true
}

impl<'a, M: Marker> fmt::Debug for DisplayExpr<'a, M>
where
    &'a Node<M>: DagLike,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(self, f)
    }
}

#[cfg(test)]
mod tests {
    use crate::human_encoding::Forest;
    use crate::jet::Core;
    use crate::types;
    use crate::RedeemNode;
    use std::collections::HashMap;
    use std::sync::Arc;

    fn parse_program(s: &str) -> Arc<RedeemNode<Core>> {
        let ctx = types::Context::new();
        let empty_witness = HashMap::new();
        Forest::<Core>::parse(s)
            .unwrap()
            .to_witness_node(&ctx, &empty_witness)
            .unwrap()
            .finalize_unpruned()
            .unwrap()
    }

    #[test]
    fn display_boolean() {
        let s = "
            false := injl unit
            true := injr unit
            main := comp pair false true unit";
        let program = parse_program(s);
        assert_eq!("(false & true); unit", program.display_expr().to_string())
    }

    #[test]
    fn display_oih() {
        let s = "
            oih := take drop iden
            input := pair (pair unit unit) unit
            output := unit
            main := comp input (comp (pair oih (take unit)) output)";
        let program = parse_program(s);
        assert_eq!(
            "((unit & unit) & unit); ((OIH & take unit); unit)",
            program.display_expr().to_string()
        )
    }
}
