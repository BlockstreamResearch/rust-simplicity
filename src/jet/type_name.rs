// Rust Simplicity Library
// Written in 2022 by
//   Christian Lewe <clewe@blockstream.com>
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

//! # Manual type specification
//!
//! Source and target types of jet nodes need to be specified manually.

/// Byte-based specification of a Simplicity type.
///
/// Because jets are black boxes, the type inference engine has no access to their
/// source and target types. Therefore, these types need to be specified manually.
///
/// The types are written in prefix (aka Polish) notation,
/// where `+` and `*` represent sum and product types, respectively,
/// and base types are represented by the following:
///
/// | char | type         |
/// |------|--------------|
/// | `1`  | unit         |
/// | `2`  | single bit   |
/// | `i`  | 32-bit word  |
/// | `l`  | 64-bit word  |
/// | `h`  | 256-bit word |
///
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct TypeName(pub(crate) &'static [u8]);
