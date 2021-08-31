use ligen::prelude::*;
use ligen::generator::{FileSet, FileGenerator, FFIGenerator, ProjectVisitor};
use ligen::generator::File;
use std::path::PathBuf;

/// CMake project generator.
#[derive(Debug, Clone)]
pub struct CMakeGenerator(pub Language);

#[derive(Debug, Clone, Copy)]
pub enum Language {
    C,
    CPP
}

impl Generator for CMakeGenerator {}

impl FileGenerator for CMakeGenerator {
    fn generate_files(&self, file_set: &mut FileSet, project: &ProjectVisitor) {
        let generator_version = env!("CARGO_PKG_VERSION");
        let project_name = &project.name().to_string();

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
    }
}

impl FFIGenerator for CMakeGenerator {
    fn generate_ffi(&self, _file: &mut File, _implementation: &ProjectVisitor) {}
}
