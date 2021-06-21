//! Generators.

mod context;

pub use context::*;

pub mod source;
// TODO: Project generator.
// pub mod project;


use proc_macro2::TokenStream;
use std::fs::create_dir_all;
use crate::ir::{Implementation, Attributes};

/// Generator trait.
pub trait Generator {
    /// Creates a new generator using contextual information and attributes.
    fn new(context: &Context, attributes: &Attributes) -> Self where Self: Sized;

    /// Pre-processes the input. The default implementation returns a transformed input with all the
    /// `Self` references replaced by the actual object name.
    fn pre_process(&self, _context: &Context, implementation: Option<&Implementation>) -> Option<Implementation> {
        implementation.map(|implementation| {
            let mut implementation = implementation.clone();
            implementation.replace_self_with_real_names();
            implementation
        })
    }

    /// Main function called in the procedural macro.
    fn generate(&self, context: &Context, implementation: Option<&Implementation>) -> TokenStream {
        let implementation = self.pre_process(context, implementation);
        let implementation = implementation.as_ref();

        let target_dir_ligen = &context.arguments.target_dir.join("ligen");
        create_dir_all(target_dir_ligen).expect("Failed to create target directory for the header");

        let project_dir = target_dir_ligen.join(&context.arguments.crate_name);

        // FIXME: include is C only. Elaborate a more generic way to guarantee the path existence.
        create_dir_all(project_dir.join("include")).expect("Failed to create include directory");
        create_dir_all(project_dir.join("lib")).expect("Failed to create lib directory");

        self.generate_files(&context, implementation);
        self.generate_externs(&context, implementation)
    }

    /// Generate FFI externs.
    fn generate_externs(&self, _context: &Context, _implementation: Option<&Implementation>) -> TokenStream {
        TokenStream::new()
    }
    
    /// Generate files.
    fn generate_files(&self, context: &Context, implementation: Option<&Implementation>);
}
