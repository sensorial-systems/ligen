//! Generators.

mod context;
mod file;
mod visitor;
mod file_generator;
mod ffi_generator;

pub use context::*;
pub use visitor::*;
pub use file::*;
pub use file_generator::*;
pub use ffi_generator::*;

use crate::prelude::*;
use crate::ir::{Attributes, Object};
use crate::utils::fs::write_file;

/// Generator trait.
pub trait Generator: FileGenerator + FFIGenerator {
    /// Creates a new generator using contextual information and attributes.
    fn new(context: &Context, attributes: &Attributes) -> Self where Self: Sized;

    /// Pre-processes the input. The default implementation returns a transformed input with all the
    /// `Self` and `self` occurrences replaced by the actual object name.
    fn pre_process(&self, _context: &Context, object: Option<&Object>) -> Option<Object> {
        object.map(|object| {
            let mut object = object.clone();
            for implementation in &mut object.implementations {
                implementation.replace_self_with_explicit_names();
            }
            object
        })
    }

    /// Main function called in the procedural proc_macro.
    fn generate(&self, context: &Context, object: Option<&Object>) -> Result<TokenStream> {
        let object = self.pre_process(context, object);
        let object = object.map(|object| Visitor::new((), object));
        let object = object.as_ref();
        let mut file_set = FileSet::default();
        self.generate_files(&context, &mut file_set, object);
        self.save_file_set(context, file_set)?;
        Ok(self.generate_ffi(&context, object))
    }

    /// Saves the file set.
    fn save_file_set(&self, context: &Context, file_set: FileSet) -> Result<()> {
        let target_ligen_dir = &context.arguments.target_dir.join("ligen");
        let project_dir = target_ligen_dir.join(&context.arguments.crate_name);
        for (_path, file) in file_set.files {
            let file_path = project_dir.join(file.path);
            write_file(&file_path, file.content)?;
        }
        Ok(())
    }
}
