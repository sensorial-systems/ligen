use ligen_ir::conventions::naming::{SnakeCase, NamingConvention};
use ligen_ir::prelude::*;
use ligen_ir::Project;
use ligen_parsing::parser::Parser;
use ligen_rust_parser::module::ModuleParser;

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
        let library = cargo_toml.lib.ok_or_else(|| Error::Message("Library not found in Cargo.toml.".into()))?;
        let library_path = directory.join(library.path.unwrap_or("src/lib.rs".into()));

        let name = NamingConvention::try_from(package.name.as_str())?;
        let mut root_module = ModuleParser.parse(library_path.as_path())?;
        root_module.identifier = SnakeCase::try_from(name.clone())?.to_string().into();
        Ok(Self::Output { name, root_module })
    }
}