// Simplicity "Human-Readable" Language
// Written in 2023 by
//   Andrew Poelstra <simplicity@wpsoftware.net>
//
// To the extent possible under law, the author(s) have dedicated all
// copyright and related and neighboring rights to this software to
// the public domain worldwide. This software is distributed without
// any warranty.
//
// You should have received a copy of the CC0 Public Domain Dedication
// along with this software.
// If not, see <http://creativecommons.org/publicdomain/zero/1.0/>.
//

/// Re-export of santiago crate
pub extern crate santiago;
/// Re-export of simplicity crate
pub extern crate simplicity;

pub mod parse;
pub mod serialize;

use simplicity::jet::Jet;
use simplicity::{CommitNode, CommitNodeInner, Cmr};
use std::collections::HashMap;
use std::{str, rc};

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

pub struct Program<J: Jet> {
    root_node: rc::Rc<CommitNode<J>>,
    node_map: HashMap<String, rc::Rc<CommitNode<J>>>,
    cmr_name_map: HashMap<Cmr, String>,
}

impl<J: Jet> str::FromStr for Program<J> {
    type Err = Vec<parse::Error>;
    fn from_str(s: &str) -> Result<Self, Vec<parse::Error>> {
        parse::parse(s)
    }
}

impl<J: Jet> Program<J> {
    /// Parses a program from a string
    pub fn parse(s: &str) -> Result<Self, Vec<parse::Error>> {
        str::FromStr::from_str(s)
    }

    /// Parses a program from a bytestring
    pub fn from_bytes(bytes: &[u8]) -> Result<Self, simplicity::Error> {
        let mut bititer = simplicity::bititer::BitIter::from(bytes.iter().cloned());
        let root: rc::Rc<CommitNode<J>> = simplicity::decode_program(&mut bititer)?;

        let mut node_map = HashMap::new();
        let mut cmr_name_map = HashMap::new();

        // Assign names to every node, in post-order iter order
        let mut namer = serialize::NameFactory::new();
        for node in rc::Rc::clone(&root).rc_iter() {
            let node_name = if std::ptr::eq(&*node, &*root) {
                "main".into()
            } else {
                namer.new_name(&node)
            };
            node_map.insert(node_name.clone(), rc::Rc::clone(&node));
            cmr_name_map.insert(node.cmr(), node_name);
        }

        Ok(Program {
            root_node: root,
            node_map,
            cmr_name_map,
        })
    }

    /// Accessor for the underlying Simplicity program
    pub fn root(&self) -> &CommitNode<J> {
        &self.root_node
    }

    /// Accessor for a specific node of the program, if it exists
    pub fn by_name(&self, name: &str) -> Option<&CommitNode<J>> {
        self.node_map.get(name).map(|rc| &**rc)
    }

    /// Serialize the program in human-readable form
    pub fn string_serialize(&self) -> String {
        struct Print {
            expr_str: String, // The X = Y part
            arrow_str: String, // The :: A -> B part
            cmr_str: String, // The # <cmr> part
        }
        fn print_lines(lines: &[Print], skip_before_end: bool) -> String {
            let mut ret = String::new();
            let expr_width = lines
                .iter()
                .map(|line| line.expr_str.len())
                .max()
                .unwrap();
            let arrow_width = lines
                .iter()
                .map(|line| line.arrow_str.len())
                .max()
                .unwrap();
            let last_line = lines.len();
            for (n, line) in lines.iter().enumerate() {
                if skip_before_end && n == last_line - 1 {
                    ret += "\n";
                }
                ret += &format!("{0:1$} {2:3$} {4}\n", line.expr_str, expr_width, line.arrow_str, arrow_width, line.cmr_str);
            }
            ret
        }

        let mut witness_lines = vec![];
        let mut const_lines = vec![];
        let mut program_lines = vec![];
        // Pass 1: compute string data for every node
        for node in self.root_node.iter() {
            let name = &self.cmr_name_map[&node.cmr()];
            let mut expr_str = match node.inner() {
                CommitNodeInner::Fail(cmr1, cmr2) => format!("{} = fail {} {}", name, cmr1, cmr2),
                CommitNodeInner::Hidden(cmr) => format!("{} = hidden {}", name, cmr),
                CommitNodeInner::Jet(ref j) => format!("{} = jet_{}", name, j),
                CommitNodeInner::Word(ref v) => format!("{} = word_jet {}", name, serialize::DisplayWord(v)),
                inner => format!("{} = {}", name, inner),
            };
            if let Some(child_cmr) = node.get_left().map(|child| child.cmr()) {
                expr_str.push(' ');
                expr_str.push_str(&self.cmr_name_map[&child_cmr]);
            }
            if let Some(child_cmr) = node.get_right().map(|child| child.cmr()) {
                expr_str.push(' ');
                expr_str.push_str(&self.cmr_name_map[&child_cmr]);
            }

            let arrow = node.arrow();
            let arrow_str = format!(":: {} -> {}", arrow.source, arrow.target)
                .replace('×', "*"); // for human-readable encoding stick with ASCII

            // All witnesses have the same CMR so don't bother printing it
            let cmr_str = if let CommitNodeInner::Witness = node.inner() {
                String::new()
            } else {
                format!("# {:.8}", node.cmr())
            };

            let print = Print { expr_str, arrow_str, cmr_str };
            if let CommitNodeInner::Witness = node.inner() {
                witness_lines.push(print);
            } else if let CommitNodeInner::Word(..) = node.inner() {
                const_lines.push(print);
            } else {
                program_lines.push(print);
            }
        }

        // Pass 2: actually print everything
        let mut ret = String::new();
        if !witness_lines.is_empty() {
            ret += "# Witnesses\n";
            ret += &print_lines(&witness_lines, false);
            ret += "\n";
        }
        if !const_lines.is_empty() {
            // FIXME detect scribes (probably need support in rust-simplicity)
            ret += "# Constants\n";
            ret += &print_lines(&const_lines, false);
            ret += "\n";
        }
        if !program_lines.is_empty() {
            ret += "# Program code\n";
            println!("# Program code");
            ret += &print_lines(&program_lines, true /* add a blank line before main */);
        }
        ret
    }
}


