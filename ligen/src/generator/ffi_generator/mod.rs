//! FFI generator module.

mod temporary_ffi_project;
mod generic_ffi_generator;
mod build_type;

pub use generic_ffi_generator::*;
pub use temporary_ffi_project::*;
pub use build_type::*;

use crate::generator::{File, ProjectVisitor};

/// FFI generator.
pub trait FFIGenerator {
    /// Generate FFI.
    fn generate_ffi(&self, file: &mut File, visitor: &ProjectVisitor);
}