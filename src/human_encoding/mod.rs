// SPDX-License-Identifier: CC0-1.0

//! The Simplicity Human-Readable Encoding
//!
//! This module provides the ability to decode and encode [`NamedCommitNode`]s
//! in a human-readable format.
//!

mod error;
mod named_node;
mod parse;
mod serialize;

use crate::dag::{DagLike, MaxSharing};
use crate::jet::Jet;
use crate::node::{self, CommitNode, NoWitness};
use crate::{Cmr, Imr, Value, WitnessNode};

use std::collections::HashMap;
use std::str;
use std::sync::Arc;

pub use self::error::{Error, ErrorSet};
pub use self::named_node::NamedCommitNode;

/// Line/column pair
///
/// There is a similar type provided by the `santiago` library but it does not implement
/// `Copy`, among many other traits, which makes it unergonomic to use. Santiago positions
/// can be converted using `.into()`.
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Default, Hash)]
pub struct Position {
    line: usize,
    column: usize,
}

impl<'a> From<&'a santiago::lexer::Position> for Position {
    fn from(position: &'a santiago::lexer::Position) -> Position {
        Position {
            line: position.line,
            column: position.column,
        }
    }
}

impl From<santiago::lexer::Position> for Position {
    fn from(position: santiago::lexer::Position) -> Position {
        Position {
            line: position.line,
            column: position.column,
        }
    }
}

/// For named construct nodes, we abuse the `witness` combinator to support typed holes.
///
/// We do this because `witness` nodes have free source and target arrows, and
/// allow us to store arbitrary data in them using the generic witness type of
/// the node. Holes are characterized entirely by their source and target type,
/// just like witnesses, and are labelled by their name.
#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub enum WitnessOrHole {
    /// This witness is an actual witness combinator (with no witness data attached,
    /// that comes later)
    Witness,
    /// This is a typed hole, with the given name.
    TypedHole(Arc<str>),
}

impl WitnessOrHole {
    pub fn shallow_clone(&self) -> Self {
        match self {
            WitnessOrHole::Witness => WitnessOrHole::Witness,
            WitnessOrHole::TypedHole(name) => WitnessOrHole::TypedHole(Arc::clone(name)),
        }
    }
}

impl<'a> From<&'a NoWitness> for WitnessOrHole {
    fn from(_: &NoWitness) -> Self {
        WitnessOrHole::Witness
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Forest<J: Jet> {
    roots: HashMap<Arc<str>, Arc<NamedCommitNode<J>>>,
}

impl<J: Jet> Forest<J> {
    /// Parses a forest from a string
    pub fn parse(s: &str) -> Result<Self, ErrorSet> {
        parse::parse(s).map(|roots| Forest { roots })
    }

    /// Parses a program from a bytestring
    pub fn from_program(root: Arc<CommitNode<J>>) -> Self {
        let root = NamedCommitNode::from_node(&root);
        let mut roots = HashMap::new();
        roots.insert("main".into(), root);
        Forest { roots }
    }

    /// Accessor for the map of roots of this forest
    pub fn roots(&self) -> &HashMap<Arc<str>, Arc<NamedCommitNode<J>>> {
        &self.roots
    }

    /// Serialize the program in human-readable form
    pub fn string_serialize(&self) -> String {
        struct Print {
            cmr: Cmr,
            imr: Option<Imr>,
            expr_str: String,  // The X = Y part
            arrow_str: String, // The :: A -> B part
        }
        fn print_lines(lines: &[Print], skip_before_end: bool) -> String {
            let mut ret = String::new();
            let expr_width = lines.iter().map(|line| line.expr_str.len()).max().unwrap();
            let arrow_width = lines.iter().map(|line| line.arrow_str.len()).max().unwrap();
            let last_line = lines.len();
            for (n, line) in lines.iter().enumerate() {
                ret += "\n";
                if skip_before_end && n == last_line - 1 {
                    ret += "\n";
                }
                ret += &format!("-- CMR: {}\n", line.cmr);
                if let Some(imr) = line.imr {
                    ret += &format!("-- IMR: {}\n", imr);
                } else {
                    ret += "-- IMR: [undetermined]\n";
                }
                ret += &format!(
                    "{0:1$} {2:3$}\n",
                    line.expr_str, expr_width, line.arrow_str, arrow_width,
                );
            }
            ret
        }

        let mut witness_lines = vec![];
        let mut const_lines = vec![];
        let mut program_lines = vec![];
        // Pass 1: compute string data for every node
        for root in self.roots.values() {
            for data in root.as_ref().post_order_iter::<MaxSharing<_>>() {
                let node = data.node;
                let name = node.name();
                let mut expr_str = match node.inner() {
                    node::Inner::AssertR(cmr, _) => format!("{} := assertr #{}", name, cmr),
                    node::Inner::Fail(entropy) => format!("{} := fail {}", name, entropy),
                    node::Inner::Jet(ref j) => format!("{} := jet_{}", name, j),
                    node::Inner::Word(ref v) => {
                        format!("{} := const {}", name, serialize::DisplayWord(v))
                    }
                    inner => format!("{} := {}", name, inner),
                };
                if let Some(child) = node.left_child() {
                    expr_str.push(' ');
                    expr_str.push_str(child.name());
                }
                if let Some(child) = node.right_child() {
                    expr_str.push(' ');
                    expr_str.push_str(child.name());
                } else if let node::Inner::AssertL(_, cmr) = node.inner() {
                    expr_str.push_str(" #");
                    expr_str.push_str(&cmr.to_string());
                } else if let node::Inner::Disconnect(_, hole_name) = node.inner() {
                    expr_str.push_str(&format!(" ?{}", hole_name));
                }

                let arrow = node.arrow();
                let arrow_str = format!(": {} -> {}", arrow.source, arrow.target).replace('×', "*"); // for human-readable encoding stick with ASCII

                let print = Print {
                    cmr: node.cmr(),
                    imr: node.imr(),
                    expr_str,
                    arrow_str,
                };
                if let node::Inner::Witness(..) = node.inner() {
                    witness_lines.push(print);
                } else if let node::Inner::Word(..) = node.inner() {
                    const_lines.push(print);
                } else {
                    program_lines.push(print);
                }
            }
        }

        // Pass 2: actually print everything
        let mut ret = String::new();
        if !witness_lines.is_empty() {
            ret += "--------------\n-- Witnesses\n--------------\n";
            ret += &print_lines(&witness_lines, false);
            ret += "\n";
        }
        if !const_lines.is_empty() {
            // FIXME detect scribes
            ret += "--------------\n-- Constants\n--------------\n";
            ret += &print_lines(&const_lines, false);
            ret += "\n";
        }
        if !program_lines.is_empty() {
            ret += "--------------\n-- Program code\n--------------\n";
            ret += &print_lines(&program_lines, true /* add a blank line before main */);
        }
        ret
    }

    /// Convert the forest into a witness node.
    ///
    /// Succeeds if the forest contains a "main" root and returns `None` otherwise.
    pub fn to_witness_node(
        &self,
        witness: &HashMap<Arc<str>, Arc<Value>>,
    ) -> Option<Arc<WitnessNode<J>>> {
        let main = self.roots.get("main")?;
        Some(main.to_witness_node(witness, self.roots()))
    }
}

#[cfg(test)]
mod tests {
    use crate::human_encoding::Forest;
    use crate::jet::{Core, Jet};
    use crate::{BitMachine, Value};
    use std::collections::HashMap;
    use std::sync::Arc;

    fn assert_finalize_ok<J: Jet>(
        s: &str,
        witness: &HashMap<Arc<str>, Arc<Value>>,
        env: &J::Environment,
    ) {
        let program = Forest::<J>::parse(s)
            .expect("Failed to parse human encoding")
            .to_witness_node(witness)
            .expect("Forest is missing expected root")
            .finalize()
            .expect("Failed to finalize");
        let mut mac = BitMachine::for_program(&program);
        mac.exec(&program, env).expect("Failed to run program");
    }

    fn assert_finalize_err<J: Jet>(
        s: &str,
        witness: &HashMap<Arc<str>, Arc<Value>>,
        env: &J::Environment,
        err_msg: &'static str,
    ) {
        let program = match Forest::<J>::parse(s)
            .expect("Failed to parse human encoding")
            .to_witness_node(witness)
            .expect("Forest is missing expected root")
            .finalize()
        {
            Ok(program) => program,
            Err(error) => {
                assert_eq!(&error.to_string(), err_msg);
                return;
            }
        };
        let mut mac = BitMachine::for_program(&program);
        match mac.exec(&program, env) {
            Ok(_) => panic!("Execution is expected to fail"),
            Err(error) => assert_eq!(&error.to_string(), err_msg),
        }
    }

    #[test]
    fn filled_witness() {
        let s = "
            a := witness
            b := witness
            main := comp
                comp
                    pair a b
                    jet_lt_8
                jet_verify
        ";

        let a_less_than_b = HashMap::from([
            (Arc::from("a"), Value::u8(0x00)),
            (Arc::from("b"), Value::u8(0x01)),
        ]);
        assert_finalize_ok::<Core>(s, &a_less_than_b, &());

        let b_greater_equal_a = HashMap::from([
            (Arc::from("a"), Value::u8(0x01)),
            (Arc::from("b"), Value::u8(0x01)),
        ]);
        assert_finalize_err::<Core>(s, &b_greater_equal_a, &(), "Jet failed during execution");
    }

    #[test]
    fn unfilled_witness() {
        let witness = HashMap::from([(Arc::from("wit1"), Value::u32(1337))]);
        assert_finalize_err::<Core>(
            "
                wit1 := witness : 1 -> 2^32
                wit2 := witness : 1 -> 2^32

                wits_are_equal := comp (pair wit1 wit2) jet_eq_32 : 1 -> 2
                main := comp wits_are_equal jet_verify            : 1 -> 1
            ",
            &witness,
            &(),
            "unable to satisfy program",
        );
    }

    #[test]
    fn unfilled_witness_pruned() {
        let s = "
            wit1 := witness
            wit2 := witness
            main := comp (pair wit1 unit) case unit wit2
        ";
        let wit2_is_pruned = HashMap::from([(Arc::from("wit1"), Value::u1(0))]);
        assert_finalize_ok::<Core>(s, &wit2_is_pruned, &());

        let wit2_is_missing = HashMap::from([(Arc::from("wit1"), Value::u1(1))]);
        // FIXME The finalization should fail
        // This doesn't happen because we don't run the program,
        // so we cannot always determine which nodes must be pruned
        assert_finalize_err::<Core>(
            s,
            &wit2_is_missing,
            &(),
            "Execution reached a pruned branch: bf12681a76fc7c00c63e583c25cc97237337d6aca30d3f4a664075445385c648"
        );

        let wit2_is_present = HashMap::from([
            (Arc::from("wit1"), Value::u1(1)),
            (Arc::from("wit2"), Value::unit()),
        ]);
        assert_finalize_ok::<Core>(s, &wit2_is_present, &());
    }

    #[test]
    fn filled_hole() {
        let empty = HashMap::new();
        assert_finalize_ok::<Core>(
            "
                id1 := iden : 2^256 * 1 -> 2^256 * 1
                main := comp (disconnect id1 ?hole) unit
                hole := unit
            ",
            &empty,
            &(),
        );
    }

    #[test]
    fn unfilled_hole() {
        let empty = HashMap::new();
        assert_finalize_err::<Core>(
            "
                wit1 := witness
                main := comp wit1 comp disconnect iden ?dis2 unit
            ",
            &empty,
            &(),
            "unable to satisfy program",
        );
    }

    #[test]
    fn witness_name_override() {
        let s = "
            wit1 := witness
            wit2 := wit1
            main := comp wit2 iden
        ";
        let wit1_populated = HashMap::from([(Arc::from("wit1"), Value::unit())]);
        assert_finalize_err::<Core>(s, &wit1_populated, &(), "unable to satisfy program");

        let wit2_populated = HashMap::from([(Arc::from("wit2"), Value::unit())]);
        assert_finalize_ok::<Core>(s, &wit2_populated, &());
    }
}
