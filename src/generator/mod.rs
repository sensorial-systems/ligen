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

/// Generator trait.
pub trait Generator: FileGenerator + FFIGenerator {
    /// Pre-processes the input. The default implementation returns a transformed input with all the
    /// `Self` and `self` occurrences replaced by the actual object name.
    fn pre_process(&self, root: &Project) -> Project {
        let mut root = root.clone();
        for object in &mut root.root_module.objects {
            for implementation in &mut object.implementations {
                implementation.replace_self_with_explicit_names();
            }
        }
        root
    }

    /// Main function called in the procedural proc_macro.
    fn generate(&self, root: &Project) -> Result<()> {
        let build_type = root.arguments.build_type;
        let root = self.pre_process(root);
        let mut file_set = FileSet::default();
        let visitor = Visitor::new((),root);
        self.generate_files(&mut file_set, &visitor);
        self.save_file_set(file_set, &visitor)?;

        let mut temporary_project = TemporaryFFIProject::new(&visitor.current.arguments.crate_name, &visitor.current.arguments.manifest_path)?;
        self.generate_ffi(&mut temporary_project.lib_file, &visitor);
        temporary_project.save_files()?;
        temporary_project.build(build_type)?;
        temporary_project.transfer_static_library_to_ligen(&visitor.current.arguments.target_dir, build_type)?;
        Ok(())
    }

    /// Saves the file set.
    fn save_file_set(&self, file_set: FileSet, visitor: &ProjectVisitor) -> Result<()> {
        let arguments = &visitor.current.arguments;
        let target_ligen_dir = &arguments.target_dir.join("ligen");
        let project_dir = target_ligen_dir.join(&arguments.crate_name);
        for (_path, file) in file_set.files {
            let file_path = project_dir.join(file.path);
            write_file(&file_path, &file.content)?;
        }
        Ok(())
    }
}
