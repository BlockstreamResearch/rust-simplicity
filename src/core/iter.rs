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

use crate::core::redeem::{RedeemNodeInner, RefWrapper};
use crate::core::types::Type;
use crate::core::Value;
use crate::jet::Application;
use crate::Error;
use std::collections::{HashMap, HashSet};
use std::fmt;
use std::hash::Hash;

/// Structure that can be iterated like a DAG _(directed acyclic graph)_.
pub trait DagIterable: Copy + Eq + Hash {
    /// Return the DAG root, if the DAG is nonempty.
    fn root(self) -> Option<Self>;

    /// Return the left child of the given `node`.
    fn get_left(self) -> Option<Self>;

    /// Return the right child of the given `node`.
    fn get_right(self) -> Option<Self>;

    /// Return a post-order iterator over the DAG.
    fn iter_post_order(&self) -> PostOrderIter<Self> {
        PostOrderIter::new(self)
    }

    /// Display the DAG as an indexed list in post order.
    ///
    /// `display_body()` formats the node body in front of the node indices.
    /// `display_aux()` formats auxiliary items after the node indices.
    fn display<F, G>(
        &self,
        f: &mut fmt::Formatter<'_>,
        display_body: F,
        display_aux: G,
    ) -> fmt::Result
    where
        F: Fn(Self, &mut fmt::Formatter<'_>) -> fmt::Result,
        G: Fn(Self, &mut fmt::Formatter<'_>) -> fmt::Result,
    {
        let it = self.iter_post_order();
        let mut node_to_index = HashMap::new();

        for (index, node) in it.enumerate() {
            write!(f, "{}: ", index)?;
            display_body(node, f)?;

            if let Some(left) = node.get_left() {
                let i_abs = node_to_index.get(&left).unwrap();

                if let Some(right) = node.get_right() {
                    let j_abs = node_to_index.get(&right).unwrap();

                    write!(f, "({}, {})", i_abs, j_abs)?;
                } else {
                    write!(f, "({})", i_abs)?;
                }
            }

            display_aux(node, f)?;
            f.write_str("\n")?;
            node_to_index.insert(node, index);
        }

        Ok(())
    }
}

/// Iterates over a DAG in _post order_.
/// That means, left children appear before right ones, and children appear before their parent.
/// Shared nodes appear only once at their leftmost position.
#[derive(Clone, Debug)]
pub struct PostOrderIter<D: DagIterable> {
    stack: Vec<D>,
    maybe_current: Option<D>,
    visited: HashSet<D>,
}

impl<D: DagIterable> PostOrderIter<D> {
    /// Create a new iterator from the given `dag`.
    pub fn new(dag: &D) -> Self {
        PostOrderIter {
            stack: Vec::new(),
            maybe_current: dag.root(),
            visited: HashSet::new(),
        }
    }
}

impl<D: DagIterable> Iterator for PostOrderIter<D> {
    type Item = D;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if let Some(current) = self.maybe_current {
                self.stack.push(current);

                if let Some(left) = current.get_left() {
                    if !self.visited.contains(&left) {
                        self.maybe_current = Some(left);
                        continue;
                    }
                }
                // else
                self.maybe_current = None;
            } else if let Some(top) = self.stack.last() {
                if let Some(right) = top.get_right() {
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

/// Convenience macro for wrappers of references to structures over
/// `<App: Application>`.
///
/// Implements `Clone`, `Copy`, `Eq` and `Hash` using pointers.
/// Implements [`DagIterable`] using `self.0.get_left()` and `self.0.get_right()`.
#[macro_export]
macro_rules! impl_ref_wrapper {
    ($wrapper:ident) => {
        impl<'a, App: Application> Clone for $wrapper<'a, App> {
            fn clone(&self) -> Self {
                $wrapper(&(self.0).clone())
            }
        }

        impl<'a, App: Application> Copy for $wrapper<'a, App> {}

        impl<'a, App: Application> PartialEq for $wrapper<'a, App> {
            fn eq(&self, other: &Self) -> bool {
                std::ptr::eq(self.0, other.0)
            }
        }

        impl<'a, App: Application> Eq for $wrapper<'a, App> {}

        impl<'a, App: Application> std::hash::Hash for $wrapper<'a, App> {
            fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
                std::ptr::hash(self.0, state)
            }
        }

        impl<'a, App: Application> $crate::core::iter::DagIterable for $wrapper<'a, App> {
            fn root(self) -> Option<Self> {
                Some(self)
            }

            fn get_left(self) -> Option<Self> {
                self.0.get_left().map(|x| RefWrapper(x))
            }

            fn get_right(self) -> Option<Self> {
                self.0.get_right().map(|x| RefWrapper(x))
            }
        }
    };
}

/// Convert the given iterator over [`RefWrapper`]s
/// into an iterator over the contained `Witness` values.
pub fn into_witness<'a, App: Application, I>(iter: I) -> impl Iterator<Item = &'a Value> + Clone
where
    I: Iterator<Item = RefWrapper<'a, App>> + Clone,
{
    iter.filter_map(|node| {
        if let RedeemNodeInner::Witness(value) = &node.0.inner {
            Some(value)
        } else {
            None
        }
    })
}

/// Iterator over witness values that asks for the value type on each iteration.
pub trait WitnessIterator {
    /// Return the next witness value of the given type.
    fn next(&mut self, ty: &Type) -> Result<Value, Error>;

    /// Consume the iterator and check the total witness length.
    fn finish(self) -> Result<(), Error>;
}

impl<I: Iterator<Item = Value>> WitnessIterator for I {
    fn next(&mut self, _ty: &Type) -> Result<Value, Error> {
        Iterator::next(self).ok_or(Error::EndOfStream)
    }

    fn finish(self) -> Result<(), Error> {
        Ok(())
    }
}
