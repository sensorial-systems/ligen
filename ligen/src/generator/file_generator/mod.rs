//! File generator module.

use crate::generator::{FileSet, ProjectVisitor};

mod file_generator_visitors;
pub use file_generator_visitors::*;

/// File generator.
pub trait FileGenerator {
    /// Generate files.
    fn generate_files(&self, file_set: &mut FileSet, visitor: &ProjectVisitor);
}
