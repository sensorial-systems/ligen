use ligen_ir::Identifier;
use ligen_ir::prelude::*;
use ligen_ir::Library;
use ligen_parsing::parser::Parser;
use ligen_rust_parser::module::ModuleParser;

pub struct LibraryParser;

impl Parser<&std::path::Path> for LibraryParser {
    type Output = Library;
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

        let identifier = Identifier::from(package.name.as_str());
        let mut root_module = ModuleParser.parse(library_path.as_path())?;
        root_module.identifier = identifier.clone();
        Ok(Self::Output { identifier, root_module })
    }
}