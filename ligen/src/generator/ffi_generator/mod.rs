//! FFI generator module.

mod temporary_ffi_project;
mod generic_ffi_generator;
mod build_profile;

pub mod cargo;

pub use generic_ffi_generator::*;
pub use temporary_ffi_project::*;
pub use build_profile::*;

use crate::generator::{File, ProjectVisitor};
use crate::marshalling::Marshaller;

/// FFI generator.
pub trait FFIGenerator {
    /// Generate FFI.
    fn generate_ffi(&self, marshaller: &Marshaller, file: &mut File, visitor: &ProjectVisitor);
}