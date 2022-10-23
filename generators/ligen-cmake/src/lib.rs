use ligen::prelude::*;
use ligen::traits::generator::file_generator::{File, FileSet, FileGenerator};
use std::path::PathBuf;
use ligen::ir::conventions::naming::SnakeCase;
// use ligen::traits::marshalling::Marshaller;

/// CMake project generator.
#[derive(Debug, Clone)]
pub struct CMakeGenerator(pub Language);

#[derive(Debug, Clone, Copy)]
pub enum Language {
    C,
    CPP
}

impl FileGenerator for CMakeGenerator {
    fn base_path(&self) -> PathBuf {
        "c".into()
    }

    fn generate_files(&self, project: &Project, file_set: &mut FileSet) -> Result<()> {
        let generator_version = env!("CARGO_PKG_VERSION");
        let project_name = SnakeCase::from(project.name.clone()).to_string();

        let content = match self.0 {
            Language::CPP => format!(
                include_str!("CMakeLists.txt.cpp"),
                generator_version = generator_version,
                project_name = project_name
            ),
            Language::C => format!(
                include_str!("CMakeLists.txt.c"),
                generator_version = generator_version,
                project_name = project_name
            )
        };
        let file = File::new(PathBuf::from("CMakeLists.txt"), content);
        file_set.insert(file);
        Ok(())
    }
}
