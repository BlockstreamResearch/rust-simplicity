use crate::core::types::VariableFactory;
use crate::jet::Application;
use std::marker::PhantomData;

/// Context for constructing a Simplicity program
#[allow(dead_code)]
pub struct Context<App: Application> {
    pub(crate) naming: VariableFactory,
    _application: PhantomData<App>,
}

impl<App: Application> Context<App> {
    /// Create a new context.
    pub fn new() -> Self {
        Self {
            naming: VariableFactory::new(),
            _application: PhantomData,
        }
    }
}

impl<App: Application> Default for Context<App> {
    fn default() -> Self {
        Self::new()
    }
}
