use ligen::parser::ParserConfig;
use rustpython_parser::ast::ArgWithDefault;
use ligen::ir::Parameter;
use crate::identifier::IdentifierParser;
use crate::prelude::*;
use crate::types::type_::TypeParser;

pub struct ParameterParser;

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
        Ok(Parameter { attributes, identifier, type_ })
    }
}