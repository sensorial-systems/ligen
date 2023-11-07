use crate::{prelude::*, module::SubPath};
use crate::object::ObjectParser;
use crate::function::FunctionParser;
use crate::identifier::IdentifierParser;
use crate::types::type_definition::TypeDefinitionParser;

pub mod metadata;
pub mod config;
pub use config::*;
pub use metadata::*;

use ligen::ir::Library;
use ligen::parser::ParserConfig;

#[derive(Default)]
pub struct PythonParser {
    pub identifier_parser: IdentifierParser,
    pub function_parser: FunctionParser,
    pub type_definition_parser: TypeDefinitionParser,
    pub metadata_parser: MetadataParser,
    pub object_parser: ObjectParser
}

impl PythonParser {
    pub fn new() -> Self {
        Default::default()
    }
}

impl Parser<&std::path::Path> for PythonParser {
    type Output = Library;
    fn parse(&self, input: &std::path::Path, config: &ParserConfig) -> Result<Self::Output> {
        let identifier = self.identifier_parser.parse(input, config)?;
        let metadata = self.metadata_parser.parse(input, config)?;
        let root_module = self.parse(SubPath(input), config)?;
        Ok(Library { identifier, metadata, root_module })
    }
    fn name(&self) -> &str {
        "Python"
    }
    fn config(&self) -> ParserConfig {
        PythonParserConfig::default().into()
    }
}