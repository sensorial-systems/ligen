use crate::module::SubPath;
use crate::prelude::*;
use crate::object::ObjectParser;
use crate::function::FunctionParser;
use crate::identifier::IdentifierParser;
use crate::types::type_definition::TypeDefinitionParser;

pub mod metadata;
pub mod config;
pub mod validator;

pub use config::*;
pub use metadata::*;
pub use validator::*;

use ligen::ir::{Registry, Library};

#[derive(Default)]
pub struct PythonParser {
    pub identifier_parser: IdentifierParser,
    pub function_parser: FunctionParser,
    pub type_definition_parser: TypeDefinitionParser,
    pub metadata_parser: MetadataParser,
    pub object_parser: ObjectParser,
    pub validator: LibraryValidator
}

impl PythonParser {
    pub fn new() -> Self {
        Default::default()
    }
}

impl PythonParser {
    fn parse_library(&self, input: &std::path::Path, config: &Config) -> Result<Library> {
        // This line replaces "-" with "_" in the file name
        let input = input.with_file_name(input.file_name().unwrap().to_string_lossy().replace('-', "_").as_str().trim());
        let input = input.as_path();
        let identifier = self.identifier_parser.transform(input, config)?;
        let metadata = self.metadata_parser.transform(input, config)?;
        let root_module = self.transform(SubPath(input), config)?;
        let mut library = Library { identifier, metadata, root_module };
        self.validator.validate(&mut library, config)?;
        Ok(library)
    }
}

impl Transformer<&std::path::Path, Registry> for PythonParser {
    fn transform(&self, input: &std::path::Path, config: &Config) -> Result<Registry> {
        let mut registry = Registry::new();
        let library = self.parse_library(input, config)?;
        for dependency in library.metadata.dependencies.iter().filter(|dependency| dependency.features.is_empty()) { // TODO: We need to support features.
            let dependency_path = input.parent().unwrap().join(dependency.identifier.to_string());
            let mut dependency_library = self.transform(dependency_path.as_path(), config)?;
            registry.libraries.append(&mut dependency_library.libraries);
        }
        registry.libraries.push(library);
        Ok(registry)
    }
    fn name(&self) -> &str {
        "Python"
    }
    fn config(&self) -> Config {
        PythonParserConfig::default().into()
    }
}