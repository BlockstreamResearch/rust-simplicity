// Rust Simplicity Library
// Written in 2022 by
//   Christian Lewe <clewe@blockstream.com>
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

use crate::core::{ProgramNode, Term, TypedNode, Value};
use crate::jet::Application;
use std::cmp;

/// Return an upper bound on the number of cells required by the given `node`
/// inside the given `program` during execution on the Bit Machine.
pub(crate) fn compute_extra_cells_bound<App: Application>(
    program: &[ProgramNode<App>],
    node: &TypedNode<Value, App>,
) -> usize {
    match node.term {
        Term::Iden | Term::Unit | Term::Fail(..) | Term::Hidden(..) | Term::Jet(..) => 0,
        Term::InjL(i) | Term::InjR(i) | Term::Take(i) | Term::Drop(i) => {
            program[node.index - i].extra_cells_bound
        }
        Term::Comp(i, j) => {
            program[node.index - i].target_ty().bit_width()
                + cmp::max(
                    program[node.index - i].extra_cells_bound,
                    program[node.index - j].extra_cells_bound,
                )
        }
        Term::Case(i, j) | Term::AssertL(i, j) | Term::AssertR(i, j) | Term::Pair(i, j) => {
            cmp::max(
                program[node.index - i].extra_cells_bound,
                program[node.index - j].extra_cells_bound,
            )
        }
        Term::Disconnect(i, j) => {
            program[node.index - i].source_ty().bit_width()
                + program[node.index - i].target_ty().bit_width()
                + cmp::max(
                    program[node.index - i].extra_cells_bound,
                    program[node.index - j].extra_cells_bound,
                )
        }
        Term::Witness(..) => node.target_ty.bit_width,
    }
}

/// Return an upper bound on the number of frames required by the given `node`
/// inside the given `program` during execution on the Bit Machine.
pub(crate) fn compute_frame_count_bound<App: Application>(
    program: &[ProgramNode<App>],
    node: &TypedNode<Value, App>,
) -> usize {
    match node.term {
        Term::Iden
        | Term::Unit
        | Term::Witness(..)
        | Term::Fail(..)
        | Term::Hidden(..)
        | Term::Jet(..) => 0,
        Term::InjL(i) | Term::InjR(i) | Term::Take(i) | Term::Drop(i) => {
            program[node.index - i].frame_count_bound
        }
        Term::Comp(i, j) => {
            1 + cmp::max(
                program[node.index - i].frame_count_bound,
                program[node.index - j].frame_count_bound,
            )
        }
        Term::Case(i, j) | Term::AssertL(i, j) | Term::AssertR(i, j) | Term::Pair(i, j) => {
            cmp::max(
                program[node.index - i].frame_count_bound,
                program[node.index - j].frame_count_bound,
            )
        }
        Term::Disconnect(i, j) => {
            2 + cmp::max(
                program[node.index - i].frame_count_bound,
                program[node.index - j].frame_count_bound,
            )
        }
    }
}
