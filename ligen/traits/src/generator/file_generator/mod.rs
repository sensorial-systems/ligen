//! File generator module.

mod file;

pub use file::*;

use crate::prelude::*;

/// File generator.
pub trait FileGenerator {
    /// Generate files.
    fn generate_files(&self, file_set: &mut FileSet, project: &Project) -> Result<()>;
}
