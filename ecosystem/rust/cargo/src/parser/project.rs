use ligen_ir::conventions::naming::NamingConvention;
use ligen_ir::prelude::*;
use ligen_ir::{Module, Project};
use ligen_parsing::Parser;

pub struct ProjectParser;

impl Parser<&std::path::Path> for ProjectParser {
    type Output = Project;
    fn parse(&self, input: &std::path::Path) -> Result<Self::Output> {
        let cargo_path = if input.is_dir() {
            input.join("Cargo.toml")
        } else {
            input.to_path_buf()
        };
        let directory = cargo_path.parent().ok_or("Failed to get directory.")?.to_path_buf();
        let cargo_toml = cargo_toml::Manifest::from_path(cargo_path.as_path())
            .map_err(|e| Error::Message(format!("Failed to read Cargo.toml: {}", e)))?;
        let package = cargo_toml.package.ok_or_else(|| Error::Message("Package not found in Cargo.toml.".into()))?;
        Ok(Self::Output {
            directory,
            name: NamingConvention::try_from(package.name.as_str())?,
            root_module: Module::default(),
        })
    }
}