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

//! Iterators over DAGs

use std::collections::HashSet;
use std::hash::Hash;

/// Structure that can be iterated like a DAG _(directed acyclic graph)_.
pub trait DagIterable: Sized {
    /// Node in the DAG
    type Node: Copy + Eq + Hash;

    /// Return the DAG root, if the DAG is nonempty.
    fn root(&self) -> Option<Self::Node>;

    /// Return the left child of the given `node`.
    fn left_of(&self, node: Self::Node) -> Option<Self::Node>;

    /// Return the right child of the given `node`.
    fn right_of(&self, node: Self::Node) -> Option<Self::Node>;

    /// Return a post-order iterator over the DAG.
    fn iter_post_order(&self) -> PostOrderIter<Self> {
        PostOrderIter::new(self)
    }
}

/// Iterates over a DAG in _post order_.
/// That means, left children appear before right ones, and children appear before their parent.
/// Shared nodes appear only once at their leftmost position.
#[derive(Clone, Debug)]
pub struct PostOrderIter<'a, D: DagIterable> {
    dag: &'a D,
    stack: Vec<D::Node>,
    maybe_current: Option<D::Node>,
    visited: HashSet<D::Node>,
}

impl<'a, D: DagIterable> PostOrderIter<'a, D> {
    /// Create a new iterator from the given `dag`.
    pub fn new(dag: &'a D) -> Self {
        PostOrderIter {
            dag,
            stack: Vec::new(),
            maybe_current: dag.root(),
            visited: HashSet::new(),
        }
    }
}

impl<'a, D: DagIterable> Iterator for PostOrderIter<'a, D> {
    type Item = D::Node;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if let Some(current) = self.maybe_current {
                self.stack.push(current);

                if let Some(left) = self.dag.left_of(current) {
                    if !self.visited.contains(&left) {
                        self.maybe_current = Some(left);
                        continue;
                    }
                }
                // else
                self.maybe_current = None;
            } else if let Some(top) = self.stack.last() {
                if let Some(right) = self.dag.right_of(*top) {
                    if !self.visited.contains(&right) {
                        self.maybe_current = Some(right);
                        continue;
                    }
                }
                // else
                let top = self.stack.pop().unwrap();
                self.visited.insert(top);

                return Some(top);
            } else {
                return None;
            }
        }
    }
}
