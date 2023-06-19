use std::sync::Arc;

use crate::types;

/// Context for constructing a Simplicity program
#[allow(dead_code)]
pub struct Context {
    pow2: Vec<types::Type>,
    unit_ty: types::Type,
}

impl Context {
    /// Create a new context.
    pub fn new() -> Self {
        let one = types::Type::unit();
        let two = types::Type::sum(one.shallow_clone(), one);
        let pow2 = std::iter::successors(Some(two), |prev| {
            Some(types::Type::product(
                prev.shallow_clone(),
                prev.shallow_clone(),
            ))
        })
        .take(32)
        .collect();

        Self {
            pow2,
            unit_ty: types::Type::unit(),
        }
    }

    /// Accessor for the unit type
    pub fn unit_ty(&self) -> types::Type {
        self.unit_ty.shallow_clone()
    }

    /// Accessor for a type representing 2^(2^n) for n between 0 and 32 inclusive.
    ///
    /// Remember that for n = 0, this returns 2^1 = 2, not 2^0 = 1! To get the
    /// unit type, instead call `Context::unit_ty`.
    pub fn nth_power_of_2(&self, n: usize) -> types::Type {
        assert!(
            n < self.pow2.len(),
            "we have cached only {} powers of 2 but {} was requested",
            self.pow2.len(),
            n,
        );
        self.pow2[n].shallow_clone()
    }

    /// Accessor for the nth power of two as a type
    pub fn nth_power_of_2_final(&self, n: usize) -> Arc<types::Final> {
        assert!(
            n < self.pow2.len(),
            "we have cached only {} powers of 2 but {} was requested",
            self.pow2.len(),
            n,
        );
        self.pow2[n].final_data().unwrap()
    }
}

impl Default for Context {
    fn default() -> Self {
        Self::new()
    }
}
