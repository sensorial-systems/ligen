use crate::constant::ConstantParser;
use crate::function::FunctionParser;
use crate::function::method::MethodParser;
use crate::identifier::IdentifierParser;
use crate::types::type_definition::TypeDefinitionParser;

#[derive(Default)]
pub struct PythonParser {
    pub identifier_parser: IdentifierParser,
    pub function_parser: FunctionParser,
    pub method_parser: MethodParser,
    pub type_definition_parser: TypeDefinitionParser,
    pub constant_parser: ConstantParser
}

impl PythonParser {
    pub fn full() -> Self {
        Default::default()
    }

    pub fn symbol() -> Self {
        let identifier_parser = IdentifierParser::new();
        let function_parser = FunctionParser::symbol();
        let method_parser = MethodParser::symbol();
        let type_definition_parser = TypeDefinitionParser::symbol();
        let constant_parser = ConstantParser::symbol();
        Self { identifier_parser, function_parser, method_parser, type_definition_parser, constant_parser }
    }
}