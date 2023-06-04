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

//! General DAG iteration utilities

use std::collections::{HashMap, HashSet};
use std::fmt;
use std::rc::Rc;

use crate::core::commit::{CommitNode, CommitNodeInner};
use crate::core::redeem::{RedeemNode, RedeemNodeInner};
use crate::core::Value;
use crate::jet;

/// Generic container for Simplicity DAGs
pub enum Dag<T> {
    /// Identity
    Iden,
    /// Unit constant
    Unit,
    /// Left injection of some child
    InjL(T),
    /// Right injection of some child
    InjR(T),
    /// Take of some child
    Take(T),
    /// Drop of some child
    Drop(T),
    /// Composition of a left and right child
    Comp(T, T),
    /// Case of a left and right child
    Case(T, T),
    /// Pair of a left and right child
    Pair(T, T),
    /// Disconnect of a left and right child
    Disconnect(T, T),
    /// Witness data (missing during commitment, inserted during redemption)
    Witness,
    /// Universal fail
    Fail,
    /// Hidden CMR
    Hidden,
    /// Application jet
    Jet,
    /// Constant word
    Word,
}

/// Object representing pointer identity of a DAG node
///
/// Used to populate a hashset to determine whether or not a given node has
/// already been visited by an iterator.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
struct PointerId(usize);

impl<D: DagLike> From<&D> for PointerId {
    fn from(dag: &D) -> Self {
        PointerId(dag.data() as *const _ as usize)
    }
}

/// A trait for any structure which has the shape of a Simplicity DAG
///
/// This should be implemented on any reference type for `CommitNode` and `RedeemNode`;
/// it cannot be implemented on these structures themselves because they depend on
pub trait DagLike: Sized {
    /// The type of the DAG node, with no references or wrappers
    type Node;

    /// A pointer to the underlying data
    fn data(&self) -> &Self::Node;

    /// Interpret the node as a DAG node
    fn as_dag_node(&self) -> Dag<Self>;

    /// Accessor for the left child of the node, if one exists
    fn left_child(&self) -> Option<Self> {
        match self.as_dag_node() {
            Dag::Iden => None,
            Dag::Unit => None,
            Dag::InjL(sub) => Some(sub),
            Dag::InjR(sub) => Some(sub),
            Dag::Take(sub) => Some(sub),
            Dag::Drop(sub) => Some(sub),
            Dag::Comp(left, _) => Some(left),
            Dag::Case(left, _) => Some(left),
            Dag::Pair(left, _) => Some(left),
            Dag::Disconnect(left, _) => Some(left),
            Dag::Witness => None,
            Dag::Fail => None,
            Dag::Hidden => None,
            Dag::Jet => None,
            Dag::Word => None,
        }
    }

    /// Accessor for the right child of the node, if one exists
    fn right_child(&self) -> Option<Self> {
        match self.as_dag_node() {
            Dag::Iden => None,
            Dag::Unit => None,
            Dag::InjL(_) => None,
            Dag::InjR(_) => None,
            Dag::Take(_) => None,
            Dag::Drop(_) => None,
            Dag::Comp(_, right) => Some(right),
            Dag::Case(_, right) => Some(right),
            Dag::Pair(_, right) => Some(right),
            Dag::Disconnect(_, right) => Some(right),
            Dag::Witness => None,
            Dag::Fail => None,
            Dag::Hidden => None,
            Dag::Jet => None,
            Dag::Word => None,
        }
    }

    /// Obtains an iterator of all the nodes rooted at the DAG, in post order.
    ///
    /// Each node is only yielded once, at the leftmost position that it
    /// appears in the DAG.
    fn post_order_iter(self) -> PostOrderIter<Self> {
        PostOrderIter {
            stack: Vec::new(),
            maybe_current: Some(self),
            visited: HashSet::new(),
        }
    }
}

impl<'a, J: jet::Jet> DagLike for &'a CommitNode<J> {
    type Node = CommitNode<J>;

    fn data(&self) -> &CommitNode<J> {
        self
    }

    #[rustfmt::skip]
    fn as_dag_node(&self) -> Dag<Self> {
        match self.inner() {
            CommitNodeInner::Iden => Dag::Iden,
            CommitNodeInner::Unit => Dag::Unit,
            CommitNodeInner::InjL(ref sub) => Dag::InjL(sub),
            CommitNodeInner::InjR(ref sub) => Dag::InjR(sub),
            CommitNodeInner::Take(ref sub) => Dag::Take(sub),
            CommitNodeInner::Drop(ref sub) => Dag::Drop(sub),
            CommitNodeInner::Comp(ref left, ref right) => Dag::Comp(left, right),
            CommitNodeInner::Case(ref left, ref right) => Dag::Case(left, right),
            CommitNodeInner::AssertL(ref left, ref right) => Dag::Case(left, right),
            CommitNodeInner::AssertR(ref left, ref right) => Dag::Case(left, right),
            CommitNodeInner::Pair(ref left, ref right) => Dag::Pair(left, right),
            CommitNodeInner::Disconnect(ref left, ref right) => Dag::Disconnect(left, right),
            CommitNodeInner::Witness => Dag::Witness,
            CommitNodeInner::Fail(..) => Dag::Fail,
            CommitNodeInner::Hidden(..) => Dag::Hidden,
            CommitNodeInner::Jet(..) => Dag::Jet,
            CommitNodeInner::Word(..) => Dag::Word,
        }
    }
}

impl<J: jet::Jet> DagLike for Rc<CommitNode<J>> {
    type Node = CommitNode<J>;

    fn data(&self) -> &CommitNode<J> {
        self
    }

    #[rustfmt::skip]
    fn as_dag_node(&self) -> Dag<Self> {
        match self.inner() {
            CommitNodeInner::Iden => Dag::Iden,
            CommitNodeInner::Unit => Dag::Unit,
            CommitNodeInner::InjL(ref sub) => Dag::InjL(Rc::clone(sub)),
            CommitNodeInner::InjR(ref sub) => Dag::InjR(Rc::clone(sub)),
            CommitNodeInner::Take(ref sub) => Dag::Take(Rc::clone(sub)),
            CommitNodeInner::Drop(ref sub) => Dag::Drop(Rc::clone(sub)),
            CommitNodeInner::Comp(ref left, ref right) => Dag::Comp(Rc::clone(left), Rc::clone(right)),
            CommitNodeInner::Case(ref left, ref right) => Dag::Case(Rc::clone(left), Rc::clone(right)),
            CommitNodeInner::AssertL(ref left, ref right) => Dag::Case(Rc::clone(left), Rc::clone(right)),
            CommitNodeInner::AssertR(ref left, ref right) => Dag::Case(Rc::clone(left), Rc::clone(right)),
            CommitNodeInner::Pair(ref left, ref right) => Dag::Pair(Rc::clone(left), Rc::clone(right)),
            CommitNodeInner::Disconnect(ref left, ref right) => Dag::Disconnect(Rc::clone(left), Rc::clone(right)),
            CommitNodeInner::Witness => Dag::Witness,
            CommitNodeInner::Fail(..) => Dag::Fail,
            CommitNodeInner::Hidden(..) => Dag::Hidden,
            CommitNodeInner::Jet(..) => Dag::Jet,
            CommitNodeInner::Word(..) => Dag::Word,
        }
    }
}

impl<'a, J: jet::Jet> DagLike for &'a RedeemNode<J> {
    type Node = RedeemNode<J>;

    fn data(&self) -> &RedeemNode<J> {
        self
    }

    #[rustfmt::skip]
    fn as_dag_node(&self) -> Dag<Self> {
        match self.inner {
            RedeemNodeInner::Iden => Dag::Iden,
            RedeemNodeInner::Unit => Dag::Unit,
            RedeemNodeInner::InjL(ref sub) => Dag::InjL(sub),
            RedeemNodeInner::InjR(ref sub) => Dag::InjR(sub),
            RedeemNodeInner::Take(ref sub) => Dag::Take(sub),
            RedeemNodeInner::Drop(ref sub) => Dag::Drop(sub),
            RedeemNodeInner::Comp(ref left, ref right) => Dag::Comp(left, right),
            RedeemNodeInner::Case(ref left, ref right) => Dag::Case(left, right),
            RedeemNodeInner::AssertL(ref left, ref right) => Dag::Case(left, right),
            RedeemNodeInner::AssertR(ref left, ref right) => Dag::Case(left, right),
            RedeemNodeInner::Pair(ref left, ref right) => Dag::Pair(left, right),
            RedeemNodeInner::Disconnect(ref left, ref right) => Dag::Disconnect(left, right),
            RedeemNodeInner::Witness(..) => Dag::Witness,
            RedeemNodeInner::Fail(..) => Dag::Fail,
            RedeemNodeInner::Hidden(..) => Dag::Hidden,
            RedeemNodeInner::Jet(..) => Dag::Jet,
            RedeemNodeInner::Word(..) => Dag::Word,
        }
    }
}

impl<J: jet::Jet> DagLike for Rc<RedeemNode<J>> {
    type Node = RedeemNode<J>;

    fn data(&self) -> &RedeemNode<J> {
        self
    }

    #[rustfmt::skip]
    fn as_dag_node(&self) -> Dag<Self> {
        match self.inner {
            RedeemNodeInner::Iden => Dag::Iden,
            RedeemNodeInner::Unit => Dag::Unit,
            RedeemNodeInner::InjL(ref sub) => Dag::InjL(Rc::clone(sub)),
            RedeemNodeInner::InjR(ref sub) => Dag::InjR(Rc::clone(sub)),
            RedeemNodeInner::Take(ref sub) => Dag::Take(Rc::clone(sub)),
            RedeemNodeInner::Drop(ref sub) => Dag::Drop(Rc::clone(sub)),
            RedeemNodeInner::Comp(ref left, ref right) => Dag::Comp(Rc::clone(left), Rc::clone(right)),
            RedeemNodeInner::Case(ref left, ref right) => Dag::Case(Rc::clone(left), Rc::clone(right)),
            RedeemNodeInner::AssertL(ref left, ref right) => Dag::Case(Rc::clone(left), Rc::clone(right)),
            RedeemNodeInner::AssertR(ref left, ref right) => Dag::Case(Rc::clone(left), Rc::clone(right)),
            RedeemNodeInner::Pair(ref left, ref right) => Dag::Pair(Rc::clone(left), Rc::clone(right)),
            RedeemNodeInner::Disconnect(ref left, ref right) => Dag::Disconnect(Rc::clone(left), Rc::clone(right)),
            RedeemNodeInner::Witness(..) => Dag::Witness,
            RedeemNodeInner::Fail(..) => Dag::Fail,
            RedeemNodeInner::Hidden(..) => Dag::Hidden,
            RedeemNodeInner::Jet(..) => Dag::Jet,
            RedeemNodeInner::Word(..) => Dag::Word,
        }
    }
}

/// Iterates over a DAG in _post order_.
///
/// That means nodes are yielded in the order (left child, right child, parent).
/// Shared nodes appear only once at their leftmost position.
#[derive(Clone, Debug)]
pub struct PostOrderIter<D: DagLike> {
    stack: Vec<D>,
    maybe_current: Option<D>,
    visited: HashSet<PointerId>,
}

impl<D: DagLike> Iterator for PostOrderIter<D> {
    type Item = D;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if let Some(current) = self.maybe_current.take() {
                let left = current.left_child();
                self.stack.push(current);

                if let Some(left) = left {
                    if !self.visited.contains(&PointerId::from(&left)) {
                        self.maybe_current = Some(left);
                        continue;
                    }
                }
                // else
                self.maybe_current = None;
            } else if let Some(top) = self.stack.last() {
                if let Some(right) = top.right_child() {
                    if !self.visited.contains(&PointerId::from(&right)) {
                        self.maybe_current = Some(right);
                        continue;
                    }
                }
                // else
                let top = self.stack.pop().unwrap();
                self.visited.insert(PointerId::from(&top));

                return Some(top);
            } else {
                return None;
            }
        }
    }
}

impl<'a, J: jet::Jet> PostOrderIter<&'a RedeemNode<J>> {
    /// Adapt the iterator to only yield witnesses
    ///
    /// The witnesses are yielded in the order in which they appear in the DAG
    /// *except* that each witness is only yielded once, and future occurences
    /// are skipped.
    pub fn into_deduped_witnesses(self) -> impl Iterator<Item = &'a Value> + Clone {
        let mut seen_imrs = HashSet::new();
        self.filter_map(move |node| {
            if let RedeemNodeInner::Witness(value) = &node.inner {
                if seen_imrs.insert(node.imr) {
                    Some(value)
                } else {
                    None
                }
            } else {
                None
            }
        })
    }
}

impl<D: DagLike> PostOrderIter<D> {
    /// Display the DAG as an indexed list in post order.
    ///
    /// `display_body()` formats the node body in front of the node indices.
    /// `display_aux()` formats auxiliary items after the node indices.
    pub fn into_display<F, G>(
        self,
        f: &mut fmt::Formatter<'_>,
        mut display_body: F,
        mut display_aux: G,
    ) -> fmt::Result
    where
        F: FnMut(&D, &mut fmt::Formatter<'_>) -> fmt::Result,
        G: FnMut(&D, &mut fmt::Formatter<'_>) -> fmt::Result,
    {
        let mut node_to_index = HashMap::new();

        for (index, node) in self.enumerate() {
            write!(f, "{}: ", index)?;
            display_body(&node, f)?;

            if let Some(left) = node.left_child() {
                let i_abs = node_to_index.get(&PointerId::from(&left)).unwrap();

                if let Some(right) = node.right_child() {
                    let j_abs = node_to_index.get(&PointerId::from(&right)).unwrap();

                    write!(f, "({}, {})", i_abs, j_abs)?;
                } else {
                    write!(f, "({})", i_abs)?;
                }
            }

            display_aux(&node, f)?;
            f.write_str("\n")?;
            node_to_index.insert(PointerId::from(&node), index);
        }

        Ok(())
    }
}
