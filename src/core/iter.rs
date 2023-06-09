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

//! Iterators over DAGs

use crate::core::Value;
use crate::types;
use crate::Error;

/// Iterator over witness values that asks for the value type on each iteration.
pub trait WitnessIterator {
    /// Return the next witness value of the given type.
    fn next(&mut self, ty: &types::Final) -> Result<Value, Error>;

    /// Consume the iterator and check the total witness length.
    fn finish(self) -> Result<(), Error>;
}

impl<I: Iterator<Item = Value>> WitnessIterator for I {
    fn next(&mut self, _ty: &types::Final) -> Result<Value, Error> {
        Iterator::next(self).ok_or(Error::NoMoreWitnesses)
    }

    fn finish(self) -> Result<(), Error> {
        Ok(())
    }
}
