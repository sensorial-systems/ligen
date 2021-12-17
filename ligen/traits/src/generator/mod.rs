//! Generators.

pub use file::*;
pub use file_generator::*;
pub use visitor::*;

use crate::prelude::*;
use ligen_utils::fs::write_file;

mod file;
use ligen_ir::visitor;
use std::path::PathBuf;

mod file_generator;
pub mod file_processor_visitor;

/// Generator trait.
pub trait Generator: FileGenerator {
    /// Generation base path.
    fn base_path(&self) -> PathBuf;

    /// Pre-processes the input. The default implementation returns a transformed input with all the
    /// `Self` and `self` occurrences replaced by the actual object name.
    fn pre_process(&self, root: &Project) -> Project {
        let mut root = root.clone();
        root.root_module.replace_self_with_explicit_names();
        root
    }

    /// Main function called in the procedural proc_macro.
    fn generate(&self, root: &Project) -> Result<()> {
        let root = self.pre_process(root);
        let mut file_set = FileSet::default();
        let visitor = Visitor::new((),root);
        self.generate_files(&mut file_set, &visitor)?;
        self.save_file_set(file_set, &visitor)?;
        Ok(())
    }

    /// Saves the file set.
    fn save_file_set(&self, file_set: FileSet, project: &ProjectVisitor) -> Result<()> {
        let target_ligen_dir = std::env::current_dir()?
            .join("target")
            .join("ligen")
            .join(self.base_path());
        let project_dir = target_ligen_dir.join(&project.name().to_string());
        for (_path, file) in file_set.files {
            let file_path = project_dir.join(file.path);
            write_file(&file_path, &file.content)?;
        }
        Ok(())
    }
}
