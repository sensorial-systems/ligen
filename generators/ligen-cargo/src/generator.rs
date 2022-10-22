use ligen_traits::generator::{FileGenerator, FileSet};
use std::path::PathBuf;
use std::str::FromStr;
use ligen_traits::prelude::*;

#[derive(Debug, Default)]
pub struct CargoGenerator;

impl FileGenerator for CargoGenerator {
    fn base_path(&self) -> PathBuf {
        PathBuf::from("rust".to_string())
    }

    fn generate_files(&self, file_set: &mut FileSet, project: &Project) -> Result<()> {
        let file = file_set.entry(&PathBuf::from_str("Cargo.toml").unwrap());
        let version = "0.1.0";
        let name = &project.name;
        let path = &project.directory;
        let path = path.display().to_string().replace("\\", "/");
        let content = format!(include_str!("Cargo.template.toml"), name = name, version = version, path = path);
        file.writeln(content);
        Ok(())
    }
}
