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

use std::collections::{HashMap, HashSet};
use std::{fmt, io};

use crate::bititer::BitIter;
use crate::core::types::FinalType;
use crate::core::{iter, LinearProgram, Term, TypedNode, TypedProgram, Value};
use crate::encode::BitWriter;
use crate::jet::Application;
use crate::merkle::cmr::Cmr;
use crate::merkle::imr::Imr;
use crate::{encode, Error};

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
    pub fn iter(&self) -> impl ExactSizeIterator<Item = &ProgramNode<App>> + Clone {
        self.nodes.iter()
    }

    /// Return an iterator over the terms of the program.
    pub fn iter_terms(&self) -> impl ExactSizeIterator<Item = &Term<Value, App>> + Clone {
        self.nodes.iter().map(|x| &x.typed.term)
    }

    /// Decode a finalized program from bits.
    pub fn decode<I: Iterator<Item = u8>>(iter: &mut BitIter<I>) -> Result<Self, Error> {
        let program = Program::decode_insane(iter)?;

        if program.has_maximal_sharing() {
            Ok(program)
        } else {
            Err(Error::SharingNotMaximal)
        }
    }

    /// Decode a finalized program from bits.
    ///
    /// **Does not check if the resulting program has maximal sharing.**
    pub fn decode_insane<I: Iterator<Item = u8>>(iter: &mut BitIter<I>) -> Result<Self, Error> {
        let typed_program = TypedProgram::<Value, App>::decode(iter)?;
        let finalized_program = typed_program.finalize_insane();
        Ok(finalized_program)
    }

    /// Encode the program into bits.
    ///
    /// Returns the number of written bits.
    pub fn encode<W: io::Write>(&self, w: &mut BitWriter<W>) -> io::Result<usize> {
        let program_bits = encode::encode_program_no_witness(self.iter_terms(), w)?;
        let witness_bits = encode::encode_witness(iter::into_witness(self.iter_terms()), w)?;
        Ok(program_bits + witness_bits)
    }

    /// Check if the program has maximal sharing.
    ///
    /// This imposes the following conditions:
    /// 1. For hidden nodes, their hash must be unique in the program.
    /// 2. For non-hidden nodes, the triple of their IMR, source type and target type
    ///    must be unique in the program.
    pub fn has_maximal_sharing(&self) -> bool {
        let mut seen_hashes = HashSet::new();
        let mut seen_keys = HashSet::new();

        for node in &self.nodes {
            if let Term::Hidden(h) = node.term() {
                if seen_hashes.contains(h) {
                    return false;
                } else {
                    seen_hashes.insert(h);
                }
            } else {
                let primary_key = (node.imr, node.source_ty().clone(), node.target_ty().clone());

                if seen_keys.contains(&primary_key) {
                    return false;
                } else {
                    seen_keys.insert(primary_key);
                }
            }
        }

        true
    }

    /// Compress the program such that it has maximal sharing (see [`Self::has_maximal_sharing`]).
    pub fn maximize_sharing(self) -> Self {
        let mut shared_program = Vec::new();
        let mut shared_idx = 0;

        let mut primary_key_to_shared_idx = HashMap::new();
        let mut hash_to_shared_idx = HashMap::new();
        let mut unshared_to_shared_idx = HashMap::new();

        for (unshared_idx, node) in self.nodes.into_iter().enumerate() {
            if let Term::Hidden(h) = node.term() {
                if let Some(previous_shared_idx) = hash_to_shared_idx.get(h) {
                    unshared_to_shared_idx.insert(unshared_idx, *previous_shared_idx);
                    continue;
                }

                hash_to_shared_idx.insert(*h, shared_idx);
            } else {
                let primary_key = (node.imr, node.source_ty().tmr, node.target_ty().tmr);

                if let Some(previous_shared_idx) = primary_key_to_shared_idx.get(&primary_key) {
                    unshared_to_shared_idx.insert(unshared_idx, *previous_shared_idx);
                    continue;
                }

                primary_key_to_shared_idx.insert(primary_key, shared_idx);
            }

            unshared_to_shared_idx.insert(unshared_idx, shared_idx);

            let mut shared_node = node;
            shared_node.typed.term = shared_node.typed.term.into_shared(
                unshared_idx,
                shared_idx,
                &unshared_to_shared_idx,
            );
            shared_node.typed.index = shared_idx;

            shared_program.push(shared_node);
            shared_idx += 1;
        }

        Program {
            nodes: shared_program,
        }
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::bititer::BitIter;
    use crate::core::UntypedProgram;
    use crate::exec::BitMachine;
    use crate::jet;
    use crate::jet::application::Core;

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
    fn execute_empty_witness() {
        let program = UntypedProgram::<(), Core>(vec![Term::Jet(&jet::core::ADD32)])
            .type_check()
            .expect("type check")
            .add_witness(Vec::new())
            .expect("witness")
            .finalize()
            .expect("finalize");

        let mut mac = BitMachine::for_program(&program);
        mac.input(&Value::prod(Value::u32(1), Value::u32(2)));
        let output = mac.exec(&program, &()).expect("execute");

        assert_eq!(Value::prod(Value::u1(0), Value::u32(3)), output);
    }

    #[test]
    fn execute_non_empty_witness() {
        let program: Program<Core> = UntypedProgram(vec![
            Term::Witness(()),
            Term::Witness(()),
            Term::Pair(2, 1),
            Term::Jet(&jet::core::ADD32),
            Term::Comp(2, 1),
        ])
        .type_check()
        .expect("type check")
        .add_witness(vec![Value::u32(1), Value::u32(2)])
        .expect("witness")
        .finalize()
        .expect("finalize");

        let mut mac = BitMachine::for_program(&program);
        let output = mac.exec(&program, &()).expect("execute");

        assert_eq!(Value::prod(Value::u1(0), Value::u32(3)), output);
    }

    #[test]
    fn maximal_sharing_non_hidden() {
        // Same combinator, same CMR
        let minimal = UntypedProgram(vec![Term::Unit, Term::InjR(1)]);
        let duplicate = UntypedProgram(vec![Term::Unit, Term::InjR(1), Term::Unit, Term::InjR(1)]);
        assert!(equal_after_sharing(minimal.clone(), duplicate, Vec::new()));

        // Same combinator, different CMR
        let non_duplicate = UntypedProgram(vec![Term::Unit, Term::InjR(1), Term::InjR(1)]);
        assert!(!equal_after_sharing(minimal, non_duplicate, Vec::new()));
    }

    #[test]
    fn maximal_sharing_hidden() {
        // Same hidden payload
        let minimal = UntypedProgram(vec![Term::Hidden(Cmr::from([0; 32]))]);
        let duplicate = UntypedProgram(vec![
            Term::Hidden(Cmr::from([0; 32])),
            Term::Hidden(Cmr::from([0; 32])),
        ]);
        assert!(equal_after_sharing(minimal.clone(), duplicate, Vec::new()));

        // Different hidden payload
        let non_duplicate = UntypedProgram(vec![
            Term::Hidden(Cmr::from([0; 32])),
            Term::Hidden(Cmr::from([1; 32])),
        ]);
        assert!(!equal_after_sharing(minimal, non_duplicate, Vec::new()));
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

        assert!(equal_after_sharing(
            single_witness.clone(),
            double_witness.clone(),
            vec![Value::u1(0), Value::u1(0)]
        ));
        assert!(!equal_after_sharing(
            single_witness.clone(),
            double_witness.clone(),
            vec![Value::u1(0), Value::u1(1)]
        ));
    }

    fn equal_after_sharing(
        left: UntypedProgram<(), Core>,
        right: UntypedProgram<(), Core>,
        witness: Vec<Value>,
    ) -> bool {
        let unshared_left = left
            .type_check()
            .expect("type check")
            .add_witness(witness.clone())
            .expect("witness")
            .finalize_insane();
        let unshared_right = right
            .type_check()
            .expect("type check")
            .add_witness(witness)
            .expect("witness")
            .finalize_insane();
        assert_ne!(unshared_left, unshared_right);

        let shared_left = unshared_left.maximize_sharing();
        let shared_right = unshared_right.maximize_sharing();
        shared_left == shared_right
    }
}
