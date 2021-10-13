//! Generators.

mod file;
mod visitor;
mod file_generator;
mod ffi_generator;

pub use visitor::*;
pub use file::*;
pub use file_generator::*;
pub use ffi_generator::*;

use crate::prelude::*;
use crate::ir::Project;
use crate::utils::fs::write_file;
use crate::generator::ffi_generator::cargo::Cargo;
use crate::marshalling::Marshaller;

/// Generator trait.
pub trait Generator: FileGenerator + FFIGenerator {
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
        self.generate_files(&mut file_set, &visitor);
        self.save_file_set(file_set, &visitor)?;

        let marshaller = Marshaller::new();

        // TODO: Separate Project and Builder.
        let mut temporary_project = TemporaryFFIProject::new(&visitor.name().to_string(), &visitor.path())?;
        self.generate_ffi(&marshaller, &mut temporary_project.lib_file, &visitor);
        temporary_project.save_files()?;
        temporary_project.build(BUILD_PROFILE)?;
        temporary_project.transfer_libraries_to_ligen(&Cargo::target_dir()?, BUILD_PROFILE)?;
        Ok(())
    }

    /// Saves the file set.
    fn save_file_set(&self, file_set: FileSet, project: &ProjectVisitor) -> Result<()> {
        let target_ligen_dir = Cargo::target_dir()?.join("ligen");
        let project_dir = target_ligen_dir.join(&project.name().to_string());
        for (_path, file) in file_set.files {
            let file_path = project_dir.join(file.path);
            write_file(&file_path, &file.content)?;
        }
        Ok(())
    }
}
