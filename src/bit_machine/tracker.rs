// SPDX-License-Identifier: CC0-1.0

//! Bit Machine Tracker
//!
//! This module provides traits for adding "trackers" to the bit machine execution
//! and pruning algorithms, which can provide debugging output or control which
//! branches are pruned. It also provides a couple example/utility trackers.
//!
//! It is a private module but all types and traits are re-exported above.

use simplicity_sys::ffi::UWORD;
use std::collections::HashSet;

use crate::jet::Jet;
use crate::{Cmr, Ihr, Value};

/// A type that keeps track of Bit Machine execution.
///
/// The trait is implemented for [`SetTracker`], that tracks which case branches were executed,
/// and it is implemented for [`NoTracker`], which is a dummy tracker that is
/// optimized out by the compiler.
///
/// The trait enables us to turn tracking on or off depending on a generic parameter.
pub trait ExecTracker<J: Jet> {
    /// Track the execution of the left branch of the case node with the given `ihr`.
    fn track_left(&mut self, ihr: Ihr);

    /// Track the execution of the right branch of the case node with the given `ihr`.
    fn track_right(&mut self, ihr: Ihr);

    /// Track the execution of a `jet` call with the given `input_buffer`, `output_buffer`, and call result `success`.
    fn track_jet_call(
        &mut self,
        jet: &J,
        input_buffer: &[UWORD],
        output_buffer: &[UWORD],
        success: bool,
    );

    /// Track the potential execution of a `dbg!` call with the given `cmr` and `value`.
    fn track_dbg_call(&mut self, cmr: &Cmr, value: Value);

    /// Check if tracking debug calls is enabled.
    fn is_track_debug_enabled(&self) -> bool;
}

/// Tracker of executed left and right branches for each case node.
#[derive(Clone, Debug, Default)]
pub struct SetTracker {
    left: HashSet<Ihr>,
    right: HashSet<Ihr>,
}

impl SetTracker {
    /// Access the set of IHRs of case nodes whose left branch was executed.
    pub fn left(&self) -> &HashSet<Ihr> {
        &self.left
    }

    /// Access the set of IHRs of case nodes whose right branch was executed.
    pub fn right(&self) -> &HashSet<Ihr> {
        &self.right
    }
}

/// Tracker that does not do anything (noop).
#[derive(Copy, Clone, Debug)]
pub struct NoTracker;

impl<J: Jet> ExecTracker<J> for SetTracker {
    fn track_left(&mut self, ihr: Ihr) {
        self.left.insert(ihr);
    }

    fn track_right(&mut self, ihr: Ihr) {
        self.right.insert(ihr);
    }

    fn track_jet_call(&mut self, _: &J, _: &[UWORD], _: &[UWORD], _: bool) {}

    fn track_dbg_call(&mut self, _: &Cmr, _: Value) {}

    fn is_track_debug_enabled(&self) -> bool {
        false
    }
}

impl<J: Jet> ExecTracker<J> for NoTracker {
    fn track_left(&mut self, _: Ihr) {}

    fn track_right(&mut self, _: Ihr) {}

    fn track_jet_call(&mut self, _: &J, _: &[UWORD], _: &[UWORD], _: bool) {}

    fn track_dbg_call(&mut self, _: &Cmr, _: Value) {}

    fn is_track_debug_enabled(&self) -> bool {
        // Set flag to test frame decoding in unit tests
        cfg!(test)
    }
}
