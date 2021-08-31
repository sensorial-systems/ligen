//! File generator module.

use crate::generator::{FileSet, ProjectVisitor};

mod visitor;
pub use visitor::*;

/// File generator.
pub trait FileGenerator {
    /// Generate files.
    fn generate_files(&self, file_set: &mut FileSet, visitor: &ProjectVisitor);
}
