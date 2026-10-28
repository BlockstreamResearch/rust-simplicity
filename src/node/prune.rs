// SPDX-License-Identifier: CC0-1.0

//! Pruning of redeem programs
//!
//! See [`RedeemNode::prune`] for how a program is pruned.

use std::collections::HashMap;
use std::sync::Arc;

use crate::bit_machine::{ExecutionError, PruneTracker};
use crate::dag::{DagLike, InternalSharing, MaxSharing, PostOrderIterItem};
use crate::jet::JetEnvironment;
use crate::types;
use crate::{BitMachine, Ihr, Value};

use super::{
    Construct, ConstructData, ConstructNode, Constructible, Converter, Hide, Inner, Node, Redeem,
    RedeemData, RedeemNode,
};

/// Prunes `program`, as described in [`RedeemNode::prune`].
pub(super) fn prune<JE: JetEnvironment, T: PruneTracker>(
    program: &RedeemNode,
    env: &JE,
    tracker: &mut T,
) -> Result<Arc<RedeemNode>, ExecutionError> {
    // 1) Merge nodes with equal IHRs. The IHR covers the source and target types of a
    // node but not the types inside it, so nodes with equal IHRs can contain case
    // nodes with different IHRs, which the tracker would mark separately, while the
    // encoding of the pruned program has a single node for all of them.
    let program = reduce_to_max_sharing(program);

    // 2) Run the Bit Machine and mark (un)used branches.
    // This is the only fallible step in the pruning process.
    let mut mac = BitMachine::for_program(&program)?;
    mac.exec_with_tracker(&program, env, tracker)?;

    // 3) Find the nodes to keep: those reachable from the root through used branches.
    let tracker = &*tracker;
    let kept = kept_nodes(&program, |ihr| {
        (tracker.contains_left(ihr), tracker.contains_right(ihr))
    });

    // 4) Rebuild the program from the kept nodes and prune out unused case branches.
    Ok(rebuild(&program, &kept))
}

fn reduce_to_max_sharing(program: &RedeemNode) -> Arc<RedeemNode> {
    program
        .convert::<MaxSharing<Redeem>, _, _>(&mut Identity)
        .expect("copying a program is infallible")
}

/// Converts a redeem program into a copy of itself. Converting with [`MaxSharing`]
/// gives a copy with a single node for each IHR, as in the program's encoding.
struct Identity;

impl Converter<Redeem, Redeem> for Identity {
    type Error = std::convert::Infallible;

    fn convert_witness(
        &mut self,
        _: &PostOrderIterItem<&RedeemNode>,
        witness: &Value,
    ) -> Result<Value, Self::Error> {
        Ok(witness.shallow_clone())
    }

    fn convert_disconnect(
        &mut self,
        _: &PostOrderIterItem<&RedeemNode>,
        right: Option<&Arc<RedeemNode>>,
        _: &Arc<RedeemNode>,
    ) -> Result<Arc<RedeemNode>, Self::Error> {
        Ok(Arc::clone(right.expect(
            "disconnect node in redeem program has a right child",
        )))
    }

    fn convert_data(
        &mut self,
        data: &PostOrderIterItem<&RedeemNode>,
        inner: Inner<&Arc<RedeemNode>, &Arc<RedeemNode>, &Value>,
    ) -> Result<Arc<RedeemData>, Self::Error> {
        let converted_inner = inner
            .map(|node| node.cached_data())
            .map_disconnect(|node| node.cached_data())
            .map_witness(Value::shallow_clone);
        Ok(Arc::new(RedeemData::new(
            data.node.arrow().shallow_clone(),
            converted_inner,
        )))
    }
}

/// Returns which branch of a case node to hide, given whether its left and
/// right branches were used.
fn branch_to_hide(used_left: bool, used_right: bool) -> Hide {
    match (used_left, used_right) {
        (true, true) => Hide::Neither,
        (false, true) => Hide::Left,
        (true, false) => Hide::Right,
        // This is not possible with the default tracker, but if a custom
        // tracker is used with `prune_with_tracker`, we might end up here. It's
        // difficult to know the reason, but it is most likely unintended. 
        (false, false) => panic!("both branches of a case node were unused, which is not possible with the default tracker"),
    }
}

/// Walks the program from the root, descending only into used branches,
/// and returns the IHRs of the nodes to keep together with which branches
/// to hide for each of them. `used` returns whether the left and right
/// branches of the case node with the given IHR were used.
///
/// `root` must have a single node for each IHR, as `reduce_to_max_sharing`
/// gives: nodes are identified by IHR, so a node whose IHR is already kept is
/// the same node, with the same children.
fn kept_nodes(root: &RedeemNode, mut used: impl FnMut(Ihr) -> (bool, bool)) -> HashMap<Ihr, Hide> {
    let mut kept = HashMap::new();
    let mut stack = vec![root];
    while let Some(node) = stack.pop() {
        if kept.contains_key(&node.ihr()) {
            continue;
        }
        let hide = match node.inner() {
            Inner::Case(..) => {
                let (used_left, used_right) = used(node.ihr());
                branch_to_hide(used_left, used_right)
            }
            _ => Hide::Neither,
        };
        kept.insert(node.ihr(), hide);
        match (node.inner(), hide) {
            (Inner::Case(_, right), Hide::Left) => stack.push(right),
            (Inner::Case(left, _), Hide::Right) => stack.push(left),
            _ => {
                stack.extend(node.left_child());
                stack.extend(node.right_child());
            }
        }
    }
    kept
}

/// Rebuilds the program bottom-up from the kept nodes alone, and turns case
/// nodes with an unused branch into assertions.
///
/// `program` must be the program which `kept` came from, with a single node
/// for each IHR.
fn rebuild(program: &RedeemNode, kept: &HashMap<Ihr, Hide>) -> Arc<RedeemNode> {
    // Nodes in hidden branches are never built, so they cannot constrain the
    // types of the pruned program.
    // Because the types of the pruned program may change,
    // we construct a temporary witness program with unfinalized types.
    types::Context::with_context(|inference_context| {
        let mut built: HashMap<Ihr, Arc<ConstructNode>> = HashMap::new();
        for data in program.post_order_iter::<MaxSharing<Redeem>>() {
            let node = data.node;
            let hide = match kept.get(&node.ihr()) {
                Some(&hide) => hide,
                // Inside a hidden branch.
                None => continue,
            };
            // The program has one node for each IHR, so the walk kept the same
            // children, and children are visited before their parents, so they
            // are built.
            let child = |child: &Arc<RedeemNode>| {
                Arc::clone(
                    built
                        .get(&child.ihr())
                        .expect("children of kept nodes are kept"),
                )
            };
            let inner = match (node.inner(), hide) {
                (Inner::Case(left, right), Hide::Left) => Inner::AssertR(left.cmr(), child(right)),
                (Inner::Case(left, right), Hide::Right) => Inner::AssertL(child(left), right.cmr()),
                (inner, _) => inner
                    .as_ref()
                    .map(&child)
                    .map_disconnect(|right| Some(child(right)))
                    // The pruned type is not finalized at this point,
                    // so we cannot prune the witness value.
                    .map_witness(|witness| Some(witness.shallow_clone())),
            };
            let data = ConstructData::from_inner(
                &inference_context,
                inner
                    .as_ref()
                    .map(|node| node.cached_data())
                    .map_witness(Option::<Value>::clone),
            )
            .expect("pruned types should check out if unpruned types check out");
            built.insert(node.ihr(), Arc::new(Node::from_parts(inner, data)));
        }
        let pruned_witness_program = built.remove(&program.ihr()).expect("root is kept");
        // Finalize the types of the witness program.
        // We obtain the pruned redeem program.
        // Once the pruned type is finalized, we can proceed to prune witness values.
        pruned_witness_program
            .convert::<InternalSharing, _, _>(&mut Finalizer)
            .expect("finalization is infallible")
    })
}

/// Finalizes the types of a pruned program, and prunes its witness values to them.
struct Finalizer;

impl<'brand> Converter<Construct<'brand>, Redeem> for Finalizer {
    type Error = std::convert::Infallible;

    fn convert_witness(
        &mut self,
        data: &PostOrderIterItem<&ConstructNode>,
        witness: &Option<Value>,
    ) -> Result<Value, Self::Error> {
        let pruned_target_ty = data
            .node
            .arrow()
            .target
            .finalize()
            .expect("pruned types should check out if unpruned types check out");
        let pruned_witness = witness
            .as_ref()
            .expect("witness node that originally stems from redeem program should be populated")
            .prune(&pruned_target_ty)
            .expect("pruned type should be shrunken version of unpruned type");
        Ok(pruned_witness)
    }

    fn convert_disconnect(
        &mut self,
        _: &PostOrderIterItem<&ConstructNode>,
        right: Option<&Arc<RedeemNode>>,
        _: &Option<Arc<ConstructNode>>,
    ) -> Result<Arc<RedeemNode>, Self::Error> {
        Ok(right.map(Arc::clone).expect(
            "disconnect node that originally stems from redeem program should have all branches",
        ))
    }

    fn convert_data(
        &mut self,
        data: &PostOrderIterItem<&ConstructNode>,
        inner: Inner<&Arc<RedeemNode>, &Arc<RedeemNode>, &Value>,
    ) -> Result<Arc<RedeemData>, Self::Error> {
        // Finalize target types of witness nodes in advance so we can prune their values.
        let final_arrow = data
            .node
            .arrow()
            .finalize()
            .expect("pruned types should check out if unpruned types check out");
        let converted_inner = inner
            .map(|node| node.cached_data())
            .map_disconnect(|node| node.cached_data())
            .map_witness(Value::shallow_clone);
        Ok(Arc::new(RedeemData::new(final_arrow, converted_inner)))
    }
}
