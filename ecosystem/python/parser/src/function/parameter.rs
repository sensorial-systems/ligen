use ligen::transformer::prelude::*;
use rustpython_parser::ast::ArgWithDefault;
use ligen::idl::Parameter;
use crate::identifier::IdentifierParser;
use crate::literal::LiteralParser;
use crate::types::type_::TypeParser;

#[derive(Default)]
pub struct ParameterParser {
    literal_parser: LiteralParser,
    type_parser: TypeParser,
    identifier_parser: IdentifierParser,
}

impl Transformer<ArgWithDefault, Parameter> for ParameterParser {
    fn transform(&self, input: ArgWithDefault, config: &Config) -> Result<Parameter> {
        let attributes = Default::default();
        let identifier = self.identifier_parser.transform(input.def.arg.as_str(), config)?;
        let type_ = if let Some(value) = input.def.annotation.and_then(|annotation| annotation.name_expr()) {
            self.type_parser.transform(&value, config)?
        } else {
            Default::default()
        };
        let default_value = if let Some(value) = input.default {
            Some(self.literal_parser.transform(&*value, config)?)
        } else {
            None
        };
        Ok(Parameter { attributes, identifier, type_, default_value })
    }
}
