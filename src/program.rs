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

use crate::bititer::BitIter;
use crate::cmr::{self, Amr, Cmr};
use crate::core::term::UnTypedProg;
use crate::core::{sha256_value, types};
use crate::{encode, extension};
use crate::{Error, Term, Value};

/// A node in a complete program, with associated metadata
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub struct ProgramNode<Ext> {
    /// The underlying node
    pub node: Term<Value, Ext>,
    /// Its index within the total program
    pub index: usize,
    /// Its Commitment Merkle Root
    pub cmr: Cmr,
    /// Its Annotated Commitment Merkle Root
    pub amr: Amr,
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
            Term::Iden => f.write_str("iden")?,
            Term::Unit => f.write_str("unit")?,
            Term::InjL(i) => write!(f, "injl({})", self.index - i)?,
            Term::InjR(i) => write!(f, "injr({})", self.index - i)?,
            Term::Take(i) => write!(f, "take({})", self.index - i)?,
            Term::Drop(i) => write!(f, "drop({})", self.index - i)?,
            Term::Comp(i, j) => write!(f, "comp({}, {})", self.index - i, self.index - j)?,
            Term::Case(i, j) => write!(f, "case({}, {})", self.index - i, self.index - j)?,
            Term::AssertL(i, j) => write!(f, "assertL({}, {})", self.index - i, self.index - j)?,
            Term::AssertR(i, j) => write!(f, "assertR({}, {})", self.index - i, self.index - j)?,
            Term::Pair(i, j) => write!(f, "pair({}, {})", self.index - i, self.index - j)?,
            Term::Disconnect(i, j) => {
                write!(f, "disconnect({}, {})", self.index - i, self.index - j)?
            }
            Term::Witness(..) => f.write_str("witness")?,
            Term::Hidden(..) => f.write_str("hidden")?,
            Term::Fail(..) => f.write_str("fail")?,
            Term::Ext(ref b) => write!(f, "[ext]{}", b)?,
            Term::Jet(ref j) => write!(f, "[jet]{}", j)?,
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

impl<Ext: extension::Jet> Program<Ext> {
    /// Obtain the node representing the root of the program DAG
    pub fn root_node(&self) -> &ProgramNode<Ext> {
        &self.nodes[self.nodes.len() - 1]
    }

    /// Decode a program from a stream of bits
    pub fn decode<I: Iterator<Item = u8>>(iter: &mut BitIter<I>) -> Result<Program<Ext>, Error> {
        // Decode a bunch of untyped, witness-less nodes
        let nodes = encode::decode_program_no_witness(&mut *iter)?;

        Program::<Ext>::from_untyped_nodes(nodes, iter)
    }

    /// Decode a program from a stream of bits
    pub fn from_untyped_nodes<I: Iterator<Item = u8>>(
        nodes: UnTypedProg<(), Ext>,
        iter: &mut BitIter<I>,
    ) -> Result<Program<Ext>, Error> {
        // Do type-checking
        let typed_nodes = types::type_check(nodes)?;

        // Parse witnesses, if available
        // FIXME actually only read as much as wit_len
        let _wit_len = match iter.next() {
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
                        Term::Iden => Term::Iden,
                        Term::Unit => Term::Unit,
                        Term::InjL(i) => Term::InjL(i),
                        Term::InjR(i) => Term::InjR(i),
                        Term::Take(i) => Term::Take(i),
                        Term::Drop(i) => Term::Drop(i),
                        Term::Comp(i, j) => Term::Comp(i, j),
                        Term::Case(i, j) => Term::Case(i, j),
                        Term::AssertL(i, j) => Term::AssertL(i, j),
                        Term::AssertR(i, j) => Term::AssertR(i, j),
                        Term::Pair(i, j) => Term::Pair(i, j),
                        Term::Disconnect(i, j) => Term::Disconnect(i, j),
                        Term::Witness(()) => Term::Witness(Value::from_bits_and_type(
                            &mut iter.by_ref(),
                            &node.target_ty,
                        )?),
                        Term::Fail(x, y) => Term::Fail(x, y),
                        Term::Hidden(x) => Term::Hidden(x),
                        Term::Ext(e) => Term::Ext(e),
                        Term::Jet(j) => Term::Jet(j),
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
                amr: Amr::from([0; 32]), // assign a random default initially
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
            // Append the typed annotated node to the program
            ret.push(final_node);
            // Use the types to compute amr
            ret[index].amr = compute_amr(&ret, &ret[index].node, index);
        }
        Ok(Program { nodes: ret })
    }

    /// Print out the program in a graphviz-parseable format
    pub fn graph_print(&self) {
        for node in &self.nodes {
            println!(
                "{} [label=\"{}\\n{}\\n{} → {}\"];",
                node.index,
                match &node.node {
                    Term::Iden => "iden".to_owned(),
                    Term::Unit => "unit".to_owned(),
                    Term::InjL(..) => "injl".to_owned(),
                    Term::InjR(..) => "injr".to_owned(),
                    Term::Take(..) => "take".to_owned(),
                    Term::Drop(..) => "drop".to_owned(),
                    Term::Comp(..) => "comp".to_owned(),
                    Term::Case(..) => "case".to_owned(),
                    Term::AssertL(..) => "assertL".to_owned(),
                    Term::AssertR(..) => "assertR".to_owned(),
                    Term::Pair(..) => "pair".to_owned(),
                    Term::Disconnect(..) => "disconnect".to_owned(),
                    Term::Witness(..) => "witness".to_owned(),
                    Term::Hidden(..) => "hidden".to_owned(),
                    Term::Fail(..) => "fail".to_owned(),
                    Term::Ext(e) => format!("[ext]{}", e), // FIXME `ext` and `jet` should passthrough
                    Term::Jet(j) => format!("[jet]{}", j),
                },
                node.index,
                node.source_ty,
                node.target_ty,
            );
            match node.node {
                Term::Iden
                | Term::Unit
                | Term::Witness(..)
                | Term::Hidden(..)
                | Term::Fail(..)
                | Term::Ext(..)
                | Term::Jet(..) => {}
                Term::InjL(i) | Term::InjR(i) | Term::Take(i) | Term::Drop(i) => {
                    println!("  {} -> {};", node.index, node.index - i);
                }
                Term::Comp(i, j)
                | Term::Case(i, j)
                | Term::AssertL(i, j)
                | Term::AssertR(i, j)
                | Term::Pair(i, j)
                | Term::Disconnect(i, j) => {
                    println!("  {} -> {} [color=red];", node.index, node.index - i);
                    println!("  {} -> {} [color=blue];", node.index, node.index - j);
                }
            }
        }
    }
}

fn compute_cmr<Ext: extension::Jet>(
    program: &[ProgramNode<Ext>],
    node: &Term<Value, Ext>,
    idx: usize,
) -> Cmr {
    let cmr_iv = node.cmr_iv();
    match *node {
        Term::Iden
        | Term::Unit
        | Term::Witness(..)
        | Term::Hidden(..)
        | Term::Ext(..)
        | Term::Jet(..) => cmr_iv,
        Term::InjL(i) | Term::InjR(i) | Term::Take(i) | Term::Drop(i) | Term::Disconnect(i, _) => {
            cmr_iv.update_1(program[idx - i].cmr)
        }
        Term::Comp(i, j)
        | Term::Case(i, j)
        | Term::Pair(i, j)
        | Term::AssertL(i, j)
        | Term::AssertR(i, j) => cmr_iv.update(program[idx - i].cmr, program[idx - j].cmr),
        Term::Fail(..) => unimplemented!(),
    }
}

fn compute_amr<Ext: extension::Jet>(
    program: &[ProgramNode<Ext>],
    node: &Term<Value, Ext>,
    idx: usize,
) -> Amr {
    let amr_iv = node.amr_iv();
    match node {
        Term::Iden | Term::Unit => amr_iv.update_1(program[idx].source_ty.tmr.into()),
        Term::InjL(i) | Term::InjR(i) => {
            // anon_i denotes the type annotation for i
            if let types::FinalTypeInner::Sum(anno_b, anno_c) = &program[idx].target_ty.ty {
                amr_iv
                    .update(program[idx].source_ty.tmr.into(), anno_b.tmr.into())
                    .update(anno_c.tmr.into(), program[idx - i].amr)
            } else {
                unreachable!("Type checked InjL/InjR must have target type as sum");
            }
        }
        Term::Take(i) | Term::Drop(i) => {
            if let types::FinalTypeInner::Product(anno_a, anno_b) = &program[idx].source_ty.ty {
                amr_iv
                    .update(anno_a.tmr.into(), anno_b.tmr.into())
                    .update(program[idx].target_ty.tmr.into(), program[idx - i].amr)
            } else {
                unreachable!("Type checked Take/Drop must have source type as product");
            }
        }
        Term::Comp(i, j) | Term::Pair(i, j) => amr_iv
            .update_1(program[idx].source_ty.tmr.into())
            .update(
                program[idx - i].target_ty.tmr.into(),
                program[idx - j].target_ty.tmr.into(), // target type of j and idx is same for comp
            )
            .update(program[idx - i].amr, program[idx - j].amr),
        Term::Case(i, j) | Term::AssertL(i, j) | Term::AssertR(i, j) => {
            if let types::FinalTypeInner::Product(anno_ab, anno_c) = &program[idx].source_ty.ty {
                // Putting two conditions in same pattern is annoying because of RefCell matching
                if let types::FinalTypeInner::Sum(anno_a, anno_b) = &anno_ab.ty {
                    amr_iv
                        .update(anno_a.tmr.into(), anno_b.tmr.into())
                        .update(anno_c.tmr.into(), program[idx].target_ty.tmr.into())
                        .update(program[idx - i].amr, program[idx - j].amr)
                } else {
                    unreachable!("Case expression typecheck");
                }
            } else {
                unreachable!("Case expression typecheck");
            }
        }
        Term::Disconnect(i, j) => {
            if let types::FinalTypeInner::Product(anno_b, anno_d) = &program[idx].target_ty.ty {
                debug_assert!(anno_d.tmr == program[idx - j].target_ty.tmr);
                amr_iv
                    .update(program[idx].source_ty.tmr.into(), anno_b.tmr.into())
                    .update(
                        program[idx - j].source_ty.tmr.into(),
                        program[idx - j].target_ty.tmr.into(),
                    )
                    .update(program[idx - i].amr, program[idx - j].amr)
            } else {
                unreachable!("Type checked Take/Drop must have source type as product");
            }
        }
        Term::Witness(ref data) => amr_iv.update_1(program[idx].source_ty.tmr.into()).update(
            program[idx].target_ty.tmr.into(),
            cmr::Amr::from(sha256_value(data.clone())),
        ),
        Term::Fail(..) => unimplemented!(),
        Term::Hidden(..) | Term::Ext(..) | Term::Jet(..) => amr_iv,
    }
}

fn compute_extra_cells_bound<Ext: extension::Jet>(
    program: &[ProgramNode<Ext>],
    node: &Term<Value, Ext>,
    idx: usize,
    witness_target_width: usize,
) -> usize {
    match *node {
        Term::Iden => 0,
        Term::Unit => 0,
        Term::InjL(i) => program[idx - i].extra_cells_bound,
        Term::InjR(i) => program[idx - i].extra_cells_bound,
        Term::Take(i) => program[idx - i].extra_cells_bound,
        Term::Drop(i) => program[idx - i].extra_cells_bound,
        Term::Comp(i, j) => {
            program[idx - i].target_ty.bit_width()
                + cmp::max(
                    program[idx - i].extra_cells_bound,
                    program[idx - j].extra_cells_bound,
                )
        }
        Term::Case(i, j) | Term::AssertL(i, j) | Term::AssertR(i, j) => cmp::max(
            program[idx - i].extra_cells_bound,
            program[idx - j].extra_cells_bound,
        ),
        Term::Pair(i, j) => cmp::max(
            program[idx - i].extra_cells_bound,
            program[idx - j].extra_cells_bound,
        ),
        Term::Disconnect(i, j) => {
            program[idx - i].source_ty.bit_width()
                + program[idx - i].target_ty.bit_width()
                + cmp::max(
                    program[idx - i].extra_cells_bound,
                    program[idx - j].extra_cells_bound,
                )
        }
        Term::Witness(..) => witness_target_width,
        Term::Fail(..) => unimplemented!(),
        Term::Hidden(..) => 0,
        Term::Ext(..) => 0, // FIXME should fallthrough
        Term::Jet(..) => 0,
    }
}

fn compute_frame_count_bound<Ext: extension::Jet>(
    program: &[ProgramNode<Ext>],
    node: &Term<Value, Ext>,
    idx: usize,
) -> usize {
    match *node {
        Term::Iden => 0,
        Term::Unit => 0,
        Term::InjL(i) => program[idx - i].frame_count_bound,
        Term::InjR(i) => program[idx - i].frame_count_bound,
        Term::Take(i) => program[idx - i].frame_count_bound,
        Term::Drop(i) => program[idx - i].frame_count_bound,
        Term::Comp(i, j) => {
            1 + cmp::max(
                program[idx - i].frame_count_bound,
                program[idx - j].frame_count_bound,
            )
        }
        Term::Case(i, j) | Term::AssertL(i, j) | Term::AssertR(i, j) => cmp::max(
            program[idx - i].frame_count_bound,
            program[idx - j].frame_count_bound,
        ),
        Term::Pair(i, j) => cmp::max(
            program[idx - i].frame_count_bound,
            program[idx - j].frame_count_bound,
        ),
        Term::Disconnect(i, j) => {
            2 + cmp::max(
                program[idx - i].frame_count_bound,
                program[idx - j].frame_count_bound,
            )
        }
        Term::Witness(..) => 0,
        Term::Fail(..) => unimplemented!(),
        Term::Hidden(..) => 0,
        Term::Ext(..) => 0, // FIXME should fallthrough
        Term::Jet(..) => 0,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::exec;

    use crate::bititer::BitIter;
    use crate::extension::{
        dummy::{DummyNode, TxEnv},
        jets::JetsNode,
    };
    use crate::Term;

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

        assert_eq!(prog.nodes.len(), 1);
        assert_eq!(prog.nodes[0].node, Term::Unit);
        // Checked against C implementation
        assert_eq!(
            prog.nodes[0].cmr.to_string(),
            "62274a89833ece8ba5ff57b28118c0063d3d4a85dd25aae06f87617604402715",
        );
    }

    #[test]
    fn injl_unit_prog() {
        // 100 01001 00100 0
        // 1000 1001 0010 0000
        let prog = vec![0x89, 0x20];
        let prog = Program::<DummyNode>::decode(&mut BitIter::from(prog.into_iter()))
            .expect("decoding program");

        prog.graph_print();
        assert_eq!(prog.nodes.len(), 2);
        assert_eq!(prog.nodes[0].node, Term::Unit);
        assert_eq!(prog.nodes[1].node, Term::InjL(1));

        // Checked against C implementation
        assert_eq!(
            prog.nodes[0].cmr.to_string(),
            "62274a89833ece8ba5ff57b28118c0063d3d4a85dd25aae06f87617604402715",
        );
        // Checked against C implementation
        assert_eq!(
            prog.nodes[1].cmr.to_string(),
            "bd0cce93e713a2ae961bf91c7d113edb0671c7869c72251364682ac8977eade7"
        );
    }

    #[test]
    fn encode_prog() {
        let mut prog: Vec<Term<(), DummyNode>> = vec![];

        prog.push(Term::Jet(JetsNode::Adder32));
        // prog.push(Node::Case(0, 1));

        let prog = Program::from_untyped_nodes(
            UnTypedProg(prog),
            &mut BitIter::from(vec![0x00].into_iter()),
        )
        .unwrap();
        prog.graph_print();
    }

    #[test]
    fn witness_and() {
        let mut prog: Vec<Term<(), DummyNode>> = vec![];

        prog.push(Term::Unit);
        prog.push(Term::InjR(1));
        prog.push(Term::Witness(()));
        prog.push(Term::Case(2, 1));
        prog.push(Term::Witness(()));
        prog.push(Term::Comp(1, 2));

        let prog = Program::from_untyped_nodes(
            UnTypedProg(prog),
            &mut BitIter::from(vec![0x80].into_iter()),
        )
        .unwrap();
        prog.graph_print();

        let mut mac = exec::BitMachine::for_program(&prog);
        // mac.input(&Value::prod(Value::u1(0), Value::Unit));
        let output = mac.exec(&prog, &TxEnv).unwrap();

        println!("{}", output);
    }
}
