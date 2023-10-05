use rustpython_parser::ast::ExprAttribute;
use ligen::ir::Attribute;
use crate::identifier::IdentifierParser;
use crate::macro_attributes::attributes::AttributesParser;
use crate::prelude::*;

pub struct AttributeParser;

impl Parser<WithSource<ExprAttribute>> for AttributeParser {
    type Output = Attribute;
    fn parse(&self, input: WithSource<ExprAttribute>) -> Result<Self::Output> {
        let source = input.source;
        let input = input.ast;
        let identifier = IdentifierParser.parse(input.attr)?;
        let attributes = AttributesParser.parse(WithSource::new(&source, input.value))?;
        Ok(Attribute::Group(identifier, attributes))
    }
}