// SPDX-License-Identifier: CC0-1.0

//! Type-directed program generator
//!
//! [`generate_program`] chooses a type first and then only combinators which can
//! have that type, so every program it produces is well-typed. It never generates
//! `fail`, assertions or jets which can fail, so every program also runs to
//! completion. Case nodes branch on witness data, constants and jet outputs, and
//! nodes are sometimes reused, so that pruning has branches to hide and nodes
//! shared between hidden and kept code.
//!
//! It reads its choices with [`Unstructured`] rather than [`crate::Extractor`]:
//! once the data runs out, `Unstructured` returns defaults rather than failing, so
//! every input gives a program.
//!
//! It does not generate:
//!
//! * separate nodes with one IHR but different types inside them. Pruning merges
//!   nodes with equal IHRs before running a program, and only unit tests cover
//!   programs which need that merge;
//! * `fail`, assertions or jets which can fail. Pruning turns hidden branches into
//!   CMRs, so what they contain does not matter to it;
//! * jets which read the transaction environment, as libsimplicity runs the pruned
//!   programs without one.

use std::sync::Arc;

use arbitrary::{Result, Unstructured};
use simplicity::dag::{InternalSharing, PostOrderIterItem};
use simplicity::jet::{Elements, Jet as _};
use simplicity::node::{
    Construct, Converter, CoreConstructible as _, DisconnectConstructible as _, Inner, Redeem,
    RedeemData, WitnessConstructible as _,
};
use simplicity::types::{self, Final};
use simplicity::{ConstructNode, RedeemNode, Value};

/// Maximum number of combinators that the generator chooses, which bounds the
/// size of the program to a small multiple of this.
const MAX_BUDGET: usize = 200;

/// Maximum number of case nodes which run on two inputs. Each one doubles the
/// cost of the program, if it is nested in another.
const MAX_TWICE: usize = 2;

/// Maximum bit width of a type for which a constant is scribed into the program.
const MAX_SCRIBE_WIDTH: usize = 16;

/// Jets which cannot fail and do not read the transaction environment.
///
/// These are Elements jets, rather than Core jets, because libsimplicity decodes
/// programs with the Elements jet codes.
const JETS: &[Elements] = &[
    Elements::Add8,
    Elements::Add16,
    Elements::Add32,
    Elements::All8,
    Elements::And8,
    Elements::Ch8,
    Elements::Complement8,
    Elements::Complement32,
    Elements::Eq1,
    Elements::Eq8,
    Elements::Eq16,
    Elements::Eq32,
    Elements::FullAdd8,
    Elements::High8,
    Elements::Increment8,
    Elements::IsOne8,
    Elements::IsZero8,
    Elements::Le8,
    Elements::LeftShift8,
    Elements::Low8,
    Elements::Lt8,
    Elements::Lt32,
    Elements::Maj8,
    Elements::Max8,
    Elements::Min8,
    Elements::Multiply8,
    Elements::Negate8,
    Elements::One8,
    Elements::Or8,
    Elements::RightShift8,
    Elements::Some8,
    Elements::Subtract8,
    Elements::Subtract32,
    Elements::Xor8,
    Elements::Xor32,
];

type Node<'brand> = Arc<ConstructNode<'brand>>;

/// Generates a program of type 1 → 1, together with its witness data.
///
/// Once `u` runs out of data the generator only makes cheap choices, so this
/// returns a program for any input.
pub fn generate_program(u: &mut Unstructured) -> Result<Arc<RedeemNode>> {
    types::Context::with_context(|ctx| {
        let mut gen = Generator {
            budget: u.int_in_range(1..=MAX_BUDGET)?,
            u,
            ctx: &ctx,
            pool: vec![],
            twice: 0,
        };
        let program = gen.root()?;
        program
            .set_arrow_to_program()
            .expect("generated programs have type 1 → 1");
        program.convert::<InternalSharing, _, _>(&mut Finalizer { u: gen.u })
    })
}

/// Finalizes the types of a generated program, and generates a value of its
/// finalized type for each witness node, which only then is known.
struct Finalizer<'a, 'u> {
    u: &'a mut Unstructured<'u>,
}

impl<'brand> Converter<Construct<'brand>, Redeem> for Finalizer<'_, '_> {
    type Error = arbitrary::Error;

    fn convert_witness(
        &mut self,
        data: &PostOrderIterItem<&ConstructNode<'brand>>,
        _: &Option<Value>,
    ) -> Result<Value> {
        let ty = data
            .node
            .arrow()
            .target
            .finalize()
            .expect("generated programs are well-typed");
        value(self.u, &ty)
    }

    fn convert_disconnect(
        &mut self,
        _: &PostOrderIterItem<&ConstructNode<'brand>>,
        right: Option<&Arc<RedeemNode>>,
        _: &Option<Node<'brand>>,
    ) -> Result<Arc<RedeemNode>> {
        Ok(Arc::clone(
            right.expect("generated disconnect nodes have a right child"),
        ))
    }

    fn convert_data(
        &mut self,
        data: &PostOrderIterItem<&ConstructNode<'brand>>,
        inner: Inner<&Arc<RedeemNode>, &Arc<RedeemNode>, &Value>,
    ) -> Result<Arc<RedeemData>> {
        let arrow = data
            .node
            .arrow()
            .finalize()
            .expect("generated programs are well-typed");
        let inner = inner
            .map(|node| node.cached_data())
            .map_disconnect(|node| node.cached_data())
            .map_witness(Value::shallow_clone);
        Ok(Arc::new(RedeemData::new(arrow, inner)))
    }
}

struct Generator<'a, 'u, 'brand> {
    u: &'a mut Unstructured<'u>,
    ctx: &'a types::Context<'brand>,
    /// Number of combinators left to choose before only leaves are generated.
    budget: usize,
    /// Every node generated so far, with the type it was generated for.
    pool: Vec<(Arc<Final>, Arc<Final>, Node<'brand>)>,
    /// Number of case nodes generated which run on two inputs.
    twice: usize,
}

impl<'brand> Generator<'_, '_, 'brand> {
    /// Generates the program, of type 1 → 1, as a combinator with children, so
    /// that the program is never a single leaf.
    fn root(&mut self) -> Result<Node<'brand>> {
        let unit = Final::unit();
        match self.u.choose_index(3)? {
            0 => self.branch(&unit, &unit),
            1 => self.branch_twice(&unit, &unit),
            _ => self.jet(&unit, &unit),
        }
    }

    /// Generates a node of type `a` → `b`.
    ///
    /// Type inference may finalize the node to a smaller type, where nothing in
    /// the program constrains part of `a` or `b`.
    fn node(&mut self, a: &Arc<Final>, b: &Arc<Final>) -> Result<Node<'brand>> {
        if self.budget == 0 {
            return self.leaf(a, b);
        }
        self.budget -= 1;

        // `choose_index` returns 0 once the data runs out, so 0 must terminate.
        let node = match self.u.choose_index(8)? {
            0 => self.leaf(a, b)?,
            1 => self.reuse(a, b)?,
            2 => self.shape(a, b)?,
            3 => {
                let mid = self.ty(2)?;
                let left = self.node(a, &mid)?;
                let right = self.node(&mid, b)?;
                typed(Node::comp(&left, &right))
            }
            4 => self.branch(a, b)?,
            5 => self.jet(a, b)?,
            6 => self.branch_twice(a, b)?,
            _ => self.disconnect(a, b)?,
        };
        self.pool
            .push((Arc::clone(a), Arc::clone(b), Arc::clone(&node)));
        Ok(node)
    }

    /// Generates a node of type `a` → `b` which has no children to generate.
    fn leaf(&mut self, a: &Arc<Final>, b: &Arc<Final>) -> Result<Node<'brand>> {
        if b.is_unit() {
            Ok(Node::unit(self.ctx))
        } else if a == b && self.u.arbitrary()? {
            Ok(Node::iden(self.ctx))
        } else if b.bit_width() <= MAX_SCRIBE_WIDTH && self.u.arbitrary()? {
            // Constant words have source type 1, so discard the input first.
            let constant = Node::scribe(self.ctx, &value(self.u, b)?);
            Ok(typed(Node::comp(&Node::unit(self.ctx), &constant)))
        } else {
            Ok(Node::witness(self.ctx, None))
        }
    }

    /// Returns a node of type `a` → `b` which was generated before, so that it is
    /// shared, or a leaf if there is none.
    fn reuse(&mut self, a: &Arc<Final>, b: &Arc<Final>) -> Result<Node<'brand>> {
        let matches: Vec<_> = self
            .pool
            .iter()
            .filter(|(pa, pb, _)| pa == a && pb == b)
            .map(|(_, _, node)| Arc::clone(node))
            .collect();
        if matches.is_empty() {
            self.leaf(a, b)
        } else {
            Ok(Arc::clone(&matches[self.u.choose_index(matches.len())?]))
        }
    }

    /// Generates a node whose combinator follows the shape of `a` or `b`.
    fn shape(&mut self, a: &Arc<Final>, b: &Arc<Final>) -> Result<Node<'brand>> {
        enum Shape {
            InjL,
            InjR,
            Pair,
            Take,
            Drop,
            Case,
        }

        let mut shapes = vec![];
        if b.as_sum().is_some() {
            shapes.extend([Shape::InjL, Shape::InjR]);
        }
        if b.as_product().is_some() {
            shapes.push(Shape::Pair);
        }
        if let Some((a0, _)) = a.as_product() {
            shapes.extend([Shape::Take, Shape::Drop]);
            if a0.as_sum().is_some() {
                shapes.push(Shape::Case);
            }
        }
        if shapes.is_empty() {
            return self.leaf(a, b);
        }

        let sum = |ty: &Arc<Final>| ty.as_sum().map(|(l, r)| (Arc::clone(l), Arc::clone(r)));
        let product =
            |ty: &Arc<Final>| ty.as_product().map(|(l, r)| (Arc::clone(l), Arc::clone(r)));
        Ok(match shapes[self.u.choose_index(shapes.len())?] {
            Shape::InjL => Node::injl(&self.node(a, &sum(b).unwrap().0)?),
            Shape::InjR => Node::injr(&self.node(a, &sum(b).unwrap().1)?),
            Shape::Pair => {
                let (b0, b1) = product(b).unwrap();
                typed(Node::pair(&self.node(a, &b0)?, &self.node(a, &b1)?))
            }
            Shape::Take => Node::take(&self.node(&product(a).unwrap().0, b)?),
            Shape::Drop => Node::drop_(&self.node(&product(a).unwrap().1, b)?),
            Shape::Case => {
                let (a0, a1) = product(a).unwrap();
                let (l, r) = sum(&a0).unwrap();
                let left = self.node(&Final::product(l, Arc::clone(&a1)), b)?;
                let right = self.node(&Final::product(r, a1), b)?;
                typed(Node::case(&left, &right))
            }
        })
    }

    /// Generates `comp (pair s iden) (case l r)` of type `a` → `b`: compute a sum
    /// from the input, then branch on it with the input available to both branches.
    fn branch(&mut self, a: &Arc<Final>, b: &Arc<Final>) -> Result<Node<'brand>> {
        let (l, r) = (self.ty(1)?, self.ty(1)?);
        let scrutinee = self.node(a, &Final::sum(Arc::clone(&l), Arc::clone(&r)))?;
        let left = self.node(&Final::product(l, Arc::clone(a)), b)?;
        let right = self.node(&Final::product(r, Arc::clone(a)), b)?;
        let input = typed(Node::pair(&scrutinee, &Node::iden(self.ctx)));
        Ok(typed(Node::comp(&input, &typed(Node::case(&left, &right)))))
    }

    /// Generates `comp (pair (comp (pair s1 iden) c) (comp (pair s2 iden) c)) (take iden)`
    /// of type `a` → `b`: a single case node `c` which runs on two inputs, so that it
    /// can take both of its branches.
    fn branch_twice(&mut self, a: &Arc<Final>, b: &Arc<Final>) -> Result<Node<'brand>> {
        if self.twice == MAX_TWICE {
            return self.branch(a, b);
        }
        self.twice += 1;
        let (l, r) = (self.ty(1)?, self.ty(1)?);
        let sum = Final::sum(Arc::clone(&l), Arc::clone(&r));
        let left = self.node(&Final::product(l, Arc::clone(a)), b)?;
        let right = self.node(&Final::product(r, Arc::clone(a)), b)?;
        let case = typed(Node::case(&left, &right));
        let first = self.run_case(a, &sum, &case)?;
        let second = self.run_case(a, &sum, &case)?;
        let both = typed(Node::pair(&first, &second));
        Ok(typed(Node::comp(&both, &Node::take(&Node::iden(self.ctx)))))
    }

    /// Generates `comp (pair s iden) case` of type `a` → `b`, for a case node of
    /// type (`sum` × `a`) → `b`.
    fn run_case(
        &mut self,
        a: &Arc<Final>,
        sum: &Arc<Final>,
        case: &Node<'brand>,
    ) -> Result<Node<'brand>> {
        let scrutinee = self.node(a, sum)?;
        let input = typed(Node::pair(&scrutinee, &Node::iden(self.ctx)));
        Ok(typed(Node::comp(&input, case)))
    }

    /// Generates `comp (disconnect s t) rest` of type `a` → `b`, where `s` has type
    /// 2^256 × `a` → `b0` × `c`, the disconnected expression `t` has type `c` → `d`,
    /// and `rest` has type `b0` × `d` → `b`.
    fn disconnect(&mut self, a: &Arc<Final>, b: &Arc<Final>) -> Result<Node<'brand>> {
        let (b0, c, d) = (self.ty(1)?, self.ty(1)?, self.ty(1)?);
        let source = Final::product(Final::two_two_n_fixed::<8>(), Arc::clone(a));
        let left = self.node(&source, &Final::product(Arc::clone(&b0), Arc::clone(&c)))?;
        let right = self.node(&c, &d)?;
        let disconnect = typed(Node::disconnect(&left, &Some(right)));
        let rest = self.node(&Final::product(b0, d), b)?;
        Ok(typed(Node::comp(&disconnect, &rest)))
    }

    /// Generates `comp s (comp jet t)` of type `a` → `b`, where `s` produces the
    /// input of the jet and `t` consumes its output.
    fn jet(&mut self, a: &Arc<Final>, b: &Arc<Final>) -> Result<Node<'brand>> {
        let jet = JETS[self.u.choose_index(JETS.len())?];
        let source = self.node(a, &jet.source_ty().to_final())?;
        let target = self.node(&jet.target_ty().to_final(), b)?;
        let jet = Node::jet(self.ctx, &jet);
        Ok(typed(Node::comp(
            &source,
            &typed(Node::comp(&jet, &target)),
        )))
    }

    /// Generates a type of at most the given depth.
    fn ty(&mut self, depth: usize) -> Result<Arc<Final>> {
        let n_choices = if depth == 0 { 4 } else { 6 };
        Ok(match self.u.choose_index(n_choices)? {
            0 => Final::unit(),
            1 => Final::two_two_n_fixed::<0>(),
            2 => Final::two_two_n_fixed::<3>(),
            // A type that some jet consumes or produces.
            3 => {
                let jet = JETS[self.u.choose_index(JETS.len())?];
                if self.u.arbitrary()? {
                    jet.source_ty().to_final()
                } else {
                    jet.target_ty().to_final()
                }
            }
            4 => Final::sum(self.ty(depth - 1)?, self.ty(depth - 1)?),
            _ => Final::product(self.ty(depth - 1)?, self.ty(depth - 1)?),
        })
    }
}

/// Returns `n` deterministic pseudorandom inputs, for unit tests which run a
/// fuzz target on many generated programs without an RNG dependency.
pub fn pseudorandom_inputs(n: usize) -> impl Iterator<Item = Vec<u8>> {
    let mut state = 0x2545_f491_4f6c_dd1d_u64;
    let mut next = move || {
        state ^= state << 13;
        state ^= state >> 7;
        state ^= state << 17;
        state
    };
    (0..n).map(move |_| {
        let len = 16 + (next() % 4096) as usize;
        (0..len).map(|_| next() as u8).collect()
    })
}

/// Generates a value of the given type.
fn value(u: &mut Unstructured, ty: &Final) -> Result<Value> {
    if let Some((l, r)) = ty.as_sum() {
        if u.arbitrary()? {
            Ok(Value::right(Arc::clone(l), value(u, r)?))
        } else {
            Ok(Value::left(value(u, l)?, Arc::clone(r)))
        }
    } else if let Some((l, r)) = ty.as_product() {
        Ok(Value::product(value(u, l)?, value(u, r)?))
    } else {
        Ok(Value::unit())
    }
}

/// Unwraps the result of constructing a node, which only fails on a type error.
fn typed<T>(result: std::result::Result<T, types::Error>) -> T {
    result.expect("the generator only combines nodes of matching types")
}

#[cfg(test)]
mod tests {
    use super::*;
    use simplicity::bit_machine::ExecutionError;
    use simplicity::dag::DagLike as _;
    use simplicity::jet::elements::ElementsEnv;
    use simplicity::BitMachine;

    #[test]
    fn generator_yield() {
        const N: usize = 1000;
        let env = ElementsEnv::dummy();
        let (mut ran, mut over_50, mut max_nodes, mut min_nodes) = (0, 0, 0, usize::MAX);
        let (mut has_case, mut has_jet, mut has_witness, mut has_disconnect) = (0, 0, 0, 0);
        let (mut has_hidden, mut kept_case, mut failed) = (0, 0, 0);
        for data in pseudorandom_inputs(N) {
            let program = generate_program(&mut Unstructured::new(&data))
                .expect("the generator accepts any input");
            let n_nodes = program
                .as_ref()
                .post_order_iter::<InternalSharing>()
                .count();
            max_nodes = max_nodes.max(n_nodes);
            min_nodes = min_nodes.min(n_nodes);
            if n_nodes > 50 {
                over_50 += 1;
            }
            let has = |program: &RedeemNode, f: fn(&Inner<_, _, _>) -> bool| {
                program
                    .post_order_iter::<InternalSharing>()
                    .any(|data| f(data.node.inner()))
            };
            has_case += usize::from(has(&program, |i| matches!(i, Inner::Case(..))));
            has_jet += usize::from(has(&program, |i| matches!(i, Inner::Jet(..))));
            has_witness += usize::from(has(&program, |i| matches!(i, Inner::Witness(..))));
            has_disconnect += usize::from(has(&program, |i| matches!(i, Inner::Disconnect(..))));

            let mut mac = match BitMachine::for_program(&program) {
                Ok(mac) => mac,
                Err(_) => continue,
            };
            mac.exec(&program, &env)
                .expect("generated programs contain nothing that fails");
            ran += 1;
            let pruned = match program.prune(&env) {
                Ok(pruned) => pruned,
                Err(ExecutionError::PruningFailed) => {
                    failed += 1;
                    continue;
                }
                Err(e) => panic!("pruning should succeed when execution does: {e}"),
            };
            has_hidden += usize::from(has(&pruned, |i| {
                matches!(i, Inner::AssertL(..) | Inner::AssertR(..))
            }));
            kept_case += usize::from(has(&pruned, |i| matches!(i, Inner::Case(..))));
        }
        let stats = format!(
            "of {N}: ran {ran}, >50 nodes {over_50} (min {min_nodes}, max {max_nodes}), \
             case {has_case}, jet {has_jet}, witness {has_witness}, disconnect {has_disconnect}, \
             pruning hid a branch \
             {has_hidden}, kept a case node {kept_case}, failed {failed}"
        );
        // The inputs are fixed, so these only change when the generator or pruning
        // does.
        assert_eq!(ran, N, "{stats}");
        assert!(min_nodes >= 3, "{stats}");
        assert!(over_50 >= N * 3 / 4, "{stats}");
        assert!(has_case >= N * 9 / 10, "{stats}");
        assert!(has_jet >= N * 9 / 10, "{stats}");
        assert!(has_witness >= N * 9 / 10, "{stats}");
        assert!(has_disconnect >= N / 2, "{stats}");
        assert!(has_hidden >= N * 8 / 10, "{stats}");
        assert!(kept_case >= N / 4, "{stats}");
        assert!(failed <= N / 100, "{stats}");
    }
}
