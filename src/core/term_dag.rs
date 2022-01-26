use crate::{cmr, extension};
use crate::{Term, UnTypedProg};
use std::collections::HashMap;
use std::hash::{Hash, Hasher};
use std::rc::Rc;

/// Simplicity expression node, including Bitcoin/Elements extensions
///
/// If Bitcoin/Elements support is not compiled (see `bitcoin` and
/// `elements` features) programs using these extensions will fail to
/// parse.
///
/// The structure stores the simplicity program as a directed acyclic graph(DAG).
/// This structure is useful for creating simplicity programs recursively.
#[derive(Clone, Eq, PartialEq, PartialOrd, Ord, Hash, Debug)]
pub enum TermDag<Witness, Extension> {
    Iden,
    Unit,
    InjL(Rc<TermDag<Witness, Extension>>),
    InjR(Rc<TermDag<Witness, Extension>>),
    Take(Rc<TermDag<Witness, Extension>>),
    Drop(Rc<TermDag<Witness, Extension>>),
    Comp(
        Rc<TermDag<Witness, Extension>>,
        Rc<TermDag<Witness, Extension>>,
    ),
    Case(
        Rc<TermDag<Witness, Extension>>,
        Rc<TermDag<Witness, Extension>>,
    ),
    AssertL(
        Rc<TermDag<Witness, Extension>>,
        Rc<TermDag<Witness, Extension>>,
    ),
    AssertR(
        Rc<TermDag<Witness, Extension>>,
        Rc<TermDag<Witness, Extension>>,
    ),
    Pair(
        Rc<TermDag<Witness, Extension>>,
        Rc<TermDag<Witness, Extension>>,
    ),
    Disconnect(
        Rc<TermDag<Witness, Extension>>,
        Rc<TermDag<Witness, Extension>>,
    ),
    Witness(Witness),
    Fail([u8; 32], [u8; 32]),
    Hidden(cmr::Cmr),
    Ext(Extension),
    Jet(extension::jets::JetsNode),
}

impl<Witness, Extension> TermDag<Witness, Extension> {
    /// Create a DAG representation from an untyped representation
    pub fn from_untyped_prog(untyped_prog: UnTypedProg<Witness, Extension>) -> Rc<Self> {
        assert!(
            !untyped_prog.0.is_empty(),
            "Untyped Program len must be greater than 0"
        );
        let mut dag_list: Vec<Rc<TermDag<_, _>>> = vec![];
        for (index, term) in untyped_prog.0.into_iter().enumerate() {
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
                //TODO: understand how Fail works and rename `a` and `b`
                Term::Fail(a, b) => Rc::new(TermDag::Fail(a, b)),
                Term::Hidden(c) => Rc::new(TermDag::Hidden(c)),
                Term::Ext(e) => Rc::new(TermDag::Ext(e)),
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
struct RcWrapper<Witness, Extension> {
    rc: Rc<TermDag<Witness, Extension>>,
}

impl<Witness, Extension> PartialEq for RcWrapper<Witness, Extension> {
    fn eq(&self, other: &Self) -> bool {
        Rc::ptr_eq(&self.rc, &other.rc)
    }
}

impl<Witness, Extension> Eq for RcWrapper<Witness, Extension> {}

impl<Witness, Extension> From<Rc<TermDag<Witness, Extension>>> for RcWrapper<Witness, Extension> {
    fn from(dag: Rc<TermDag<Witness, Extension>>) -> Self {
        Self { rc: dag }
    }
}

impl<Witness, Extension> Hash for RcWrapper<Witness, Extension>
where
    Witness: Hash,
    Extension: Hash,
{
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.rc.hash(state);
    }
}

impl<Witness, Extension> TermDag<Witness, Extension>
where
    Witness: Hash + Clone,
    Extension: Hash + Clone,
{
    /// Convert a TermDag into into a untyped program vec.
    pub fn into_untyped_prog(self) -> UnTypedProg<Witness, Extension> {
        // helper function to recrusively compute the index positions
        // of the children.
        fn into_helper<Witness, Extension>(
            dag: Rc<TermDag<Witness, Extension>>,
            index_map: &mut HashMap<RcWrapper<Witness, Extension>, usize>,
            prog: &mut Vec<Term<Witness, Extension>>,
        ) -> usize
        where
            RcWrapper<Witness, Extension>: Hash,
            Witness: Clone,
            Extension: Clone,
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
                TermDag::Ext(ref e) => prog.push(Term::Ext(e.clone())),
                TermDag::Jet(ref j) => prog.push(Term::Jet(*j)),
            }
            // insert the current node remembering it's index for reusing
            index_map.insert(RcWrapper::from(dag), prog.len() - 1);
            prog.len() - 1
        }

        let mut prog = vec![];
        let mut index_map = HashMap::new();
        let _len = into_helper(Rc::new(self), &mut index_map, &mut prog);

        UnTypedProg(prog)
    }
}
