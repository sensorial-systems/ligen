//! Generators.

use crate::prelude::*;

mod file_generator;
pub use file_generator::*;

/// Generator trait.
pub trait Generator: FileGenerator {
    /// The Generator's entry point.
    fn generate(&self, project: &Project) -> Result<()>;
}
