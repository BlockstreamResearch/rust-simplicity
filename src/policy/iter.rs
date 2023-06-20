// Simplcity Policy language
// Written in 2023 by
//     Andrew Poelstra <apoelstra@blockstream.com>
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

//! AST Iterators
//!

use elements_miniscript::ToPublicKey;

use super::ast::Fragment;
use super::Policy;

pub struct RtlPolicyIterator<'pol, Pk: ToPublicKey> {
    stack: Vec<(&'pol Policy<Pk>, usize)>,
}

impl<'pol, Pk: ToPublicKey> RtlPolicyIterator<'pol, Pk> {
    /// Creates a new RTL post-order iterator rooted at the given policy
    pub fn new(root: &'pol Policy<Pk>) -> Self {
        let mut ret = RtlPolicyIterator {
            stack: Vec::with_capacity(128),
        };
        ret.push_new(root);
        ret
    }

    /// Internal helper function to push a fresh subpolicy onto the stack
    fn push_new(&mut self, pol: &'pol Policy<Pk>) {
        let max_index = match pol.fragment {
            Fragment::Trivial => 0,
            Fragment::Unsatisfiable(..) => 0,
            Fragment::Key(..) => 0,
            Fragment::After(..) => 0,
            Fragment::Older(..) => 0,
            Fragment::Sha256(..) => 0,
            Fragment::And { .. } => 2,
            Fragment::Or { .. } => 2,
            Fragment::Threshold(k, _) => k,
        };
        self.stack.push((pol, max_index));
    }
}

impl<'pol, Pk: ToPublicKey> Iterator for RtlPolicyIterator<'pol, Pk> {
    type Item = &'pol Policy<Pk>;

    fn next(&mut self) -> Option<&'pol Policy<Pk>> {
        loop {
            let (top, n) = self.stack.pop()?;
            if n == 0 {
                return Some(top);
            }
            self.stack.push((top, n - 1));

            match (&top.fragment, n) {
                (Fragment::Trivial, _)
                | (Fragment::Unsatisfiable(..), _)
                | (Fragment::Key(..), _)
                | (Fragment::After(..), _)
                | (Fragment::Older(..), _)
                | (Fragment::Sha256(..), _) => unreachable!(),
                (Fragment::And { left, right: _ }, 1) => self.push_new(left),
                (Fragment::And { left: _, right }, 2) => self.push_new(right),
                (Fragment::And { .. }, _) => unreachable!(),
                (Fragment::Or { left, right: _ }, 1) => self.push_new(left),
                (Fragment::Or { left: _, right }, 2) => self.push_new(right),
                (Fragment::Or { .. }, _) => unreachable!(),
                (Fragment::Threshold(_, summands), i) => self.push_new(&summands[i - 1]),
            }
        }
    }
}
