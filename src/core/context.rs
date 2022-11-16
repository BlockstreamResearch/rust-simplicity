use crate::core::types::VariableFactory;
use crate::jet::Jet;
use std::marker::PhantomData;

/// Context for constructing a Simplicity program
#[allow(dead_code)]
pub struct Context<App: Jet> {
    pub(crate) naming: VariableFactory,
    _application: PhantomData<App>,
}

impl<App: Jet> Context<App> {
    /// Create a new context.
    pub fn new() -> Self {
        Self {
            naming: VariableFactory::new(),
            _application: PhantomData,
        }
    }
}

impl<App: Jet> Default for Context<App> {
    fn default() -> Self {
        Self::new()
    }
}
