use ligen::prelude::*;
use ligen::traits::generator::file_generator::{File, FileSet, FileGenerator};
use std::path::PathBuf;
use ligen::idl::conventions::naming::SnakeCase;

/// CMake library generator.
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

    fn generate_files(&self, library: &Library, file_set: &mut FileSet) -> Result<()> {
        let generator_version = env!("CARGO_PKG_VERSION");
        let library_name = SnakeCase::try_from(library.name.clone())?.to_string();

        let content = match self.0 {
            Language::CPP => format!(
                include_str!("CMakeLists.txt.cpp"),
                generator_version = generator_version,
                library_name = library_name
            ),
            Language::C => format!(
                include_str!("CMakeLists.txt.c"),
                generator_version = generator_version,
                library_name = library_name
            )
        };
        let file = File::new(PathBuf::from("CMakeLists.txt"), content);
        file_set.insert(file);
        Ok(())
    }
}
