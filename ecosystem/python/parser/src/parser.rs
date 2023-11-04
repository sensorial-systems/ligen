use crate::{prelude::*, module::SubPath};
use crate::object::ObjectParser;
use crate::function::FunctionParser;
use crate::identifier::IdentifierParser;
use crate::types::type_definition::TypeDefinitionParser;

use ligen::ir::Library;

#[derive(Default)]
pub struct PythonParser {
    pub identifier_parser: IdentifierParser,
    pub function_parser: FunctionParser,
    pub type_definition_parser: TypeDefinitionParser,
    pub object_parser: ObjectParser
}

impl PythonParser {
    pub fn full() -> Self {
        Default::default()
    }

    pub fn symbol() -> Self {
        let identifier_parser = IdentifierParser::new();
        let function_parser = FunctionParser::symbol();
        let type_definition_parser = TypeDefinitionParser::symbol();
        let object_parser = ObjectParser::symbol();
        Self { identifier_parser, function_parser, type_definition_parser, object_parser }
    }
}

impl Parser<&std::path::Path> for PythonParser {
    type Output = Library;
    fn parse(&self, input: &std::path::Path) -> Result<Self::Output> {
        let name = self.identifier_parser.parse(input)?;
        let root_module = self.parse(SubPath(input))?;
        Ok(Library { identifier: name, root_module })
    }
}