// SPDX-License-Identifier: CC0-1.0

//! Bit Machine Tracker
//!
//! This module provides traits for adding "trackers" to the bit machine execution
//! and pruning algorithms, which can provide debugging output or control which
//! branches are pruned. It also provides a couple example/utility trackers.
//!
//! It is a private module but all types and traits are re-exported above.

use std::collections::HashSet;

use crate::jet::Jet;
use crate::node::Inner;
use crate::{Ihr, RedeemNode, Value};

/// Write frame of a terminal (childless) Simplicity program node.
///
/// When a terminal node of a program is encountered in the Bit Machine, it
/// has a well-defined "output": the contents of the topmost write frame in
/// the machine. In particular, for `witness` nodes this will be the witness
/// data, for jets it will be the result of the jet, and so on.
///
/// For non-terminal nodes, the Bit Machine typically does some setup, then
/// executes the nodes' children, then does some teardown. So at no point is
/// there a well-defined "output" we can provide.
#[derive(Debug, Clone)]
pub enum NodeOutput<'m> {
    /// Non-terminal node, which has no output.
    NonTerminal,
    /// Node was a jet which failed, i.e. aborted the program, and therefore
    /// has no output.
    JetFailed,
    /// Node succeeded. This is its output frame.
    Success(super::FrameIter<'m>),
}

/// An object which can be used to introspect the execution of the Bit Machine.
///
/// If this tracker records accesses to the left and right children of `Case` nodes, you
/// may want to also implement [`PruneTracker`] so that this data can be used by
/// [`RedeemNode::prune_with_tracker`] to prune the program. The most straightforward
/// way to do this is to embed a [`SetTracker`] in your tracker and forward all the trait
/// methods to that.
pub trait ExecTracker<J: Jet> {
    /// Called immediately after a specific node of the program is executed, but before
    /// its children are executed.
    ///
    /// More precisely, this iterates through the through the Simplicity program tree in
    /// *pre* ordering. That is, for the program `comp iden unit` the nodes will be visited
    /// in the order `comp`, `iden`, `unit`.
    ///
    /// This method can be used for logging, to track left or right accesses of the children of a
    /// `Case` node (to do this, call `input.peek_bit()`; false means left and true means right),
    /// to extract debug information (which may be embedded in the hidden CMR in `AssertL`
    /// and `AssertR` nodes, depending how the program was constructed), and so on.
    ///
    /// The provided arguments are:
    ///   * `node` is the node which was just visited.
    ///   * `input` is an iterator over the read frame when the node's execution began
    ///   * for terminal nodes (`witness`, `unit`, `iden` and jets), `output` is an iterator
    ///     the write frame after the node has executed. See [`NodeOutput`] for more information.
    fn visit_node(&mut self, _node: &RedeemNode<J>, _input: super::FrameIter, _output: NodeOutput) {
    }
}

pub trait PruneTracker<J: Jet>: ExecTracker<J> {
    /// Returns true if the left branch of the of the `Case` node with the IHR `ihr` was taken.
    fn contains_left(&self, ihr: Ihr) -> bool;

    /// Returns true if the right branch of the of the `Case` node with the IHR `ihr` was taken.
    fn contains_right(&self, ihr: Ihr) -> bool;
}

/// Tracker of executed left and right branches for each case node.
#[derive(Clone, Debug, Default)]
pub struct SetTracker {
    left: HashSet<Ihr>,
    right: HashSet<Ihr>,
}

impl<J: Jet> ExecTracker<J> for SetTracker {
    fn visit_node<'d>(
        &mut self,
        node: &RedeemNode<J>,
        mut input: super::FrameIter,
        _output: NodeOutput,
    ) {
        match (node.inner(), input.next()) {
            (Inner::AssertL(..) | Inner::Case(..), Some(false)) => {
                self.left.insert(node.ihr());
            }
            (Inner::AssertR(..) | Inner::Case(..), Some(true)) => {
                self.right.insert(node.ihr());
            }
            _ => {}
        }
    }
}

impl<J: Jet> PruneTracker<J> for SetTracker {
    fn contains_left(&self, ihr: Ihr) -> bool {
        self.left.contains(&ihr)
    }

    fn contains_right(&self, ihr: Ihr) -> bool {
        self.right.contains(&ihr)
    }
}

/// Tracker that does not do anything (noop).
#[derive(Copy, Clone, Debug)]
pub struct NoTracker;

impl<J: Jet> ExecTracker<J> for NoTracker {
    fn visit_node<'d>(
        &mut self,
        node: &RedeemNode<J>,
        mut input: super::FrameIter,
        output: NodeOutput,
    ) {
        if cfg!(test) {
            // In unit tests, attempt to decode values from the frames, confirming that
            // decoding works.
            Value::from_padded_bits(&mut input, &node.arrow().source)
                .expect("decoding input should work");
            if let NodeOutput::Success(mut output) = output {
                Value::from_padded_bits(&mut output, &node.arrow().target)
                    .expect("decoding output should work");
            }
        }
    }
}

/// Tracker that just outputs all its activity to stderr.
#[derive(Clone, Debug, Default)]
pub struct StderrTracker {
    exec_count: usize,
    inner: SetTracker,
}

impl StderrTracker {
    /// Constructs a new empty [`StderrTracker`], ready for use.
    pub fn new() -> Self {
        Self::default()
    }
}

impl<J: Jet> ExecTracker<J> for StderrTracker {
    fn visit_node(&mut self, node: &RedeemNode<J>, input: super::FrameIter, output: NodeOutput) {
        let input_val = Value::from_padded_bits(&mut input.clone(), &node.arrow().source)
            .expect("input from bit machine will always be well-formed");
        eprintln!(
            "[{:4}] exec {:10} {}",
            self.exec_count,
            node.inner(),
            node.arrow()
        );
        eprintln!("       input {input_val}");
        match output.clone() {
            NodeOutput::NonTerminal => { /* don't bother describing non-terminal output */ }
            NodeOutput::JetFailed => eprintln!("    JET FAILED"),
            NodeOutput::Success(mut output) => {
                let output_val = Value::from_padded_bits(&mut output, &node.arrow().target)
                    .expect("output from bit machine will always be well-formed");
                eprintln!("       output {output_val}");
            }
        }

        if let crate::node::Inner::AssertL(_, cmr) = node.inner() {
            // SimplicityHL, when compiling in "debug mode", tags nodes by inserting
            // synthetic AssertL nodes where the "cmr" is actually a key into a lookup
            // table of debug information. An implementation of ExecTracker within
            // the compiler itself might do a lookup here to output more useful
            // information to the user.
            eprintln!("      [debug] assertL CMR {cmr}");
        }

        ExecTracker::<J>::visit_node(&mut self.inner, node, input, output);
        self.exec_count += 1;
        eprintln!();
    }
}

impl<J: Jet> PruneTracker<J> for StderrTracker {
    fn contains_left(&self, ihr: Ihr) -> bool {
        if PruneTracker::<J>::contains_left(&self.inner, ihr) {
            true
        } else {
            eprintln!("Pruning unexecuted left child of IHR {ihr}");
            false
        }
    }

    fn contains_right(&self, ihr: Ihr) -> bool {
        if PruneTracker::<J>::contains_right(&self.inner, ihr) {
            true
        } else {
            eprintln!("Pruning unexecuted right child of IHR {ihr}");
            false
        }
    }
}
