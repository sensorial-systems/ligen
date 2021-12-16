use ligen::prelude::*;
use ligen::ir::Attributes;
use ligen::generator::{Context, FFIGenerator, ImplementationVisitor};

mod files;

/// Generator.
#[derive(Debug, Clone)]
pub struct Generator;

impl ligen::Generator for Generator {
    fn new(_context: &Context, _attributes: &Attributes) -> Self {
        Self
    }
}


impl FFIGenerator for Generator {
    fn generate_ffi(&self, _context: &Context, _implementation: Option<&ImplementationVisitor>) -> TokenStream {
        Default::default()
    }
}