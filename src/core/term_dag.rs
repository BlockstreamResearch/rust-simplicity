// Rust Simplicity Library
// Written in 2020 by
//   Andrew Poelstra <apoelstra@blockstream.com>
//   Sanket Kanjalkar <sanket1729@gmail.com>
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

//! # Simplicity DAGs
//!
//! DAGs are the logical structure of Simplicity programs.
//! Each program has a root node,
//! and each node is a leaf or has left or right children.
//!
//! This DAG representation is meant for composing programs by hand.
//! One should use methods that return reference-counting pointers (see [`std::rc`])
//! and copy duplicate sub-DAGs by [`Rc::clone()`].
//! This keeps memory usage minimal.
//!
//! For (de)serialization and execution, the DAG needs to be converted into a linear program.

use crate::core::iter::DagIterable;
use crate::jet::{Application, JetNode};
use crate::merkle::cmr::Cmr;
use crate::{Term, UntypedProgram, Value};
use std::collections::HashMap;
use std::hash::{Hash, Hasher};
use std::rc::Rc;

/// Simplicity DAG for some witness type and application.
/// A DAG is a _directed acyclic graph_ consisting of _nodes_ and _edges_.
/// There is a _root_,
/// nodes may have left or right _children_,
/// and nodes without children are called _leaves_.
///
/// Nodes refer to other nodes via reference-counted pointers to heap memory.
/// If possible, duplicate DAGs make use of this fact and reference the same memory.
#[derive(Clone, Eq, PartialEq, PartialOrd, Ord, Hash, Debug)]
pub enum TermDag<Witness, App: Application> {
    /// Identity
    Iden,
    /// Unit constant
    Unit,
    /// Left injection of some child
    InjL(Rc<TermDag<Witness, App>>),
    /// Right injection of some child
    InjR(Rc<TermDag<Witness, App>>),
    /// Take of some child
    Take(Rc<TermDag<Witness, App>>),
    /// Drop of some child
    Drop(Rc<TermDag<Witness, App>>),
    /// Composition of a left and right child
    Comp(Rc<TermDag<Witness, App>>, Rc<TermDag<Witness, App>>),
    /// Case of a left and right child
    Case(Rc<TermDag<Witness, App>>, Rc<TermDag<Witness, App>>),
    /// Left assertion of a left and right child.
    ///
    /// Right child must be [`Self::Hidden`]
    AssertL(Rc<TermDag<Witness, App>>, Rc<TermDag<Witness, App>>),
    /// Right assertion of a left and right child.
    ///
    /// Left child must be [`Self::Hidden`]
    AssertR(Rc<TermDag<Witness, App>>, Rc<TermDag<Witness, App>>),
    /// Pair of a left and right child
    Pair(Rc<TermDag<Witness, App>>, Rc<TermDag<Witness, App>>),
    /// Disconnect of a left and right child
    Disconnect(Rc<TermDag<Witness, App>>, Rc<TermDag<Witness, App>>),
    /// Witness data
    Witness(Witness),
    /// Fail
    Fail(Cmr, Cmr),
    /// Hidden
    Hidden(Cmr),
    /// Application jet
    Jet(&'static JetNode<App>),
}

impl<Witness, App: Application> TermDag<Witness, App> {
    /// Create a DAG with a single [`Self::Iden`] node
    pub fn iden() -> Rc<Self> {
        Rc::new(TermDag::Iden)
    }

    /// Create a DAG with a single [`Self::Unit`] node
    pub fn unit() -> Rc<Self> {
        Rc::new(TermDag::Unit)
    }

    /// Create a DAG with root [`Self::InjL`] and the given `child`
    pub fn injl(child: Rc<Self>) -> Rc<Self> {
        Rc::new(TermDag::InjL(child))
    }

    /// Create a DAG with root [`Self::InjR`] and the given `child`
    pub fn injr(child: Rc<Self>) -> Rc<Self> {
        Rc::new(TermDag::InjR(child))
    }

    /// Create a DAG with root [`Self::Take`] and the given `child`
    pub fn take(child: Rc<Self>) -> Rc<Self> {
        Rc::new(TermDag::Take(child))
    }

    /// Create a DAG with root [`Self::Drop`] and the given `child`
    pub fn drop(child: Rc<Self>) -> Rc<Self> {
        Rc::new(TermDag::Drop(child))
    }

    /// Create a DAG with root [`Self::Comp`] and the given `left` and `right` child
    pub fn comp(left: Rc<Self>, right: Rc<Self>) -> Rc<Self> {
        Rc::new(TermDag::Comp(left, right))
    }

    /// Create a DAG with root [`Self::Case`] and the given `left` and `right` child
    pub fn case(left: Rc<Self>, right: Rc<Self>) -> Rc<Self> {
        Rc::new(TermDag::Case(left, right))
    }

    /// Create a DAG with root [`Self::AssertL`] and the given `left` and `right` child
    pub fn assertl(left: Rc<Self>, right: Rc<Self>) -> Rc<Self> {
        Rc::new(TermDag::AssertL(left, right))
    }

    /// Create a DAG with root [`Self::AssertR`] and the given `left` and `right` child
    pub fn assertr(left: Rc<Self>, right: Rc<Self>) -> Rc<Self> {
        Rc::new(TermDag::AssertR(left, right))
    }

    /// Create a DAG with root [`Self::Pair`] and the given `left` and `right` child
    pub fn pair(left: Rc<Self>, right: Rc<Self>) -> Rc<Self> {
        Rc::new(TermDag::Pair(left, right))
    }

    /// Create a DAG with root [`Self::Disconnect`] and the given `left` and `right` child
    pub fn disconnect(left: Rc<Self>, right: Rc<Self>) -> Rc<Self> {
        Rc::new(TermDag::Disconnect(left, right))
    }

    /// Create a DAG with a single [`Self::Witness`] node that contains the given `value`
    pub fn witness(value: Witness) -> Rc<Self> {
        Rc::new(TermDag::Witness(value))
    }

    /// Create a DAG with a single [`Self::Fail`] node that contains the given `left` and `right` hashes
    pub fn fail(left: Cmr, right: Cmr) -> Rc<Self> {
        Rc::new(TermDag::Fail(left, right))
    }

    /// Create a DAG with a single [`Self::Hidden`] node that contains the given `hash`
    pub fn hidden(hash: Cmr) -> Rc<Self> {
        Rc::new(TermDag::Hidden(hash))
    }

    /// Create a DAG with a single [`Self::Jet`] node that contains the given `jet`
    /// and performs some associated black-box execution
    pub fn jet(jet: &'static JetNode<App>) -> Rc<Self> {
        Rc::new(TermDag::Jet(jet))
    }

    /// Create a DAG that takes any input and returns `value` as constant output.
    ///
    /// _Overall type: A → B where value: B_
    pub fn scribe(value: &Value) -> Rc<TermDag<Witness, App>> {
        match value {
            Value::Unit => TermDag::unit(),
            Value::SumL(l) => {
                let l = TermDag::scribe(l);
                TermDag::injl(l)
            }
            Value::SumR(r) => {
                let r = TermDag::scribe(r);
                TermDag::injr(r)
            }
            Value::Prod(l, r) => {
                let l = TermDag::scribe(l);
                let r = TermDag::scribe(r);
                TermDag::pair(l, r)
            }
        }
    }

    /// Create a DAG that takes any input and returns bit `0` as constant output.
    ///
    /// _Overall type: A → 2_
    pub fn bit_false() -> Rc<Self> {
        TermDag::injl(TermDag::unit())
    }

    /// Create a DAG that takes any input and returns bit `1` as constant output.
    ///
    /// _Overall type: A → 2_
    pub fn bit_true() -> Rc<Self> {
        TermDag::injr(TermDag::unit())
    }

    /// Create a DAG that takes a bit and an input,
    /// such that the `left` child is evaluated on the input if the bit is `1` _(if branch)_
    /// and the `right` child is evaluated on the input otherwise _(else branch)_.
    ///
    /// _Overall type: 2 × A → B where left: A → B and right: A → B_
    pub fn cond(left: Rc<Self>, right: Rc<Self>) -> Rc<Self> {
        TermDag::case(TermDag::drop(right), TermDag::drop(left))
    }

    /// Create a DAG that computes Boolean _NOT_ of the `child`.
    ///
    /// _Overall type: A → 2 where child: A → 2_
    #[allow(clippy::should_implement_trait)]
    pub fn not(child: Rc<Self>) -> Rc<Self> {
        TermDag::comp(
            TermDag::pair(child, TermDag::unit()),
            TermDag::cond(TermDag::bit_false(), TermDag::bit_true()),
        )
    }

    /// Create a DAG that computes Boolean _AND_ of the `left` and `right` child.
    ///
    /// _Overall type: A → 2 where left: A → 2 and right: A → 2_
    pub fn and(left: Rc<Self>, right: Rc<Self>) -> Rc<Self> {
        TermDag::comp(
            TermDag::pair(left, TermDag::iden()),
            TermDag::cond(right, TermDag::bit_false()),
        )
    }

    /// Create a DAG that computes Boolean _OR_ of the `left` and `right`.
    ///
    /// _Overall type: A → 2 where left: A → 2 and right: A → 2_
    pub fn or(left: Rc<Self>, right: Rc<Self>) -> Rc<Self> {
        TermDag::comp(
            TermDag::pair(left, TermDag::iden()),
            TermDag::cond(TermDag::bit_true(), right),
        )
    }

    /// Return the left child of the given DAG root, if there is such a child.
    pub fn get_left(&self) -> Option<&Self> {
        match self {
            TermDag::InjL(l) | TermDag::InjR(l) | TermDag::Take(l) | TermDag::Drop(l) => Some(l),
            TermDag::Comp(l, _)
            | TermDag::Case(l, _)
            | TermDag::Pair(l, _)
            | TermDag::AssertL(l, _)
            | TermDag::AssertR(l, _)
            | TermDag::Disconnect(l, _) => Some(l),
            _ => None,
        }
    }

    /// Return the right child of the given DAG root, if there is such a child.
    pub fn get_right(&self) -> Option<&Self> {
        match self {
            TermDag::Comp(_, r)
            | TermDag::Case(_, r)
            | TermDag::Pair(_, r)
            | TermDag::AssertL(_, r)
            | TermDag::AssertR(_, r)
            | TermDag::Disconnect(_, r) => Some(r),
            _ => None,
        }
    }
}

/// Wrapper for `&'a TermDag<Witness, App>` that implements `Eq` and `Hash`
/// via pointer equality and pointer hashing, respectively.
#[derive(Debug)]
pub struct RefWrapper<'a, Witness, App: Application>(pub &'a TermDag<Witness, App>);

impl<'a, Witness, App: Application> RefWrapper<'a, Witness, App> {
    /// Read `dag_to_index` and return the relative index of `self` in `program`.
    /// Because children appear before parents in post order,
    /// their index is known when processing the parent.
    fn get_relative_index(
        &self,
        program: &[Term<Witness, App>],
        dag_to_index: &HashMap<Self, usize>,
    ) -> usize {
        let index = dag_to_index
            .get(self)
            .expect("children come before parent in post order");
        program.len() - index
    }
}

impl<'a, Witness, App: Application> Clone for RefWrapper<'a, Witness, App> {
    fn clone(&self) -> Self {
        RefWrapper(<&TermDag<Witness, App>>::clone(&self.0))
    }
}

impl<'a, Witness, App: Application> Copy for RefWrapper<'a, Witness, App> {}

impl<'a, Witness, App: Application> PartialEq for RefWrapper<'a, Witness, App> {
    fn eq(&self, other: &Self) -> bool {
        std::ptr::eq(self.0, other.0)
    }
}

impl<'a, Witness, App: Application> Eq for RefWrapper<'a, Witness, App> {}

impl<'a, Witness, App: Application> Hash for RefWrapper<'a, Witness, App> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        std::ptr::hash(self.0, state)
    }
}

impl<'a, Witness, App: Application> DagIterable for &'a TermDag<Witness, App> {
    type Node = RefWrapper<'a, Witness, App>;

    fn root(&self) -> Option<Self::Node> {
        Some(RefWrapper(self))
    }

    fn left_of(&self, node: Self::Node) -> Option<Self::Node> {
        node.0.get_left().map(|l| RefWrapper(l))
    }

    fn right_of(&self, node: Self::Node) -> Option<Self::Node> {
        node.0.get_right().map(|r| RefWrapper(r))
    }
}

impl<Witness, App: Application> TermDag<Witness, App>
where
    Witness: Clone,
{
    /// Create a Simplicity DAG from a linear program.
    pub fn from_untyped_program(program: &UntypedProgram<Witness, App>) -> Rc<Self> {
        assert!(!program.0.is_empty(), "Program must be non-empty");
        let mut dag_list: Vec<Rc<TermDag<_, _>>> = vec![];
        for (index, term) in program.0.iter().enumerate() {
            let dag = match term {
                Term::Iden => TermDag::iden(),
                Term::Unit => TermDag::unit(),
                Term::InjL(l) => TermDag::injl(dag_list[index - l].clone()),
                Term::InjR(r) => TermDag::injr(dag_list[index - r].clone()),
                Term::Take(l) => TermDag::take(dag_list[index - l].clone()),
                Term::Drop(r) => TermDag::drop(dag_list[index - r].clone()),
                Term::Comp(l, r) => {
                    TermDag::comp(dag_list[index - l].clone(), dag_list[index - r].clone())
                }
                Term::Case(l, r) => {
                    TermDag::case(dag_list[index - l].clone(), dag_list[index - r].clone())
                }
                Term::AssertL(l, r) => {
                    TermDag::assertl(dag_list[index - l].clone(), dag_list[index - r].clone())
                }
                Term::AssertR(l, r) => {
                    TermDag::assertr(dag_list[index - l].clone(), dag_list[index - r].clone())
                }
                Term::Pair(l, r) => {
                    TermDag::pair(dag_list[index - l].clone(), dag_list[index - r].clone())
                }
                Term::Disconnect(l, r) => {
                    TermDag::disconnect(dag_list[index - l].clone(), dag_list[index - r].clone())
                }
                Term::Witness(w) => TermDag::witness(w.clone()),
                Term::Fail(hl, hr) => TermDag::fail(*hl, *hr),
                Term::Hidden(h) => TermDag::hidden(*h),
                Term::Jet(j) => TermDag::jet(j),
            };
            dag_list.push(dag);
        }
        Rc::clone(dag_list.last().unwrap())
    }

    /// Convert a Simplicity DAG into a linear program.
    ///
    /// The program is guaranteed to be in canonical order.
    pub fn to_untyped_program(&self) -> UntypedProgram<Witness, App> {
        let it_post_order = self.iter_post_order();
        let mut program = Vec::new();
        let mut dag_to_index = HashMap::new();

        for dag in it_post_order {
            let term = if let Some(l) = dag.0.get_left() {
                // Program of left child is already added
                let l_idx = RefWrapper(l).get_relative_index(&program, &dag_to_index);

                if let Some(r) = dag.0.get_right() {
                    // Program of right child is already added
                    let r_idx = RefWrapper(r).get_relative_index(&program, &dag_to_index);

                    match dag.0 {
                        TermDag::Comp(_, _) => Term::Comp(l_idx, r_idx),
                        TermDag::Case(_, _) => Term::Case(l_idx, r_idx),
                        TermDag::Pair(_, _) => Term::Pair(l_idx, r_idx),
                        TermDag::AssertL(_, _) => Term::AssertL(l_idx, r_idx),
                        TermDag::AssertR(_, _) => Term::AssertR(l_idx, r_idx),
                        TermDag::Disconnect(_, _) => Term::Disconnect(l_idx, r_idx),
                        _ => unreachable!(),
                    }
                } else {
                    match dag.0 {
                        TermDag::InjL(_) => Term::InjL(l_idx),
                        TermDag::InjR(_) => Term::InjR(l_idx),
                        TermDag::Take(_) => Term::Take(l_idx),
                        TermDag::Drop(_) => Term::Drop(l_idx),
                        _ => unreachable!(),
                    }
                }
            } else {
                match dag.0 {
                    TermDag::Unit => Term::Unit,
                    TermDag::Iden => Term::Iden,
                    TermDag::Witness(w) => Term::Witness(w.clone()),
                    TermDag::Fail(hl, hr) => Term::Fail(*hl, *hr),
                    TermDag::Hidden(h) => Term::Hidden(*h),
                    TermDag::Jet(j) => Term::Jet(j),
                    _ => unreachable!(),
                }
            };

            dag_to_index.insert(dag, program.len());
            program.push(term);
        }

        UntypedProgram(program)
    }
}
