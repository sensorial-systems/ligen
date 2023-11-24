use ligen::parser::ParserConfig;
use rustpython_parser::ast::ArgWithDefault;
use ligen::ir::Parameter;
use crate::identifier::IdentifierParser;
use crate::literal::LiteralParser;
use crate::prelude::*;
use crate::types::type_::TypeParser;

#[derive(Default)]
pub struct ParameterParser {
    literal_parser: LiteralParser,
}

impl Parser<ArgWithDefault> for ParameterParser {
    type Output = Parameter;
    fn parse(&self, input: ArgWithDefault, config: &ParserConfig) -> Result<Self::Output> {
        let attributes = Default::default();
        let identifier = IdentifierParser::new().parse(input.def.arg.as_str(), config)?;
        let type_ = if let Some(value) = input.def.annotation.and_then(|annotation| annotation.name_expr()) {
            TypeParser::default().parse(&value, config)?
        } else {
            Default::default()
        };
        let default_value = if let Some(value) = input.default {
            Some(self.literal_parser.parse(&*value, config)?)
        } else {
            None
        };
        Ok(Parameter { attributes, identifier, type_, default_value })
    }
}
