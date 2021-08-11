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
use crate::ir::{Attributes, Project};
use crate::utils::fs::write_file;

/// Generator trait.
pub trait Generator: FileGenerator + FFIGenerator {
    /// Creates a new generator using contextual information and attributes.
    fn new(context: &Context, attributes: &Attributes) -> Self where Self: Sized;

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
        THIS PROCESS NEEDS TO BE FIXED.
            RUNNING CARGO BUILD HERE CREATES A CALL LOOP BECAUSE THE GENERATED PROJECT POINTS TO THE PROJECT THAT HAS THE BUILD.RS THAT RUNS THIS PROCESS.
        // FIXME: This process is a mess. We need to clean this up.
        let root = self.pre_process(root);
        let temporary_directory = std::env::temp_dir();
        let cargo_file = File::new(temporary_directory.join("Cargo.toml"), format!(include_str!("Cargo.template.toml"), name = &root.arguments.crate_name, version = "0.1.0", path = root.arguments.manifest_path.parent().expect("Couldn't get manifest_path's parent.").join("examples").join("counter").display()));
        let mut lib_file = File::new(temporary_directory.join("src").join("lib.rs"), String::new());
        let mut file_set = FileSet::default();
        let visitor = Visitor::new((),root);
        self.generate_files(&mut file_set, &visitor);
        self.save_file_set(file_set, &visitor)?;
        self.generate_ffi(&mut lib_file, &visitor);
        lib_file.save()?;
        cargo_file.save()?;
        std::process::Command::new("cargo")
            .arg("build")
            .arg("--release")
            .arg("--manifest-path")
            .arg(cargo_file.path.display().to_string())
            .status()
            .expect("Couldn't compile project.");
        crate::utils::fs::copy(&temporary_directory.join("target").join("release").join(&format!("lib{}.a", visitor.current.arguments.crate_name)), &visitor.current.arguments.target_dir.join("ligen"))?;
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
