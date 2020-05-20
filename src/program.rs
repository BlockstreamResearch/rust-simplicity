// Rust Simplicity Library
// Written in 2020 by
//   Andrew Poelstra <apoelstra@blockstream.com>
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

//! # Simplicity Programs
//!
//! Programs are lists of Simplicity nodes which reference each other (only
//! pointing backwards, so we have a DAG), and which cache other auxiliary
//! data.
//!

use bitcoin_hashes::{Hash, sha256};
use std::{cmp, fmt};
use std::sync::Arc;

use {Error, Node, Value};
use bititer::BitIter;
use encode;
use types;

/// A node in a complete program, with associated metadata
pub struct ProgramNode {
    /// The underlying node
    pub node: Node<Value>,
    /// Its index within the total program
    pub index: usize,
    /// Its Commitment Merkle Root
    pub cmr: sha256::Hash,
    /// Source type for this node
    pub source_ty: Arc<types::FinalType>,
    /// Target type for this node
    pub target_ty: Arc<types::FinalType>,
    /// Upper bound on the number of cells required in the Bit
    /// Machine by this node
    pub extra_cells_bound: usize,
    /// Upper bound on the number of cells required in the Bit
    /// Machine by this node
    pub frame_count_bound: usize,
}

impl fmt::Display for ProgramNode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}] ", self.index)?;
        match self.node {
            Node::Iden => f.write_str("iden")?,
            Node::Unit => f.write_str("unit")?,
            Node::InjL(i) => write!(f, "injl({})", i)?,
            Node::InjR(i) => write!(f, "injr({})", i)?,
            Node::Take(i) => write!(f, "take({})", i)?,
            Node::Drop(i) => write!(f, "drop({})", i)?,
            Node::Comp(i, j) => write!(f, "comp({}, {})", i, j)?,
            Node::Case(i, j) => write!(f, "case({}, {})", i, j)?,
            Node::Pair(i, j) => write!(f, "pair({}, {})", i, j)?,
            Node::Disconnect(i, j) => write!(f, "disconnect({}, {})", i, j)?,
            Node::Witness(..) => f.write_str("witness")?,
            Node::Hidden(..) => f.write_str("hidden")?,
            Node::Fail(..) => f.write_str("fail")?,
            Node::Bitcoin(..) => unimplemented!(),
        }
        write!(f, ": {} → {}", self.source_ty, self.target_ty,)
    }
}

/// A fully parsed, witnesses-included Simplicity program
pub struct Program {
    /// The list of nodes in the program
    pub nodes: Vec<ProgramNode>,
}

impl Program {
    /// Decode a program from a stream of bits
    pub fn decode<I: Iterator<Item = u8>>(
        iter: &mut BitIter<I>,
    ) -> Result<Program, Error> {
        // Decode a bunch of untyped, witness-less nodes
        let nodes = encode::decode_program_no_witness(&mut *iter)?;

        // Do type-checking
        let typed_nodes = types::type_check(nodes);

        // Parse witnesses, if available
        // FIXME actually only read as much as wit_len
        let wit_len = match iter.next() {
            Some(false) => 0,
            Some(true) => encode::decode_natural(&mut *iter)
                .expect("decoding witness length"),
            None => panic!("No witness data"),
        };

        let typed_nodes = typed_nodes
            .into_iter()
            .map(|node| Ok(types::TypedNode {
                node: match node.node {
                    // really, Rust???
                    Node::Iden => Node::Iden,
                    Node::Unit => Node::Unit,
                    Node::InjL(i) => Node::InjL(i),
                    Node::InjR(i) => Node::InjR(i),
                    Node::Take(i) => Node::Take(i),
                    Node::Drop(i) => Node::Drop(i),
                    Node::Comp(i, j) => Node::Comp(i, j),
                    Node::Case(i, j) => Node::Case(i, j),
                    Node::Pair(i, j) => Node::Pair(i, j),
                    Node::Disconnect(i, j) => Node::Disconnect(i, j),
                    Node::Witness(()) => Node::Witness(
                        Value::from_witness(&mut iter.by_ref().take(wit_len), &node.target_ty)?
                    ),
                    Node::Fail(x, y) => Node::Fail(x, y),
                    Node::Hidden(x) => Node::Hidden(x),
                    Node::Bitcoin(b) => Node::Bitcoin(b),
                },
                source_ty: node.source_ty,
                target_ty: node.target_ty,
            }))
            .collect::<Result<Vec<_>, _>>()?;

        // Compute cached data and return
        let mut ret = Vec::<ProgramNode>::with_capacity(typed_nodes.len());
        for (index, node) in typed_nodes.into_iter().enumerate() {
            let final_node = ProgramNode {
                index: index,
                cmr: sha256::Hash::hash(b"compute me FIXME FIXME"),
                extra_cells_bound: match node.node {
                    Node::Iden => 0,
                    Node::Unit => 0,
                    Node::InjL(i) => ret[i].extra_cells_bound,
                    Node::InjR(i) => ret[i].extra_cells_bound,
                    Node::Take(i) => ret[i].extra_cells_bound,
                    Node::Drop(i) => ret[i].extra_cells_bound,
                    Node::Comp(i, j) => ret[i].target_ty.bit_width()
                        + cmp::max(
                            ret[i].extra_cells_bound,
                            ret[j].extra_cells_bound,
                        ),
                    Node::Case(i, j) => cmp::max(
                        ret[i].extra_cells_bound,
                        ret[j].extra_cells_bound,
                    ),
                    Node::Pair(i, j) => cmp::max(
                        ret[i].extra_cells_bound,
                        ret[j].extra_cells_bound,
                    ),
                    Node::Disconnect(..) => unimplemented!(),
                    Node::Witness(..) => node.target_ty.bit_width(),
                    Node::Fail(..) => unimplemented!(),
                    Node::Hidden(..) => 0,
                    Node::Bitcoin(..) => 0,
                },
                frame_count_bound: match node.node {
                    Node::Iden => 0,
                    Node::Unit => 0,
                    Node::InjL(i) => ret[i].frame_count_bound,
                    Node::InjR(i) => ret[i].frame_count_bound,
                    Node::Take(i) => ret[i].frame_count_bound,
                    Node::Drop(i) => ret[i].frame_count_bound,
                    Node::Comp(i, j) => 1
                        + cmp::max(
                            ret[i].frame_count_bound,
                            ret[j].frame_count_bound,
                        ),
                    Node::Case(i, j) => cmp::max(
                        ret[i].frame_count_bound,
                        ret[j].frame_count_bound,
                    ),
                    Node::Pair(i, j) => cmp::max(
                        ret[i].frame_count_bound,
                        ret[j].frame_count_bound,
                    ),
                    Node::Disconnect(..) => unimplemented!(),
                    Node::Witness(..) => 0,
                    Node::Fail(..) => unimplemented!(),
                    Node::Hidden(..) => 0,
                    Node::Bitcoin(..) => 0,
                },
                node: node.node,
                source_ty: node.source_ty,
                target_ty: node.target_ty,
            };
            ret.push(final_node);
        }

        Ok(Program {
            nodes: ret,
        })
    }

    /// Print out the program in a graphviz-parseable format
    pub fn graph_print(&self) {
        for node in &self.nodes {
            println!(
                "{} [label=\"{}\\n{}\\n{}→{}\"];",
                node.index,
                match node.node {
                    Node::Iden => "iden",
                    Node::Unit => "unit",
                    Node::InjL(..) => "injl",
                    Node::InjR(..) => "injr",
                    Node::Take(..) => "take",
                    Node::Drop(..) => "drop",
                    Node::Comp(..) => "comp",
                    Node::Case(..) => "case",
                    Node::Pair(..) => "pair",
                    Node::Disconnect(..) => "disconnect",
                    Node::Witness(..) => "witness",
                    Node::Hidden(..) => "hidden",
                    Node::Fail(..) => "fail",
                    Node::Bitcoin(..) => "[bitcoin]",
                },
                node.index,
                node.source_ty,
                node.target_ty,
            );
            match node.node {
                Node::Iden | Node::Unit | Node::Witness(..) | Node::Hidden(..) | Node::Fail(..) | Node::Bitcoin(..) => {
                }
                Node::InjL(i) | Node::InjR(i) | Node::Take(i) | Node::Drop(i) => {
                    println!("  {} -> {};", node.index, i);
                }
                Node::Comp(i, j)
                | Node::Case(i, j)
                | Node::Pair(i, j)
                | Node::Disconnect(i, j) => {
                    println!("  {} -> {} [color=red];", node.index, i);
                    println!("  {} -> {} [color=blue];", node.index, j);
                }
            }
        }
    }
}




