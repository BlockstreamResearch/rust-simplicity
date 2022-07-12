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
use std::{fmt, sync::Arc};

use crate::analysis;
use crate::bititer::BitIter;
use crate::core::types::FinalType;
use crate::core::{LinearProgram, Term, TypedNode, TypedProgram, UntypedProgram, Value};
use crate::jet::Application;
use crate::merkle::cmr::Cmr;
use crate::merkle::imr;
use crate::merkle::imr::Imr;
use crate::Error;

/// Simplicity node with full metadata for the time of redemption.
/// This type of node is executed on the Bit Machine.
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct ProgramNode<App: Application> {
    /// Node with commitment metadata
    pub typed: TypedNode<Value, App>,
    /// Identity Merkle root of the node
    pub imr: Imr,
    /// Upper bound on the number of cells required in the Bit Machine by this node
    pub extra_cells_bound: usize,
    /// Upper bound on the number of frames required in the Bit Machine by this node
    pub frame_count_bound: usize,
}

impl<App: Application> fmt::Display for ProgramNode<App> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(&self.typed, f)
    }
}

impl<App: Application> ProgramNode<App> {
    /// Return the underlying term of the node.
    pub fn term(&self) -> &Term<Value, App> {
        &self.typed.term
    }

    /// Return the source type of the node.
    pub fn source_ty(&self) -> &FinalType {
        &self.typed.source_ty
    }

    /// Return the target type of the node.
    pub fn target_ty(&self) -> &FinalType {
        &self.typed.target_ty
    }

    /// Return the index of the node inside the surrounding program.
    pub fn index(&self) -> usize {
        self.typed.index
    }

    /// Return the CMR of the node.
    pub fn cmr(&self) -> Cmr {
        self.typed.cmr
    }
}

/// Finalized Simplicity program (see [`ProgramNode`]).
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct Program<App: Application> {
    pub nodes: Vec<ProgramNode<App>>,
}

impl<App: Application> Program<App> {
    /// Return an iterator over the nodes of the program.
    pub fn iter(&self) -> impl Iterator<Item = &ProgramNode<App>> {
        self.nodes.iter()
    }

    /// Decode a program from a stream of bits
    pub fn decode<I: Iterator<Item = u8>>(iter: &mut BitIter<I>) -> Result<Program<App>, Error> {
        let untyped_program = UntypedProgram::decode(iter)?;
        Program::<App>::from_untyped_program(untyped_program, iter)
    }

    /// Decode a program from a stream of bits
    pub fn from_untyped_program<I: Iterator<Item = u8>>(
        untyped_program: UntypedProgram<(), App>,
        iter: &mut BitIter<I>,
    ) -> Result<Program<App>, Error> {
        let typed_program = untyped_program.type_check()?;
        let witness_program = typed_program.decode_witness(iter)?;
        let finalized_program = compress_and_finalize(witness_program);
        Ok(finalized_program)
    }
}

impl<App: Application> LinearProgram for Program<App> {
    type Node = ProgramNode<App>;

    fn is_empty(&self) -> bool {
        self.nodes.is_empty()
    }

    fn len(&self) -> usize {
        self.nodes.len()
    }

    fn root(&self) -> &Self::Node {
        &self.nodes[self.nodes.len() - 1]
    }
}

impl<App: Application> IntoIterator for Program<App> {
    type Item = ProgramNode<App>;
    type IntoIter = std::vec::IntoIter<ProgramNode<App>>;

    fn into_iter(self) -> Self::IntoIter {
        self.nodes.into_iter()
    }
}

/// Primary key for non-hidden nodes in shared (typed) Simplicity DAGs.
/// _[`Term::Hidden`] nodes have their hash as primary key_
type PrimaryKey = (Imr, Arc<FinalType>, Arc<FinalType>);

fn compress_and_finalize<App: Application>(
    typed_program: TypedProgram<Value, App>,
) -> Program<App> {
    let mut shared_program = Vec::<ProgramNode<App>>::new();
    let mut shared_idx = 0;

    let mut primary_key_to_shared_idx: HashMap<PrimaryKey, usize> = HashMap::new();
    let mut hash_to_shared_idx: HashMap<Cmr, usize> = HashMap::new();
    let mut unshared_to_shared_idx: HashMap<usize, usize> = HashMap::new();

    for (unshared_idx, typed_node) in typed_program.0.into_iter().enumerate() {
        let shared_node = TypedNode {
            term: typed_node
                .term
                .into_shared(unshared_idx, shared_idx, &unshared_to_shared_idx),
            source_ty: typed_node.source_ty,
            target_ty: typed_node.target_ty,
            index: shared_idx,
            cmr: typed_node.cmr,
        };
        let imr = imr::compute_imr(&shared_program, &shared_node, shared_idx);

        if let Term::Hidden(h) = &shared_node.term {
            if let Some(previous_shared_idx) = hash_to_shared_idx.get(h) {
                unshared_to_shared_idx.insert(unshared_idx, *previous_shared_idx);
                continue;
            }

            hash_to_shared_idx.insert(*h, shared_idx);
        } else {
            let primary_key = (
                imr,
                shared_node.source_ty.clone(),
                shared_node.target_ty.clone(),
            );

            if let Some(previous_shared_idx) = primary_key_to_shared_idx.get(&primary_key) {
                unshared_to_shared_idx.insert(unshared_idx, *previous_shared_idx);
                continue;
            }

            primary_key_to_shared_idx.insert(primary_key, shared_idx);
        }

        unshared_to_shared_idx.insert(unshared_idx, shared_idx);

        let extra_cells_bound = analysis::compute_extra_cells_bound(&shared_program, &shared_node);
        let frame_count_bound = analysis::compute_frame_count_bound(&shared_program, &shared_node);

        let finalized_node = ProgramNode {
            typed: shared_node,
            imr,
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::bititer::BitIter;
    use crate::jet::application::Core;
    use crate::{exec, jet};

    #[test]
    fn simple_unit_prog() {
        // vec![0 0 1 0 0 1 0 0] = vec![0x24]
        // prog_len = 1 :vec![0 1 0 0 1 0 0]
        // non a extension or jets node : vec![1 0 0 1 0 0]
        // code = 2 [1 0 ] : vec![0 1 0 0]
        // subcode = 1 [0 1]:  vec![0 0] => Parsed unit node.
        // witness len = 0 vec![0]
        let prog = vec![0x24];
        let prog = Program::<Core>::decode(&mut BitIter::from(prog.into_iter()))
            .expect("decoding program");

        assert_eq!(prog.nodes.len(), 1);
        assert_eq!(prog.nodes[0].typed.term, Term::Unit);
        // Checked against C implementation
        assert_eq!(
            prog.nodes[0].cmr().to_string(),
            "62274a89833ece8ba5ff57b28118c0063d3d4a85dd25aae06f87617604402715",
        );
    }

    #[test]
    fn injl_unit_prog() {
        // 100 01001 00100 0
        // 1000 1001 0010 0000
        let prog = vec![0x89, 0x20];
        let prog = Program::<Core>::decode(&mut BitIter::from(prog.into_iter()))
            .expect("decoding program");

        assert_eq!(prog.nodes.len(), 2);
        assert_eq!(prog.nodes[0].typed.term, Term::Unit);
        assert_eq!(prog.nodes[1].typed.term, Term::InjL(1));

        // Checked against C implementation
        assert_eq!(
            prog.nodes[0].cmr().to_string(),
            "62274a89833ece8ba5ff57b28118c0063d3d4a85dd25aae06f87617604402715",
        );
        // Checked against C implementation
        assert_eq!(
            prog.nodes[1].cmr().to_string(),
            "bd0cce93e713a2ae961bf91c7d113edb0671c7869c72251364682ac8977eade7"
        );
    }

    #[test]
    fn encode_prog() {
        let prog: UntypedProgram<(), Core> = UntypedProgram(vec![
            Term::Jet(&jet::core::ADD32),
            // Node::Case(0, 1),
        ]);

        Program::from_untyped_program(prog, &mut BitIter::from(vec![0x00].into_iter())).unwrap();
    }

    #[test]
    fn witness_and() {
        let prog: UntypedProgram<(), Core> = UntypedProgram(vec![
            Term::Unit,
            Term::InjR(1),
            Term::Witness(()),
            Term::Case(2, 1),
            Term::Witness(()),
            Term::Comp(1, 2),
        ]);

        // Witness [Value::u1(0), Value::Unit]: '1' + '10' + '0'
        let mut iter = BitIter::from([0b_1_10_0_0000].iter().cloned());
        let prog = Program::from_untyped_program(prog, &mut iter).unwrap();

        let mut mac = exec::BitMachine::for_program(&prog);
        let output = mac.exec(&prog, &()).unwrap();

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

        // Witness [Value::u1(0), Value::u1(0)]: '1' + '100' + '00'
        assert!(equal_after_sharing(
            single_witness.clone(),
            double_witness.clone(),
            &[0b_1_100_00_00]
        ));
        // Witness [Value::u1(0), Value::u1(1)]: '1' + '100' + '01'
        assert!(!equal_after_sharing(
            single_witness,
            double_witness,
            &[0b_1_100_01_00]
        ));
    }

    fn equal_after_sharing(
        left: UntypedProgram<(), Core>,
        right: UntypedProgram<(), Core>,
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
