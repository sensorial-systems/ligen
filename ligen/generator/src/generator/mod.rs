//! Generators.

use crate::prelude::*;

/// Generator trait.
pub trait Generator {
    /// The Generator's entry point.
    fn generate(&self, library: &ligen_ir::Library) -> Result<()>;
}
