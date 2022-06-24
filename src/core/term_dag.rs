use crate::jet::{Application, JetNode};
use crate::merkle::cmr::Cmr;
use crate::{Term, UntypedProgram};
use std::collections::HashMap;
use std::rc::Rc;

/// Untyped Simplicity DAG _(directed acyclic graph)_.
/// May include Bitcoin/Elements extensions (see [`Term`]).
///
/// A DAG consists of a combinator as parent node and its payload
/// _(references to sub-DAGs, witness data, etc.)_.
/// A DAG corresponds to an untyped Simplicity program.
///
/// References to sub-DAGs are pointers to heap memory.
///
/// The DAG representation is used for inductively constructing Simplicity programs
/// that are later translated into the node representation.
#[derive(Clone, Eq, PartialEq, PartialOrd, Ord, Hash, Debug)]
pub enum TermDag<Witness, App: Application> {
    Iden,
    Unit,
    InjL(Rc<TermDag<Witness, App>>),
    InjR(Rc<TermDag<Witness, App>>),
    Take(Rc<TermDag<Witness, App>>),
    Drop(Rc<TermDag<Witness, App>>),
    Comp(Rc<TermDag<Witness, App>>, Rc<TermDag<Witness, App>>),
    Case(Rc<TermDag<Witness, App>>, Rc<TermDag<Witness, App>>),
    AssertL(Rc<TermDag<Witness, App>>, Rc<TermDag<Witness, App>>),
    AssertR(Rc<TermDag<Witness, App>>, Rc<TermDag<Witness, App>>),
    Pair(Rc<TermDag<Witness, App>>, Rc<TermDag<Witness, App>>),
    Disconnect(Rc<TermDag<Witness, App>>, Rc<TermDag<Witness, App>>),
    Witness(Witness),
    Fail(Cmr, Cmr),
    Hidden(Cmr),
    Jet(&'static JetNode<App>),
}

impl<Witness, App: Application> TermDag<Witness, App> {
    /// Create a DAG with a single `iden` node
    pub fn iden() -> Rc<Self> {
        Rc::new(TermDag::Iden)
    }

    /// Create a DAG with a single `unit` node
    pub fn unit() -> Rc<Self> {
        Rc::new(TermDag::Unit)
    }

    /// Create a DAG with root `injl` and the given `child`
    pub fn injl(child: Rc<Self>) -> Rc<Self> {
        Rc::new(TermDag::InjL(child))
    }

    /// Create a DAG with root `injr` and the given `child`
    pub fn injr(child: Rc<Self>) -> Rc<Self> {
        Rc::new(TermDag::InjR(child))
    }

    /// Create a DAG with root `take` and the given `child`
    pub fn take(child: Rc<Self>) -> Rc<Self> {
        Rc::new(TermDag::Take(child))
    }

    /// Create a DAG with root `drop` and the given `child`
    pub fn drop(child: Rc<Self>) -> Rc<Self> {
        Rc::new(TermDag::Drop(child))
    }

    /// Create a DAG with root `comp` and the given `left` and `right` child
    pub fn comp(left: Rc<Self>, right: Rc<Self>) -> Rc<Self> {
        Rc::new(TermDag::Comp(left, right))
    }

    /// Create a DAG with root `case` and the given `left` and `right` child
    pub fn case(left: Rc<Self>, right: Rc<Self>) -> Rc<Self> {
        Rc::new(TermDag::Case(left, right))
    }

    /// Create a DAG with root `assertl` and the given `left` and `right` child
    pub fn assertl(left: Rc<Self>, right: Rc<Self>) -> Rc<Self> {
        Rc::new(TermDag::AssertL(left, right))
    }

    /// Create a DAG with root `assertr` and the given `left` and `right` child
    pub fn assertr(left: Rc<Self>, right: Rc<Self>) -> Rc<Self> {
        Rc::new(TermDag::AssertR(left, right))
    }

    /// Create a DAG with root `pair` and the given `left` and `right` child
    pub fn pair(left: Rc<Self>, right: Rc<Self>) -> Rc<Self> {
        Rc::new(TermDag::Pair(left, right))
    }

    /// Create a DAG with root `disconnect` and the given `left` and `right` child
    pub fn disconnect(left: Rc<Self>, right: Rc<Self>) -> Rc<Self> {
        Rc::new(TermDag::Disconnect(left, right))
    }

    /// Create a DAG with a single `witness` node that contains the given `value`
    pub fn witness(value: Witness) -> Rc<Self> {
        Rc::new(TermDag::Witness(value))
    }

    /// Create a DAG with a single `fail` node that contains the given `left` and `right` hashes
    pub fn fail(left: Cmr, right: Cmr) -> Rc<Self> {
        Rc::new(TermDag::Fail(left, right))
    }

    /// Create a DAG with a single `hidden` node that contains the given `hash`
    pub fn hidden(hash: Cmr) -> Rc<Self> {
        Rc::new(TermDag::Hidden(hash))
    }

    /// Create a DAG with a single `jet` node that performs some black-box execution.
    pub fn jet(jet: &'static JetNode<App>) -> Rc<Self> {
        Rc::new(TermDag::Jet(jet))
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

    /// Visit the DAG and return a stack that can be popped such that the nodes are in post order,
    /// i.e., children appear before their parent.
    fn get_post_order_stack(&self) -> Vec<&TermDag<Witness, App>> {
        let mut visited = Vec::new();
        let mut to_visit = vec![self];

        while let Some(dag) = to_visit.pop() {
            visited.push(dag);

            if let Some(l) = dag.get_left() {
                if !l.is_cross_reference(&to_visit) {
                    to_visit.push(l);
                }
            }
            if let Some(r) = dag.get_right() {
                if !r.is_cross_reference(&to_visit) {
                    to_visit.push(r);
                }
            }
        }

        visited
    }

    /// Return whether `self` is a cross-reference to a node that appears
    /// earlier in the post-order of the DAG.
    ///
    /// Using the two-stack approach of [`Self::get_post_order_stack`],
    /// nodes that appear earlier are reachable from the nodes in `to_visit`.
    fn is_cross_reference(&self, to_visit: &[&TermDag<Witness, App>]) -> bool {
        for dag in to_visit {
            if self.is_reachable_from(dag) {
                return true;
            }
        }

        false
    }

    // TODO: Slow for large programs, like HASHBLOCK or SCHNORR
    // Memoize reachable nodes? Save unique pairs of parents to leftmost child (requires additional iteration).
    /// Return whether the DAG root `self` is reachable from the DAG root `other`.
    ///
    /// Performs DFS internally.
    fn is_reachable_from(&self, other: &Self) -> bool {
        let mut to_visit = vec![other];

        while let Some(node) = to_visit.pop() {
            if std::ptr::eq(node, self) {
                return true;
            }

            if let Some(l) = node.get_left() {
                to_visit.push(l);
            }
            if let Some(r) = node.get_right() {
                to_visit.push(r);
            }
        }

        false
    }

    /// Read `dag_to_index` and return the relative index of `self` in `program`.
    /// Because children appear before parents in post order,
    /// their index is known when processing the parent.
    fn get_relative_index(
        &self,
        program: &[Term<Witness, App>],
        dag_to_index: &HashMap<*const TermDag<Witness, App>, usize>,
    ) -> usize {
        let ptr: *const TermDag<_, _> = self;
        let index = dag_to_index
            .get(&ptr)
            .expect("children come before parent in post order");
        program.len() - index
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
        let mut post_order_stack = self.get_post_order_stack();
        let mut program = Vec::with_capacity(post_order_stack.len());
        let mut dag_to_index = HashMap::new();

        while let Some(dag) = post_order_stack.pop() {
            let term = if let Some(l) = dag.get_left() {
                // Program of left child is already added
                let l_idx = l.get_relative_index(&program, &dag_to_index);

                if let Some(r) = dag.get_right() {
                    // Program of right child is already added
                    let r_idx = r.get_relative_index(&program, &dag_to_index);

                    match dag {
                        TermDag::Comp(_, _) => Term::Comp(l_idx, r_idx),
                        TermDag::Case(_, _) => Term::Case(l_idx, r_idx),
                        TermDag::Pair(_, _) => Term::Pair(l_idx, r_idx),
                        TermDag::AssertL(_, _) => Term::AssertL(l_idx, r_idx),
                        TermDag::AssertR(_, _) => Term::AssertR(l_idx, r_idx),
                        TermDag::Disconnect(_, _) => Term::Disconnect(l_idx, r_idx),
                        _ => unreachable!(),
                    }
                } else {
                    match dag {
                        TermDag::InjL(_) => Term::InjL(l_idx),
                        TermDag::InjR(_) => Term::InjR(l_idx),
                        TermDag::Take(_) => Term::Take(l_idx),
                        TermDag::Drop(_) => Term::Drop(l_idx),
                        _ => unreachable!(),
                    }
                }
            } else {
                match dag {
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
