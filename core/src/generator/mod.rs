//! Generators.

mod context;

pub use context::*;
use crate::prelude::*;

pub mod file;

use proc_macro2::TokenStream;
use crate::ir::{Implementation, Attributes};
use crate::generator::file::FileSet;
use crate::utils::fs::write_file;

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
    fn generate(&self, context: &Context, implementation: Option<&Implementation>) -> Result<TokenStream> {
        let implementation = self.pre_process(context, implementation);
        let implementation = implementation.as_ref();

        // let target_dir_ligen = &context.arguments.target_dir.join("ligen");
        // create_dir_all(target_dir_ligen).expect("Failed to create target directory for the header");

        // let project_dir = target_dir_ligen.join(&context.arguments.crate_name);

        // FIXME: include is C only. Elaborate a more generic way to guarantee the path existence.
        // create_dir_all(project_dir.join("include")).expect("Failed to create include directory");
        // create_dir_all(project_dir.join("lib")).expect("Failed to create lib directory");

        let file_set = self.generate_files(&context, implementation);
        self.save_file_set(context, file_set)?;
        Ok(self.generate_externs(&context, implementation))
    }

    /// Saves the file set.
    fn save_file_set(&self, context: &Context, file_set: FileSet) -> Result<()> {
        let target_ligen_dir = &context.arguments.target_dir.join("ligen");
        let project_dir = target_ligen_dir.join(&context.arguments.crate_name);
        for file in file_set.files {
            let file_path = project_dir.join(file.path);
            write_file(&file_path, file.content)?;
        }
        Ok(())
    }

    /// Generate FFI externs.
    fn generate_externs(&self, _context: &Context, _implementation: Option<&Implementation>) -> TokenStream {
        TokenStream::new()
    }
    
    /// Generate files.
    fn generate_files(&self, context: &Context, implementation: Option<&Implementation>) -> FileSet;
}
