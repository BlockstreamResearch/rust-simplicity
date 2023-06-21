// Rust Simplicity Library
// Written in 2023 by
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

use super::FailEntropy;
use crate::jet::Jet;
use crate::{Cmr, Value};

use std::fmt;
use std::sync::Arc;

/// Internal "Simplicity DAG" structure.
///
/// This structure is used to indicate the type of a node and provide
/// pointers or references to its children, if any.
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Debug, Hash)]
pub enum Inner<C, J: Jet, W> {
    /// Identity
    Iden,
    /// Unit constant
    Unit,
    /// Left injection of some child
    InjL(C),
    /// Right injection of some child
    InjR(C),
    /// Take of some child
    Take(C),
    /// Drop of some child
    Drop(C),
    /// Composition of a left and right child
    Comp(C, C),
    /// Case of a left and right child
    Case(C, C),
    /// Left assertion of a left and right child.
    AssertL(C, Cmr),
    /// Right assertion of a left and right child.
    AssertR(Cmr, C),
    /// Pair of a left and right child
    Pair(C, C),
    /// Disconnect of a left and right child
    Disconnect(C, C),
    /// Witness data (missing during commitment, inserted during redemption)
    Witness(W),
    /// Universal fail
    Fail(FailEntropy),
    /// Application jet
    Jet(J),
    /// Constant word
    Word(Arc<Value>),
}

impl<C, J: Jet, W> Inner<C, J, W> {
    /// Convert a node's combinator data to a different type.
    pub fn map<D, F: FnMut(C) -> D>(self, mut f: F) -> Inner<D, J, W> {
        match self {
            Inner::Iden => Inner::Iden,
            Inner::Unit => Inner::Unit,
            Inner::InjL(c) => Inner::InjL(f(c)),
            Inner::InjR(c) => Inner::InjR(f(c)),
            Inner::Take(c) => Inner::Take(f(c)),
            Inner::Drop(c) => Inner::Drop(f(c)),
            Inner::Comp(cl, cr) => Inner::Comp(f(cl), f(cr)),
            Inner::Case(cl, cr) => Inner::Case(f(cl), f(cr)),
            Inner::AssertL(c, cmr) => Inner::AssertL(f(c), cmr),
            Inner::AssertR(cmr, c) => Inner::AssertR(cmr, f(c)),
            Inner::Pair(cl, cr) => Inner::Pair(f(cl), f(cr)),
            Inner::Disconnect(cl, cr) => Inner::Disconnect(f(cl), f(cr)),
            Inner::Witness(w) => Inner::Witness(w),
            Inner::Fail(entropy) => Inner::Fail(entropy),
            Inner::Jet(j) => Inner::Jet(j),
            Inner::Word(w) => Inner::Word(w),
        }
    }

    /// Convert a node's combinator data to a different type.
    pub fn map_result<D, E, F: FnMut(C) -> Result<D, E>>(
        self,
        mut f: F,
    ) -> Result<Inner<D, J, W>, E> {
        Ok(match self {
            Inner::Iden => Inner::Iden,
            Inner::Unit => Inner::Unit,
            Inner::InjL(c) => Inner::InjL(f(c)?),
            Inner::InjR(c) => Inner::InjR(f(c)?),
            Inner::Take(c) => Inner::Take(f(c)?),
            Inner::Drop(c) => Inner::Drop(f(c)?),
            Inner::Comp(cl, cr) => Inner::Comp(f(cl)?, f(cr)?),
            Inner::Case(cl, cr) => Inner::Case(f(cl)?, f(cr)?),
            Inner::AssertL(c, cmr) => Inner::AssertL(f(c)?, cmr),
            Inner::AssertR(cmr, c) => Inner::AssertR(cmr, f(c)?),
            Inner::Pair(cl, cr) => Inner::Pair(f(cl)?, f(cr)?),
            Inner::Disconnect(cl, cr) => Inner::Disconnect(f(cl)?, f(cr)?),
            Inner::Witness(w) => Inner::Witness(w),
            Inner::Fail(entropy) => Inner::Fail(entropy),
            Inner::Jet(j) => Inner::Jet(j),
            Inner::Word(w) => Inner::Word(w),
        })
    }

    /// Convert a node's combinator data to a different type, mapping each
    /// child separately.
    ///
    /// Importantly, the child of an `AssertR` node is considered the left
    /// child, because as a DAG node, this is the sole (left) child, even
    /// though as a combinator, it is a right child.
    pub fn map_left_right<D, FL, FR>(self, fl: FL, fr: FR) -> Inner<D, J, W>
    where
        FL: FnOnce(C) -> D,
        FR: FnOnce(C) -> D,
    {
        match self {
            Inner::Iden => Inner::Iden,
            Inner::Unit => Inner::Unit,
            Inner::InjL(c) => Inner::InjL(fl(c)),
            Inner::InjR(c) => Inner::InjR(fl(c)),
            Inner::Take(c) => Inner::Take(fl(c)),
            Inner::Drop(c) => Inner::Drop(fl(c)),
            Inner::Comp(cl, cr) => Inner::Comp(fl(cl), fr(cr)),
            Inner::Case(cl, cr) => Inner::Case(fl(cl), fr(cr)),
            Inner::AssertL(c, cmr) => Inner::AssertL(fl(c), cmr),
            Inner::AssertR(cmr, c) => Inner::AssertR(cmr, fl(c)),
            Inner::Pair(cl, cr) => Inner::Pair(fl(cl), fr(cr)),
            Inner::Disconnect(cl, cr) => Inner::Disconnect(fl(cl), fr(cr)),
            Inner::Witness(w) => Inner::Witness(w),
            Inner::Fail(entropy) => Inner::Fail(entropy),
            Inner::Jet(j) => Inner::Jet(j),
            Inner::Word(w) => Inner::Word(w),
        }
    }

    /// Take references to all contained data.
    pub fn as_ref(&self) -> Inner<&C, J, &W> {
        match self {
            Inner::Iden => Inner::Iden,
            Inner::Unit => Inner::Unit,
            Inner::InjL(c) => Inner::InjL(c),
            Inner::InjR(c) => Inner::InjR(c),
            Inner::Take(c) => Inner::Take(c),
            Inner::Drop(c) => Inner::Drop(c),
            Inner::Comp(cl, cr) => Inner::Comp(cl, cr),
            Inner::Case(cl, cr) => Inner::Case(cl, cr),
            Inner::AssertL(c, cmr) => Inner::AssertL(c, *cmr),
            Inner::AssertR(cmr, c) => Inner::AssertR(*cmr, c),
            Inner::Pair(cl, cr) => Inner::Pair(cl, cr),
            Inner::Disconnect(cl, cr) => Inner::Disconnect(cl, cr),
            Inner::Witness(w) => Inner::Witness(w),
            Inner::Fail(entropy) => Inner::Fail(*entropy),
            Inner::Jet(j) => Inner::Jet(*j),
            Inner::Word(w) => Inner::Word(Arc::clone(w)),
        }
    }

    /// Convert a node's witness data to a different type.
    pub fn map_witness<V, F: FnOnce(W) -> V>(self, f: F) -> Inner<C, J, V> {
        match self {
            Inner::Iden => Inner::Iden,
            Inner::Unit => Inner::Unit,
            Inner::InjL(c) => Inner::InjL(c),
            Inner::InjR(c) => Inner::InjR(c),
            Inner::Take(c) => Inner::Take(c),
            Inner::Drop(c) => Inner::Drop(c),
            Inner::Comp(cl, cr) => Inner::Comp(cl, cr),
            Inner::Case(cl, cr) => Inner::Case(cl, cr),
            Inner::AssertL(c, cmr) => Inner::AssertL(c, cmr),
            Inner::AssertR(cmr, c) => Inner::AssertR(cmr, c),
            Inner::Pair(cl, cr) => Inner::Pair(cl, cr),
            Inner::Disconnect(cl, cr) => Inner::Disconnect(cl, cr),
            Inner::Witness(w) => Inner::Witness(f(w)),
            Inner::Fail(entropy) => Inner::Fail(entropy),
            Inner::Jet(j) => Inner::Jet(j),
            Inner::Word(w) => Inner::Word(w),
        }
    }

    /// Convert a node's witness data to a different type.
    pub fn map_witness_result<V, E, F: FnOnce(W) -> Result<V, E>>(
        self,
        f: F,
    ) -> Result<Inner<C, J, V>, E> {
        Ok(match self {
            Inner::Iden => Inner::Iden,
            Inner::Unit => Inner::Unit,
            Inner::InjL(c) => Inner::InjL(c),
            Inner::InjR(c) => Inner::InjR(c),
            Inner::Take(c) => Inner::Take(c),
            Inner::Drop(c) => Inner::Drop(c),
            Inner::Comp(cl, cr) => Inner::Comp(cl, cr),
            Inner::Case(cl, cr) => Inner::Case(cl, cr),
            Inner::AssertL(c, cmr) => Inner::AssertL(c, cmr),
            Inner::AssertR(cmr, c) => Inner::AssertR(cmr, c),
            Inner::Pair(cl, cr) => Inner::Pair(cl, cr),
            Inner::Disconnect(cl, cr) => Inner::Disconnect(cl, cr),
            Inner::Witness(w) => Inner::Witness(f(w)?),
            Inner::Fail(entropy) => Inner::Fail(entropy),
            Inner::Jet(j) => Inner::Jet(j),
            Inner::Word(w) => Inner::Word(w),
        })
    }
}

impl<C, J: Jet, W> Inner<Option<C>, J, W> {
    /// Convert an `Inner<Option<C>, J, W>` to an `Option<Inner<C, J, W>>`.
    pub fn transpose(self) -> Option<Inner<C, J, W>> {
        Some(match self {
            Inner::Iden => Inner::Iden,
            Inner::Unit => Inner::Unit,
            Inner::InjL(c) => Inner::InjL(c?),
            Inner::InjR(c) => Inner::InjR(c?),
            Inner::Take(c) => Inner::Take(c?),
            Inner::Drop(c) => Inner::Drop(c?),
            Inner::Comp(cl, cr) => Inner::Comp(cl?, cr?),
            Inner::Case(cl, cr) => Inner::Case(cl?, cr?),
            Inner::AssertL(c, cmr) => Inner::AssertL(c?, cmr),
            Inner::AssertR(cmr, c) => Inner::AssertR(cmr, c?),
            Inner::Pair(cl, cr) => Inner::Pair(cl?, cr?),
            Inner::Disconnect(cl, cr) => Inner::Disconnect(cl?, cr?),
            Inner::Witness(w) => Inner::Witness(w),
            Inner::Fail(entropy) => Inner::Fail(entropy),
            Inner::Jet(j) => Inner::Jet(j),
            Inner::Word(w) => Inner::Word(w),
        })
    }
}

impl<C, J: Jet, W> Inner<C, J, Option<W>> {
    /// Convert an `Inner<C, J, Option<W>>` to an `Option<Inner<C, J, W>>`.
    pub fn transpose_witness(self) -> Option<Inner<C, J, W>> {
        Some(match self {
            Inner::Iden => Inner::Iden,
            Inner::Unit => Inner::Unit,
            Inner::InjL(c) => Inner::InjL(c),
            Inner::InjR(c) => Inner::InjR(c),
            Inner::Take(c) => Inner::Take(c),
            Inner::Drop(c) => Inner::Drop(c),
            Inner::Comp(cl, cr) => Inner::Comp(cl, cr),
            Inner::Case(cl, cr) => Inner::Case(cl, cr),
            Inner::AssertL(c, cmr) => Inner::AssertL(c, cmr),
            Inner::AssertR(cmr, c) => Inner::AssertR(cmr, c),
            Inner::Pair(cl, cr) => Inner::Pair(cl, cr),
            Inner::Disconnect(cl, cr) => Inner::Disconnect(cl, cr),
            Inner::Witness(w) => Inner::Witness(w?),
            Inner::Fail(entropy) => Inner::Fail(entropy),
            Inner::Jet(j) => Inner::Jet(j),
            Inner::Word(w) => Inner::Word(w),
        })
    }
}

impl<C, J: Jet, W> fmt::Display for Inner<C, J, W> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Inner::Iden => f.write_str("iden"),
            Inner::Unit => f.write_str("unit"),
            Inner::InjL(_) => f.write_str("injl"),
            Inner::InjR(_) => f.write_str("injr"),
            Inner::Take(_) => f.write_str("take"),
            Inner::Drop(_) => f.write_str("drop"),
            Inner::Comp(_, _) => f.write_str("comp"),
            Inner::Case(_, _) => f.write_str("case"),
            Inner::AssertL(_, _) => f.write_str("assertl"),
            Inner::AssertR(_, _) => f.write_str("assertr"),
            Inner::Pair(_, _) => f.write_str("pair"),
            Inner::Disconnect(_, _) => f.write_str("disconnect"),
            Inner::Witness(..) => f.write_str("witness"),
            Inner::Fail(..) => f.write_str("fail"),
            Inner::Jet(jet) => write!(f, "jet({})", jet),
            Inner::Word(w) => write!(f, "word({})", w),
        }
    }
}
