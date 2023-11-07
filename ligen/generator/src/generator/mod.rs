//! Generators.

use crate::prelude::*;

pub mod file_generator;

/// Generator trait.
pub trait Generator {
    /// The Generator's entry point.
    fn generate(&self, library: &ligen_ir::Library, folder: &std::path::Path) -> Result<()>;
}
