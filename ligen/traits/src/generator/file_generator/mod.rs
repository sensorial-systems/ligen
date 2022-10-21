//! File generator module.

use crate::prelude::*;
use crate::generator::FileSet;

/// File generator.
pub trait FileGenerator {
    /// Generate files.
    fn generate_files(&self, file_set: &mut FileSet, project: &Project) -> Result<()>;
}
