use rustpython_parser::ast::ArgWithDefault;
use ligen::ir::Parameter;
use crate::identifier::IdentifierParser;
use crate::prelude::*;
use crate::types::type_::TypeParser;

pub struct ParameterParser;

impl Parser<ArgWithDefault> for ParameterParser {
    type Output = Parameter;
    fn parse(&self, input: ArgWithDefault) -> Result<Self::Output> {
        let attributes = Default::default();
        let identifier = IdentifierParser::new().parse(input.def.arg.as_str())?;
        let type_ = if let Some(value) = input.def.annotation.and_then(|annotation| annotation.name_expr()) {
            TypeParser.parse(value)?
        } else {
            Default::default()
        };
        Ok(Parameter { attributes, identifier, type_ })
    }
}