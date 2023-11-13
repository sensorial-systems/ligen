pub mod prelude;
pub mod module_generator;
pub use module_generator::*;

pub mod templates;

use std::path::PathBuf;

use ligen_ir::Library;
use prelude::*;

use ligen_generator::file_generator::{FileGenerator, FileSet, Template};
use ligen_utils::tree::IsTree;

#[derive(Debug, Default)]
pub struct LibraryGenerator {}

impl LibraryGenerator {
    pub fn generate_project_file(&self, library: &Library, file_set: &mut FileSet) -> Result<()> {
        let file = file_set.entry(PathBuf::from("Cargo.toml"));
        let mut template = Template::new();
        template.register_template("project", templates::CARGO)?;
        let content = template.render("project", library)?;
        file.write(content);
        Ok(())
    }

    pub fn generate_lib_file(&self, library: &Library, file_set: &mut FileSet) -> Result<()> {
        let file = file_set.entry(PathBuf::from("src").join("lib.rs"));
        let section = file.section.branch("documentation");
        section.writeln(library.metadata.description.split('\n').map(|s| format!("//! {}", s)).collect::<Vec<String>>().join("\n"));
        Ok(())
    }

    pub fn generate_readme(&self, library: &Library, file_set: &mut FileSet) -> Result<()> {
        let file = file_set.entry(PathBuf::from("README.md"));
        file.write(&library.metadata.description);
        Ok(())
    }
}

impl FileGenerator for LibraryGenerator {
    type Input = Library;
    fn base_path(&self) -> PathBuf {
        PathBuf::from("rust".to_string())
    }

    fn generate_files(&self, library: &Library, file_set: &mut FileSet) -> Result<()> {
        self.generate_project_file(library, file_set)?;
        self.generate_lib_file(library, file_set)?;
        Ok(())
    }
}
