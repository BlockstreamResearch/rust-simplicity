use std::marker::PhantomData;
use std::sync::Arc;

use crate::core::types::{precomputed_square, RcVar, Type, Variable};
use crate::jet::Jet;
use crate::types;

/// Context for constructing a Simplicity program
#[allow(dead_code)]
pub struct Context<J: Jet> {
    pub(crate) naming: types::variable::Factory,
    pow2: Vec<RcVar>,
    _jet: PhantomData<J>,
}

impl<J: Jet> Context<J> {
    /// Create a new context.
    pub fn new() -> Self {
        let two = Variable::precomputed_2();
        let pow2 = std::iter::successors(Some(two), |prev| Some(precomputed_square(prev)))
            .take(32)
            .collect();

        Self {
            naming: types::variable::Factory::new(),
            pow2,
            _jet: PhantomData,
        }
    }

    /// Accessor for the nth power of two as a type
    pub fn nth_power_of_2_type(&self, n: usize) -> Arc<Type> {
        assert!(
            n < self.pow2.len(),
            "we have cached only {} powers of 2 but {} was requested",
            self.pow2.len(),
            n,
        );
        self.pow2[n].borrow().precomputed_finalized()
    }

    /// Accessor for the nth power of two as an `RcVar`
    pub(crate) fn nth_power_of_2_rcvar(&self, n: usize) -> &RcVar {
        assert!(
            n < self.pow2.len(),
            "we have cached only {} powers of 2 but {} was requested",
            self.pow2.len(),
            n,
        );
        &self.pow2[n]
    }
}

impl<J: Jet> Default for Context<J> {
    fn default() -> Self {
        Self::new()
    }
}
