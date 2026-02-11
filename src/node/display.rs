use std::fmt;
use std::sync::OnceLock;

use crate::dag::{
    Dag, DagLike, InternalSharing, MaxSharing, NoSharing, PostOrderIterItem, SharingTracker,
};
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

/// The output format for [`DisplayAsGraph`].
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GraphFormat {
    /// Graphviz DOT format, renderable with `dot -Tsvg` or similar tools.
    Dot,
    /// Mermaid diagram format, renderable in Markdown or the Mermaid live editor.
    Mermaid,
}

/// The node-sharing level for [`DisplayAsGraph`].
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SharingLevel {
    /// No sharing: every use of a node is visited separately (may be exponentially large).
    None,
    /// Internal sharing: nodes shared within the expression are visited once.
    Internal,
    /// Maximum sharing: maximize sharing across the entire expression.
    Max,
}

/// Display a Simplicity expression as a graph in a chosen format.
///
/// Construct via [`Node::display_as_dot`], [`Node::display_as_mermaid`], or
/// [`DisplayAsGraph::new`]. The [`fmt::Display`] impl renders using the stored
/// `format` and `sharing` fields; [`to_dot_string`](DisplayAsGraph::to_dot_string)
/// and [`to_mermaid_string`](DisplayAsGraph::to_mermaid_string) always render in
/// the named format using the stored sharing level.
pub struct DisplayAsGraph<'a, M: Marker> {
    node: &'a Node<M>,
    /// Output format (DOT or Mermaid).
    pub format: GraphFormat,
    /// Node-sharing level used when rendering.
    pub sharing: SharingLevel,
}

impl<'a, M: Marker> DisplayAsGraph<'a, M> {
    /// Create a new `DisplayAsGraph` with the given format and sharing level.
    pub fn new(node: &'a Node<M>, format: GraphFormat, sharing: SharingLevel) -> Self {
        Self {
            node,
            format,
            sharing,
        }
    }

    /// Render as a Graphviz DOT string using the stored sharing level.
    pub fn to_dot_string(&self) -> String
    where
        &'a Node<M>: DagLike,
    {
        let mut result = String::new();
        match self.render(GraphFormat::Dot, &mut result) {
            Ok(_) => result,
            Err(e) => format!("Could not display as string: {}", e),
        }
    }

    /// Render as a Mermaid string using the stored sharing level.
    pub fn to_mermaid_string(&self) -> String
    where
        &'a Node<M>: DagLike,
    {
        let mut result = String::new();
        match self.render(GraphFormat::Mermaid, &mut result) {
            Ok(_) => result,
            Err(e) => format!("Could not display as string: {}", e),
        }
    }

    fn render<W: fmt::Write>(&self, graph_format: GraphFormat, w: &mut W) -> fmt::Result
    where
        &'a Node<M>: DagLike,
    {
        match self.sharing {
            SharingLevel::None => self.render_with::<NoSharing, _>(graph_format, w),
            SharingLevel::Internal => self.render_with::<InternalSharing, _>(graph_format, w),
            SharingLevel::Max => self.render_with::<MaxSharing<M>, _>(graph_format, w),
        }
    }

    fn render_with<S, W>(&self, graph_format: GraphFormat, w: &mut W) -> fmt::Result
    where
        S: SharingTracker<&'a Node<M>> + Default,
        W: fmt::Write,
    {
        let node_label = |data: &PostOrderIterItem<&Node<M>>| -> String {
            match data.node.inner() {
                Inner::Witness(_) => format!("witness({})", data.index),
                Inner::Word(word) => format!("word({})", shorten(word.to_string(), 12)),
                _ => data.node.inner().to_string(),
            }
        };

        match graph_format {
            GraphFormat::Dot => {
                writeln!(w, "digraph G {{")?;
                writeln!(w, "ordering=\"out\";")?;
                for data in self.node.post_order_iter::<S>() {
                    writeln!(w, "  node{}[label=\"{}\"];", data.index, node_label(&data))?;
                    if let Some(left) = data.left_index {
                        writeln!(w, "  node{}->node{};", data.index, left)?;
                    }
                    if let Some(right) = data.right_index {
                        writeln!(w, "  node{}->node{};", data.index, right)?;
                    }
                }
                writeln!(w, "}}")?;
            }
            GraphFormat::Mermaid => {
                writeln!(w, "flowchart TD")?;
                for data in self.node.post_order_iter::<S>() {
                    match data.node.inner() {
                        Inner::Case(..) => {
                            writeln!(w, "  node{}{{\"{}\"}}", data.index, node_label(&data))?;
                        }
                        _ => {
                            writeln!(w, "  node{}[\"{}\"]", data.index, node_label(&data))?;
                        }
                    }

                    if let Some(left) = data.left_index {
                        writeln!(w, "  node{} --> node{}", data.index, left)?;
                    }
                    if let Some(right) = data.right_index {
                        writeln!(w, "  node{} --> node{}", data.index, right)?;
                    }
                }
            }
        }

        Ok(())
    }
}

impl<'a, M: Marker> fmt::Display for DisplayAsGraph<'a, M>
where
    &'a Node<M>: DagLike,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.render(self.format, f)
    }
}

fn shorten<S: AsRef<str>>(s: S, max_len: usize) -> String {
    let s = s.as_ref();
    let chars: Vec<char> = s.chars().collect();
    if chars.len() <= max_len {
        s.to_string()
    } else {
        let dots = "...";
        let available = max_len.saturating_sub(dots.len());
        let start_len = available.div_ceil(2); // Slightly favor the start
        let end_len = available / 2;

        let start: String = chars[..start_len].iter().collect();
        let end: String = chars[chars.len() - end_len..].iter().collect();
        format!("{}{}{}", start, dots, end)
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
        types::Context::with_context(|ctx| {
            let empty_witness = HashMap::new();
            Forest::<Core>::parse(s)
                .unwrap()
                .to_witness_node(&ctx, &empty_witness)
                .unwrap()
                .finalize_unpruned()
                .unwrap()
        })
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

    #[test]
    fn display_as_dot() {
        let s = "
            oih := take drop iden
            input := pair (pair unit unit) unit
            output := unit
            main := comp input (comp (pair oih (take unit)) output)";
        let program = parse_program(s);
        let str = program
            .display_as_dot()
            .to_string()
            .replace(" ", "")
            .replace("\n", "");
        let expected = "
        digraph G {
ordering=\"out\";
  node0[label=\"unit\"];
  node1[label=\"unit\"];
  node2[label=\"pair\"];
  node2->node0;
  node2->node1;
  node3[label=\"unit\"];
  node4[label=\"pair\"];
  node4->node2;
  node4->node3;
  node5[label=\"iden\"];
  node6[label=\"drop\"];
  node6->node5;
  node7[label=\"take\"];
  node7->node6;
  node8[label=\"unit\"];
  node9[label=\"take\"];
  node9->node8;
  node10[label=\"pair\"];
  node10->node7;
  node10->node9;
  node11[label=\"unit\"];
  node12[label=\"comp\"];
  node12->node10;
  node12->node11;
  node13[label=\"comp\"];
  node13->node4;
  node13->node12;
}"
        .replace(" ", "")
        .replace("\n", "");

        assert_eq!(str, expected);
    }
}
