use ligen_ir::Library;
use ligen_generator::file_generator::{FileGenerator, FileSet};
use std::path::PathBuf;
use std::str::FromStr;
use ligen_traits::prelude::*;

#[derive(Debug, Default)]
pub struct CargoGenerator;

impl FileGenerator for CargoGenerator {
    fn base_path(&self) -> PathBuf {
        PathBuf::from("rust".to_string())
    }

    fn generate_files(&self, library: &Library, file_set: &mut FileSet) -> Result<()> {
        let file = file_set.entry(&PathBuf::from_str("Cargo.toml").unwrap());
        let version = "0.1.0";
        let name = &library.identifier;
        // FIXME: This is a placeholder and it will fail.
        let path = PathBuf::default();
        let path = path.display().to_string().replace('\\', "/");
        let content = format!(include_str!("Cargo.template.toml"), name = name, version = version, path = path);
        file.section("root").writeln(content);
        Ok(())
    }
}
