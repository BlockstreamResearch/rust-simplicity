// Rust Simplicity Library
// Written in 2023 by
//   Andrew Poelstra <rust-simplicity@wpsoftware.net>
//   Christian Lewe <clewe@blockstream.com>
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

//! The Simplicity Human-Readable Encoding
//!
//! This module provides the ability to decode and encode [`crate::NamedCommitNode`]s
//! in a human-readable format.
//!

mod named_node;

use crate::jet::Jet;
use crate::node::CommitNode;

use std::collections::HashMap;
use std::str;
use std::sync::Arc;

pub use self::named_node::NamedCommitNode;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Forest<J: Jet> {
    roots: HashMap<Arc<str>, Arc<NamedCommitNode<J>>>,
}

impl<J: Jet> Forest<J> {
    /// Parses a program from a bytestring
    pub fn from_program(root: Arc<CommitNode<J>>) -> Self {
        let root = NamedCommitNode::from_node(&root);
        let mut roots = HashMap::new();
        roots.insert("main".into(), root);
        Forest { roots }
    }
}
