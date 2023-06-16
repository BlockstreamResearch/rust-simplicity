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

//! Automatic naming of variables

use simplicity::{CommitNode, core::commit::CommitNodeInner, jet::Jet};

pub struct NameFactory {
    program_idx: usize,
    witness_idx: usize,
    const_idx: usize,
}

impl NameFactory {
    pub fn new() -> Self {
        NameFactory {
            witness_idx: 0,
            program_idx: 0,
            const_idx: 0,
        }
    }

    pub fn new_name<J: Jet>(&mut self, node: &CommitNode<J>) -> String {
        let idx = if let CommitNodeInner::Witness = node.inner() {
            self.witness_idx += 1;
            self.witness_idx
        } else if let CommitNodeInner::Word(..) = node.inner() {
            self.const_idx += 1;
            self.const_idx
        } else {
            self.program_idx += 1;
            self.program_idx
        };
        match node.inner() {
            // Normal combinators get 2-letter names
            CommitNodeInner::Unit => format!("ut{}", idx),
            CommitNodeInner::Iden => format!("id{}", idx),
            CommitNodeInner::InjL(..) => format!("jl{}", idx),
            CommitNodeInner::InjR(..) => format!("jr{}", idx),
            CommitNodeInner::Take(..) => format!("tk{}", idx),
            CommitNodeInner::Drop(..) => format!("dp{}", idx),
            CommitNodeInner::Comp(..) => format!("cp{}", idx),
            CommitNodeInner::Case(..) => format!("cs{}", idx),
            CommitNodeInner::AssertL(..) => format!("al{}", idx),
            CommitNodeInner::AssertR(..) => format!("ar{}", idx),
            CommitNodeInner::Pair(..) => format!("pr{}", idx),
            CommitNodeInner::Jet(..) => format!("jt{}", idx),
            // Fail shouldn't ever be used but it has an encoding, so..
            CommitNodeInner::Fail(..) => format!("FAIL{}", idx),
            // Disconnect is weird so I figured I should make it more visible
            CommitNodeInner::Disconnect(..) => format!("disc{}", idx),
            // Words go in their own section and can also have longer names
            CommitNodeInner::Word(..) => format!("const{}", idx),
            // Ditto for witnesses
            CommitNodeInner::Witness => format!("wit{}", idx),
        }
    }
}


