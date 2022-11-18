use crate::core::types::VariableFactory;
use crate::jet::Jet;
use std::marker::PhantomData;

/// Context for constructing a Simplicity program
#[allow(dead_code)]
pub struct Context<J: Jet> {
    pub(crate) naming: VariableFactory,
    _jet: PhantomData<J>,
}

impl<J: Jet> Context<J> {
    /// Create a new context.
    pub fn new() -> Self {
        Self {
            naming: VariableFactory::new(),
            _jet: PhantomData,
        }
    }
}

impl<J: Jet> Default for Context<J> {
    fn default() -> Self {
        Self::new()
    }
}
