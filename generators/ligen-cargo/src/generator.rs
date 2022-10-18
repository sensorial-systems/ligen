use ligen_traits::generator::{Generator, FileGenerator, FileSet};
use std::path::PathBuf;
use std::str::FromStr;
use ligen_traits::prelude::*;
use ligen_ir::visitor::ProjectVisitor;

#[derive(Debug, Default)]
pub struct CargoGenerator;

impl Generator for CargoGenerator {
    fn base_path(&self) -> PathBuf {
        PathBuf::from("rust".to_string())
    }
}

impl FileGenerator for CargoGenerator {
    fn generate_files(&self, file_set: &mut FileSet, visitor: &ProjectVisitor) -> Result<()> {
        let file = file_set.entry(&PathBuf::from_str("Cargo.toml").unwrap());
        let version = "0.1.0";
        let name = &visitor.current.name;
        let path = &visitor.current.directory;
        let path = path.display().to_string().replace("\\", "/");
        let content = format!(include_str!("Cargo.template.toml"), name = name, version = version, path = path);
        file.writeln(content);
        Ok(())
    }
}
