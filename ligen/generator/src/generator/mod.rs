//! Generators.

use crate::prelude::*;

pub mod file_generator;

/// Generator trait.
pub trait Generator {
    type Input;
    /// The Generator's entry point.
    fn generate(&self, input: &Self::Input, folder: &std::path::Path) -> Result<()>;
}
