pub mod attribute;

use crate::identifier::IdentifierParser;
use crate::literal::LiteralParser;
use crate::prelude::*;
use ligen::parser::ParserConfig;
use ligen::ir::{Attributes, Attribute, macro_attributes::{Group, Named}};
use rustpython_parser::ast::{Expr, Keyword};

#[derive(Default)]
pub struct AttributesParser {}

impl Parser<WithSource<&Vec<Expr>>> for AttributesParser {
    type Output = Attributes;
    fn parse(&self, input: WithSource<&Vec<Expr>>, config: &ParserConfig) -> Result<Self::Output> {
        let mut attributes = Attributes::default();
        for expr in input.ast {
            let attribute = self.parse(input.sub(expr), config)?;
            attributes.push(attribute);
        }
        Ok(attributes)
    }
}

impl Parser<WithSource<&Vec<Keyword>>> for AttributesParser {
    type Output = Attributes;
    fn parse(&self, input: WithSource<&Vec<Keyword>>, config: &ParserConfig) -> Result<Self::Output> {
        let mut attributes = Attributes::default();
        for keyword in input.ast {
            let attribute = self.parse(input.sub(keyword), config)?;
            attributes.push(attribute);
        }
        Ok(attributes)
    }
} 

impl Parser<WithSource<&Keyword>> for AttributesParser {
    type Output = Attribute;
    fn parse(&self, input: WithSource<&Keyword>, config: &ParserConfig) -> Result<Self::Output> {
        let name = input
            .ast
            .arg
            .as_ref()
            .map(|arg| arg.to_string())
            .ok_or_else(|| Error::Message("Failed to parse attribute name".to_string()))?;
        let identifier = IdentifierParser::default().parse(name, config)?;
        let literal = LiteralParser::default().parse(&input.ast.value, config)?;
        Ok(Named::new(identifier, literal).into())
    }
}

impl Parser<WithSource<&Expr>> for AttributesParser {
    type Output = Attribute;
    fn parse(&self, input: WithSource<&Expr>, config: &ParserConfig) -> Result<Self::Output> {
        match input.ast {
            Expr::Call(expr) => {
                let name = expr.func.as_name_expr().map(|value| value.id.to_string()).expect("Failed to parse attribute name");
                let identifier = IdentifierParser::default().parse(name, config)?;
                let mut attributes = self.parse(input.sub(&expr.args), config)?;
                let keywords = self.parse(input.sub(&expr.keywords), config)?;
                attributes.attributes.extend(keywords.attributes);
                Ok(Group::new(identifier, attributes).into())
            },
            Expr::Name(expr) => {
                let name = expr.id.to_string();
                let identifier = IdentifierParser::default().parse(name, config)?;
                let attributes = Attributes::default();
                Ok(Group::new(identifier, attributes).into())
            },
            Expr::Attribute(expr) => {
                let name = expr.attr.to_string();
                let identifier = IdentifierParser::default().parse(name, config)?;
                let attributes = Attributes::default();
                Ok(Group::new(identifier, attributes).into())
            }
            _ => Err(Error::Message(format!("Invalid attribute {:?}", input.ast)))
        }
    }
}
