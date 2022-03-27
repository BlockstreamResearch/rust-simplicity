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

use std::collections::HashMap;
use std::{cmp, fmt, sync::Arc};

use crate::bititer::BitIter;
use crate::core::term::UntypedProgram;
use crate::core::types;
use crate::core::types::{FinalType, TypedNode, TypedProgram};
use crate::merkle::cmr::Cmr;
use crate::merkle::common::{MerkleRoot, TermMerkleRoot};
use crate::merkle::imr::Imr;
use crate::{encode, extension};
use crate::{Error, Term, Value};

/// Single, finalized Simplicity node.
/// Includes witness data (encoded as [`Value`]).
/// May include Bitcoin/Elements extensions (see [`Term`]).
///
/// A node consists of a combinator, its payload (see [`Term`]),
/// its source and target types (see [`TypedNode`]),
/// as well as additional metadata.
/// A list of nodes forms a finalized Simplicity program,
/// which represents a finalized Simplicity DAG.
///
/// Nodes have no meaning without a program.
/// Finalized programs are executed on the Bit Machine.
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub struct ProgramNode<Ext> {
    /// Combinator and payload with witness data
    pub node: Term<Value, Ext>,
    /// Index of node in encompassing program
    pub index: usize,
    /// Commitment Merkle root of node
    pub cmr: Cmr,
    /// Identity Merkle root of node
    pub imr: Imr,
    /// Source type of combinator
    pub source_ty: Arc<types::FinalType>,
    /// Target type of combinator
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

/// Finalized Simplicity program,
/// i.e., program of finalized Simplicity nodes (see [`ProgramNode`]).
///
/// Finalized programs are executed on the Bit Machine.
#[derive(Debug, Eq, PartialEq)]
pub struct Program<Ext> {
    /// List of finalized nodes
    pub nodes: Vec<ProgramNode<Ext>>,
}

impl<Ext: extension::Jet> Program<Ext> {
    /// Obtain the node representing the root of the program DAG
    pub fn root_node(&self) -> &ProgramNode<Ext> {
        &self.nodes[self.nodes.len() - 1]
    }

    /// Decode a program from a stream of bits
    pub fn decode<I: Iterator<Item = u8>>(bits: &mut BitIter<I>) -> Result<Program<Ext>, Error> {
        let untyped_program = encode::decode_program_no_witness(&mut *bits)?;
        Program::<Ext>::from_untyped_program(untyped_program, bits)
    }

    /// Decode a program from a stream of bits
    pub fn from_untyped_program<I: Iterator<Item = u8>>(
        untyped_program: UntypedProgram<(), Ext>,
        witness_bits: &mut BitIter<I>,
    ) -> Result<Program<Ext>, Error> {
        let typed_program = types::type_check(untyped_program)?;

        // Parse witnesses, if available
        // FIXME actually only read as much as wit_len
        let _wit_len = match witness_bits.next() {
            Some(false) => 0,
            Some(true) => encode::decode_natural(&mut *witness_bits, None)?,
            None => return Err(Error::EndOfStream),
        };

        let typed_program = add_witness_data(typed_program, witness_bits)?;
        let finalized_program = compress_and_finalize(typed_program);
        Ok(finalized_program)
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

fn add_witness_data<Ext, I>(
    typed_program: TypedProgram<(), Ext>,
    witness_bits: &mut BitIter<I>,
) -> Result<TypedProgram<Value, Ext>, Error>
where
    I: Iterator<Item = u8>,
{
    let ret = typed_program
        .0
        .into_iter()
        .map::<Result<_, Error>, _>(|node| {
            Ok(TypedNode {
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
                        witness_bits.by_ref(),
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

    Ok(TypedProgram(ret))
}

/// Primary key for non-hidden nodes in shared (typed) Simplicity DAGs.
/// _[`Term::Hidden`] nodes have their hash as primary key_
type PrimaryKey = (Imr, Arc<FinalType>, Arc<FinalType>);

fn compress_and_finalize<Ext: extension::Jet>(
    typed_program: TypedProgram<Value, Ext>,
) -> Program<Ext> {
    let mut shared_program = Vec::<ProgramNode<Ext>>::new();
    let mut shared_idx = 0;

    let mut primary_key_to_shared_idx: HashMap<PrimaryKey, usize> = HashMap::new();
    let mut hash_to_shared_idx: HashMap<Cmr, usize> = HashMap::new();
    let mut unshared_to_shared_idx: HashMap<usize, usize> = HashMap::new();

    for (unshared_idx, typed_node) in typed_program.0.into_iter().enumerate() {
        let shared_node =
            typed_node
                .node
                .into_shared(unshared_idx, shared_idx, &unshared_to_shared_idx);
        let imr = compute_imr(
            &shared_program,
            &shared_node,
            shared_idx,
            &typed_node.target_ty,
        );

        if let Term::Hidden(h) = &shared_node {
            if let Some(previous_shared_idx) = hash_to_shared_idx.get(h) {
                unshared_to_shared_idx.insert(unshared_idx, *previous_shared_idx);
                continue;
            }

            hash_to_shared_idx.insert(*h, shared_idx);
        } else {
            let primary_key = (
                imr,
                typed_node.source_ty.clone(),
                typed_node.target_ty.clone(),
            );

            if let Some(previous_shared_idx) = primary_key_to_shared_idx.get(&primary_key) {
                unshared_to_shared_idx.insert(unshared_idx, *previous_shared_idx);
                continue;
            }

            primary_key_to_shared_idx.insert(primary_key, shared_idx);
        }

        unshared_to_shared_idx.insert(unshared_idx, shared_idx);

        let cmr = compute_cmr(&shared_program, &shared_node, shared_idx);
        let extra_cells_bound = compute_extra_cells_bound(
            &shared_program,
            &shared_node,
            shared_idx,
            typed_node.target_ty.bit_width(),
        );
        let frame_count_bound =
            compute_frame_count_bound(&shared_program, &shared_node, shared_idx);

        let finalized_node = ProgramNode {
            node: shared_node,
            index: shared_idx,
            cmr,
            imr,
            source_ty: typed_node.source_ty,
            target_ty: typed_node.target_ty,
            extra_cells_bound,
            frame_count_bound,
        };

        shared_program.push(finalized_node);
        shared_idx += 1;
    }

    Program {
        nodes: shared_program,
    }
}

fn compute_cmr<Ext: extension::Jet>(
    program: &[ProgramNode<Ext>],
    node: &Term<Value, Ext>,
    idx: usize,
) -> Cmr {
    let cmr_iv = Cmr::get_iv(node);

    match *node {
        Term::Iden
        | Term::Unit
        | Term::Witness(..)
        | Term::Fail(..)
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
    }
}

fn compute_imr<Ext: extension::Jet>(
    program: &[ProgramNode<Ext>],
    node: &Term<Value, Ext>,
    idx: usize,
    target_ty: &FinalType,
) -> Imr {
    let imr_iv = Imr::get_iv(node);

    match *node {
        Term::Iden
        | Term::Unit
        | Term::Fail(..)
        | Term::Hidden(..)
        | Term::Ext(..)
        | Term::Jet(..) => imr_iv,
        Term::InjL(i) | Term::InjR(i) | Term::Take(i) | Term::Drop(i) => {
            imr_iv.update_1(program[idx - i].imr)
        }
        Term::Comp(i, j)
        | Term::Case(i, j)
        | Term::Pair(i, j)
        | Term::AssertL(i, j)
        | Term::AssertR(i, j)
        | Term::Disconnect(i, j) => imr_iv.update(program[idx - i].imr, program[idx - j].imr),
        Term::Witness(ref value) => imr_iv.update_value(value, target_ty),
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
        let prog: UntypedProgram<(), DummyNode> = UntypedProgram(vec![
            Term::Jet(JetsNode::Adder32),
            // Node::Case(0, 1),
        ]);

        let prog = Program::from_untyped_program(prog, &mut BitIter::from(vec![0x00].into_iter()))
            .unwrap();
        prog.graph_print();
    }

    #[test]
    fn witness_and() {
        let prog: UntypedProgram<(), DummyNode> = UntypedProgram(vec![
            Term::Unit,
            Term::InjR(1),
            Term::Witness(()),
            Term::Case(2, 1),
            Term::Witness(()),
            Term::Comp(1, 2),
        ]);

        let prog = Program::from_untyped_program(prog, &mut BitIter::from(vec![0x80].into_iter()))
            .unwrap();
        prog.graph_print();

        let mut mac = exec::BitMachine::for_program(&prog);
        // mac.input(&Value::prod(Value::u1(0), Value::Unit));
        let output = mac.exec(&prog, &TxEnv).unwrap();

        println!("{}", output);
    }

    #[test]
    fn maximal_sharing_non_hidden() {
        // Same combinator, same CMR
        let minimal = UntypedProgram(vec![Term::Unit, Term::InjR(1)]);
        let duplicate = UntypedProgram(vec![Term::Unit, Term::InjR(1), Term::Unit, Term::InjR(1)]);
        assert!(equal_after_sharing(minimal.clone(), duplicate, &[0x00]));

        // Same combinator, different CMR
        let non_duplicate = UntypedProgram(vec![Term::Unit, Term::InjR(1), Term::InjR(1)]);
        assert!(!equal_after_sharing(minimal, non_duplicate, &[0x00]));
    }

    #[test]
    fn maximal_sharing_hidden() {
        // Same hidden payload
        let minimal = UntypedProgram(vec![Term::Hidden(Cmr::from([0; 32]))]);
        let duplicate = UntypedProgram(vec![
            Term::Hidden(Cmr::from([0; 32])),
            Term::Hidden(Cmr::from([0; 32])),
        ]);
        assert!(equal_after_sharing(minimal.clone(), duplicate, &[0x00]));

        // Different hidden payload
        let non_duplicate = UntypedProgram(vec![
            Term::Hidden(Cmr::from([0; 32])),
            Term::Hidden(Cmr::from([1; 32])),
        ]);
        assert!(!equal_after_sharing(minimal, non_duplicate, &[0x00]));
    }

    #[test]
    fn maximal_sharing_witness() {
        // Program that takes 2 bits of witness data
        let double_witness = UntypedProgram(vec![
            Term::Unit,
            Term::InjR(1),
            Term::Witness(()),
            // cond
            Term::Drop(1),
            Term::Drop(3),
            Term::Case(2, 1),
            // begin non-duplicate
            Term::Witness(()),
            Term::Drop(1),
            Term::Case(1, 4),
        ]);
        // Same program with maximum sharing, provided that both witness bits are equal
        let single_witness = UntypedProgram(vec![
            Term::Unit,
            Term::InjR(1),
            Term::Witness(()),
            Term::Drop(1),
            Term::Drop(3),
            Term::Case(2, 1),
        ]);

        // Leading '0' bit + two '0' witness bits
        assert!(equal_after_sharing(
            single_witness.clone(),
            double_witness.clone(),
            &[0x00]
        ));
        // Leading '0' bit + '1' witness bit + '0' witness bit
        assert!(!equal_after_sharing(
            single_witness,
            double_witness,
            &[0x55]
        ));
    }

    fn equal_after_sharing(
        left: UntypedProgram<(), DummyNode>,
        right: UntypedProgram<(), DummyNode>,
        witness_bytes: &[u8],
    ) -> bool {
        let shared_left =
            Program::from_untyped_program(left, &mut BitIter::from(witness_bytes.iter().cloned()))
                .unwrap();
        let shared_right =
            Program::from_untyped_program(right, &mut BitIter::from(witness_bytes.iter().cloned()))
                .unwrap();

        shared_left == shared_right
    }
}
