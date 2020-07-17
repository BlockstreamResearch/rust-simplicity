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

use std::{cmp, fmt, sync::Arc};

use bititer::BitIter;
use cmr::{self, Cmr};
use encode;
use extension::{self, Node as ExtNode};
use types;
use Error;
use Node;
use Value;

/// A node in a complete program, with associated metadata
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub struct ProgramNode<Ext> {
    /// The underlying node
    pub node: Node<Value, Ext>,
    /// Its index within the total program
    pub index: usize,
    /// Its Commitment Merkle Root
    pub cmr: Cmr,
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

impl<Ext: fmt::Display> fmt::Display for ProgramNode<Ext> {
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
            Node::Ext(ref b) => write!(f, "[ext]{}", b)?,
            Node::Jet(ref j) => write!(f, "[jet]{}", j)?,
        }
        write!(f, ": {} → {}", self.source_ty, self.target_ty,)
    }
}

/// A fully parsed, witnesses-included Simplicity program
#[derive(Debug)]
pub struct Program<Ext> {
    /// The list of nodes in the program
    pub nodes: Vec<ProgramNode<Ext>>,
}

/// Scribe progra: for any value of a Simplicity type b :B, the constant function
/// from A -> B can be realized by a Simplicity expression called scribe.  
/// Refer to 3.4 section of the Tech Report for details.
/// This returns a list of untyped nodes.
pub fn scribe<Ext>(b: Value) -> Vec<Node<(), Ext>> {
    match b {
        Value::Unit => vec![Node::Unit],
        Value::SumL(l) => {
            let mut nodes = scribe(*l);
            nodes.push(Node::InjL(1));
            nodes
        }
        Value::SumR(r) => {
            let mut nodes = scribe(*r);
            nodes.push(Node::InjR(1));
            nodes
        }
        Value::Prod(l, r) => {
            let mut nodes = scribe(*l);
            let r_nodes = scribe(*r);
            let li = r_nodes.len() + 1;
            nodes.extend(r_nodes);
            nodes.push(Node::Pair(li, 1));
            nodes
        }
    }
}

impl<Ext: extension::Node> Program<Ext> {
    /// Obtain the node representing the root of the program DAG
    pub fn root_node(&self) -> &ProgramNode<Ext> {
        &self.nodes[self.nodes.len() - 1]
    }

    /// Decode a program from a stream of bits
    pub fn decode<I: Iterator<Item = u8>>(iter: &mut BitIter<I>) -> Result<Program<Ext>, Error> {
        // Decode a bunch of untyped, witness-less nodes
        // dbg!(iter.cloned().collect::<Vec<bool>>());
        let nodes = encode::decode_program_no_witness(&mut *iter)?;

        // Do type-checking
        let typed_nodes = types::type_check(nodes)?;

        // Parse witnesses, if available
        // FIXME actually only read as much as wit_len
        let wit_len = match iter.next() {
            Some(false) => 0,
            Some(true) => encode::decode_natural(&mut *iter, None)?,
            None => return Err(Error::EndOfStream),
        };

        let typed_nodes: Vec<types::TypedNode<Value, Ext>> = typed_nodes
            .into_iter()
            .map::<Result<types::TypedNode<Value, Ext>, Error>, _>(|node| {
                Ok(types::TypedNode {
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
                        Node::Witness(()) => Node::Witness(Value::from_bits_and_type(
                            &mut iter.by_ref().take(wit_len),
                            &node.target_ty,
                        )?),
                        Node::Fail(x, y) => Node::Fail(x, y),
                        Node::Hidden(x) => Node::Hidden(x),
                        Node::Ext(e) => Node::Ext(e),
                        Node::Jet(j) => Node::Jet(j),
                    },
                    source_ty: node.source_ty,
                    target_ty: node.target_ty,
                })
            })
            .collect::<Result<Vec<_>, _>>()?;

        // Compute cached data and return
        let mut ret = Vec::<ProgramNode<Ext>>::with_capacity(typed_nodes.len());
        for (index, node) in typed_nodes.into_iter().enumerate() {
            let final_node = ProgramNode {
                index: index,
                cmr: compute_cmr(&ret, &node.node, index),
                extra_cells_bound: compute_extra_cells_bound(
                    &ret,
                    &node.node,
                    index,
                    node.target_ty.bit_width(),
                ),
                frame_count_bound: compute_frame_count_bound(&ret, &node.node, index),
                node: node.node,
                source_ty: node.source_ty,
                target_ty: node.target_ty,
            };
            ret.push(final_node);
        }

        Ok(Program { nodes: ret })
    }

    /// Decode a program from a stream of bits
    pub fn from_untyped_nodes<I: Iterator<Item = u8>>(
        nodes: Vec<Node<(), Ext>>,
        iter: &mut BitIter<I>,
    ) -> Result<Program<Ext>, Error> {
        // Do type-checking
        let typed_nodes = types::type_check(nodes)?;

        // Parse witnesses, if available
        // FIXME actually only read as much as wit_len
        let wit_len = match iter.next() {
            Some(false) => 0,
            Some(true) => encode::decode_natural(&mut *iter, None)?,
            None => return Err(Error::EndOfStream),
        };

        let typed_nodes = typed_nodes
            .into_iter()
            .map::<Result<_, Error>, _>(|node| {
                Ok(types::TypedNode {
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
                        Node::Witness(()) => Node::Witness(Value::from_bits_and_type(
                            &mut iter.by_ref(),
                            &node.target_ty,
                        )?),
                        Node::Fail(x, y) => Node::Fail(x, y),
                        Node::Hidden(x) => Node::Hidden(x),
                        Node::Ext(e) => Node::Ext(e),
                        Node::Jet(j) => Node::Jet(j),
                    },
                    source_ty: node.source_ty,
                    target_ty: node.target_ty,
                })
            })
            .collect::<Result<Vec<_>, _>>()?;

        // Compute cached data and return
        let mut ret = Vec::<ProgramNode<Ext>>::with_capacity(typed_nodes.len());
        for (index, node) in typed_nodes.into_iter().enumerate() {
            let final_node = ProgramNode {
                index: index,
                cmr: compute_cmr(&ret, &node.node, index),
                extra_cells_bound: compute_extra_cells_bound(
                    &ret,
                    &node.node,
                    index,
                    node.target_ty.bit_width(),
                ),
                frame_count_bound: compute_frame_count_bound(&ret, &node.node, index),
                node: node.node,
                source_ty: node.source_ty,
                target_ty: node.target_ty,
            };
            ret.push(final_node);
        }

        Ok(Program { nodes: ret })
    }

    /// Print out the program in a graphviz-parseable format
    pub fn graph_print(&self) {
        for node in &self.nodes {
            println!(
                "{} [label=\"{}\\n{}\\n{} → {}\"];",
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
                    Node::Ext(..) => "[ext]", // FIXME `ext` and `jet` should passthrough
                    Node::Jet(..) => "[jet]",
                },
                node.index,
                node.source_ty,
                node.target_ty,
            );
            match node.node {
                Node::Iden
                | Node::Unit
                | Node::Witness(..)
                | Node::Hidden(..)
                | Node::Fail(..)
                | Node::Ext(..)
                | Node::Jet(..) => {}
                Node::InjL(i) | Node::InjR(i) | Node::Take(i) | Node::Drop(i) => {
                    println!("  {} -> {};", node.index, node.index - i);
                }
                Node::Comp(i, j) | Node::Case(i, j) | Node::Pair(i, j) | Node::Disconnect(i, j) => {
                    println!("  {} -> {} [color=red];", node.index, node.index - i);
                    println!("  {} -> {} [color=blue];", node.index, node.index - j);
                }
            }
        }
    }
}

fn compute_cmr<Ext: extension::Node>(
    program: &[ProgramNode<Ext>],
    node: &Node<Value, Ext>,
    idx: usize,
) -> Cmr {
    match *node {
        Node::Iden => cmr::tag::iden(),
        Node::Unit => cmr::tag::unit(),
        Node::InjL(i) => cmr::tag::injl().update_1(program[idx - i].cmr),
        Node::InjR(i) => cmr::tag::injr().update_1(program[idx - i].cmr),
        Node::Take(i) => cmr::tag::take().update_1(program[idx - i].cmr),
        Node::Drop(i) => cmr::tag::drop().update_1(program[idx - i].cmr),
        Node::Comp(i, j) => cmr::tag::comp().update(program[idx - i].cmr, program[idx - j].cmr),
        Node::Case(i, j) => cmr::tag::case().update(program[idx - i].cmr, program[idx - j].cmr),
        Node::Pair(i, j) => cmr::tag::pair().update(program[idx - i].cmr, program[idx - j].cmr),
        Node::Disconnect(i, _) => cmr::tag::disconnect().update_1(program[idx - i].cmr),
        Node::Witness(..) => cmr::tag::witness(),
        Node::Fail(..) => unimplemented!(),
        Node::Hidden(cmr) => cmr,
        Node::Ext(ref b) => b.cmr(),
        Node::Jet(ref j) => j.cmr(),
    }
}

fn compute_extra_cells_bound<Ext: extension::Node>(
    program: &[ProgramNode<Ext>],
    node: &Node<Value, Ext>,
    idx: usize,
    witness_target_width: usize,
) -> usize {
    match *node {
        Node::Iden => 0,
        Node::Unit => 0,
        Node::InjL(i) => program[idx - i].extra_cells_bound,
        Node::InjR(i) => program[idx - i].extra_cells_bound,
        Node::Take(i) => program[idx - i].extra_cells_bound,
        Node::Drop(i) => program[idx - i].extra_cells_bound,
        Node::Comp(i, j) => {
            program[idx - i].target_ty.bit_width()
                + cmp::max(
                    program[idx - i].extra_cells_bound,
                    program[idx - j].extra_cells_bound,
                )
        }
        Node::Case(i, j) => cmp::max(
            program[idx - i].extra_cells_bound,
            program[idx - j].extra_cells_bound,
        ),
        Node::Pair(i, j) => cmp::max(
            program[idx - i].extra_cells_bound,
            program[idx - j].extra_cells_bound,
        ),
        Node::Disconnect(i, j) => {
            program[idx - i].source_ty.bit_width()
                + program[idx - i].target_ty.bit_width()
                + cmp::max(
                    program[idx - i].extra_cells_bound,
                    program[idx - j].extra_cells_bound,
                )
        }
        Node::Witness(..) => witness_target_width,
        Node::Fail(..) => unimplemented!(),
        Node::Hidden(..) => 0,
        Node::Ext(..) => 0, // FIXME should fallthrough
        Node::Jet(..) => 0,
    }
}

fn compute_frame_count_bound<Ext: extension::Node>(
    program: &[ProgramNode<Ext>],
    node: &Node<Value, Ext>,
    idx: usize,
) -> usize {
    match *node {
        Node::Iden => 0,
        Node::Unit => 0,
        Node::InjL(i) => program[idx - i].frame_count_bound,
        Node::InjR(i) => program[idx - i].frame_count_bound,
        Node::Take(i) => program[idx - i].frame_count_bound,
        Node::Drop(i) => program[idx - i].frame_count_bound,
        Node::Comp(i, j) => {
            1 + cmp::max(
                program[idx - i].frame_count_bound,
                program[idx - j].frame_count_bound,
            )
        }
        Node::Case(i, j) => cmp::max(
            program[idx - i].frame_count_bound,
            program[idx - j].frame_count_bound,
        ),
        Node::Pair(i, j) => cmp::max(
            program[idx - i].frame_count_bound,
            program[idx - j].frame_count_bound,
        ),
        Node::Disconnect(i, j) => {
            2 + cmp::max(
                program[idx - i].frame_count_bound,
                program[idx - j].frame_count_bound,
            )
        }
        Node::Witness(..) => 0,
        Node::Fail(..) => unimplemented!(),
        Node::Hidden(..) => 0,
        Node::Ext(..) => 0, // FIXME should fallthrough
        Node::Jet(..) => 0,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use exec;

    use bititer::BitIter;
    use extension::{
        dummy::{Node as DummyNode, TxEnv},
        jets::Node as JetsNode,
    };
    use Node;

    #[test]
    fn simple_unit_prog() {
        // vec![0 0 1 0 0 1 0 0] = vec![0x24]
        // prog_len = 1 :vec![0 1 0 0 1 0 0]
        // non a extension or jets node : vec![1 0 0 1 0 0]
        // code = 2 [1 0 ] : vec![0 1 0 0]
        // subcode = 1 [0 1]:  vec![0 0] => Parsed unit node.
        // witness len = 0 vec![0]
        let prog = vec![0x24];
        let prog = Program::<DummyNode>::decode(&mut BitIter::from(prog.into_iter()))
            .expect("decoding program");

        // dbg!(&prog);
        assert_eq!(prog.nodes.len(), 1);
        assert_eq!(prog.nodes[0].node, Node::Unit);
        // Checked against C implementation
        assert_eq!(
            prog.nodes[0].cmr.to_string(),
            "d723083cff3c75e29f296707ecf2750338f100591c86e0c71717f807ff3cf69d",
        );
    }

    #[test]
    fn injl_unit_prog() {
        // 100 01001 00100 0
        // 1000 1001 0010 0000
        let prog = vec![0x89, 0x20];
        let prog = Program::<DummyNode>::decode(&mut BitIter::from(prog.into_iter()))
            .expect("decoding program");

        dbg!(&prog);
        prog.graph_print();
        assert_eq!(prog.nodes.len(), 2);
        assert_eq!(prog.nodes[0].node, Node::Unit);
        assert_eq!(prog.nodes[1].node, Node::InjL(1));

        // Checked against C implementation
        assert_eq!(
            prog.nodes[0].cmr.to_string(),
            "d723083cff3c75e29f296707ecf2750338f100591c86e0c71717f807ff3cf69d",
        );
        // Checked against C implementation
        assert_eq!(
            prog.nodes[1].cmr.to_string(),
            "7a4ebcbd3be89bb9dfd901fdbeff16cfa80aa36363785b14615cbdd3f0ae1f0a"
        );
    }

    #[test]
    fn encode_prog() {
        let mut prog: Vec<Node<(), DummyNode>> = vec![];

        prog.push(Node::Jet(JetsNode::Adder32));
        // prog.push(Node::Case(0, 1));

        let prog =
            Program::from_untyped_nodes(prog, &mut BitIter::from(vec![0x00].into_iter())).unwrap();
        // dbg!(&prog);
        prog.graph_print();
    }

    #[test]
    fn witness_and() {
        let mut prog: Vec<Node<(), DummyNode>> = vec![];

        prog.push(Node::Unit);
        prog.push(Node::InjR(1));
        prog.push(Node::Witness(()));
        prog.push(Node::Case(2, 1));
        prog.push(Node::Witness(()));
        prog.push(Node::Comp(1, 2));

        let prog =
            Program::from_untyped_nodes(prog, &mut BitIter::from(vec![0x80].into_iter())).unwrap();
        dbg!(&prog);
        prog.graph_print();

        let mut mac = exec::BitMachine::for_program(&prog);
        // mac.input(&Value::prod(Value::u1(0), Value::Unit));
        let output = mac.exec(&prog, &TxEnv);

        println!("{}", output);
    }
}
