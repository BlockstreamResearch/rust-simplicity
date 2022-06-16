use crate::jet::{Application, JetNode};
use crate::merkle::cmr::Cmr;
use crate::{Term, UntypedProgram};
use std::collections::HashMap;
use std::hash::{Hash, Hasher};
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

impl<App: Application> TermDag<(), App> {
    /// Create DAG with single `iden` node
    pub fn iden() -> Rc<Self> {
        Rc::new(TermDag::Iden)
    }

    /// Create DAG with single `unit` node
    pub fn unit() -> Rc<Self> {
        Rc::new(TermDag::Unit)
    }

    /// Create DAG with `injl` root that points to given sub-DAG
    pub fn injl(sub_dag: Rc<Self>) -> Rc<Self> {
        Rc::new(TermDag::InjL(sub_dag))
    }

    /// Create DAG with `injr` root that points to given sub-DAG
    pub fn injr(sub_dag: Rc<Self>) -> Rc<Self> {
        Rc::new(TermDag::InjR(sub_dag))
    }

    /// Create DAG with `take` root that points to given sub-DAG
    pub fn take(sub_dag: Rc<Self>) -> Rc<Self> {
        Rc::new(TermDag::Take(sub_dag))
    }

    /// Create DAG with `drop` root that points to given sub-DAG
    pub fn drop(sub_dag: Rc<Self>) -> Rc<Self> {
        Rc::new(TermDag::Drop(sub_dag))
    }

    /// Create DAG with `comp` root that points to given `left` and `right` sub-DAGs
    pub fn comp(left_dag: Rc<Self>, right_dag: Rc<Self>) -> Rc<Self> {
        Rc::new(TermDag::Comp(left_dag, right_dag))
    }

    /// Create DAG with `case` root that points to given `left` and `right` sub-DAGs
    pub fn case(left_dag: Rc<Self>, right_dag: Rc<Self>) -> Rc<Self> {
        Rc::new(TermDag::Case(left_dag, right_dag))
    }

    /// Create DAG with `pair` root that points to given `left` and `right` sub-DAGs
    pub fn pair(left_dag: Rc<Self>, right_dag: Rc<Self>) -> Rc<Self> {
        Rc::new(TermDag::Pair(left_dag, right_dag))
    }

    /// Create DAG with `disconnect` root that points to given `left` and `right` sub-DAGs
    pub fn disconnect(left_dag: Rc<Self>, right_dag: Rc<Self>) -> Rc<Self> {
        Rc::new(TermDag::Disconnect(left_dag, right_dag))
    }

    /// Create DAG with single `witness` node _(with no witness data)_
    pub fn witness() -> Rc<Self> {
        Rc::new(TermDag::Witness(()))
    }

    /// Create DAG with single `fail` node that contains the given `left` and `right` hashes
    pub fn fail(left_hash: Cmr, right_hash: Cmr) -> Rc<Self> {
        Rc::new(TermDag::Fail(left_hash, right_hash))
    }

    /// Create DAG with single `hidden` node that contains the given hash
    pub fn hidden(hash: Cmr) -> Rc<Self> {
        Rc::new(TermDag::Hidden(hash))
    }

    /// Create DAG with single `jet_node` that performs some black-box execution.
    pub fn jet(jet: &'static JetNode<App>) -> Rc<Self> {
        Rc::new(TermDag::Jet(jet))
    }
}

impl<Witness, App: Application> TermDag<Witness, App> {
    /// Create untyped DAG from node representation
    pub fn from_untyped_program(program: UntypedProgram<Witness, App>) -> Rc<Self> {
        assert!(!program.0.is_empty(), "Program must be non-empty");
        let mut dag_list: Vec<Rc<TermDag<_, _>>> = vec![];
        for (index, term) in program.0.into_iter().enumerate() {
            let dag = match term {
                Term::Iden => Rc::new(TermDag::Iden),
                Term::Unit => Rc::new(TermDag::Unit),
                Term::InjL(l) => Rc::new(TermDag::InjL(Rc::clone(&dag_list[index - l]))),
                Term::InjR(r) => Rc::new(TermDag::InjR(Rc::clone(&dag_list[index - r]))),
                Term::Take(l) => Rc::new(TermDag::Take(Rc::clone(&dag_list[index - l]))),
                Term::Drop(r) => Rc::new(TermDag::Drop(Rc::clone(&dag_list[index - r]))),
                Term::Comp(l, r) => Rc::new(TermDag::Comp(
                    Rc::clone(&dag_list[index - l]),
                    Rc::clone(&dag_list[index - r]),
                )),
                Term::Case(l, r) => Rc::new(TermDag::Case(
                    Rc::clone(&dag_list[index - l]),
                    Rc::clone(&dag_list[index - r]),
                )),
                Term::AssertL(l, r) => Rc::new(TermDag::AssertL(
                    Rc::clone(&dag_list[index - l]),
                    Rc::clone(&dag_list[index - r]),
                )),
                Term::AssertR(l, r) => Rc::new(TermDag::AssertR(
                    Rc::clone(&dag_list[index - l]),
                    Rc::clone(&dag_list[index - r]),
                )),
                Term::Pair(l, r) => Rc::new(TermDag::Pair(
                    Rc::clone(&dag_list[index - l]),
                    Rc::clone(&dag_list[index - r]),
                )),
                Term::Disconnect(l, r) => Rc::new(TermDag::Disconnect(
                    Rc::clone(&dag_list[index - l]),
                    Rc::clone(&dag_list[index - r]),
                )),
                Term::Witness(w) => Rc::new(TermDag::Witness(w)),
                Term::Fail(hl, hr) => Rc::new(TermDag::Fail(hl, hr)),
                Term::Hidden(h) => Rc::new(TermDag::Hidden(h)),
                Term::Jet(j) => Rc::new(TermDag::Jet(j)),
            };
            dag_list.push(dag);
        }
        Rc::clone(dag_list.last().unwrap())
    }
}

// A direct comparison for Rc<> results in comparison
// of underllying inner values whereas we desire to
// compare the referececs.
#[derive(Debug)]
struct RcWrapper<Witness, App: Application> {
    rc: Rc<TermDag<Witness, App>>,
}

impl<Witness, App: Application> PartialEq for RcWrapper<Witness, App> {
    fn eq(&self, other: &Self) -> bool {
        Rc::ptr_eq(&self.rc, &other.rc)
    }
}

impl<Witness, App: Application> Eq for RcWrapper<Witness, App> {}

impl<Witness, App: Application> From<Rc<TermDag<Witness, App>>> for RcWrapper<Witness, App> {
    fn from(dag: Rc<TermDag<Witness, App>>) -> Self {
        Self { rc: dag }
    }
}

impl<Witness, App> Hash for RcWrapper<Witness, App>
where
    Witness: Hash,
    App: Hash + Application,
{
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.rc.hash(state);
    }
}

impl<Witness, App> TermDag<Witness, App>
where
    Witness: Hash + Clone,
    App: Hash + Clone + Application,
{
    /// Convert untyped DAG into node representation.
    pub fn into_untyped_program(self) -> UntypedProgram<Witness, App> {
        // helper function to recrusively compute the index positions
        // of the children.
        fn into_helper<Witness, App>(
            dag: Rc<TermDag<Witness, App>>,
            index_map: &mut HashMap<RcWrapper<Witness, App>, usize>,
            prog: &mut Vec<Term<Witness, App>>,
        ) -> usize
        where
            RcWrapper<Witness, App>: Hash,
            Witness: Clone,
            App: Clone + Application,
        {
            // insert one child into prog
            macro_rules! insert_one_child {
                ($term: expr, $l : expr) => {
                    match index_map.get(&RcWrapper::from(Rc::clone($l))) {
                        Some(ind) => prog.push($term(prog.len() - ind)),
                        None => {
                            let ind = into_helper(Rc::clone($l), index_map, prog);
                            prog.push($term(prog.len() - ind));
                        }
                    }
                };
            }

            // insert two children into prog
            macro_rules! insert_two_child {
                ($term: expr, $l : expr, $r: expr) => {{
                    let l_ind = match index_map.get(&RcWrapper::from(Rc::clone($l))) {
                        Some(ind) => *ind,
                        None => into_helper(Rc::clone($l), index_map, prog),
                    };
                    let r_ind = match index_map.get(&RcWrapper::from(Rc::clone($r))) {
                        Some(ind) => *ind,
                        None => into_helper(Rc::clone($r), index_map, prog),
                    };
                    prog.push($term(prog.len() - l_ind, prog.len() - r_ind));
                }};
            }

            if index_map.contains_key(&RcWrapper::from(Rc::clone(&dag))) {
                return *index_map.get(&RcWrapper::from(Rc::clone(&dag))).unwrap();
            }
            match dag.as_ref() {
                TermDag::Unit => prog.push(Term::Unit),
                TermDag::Iden => prog.push(Term::Iden),
                TermDag::InjL(l) => insert_one_child!(Term::InjL, l),
                TermDag::InjR(r) => insert_one_child!(Term::InjR, r),
                TermDag::Take(l) => insert_one_child!(Term::Take, l),
                TermDag::Drop(r) => insert_one_child!(Term::Drop, r),
                TermDag::Comp(l, r) => insert_two_child!(Term::Comp, l, r),
                TermDag::Case(l, r) => insert_two_child!(Term::Case, l, r),
                TermDag::AssertL(l, r) => insert_two_child!(Term::AssertL, l, r),
                TermDag::AssertR(l, r) => insert_two_child!(Term::AssertR, l, r),
                TermDag::Pair(l, r) => insert_two_child!(Term::Pair, l, r),
                TermDag::Disconnect(l, r) => insert_two_child!(Term::Disconnect, l, r),
                TermDag::Witness(ref w) => prog.push(Term::Witness(w.clone())),
                TermDag::Fail(a, b) => prog.push(Term::Fail(*a, *b)),
                TermDag::Hidden(cmr) => prog.push(Term::Hidden(*cmr)),
                TermDag::Jet(j) => prog.push(Term::Jet(*j)),
            }
            // insert the current node remembering it's index for reusing
            index_map.insert(RcWrapper::from(dag), prog.len() - 1);
            prog.len() - 1
        }

        let mut prog = vec![];
        let mut index_map = HashMap::new();
        let _len = into_helper(Rc::new(self), &mut index_map, &mut prog);

        UntypedProgram(prog)
    }
}
