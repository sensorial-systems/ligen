pub mod prelude;
pub mod module;
pub mod templates;
pub mod identifier;
pub mod type_;

pub use module::*;
use std::path::PathBuf;

use ligen_ir::Library;

use ligen_generator::file_generator::{FileGenerator, FileSet, Template};
use is_tree::IsTree;


#[derive(Default)]
pub struct LibraryGenerator {
    pub module_generator: ModuleGenerator
}

impl LibraryGenerator {
    pub fn generate_project_file(&self, library: &Library, file_set: &mut FileSet) -> Result<()> {
        let file = file_set.entry(PathBuf::from(library.identifier.to_string()).join("Cargo.toml"));
        let mut template = Template::new();
        template.register_template("project", templates::CARGO)?;
        let content = template.render("project", library)?;
        file.write(content);
        Ok(())
    }

    // TODO: Move the module documentation logic to ModuleGenerator. If the documentation isn't present in the module, use library.metadata.description in the root module.
    pub fn generate_lib_file(&self, library: &Library, file_set: &mut FileSet) -> Result<()> {
        let file = file_set.entry(PathBuf::from(library.identifier.to_string()).join("src").join("lib.rs"));
        todo!("Implement here");
        // let section = file.section.branch("documentation");
        // section.writeln(library.metadata.description.split('\n').map(|s| format!("//! {}", s)).collect::<Vec<String>>().join("\n"));
        // Ok(())
    }

    pub fn generate_readme(&self, library: &Library, file_set: &mut FileSet) -> Result<()> {
        let file = file_set.entry(PathBuf::from(library.identifier.to_string()).join("README.md"));
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
        todo!("Implement here");
        // library
        //     .root_module
        //     .iter()
        //     .try_for_each(|module|
        //         self.module_generator.generate_module(library, module, file_set)
        //     )?;
        Ok(())
    }
}
