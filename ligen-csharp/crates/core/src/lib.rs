use ligen::prelude::*;
use ligen::ir::Attributes;
use ligen::generator::{Context, FileSet, FileGenerator, FFIGenerator, ImplementationVisitor};

/// Generator.
#[derive(Debug, Clone)]
pub struct Generator;

impl ligen::Generator for Generator {
    fn new(_context: &Context, _attributes: &Attributes) -> Self {
        Self
    }
}

impl FileGenerator for Generator {
    fn generate_files(&self, _context: &Context, _file_set: &mut FileSet, _implementation: Option<&ImplementationVisitor>) {
    }
}

impl FFIGenerator for Generator {
    fn generate_ffi(&self, _context: &Context, _implementation: Option<&ImplementationVisitor>) -> TokenStream {
        Default::default()
    }
}