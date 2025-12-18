// SPDX-License-Identifier: CC0-1.0

use core::convert::Infallible;
use core::marker::PhantomData;

pub struct CoreEnv<T>(PhantomData<T>);

impl CoreEnv<Infallible> {
    // After update to rust-simplicity this will be a Self. For now it needs to be ()
    // for compatibility with the existing impl of Jet for Core.
    pub const EMPTY: () = ();
}
