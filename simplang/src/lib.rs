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
use simplicity::{CommitNode, CommitNodeInner, core::Value};
use simplicity::dag::{DagLike, InternalSharing, SharingTracker};
use simplicity::types::Type;
use std::collections::HashMap;
use std::{str, marker, rc};

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

pub struct Program<J: Jet, S: SharingTracker<rc::Rc<CommitNode<J>>> = InternalSharing> {
    root_node: rc::Rc<CommitNode<J>>,
    node_map: HashMap<String, rc::Rc<CommitNode<J>>>,
    name_map: HashMap<usize, String>,
    witness_map: HashMap<String, (Type, Option<Value>)>,
    phantom: marker::PhantomData<S>,
}

impl<J: Jet> str::FromStr for Program<J, InternalSharing> {
    type Err = Vec<parse::Error>;
    fn from_str(s: &str) -> Result<Self, Vec<parse::Error>> {
        parse::parse(s)
    }
}

impl<J: Jet> Program<J, InternalSharing> {
    /// Parses a program from a string
    pub fn parse(s: &str) -> Result<Self, Vec<parse::Error>> {
        str::FromStr::from_str(s)
    }
}

impl<J: Jet, S: SharingTracker<rc::Rc<CommitNode<J>>> + Default> Program<J, S> {
    /// Parses a program from a bytestring
    pub fn from_bytes(bytes: &[u8]) -> Result<Self, simplicity::Error> {
        let mut bititer = simplicity::BitIter::from(bytes.iter().cloned());
        let ret = Program::<J, S> {
            root_node: simplicity::decode_program(&mut bititer)?,
            node_map: Default::default(),
            name_map: Default::default(),
            witness_map: Default::default(),
            phantom: Default::default(),
        };
        Ok(ret.relabelled::<S>())
    }

    /// Accessor for the underlying Simplicity program
    pub fn root(&self) -> &CommitNode<J> {
        &self.root_node
    }

    /// Accessor for a specific node of the program, if it exists
    pub fn by_name(&self, name: &str) -> Option<&CommitNode<J>> {
        self.node_map.get(name).map(|rc| &**rc)
    }

    /// Accessor for the witness map, to allow populating witnesses or seeing what witness
    /// names/types are available to populate.
    pub fn witnesses(&mut self) -> &mut HashMap<String, (Type, Option<Value>)> {
        &mut self.witness_map
    }

    pub fn relabelled<Sx>(&self) -> Program<J, Sx>
    where Sx: SharingTracker<rc::Rc<CommitNode<J>>> + Default
    {
        let mut new = Program {
            root_node: rc::Rc::clone(&self.root_node),
            node_map: HashMap::new(),
            name_map: HashMap::new(),
            witness_map: HashMap::new(),
            phantom: Default::default(),
        };

        // Assign names to every node, in post-order iter order
        let mut namer = serialize::NameFactory::new();
        for data in rc::Rc::clone(&self.root_node).post_order_iter::<Sx>() {
            let node_name = if std::ptr::eq(&*data.node, self.root_node.as_ref()) {
                "main".into()
            } else {
                namer.new_name(&data.node)
            };
            new.node_map.insert(node_name.clone(), rc::Rc::clone(&data.node));
            if let CommitNodeInner::Witness = data.node.inner() {
                new.witness_map.insert(node_name.clone(), (data.node.arrow().target.shallow_clone(), None));
            }
            new.name_map.insert(data.index, node_name);
        }
        new
    }

    /// Serialize the program in human-readable form
    pub fn string_serialize(&self) -> String
    {
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
        for data in rc::Rc::clone(&self.root_node).post_order_iter::<S>() {
            let node = data.node;
            let name = &self.name_map[&data.index];
            let mut expr_str = match node.inner() {
                CommitNodeInner::Fail(cmr1, cmr2) => format!("{} = fail {} {}", name, cmr1, cmr2),
                CommitNodeInner::Jet(ref j) => format!("{} = jet_{}", name, j),
                CommitNodeInner::Word(ref v) => format!("{} = word_jet {}", name, serialize::DisplayWord(v)),
                inner => format!("{} = {}", name, inner),
            };
            if let Some(idx) = data.left_index {
                expr_str.push(' ');
                expr_str.push_str(&self.name_map[&idx]);
            }
            if let Some(idx) = data.right_index {
                expr_str.push(' ');
                expr_str.push_str(&self.name_map[&idx]);
            }

            let arrow = node.arrow();
            let arrow_str = format!(":: {} -> {}", arrow.source, arrow.target)
                .replace('×', "*"); // for human-readable encoding stick with ASCII

            // All witnesses have the same CMR so don't bother printing it
            let cmr_str = if let CommitNodeInner::Witness = node.inner() {
                String::new()
            } else {
                format!("# cmr {:.8}...", node.cmr())
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
            ret += &print_lines(&program_lines, true /* add a blank line before main */);
        }
        ret
    }
}


