//! File generator module.

use crate::generator::{FileSet, Context};

/// File generator.
pub trait FileGenerator {
    fn generate_file_set(&self, context: &Context, file_set: &mut FileSet);
}