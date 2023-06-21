use std::marker::PhantomData;

use crate::jet::Jet;

/// Context for constructing a Simplicity program
#[allow(dead_code)]
pub struct Context<J: Jet> {
    _jet: PhantomData<J>,
}

impl<J: Jet> Context<J> {
    /// Create a new context.
    pub fn new() -> Self {
        Self { _jet: PhantomData }
    }
}

impl<J: Jet> Default for Context<J> {
    fn default() -> Self {
        Self::new()
    }
}
