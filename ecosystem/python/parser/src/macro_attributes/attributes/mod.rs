pub mod attribute;

use crate::identifier::IdentifierParser;
use crate::literal::LiteralParser;
use crate::path::PathParser;
use crate::prelude::*;
use ligen::idl::{Attributes, Attribute, Group, Named};
use rustpython_parser::ast::{Expr, Keyword, Ranged};

#[derive(Default)]
pub struct AttributesParser {
    path_parser: PathParser,
    identifier_parser: IdentifierParser,
    literal_parser: LiteralParser,
}

impl Transformer<WithSource<&Vec<Expr>>, Attributes> for AttributesParser {
    fn transform(&self, input: WithSource<&Vec<Expr>>, config: &Config) -> Result<Attributes> {
        let mut attributes = Attributes::default();
        for expr in input.ast {
            let attribute = self.transform(input.sub(expr), config)?;
            attributes.push(attribute);
        }
        Ok(attributes)
    }
}

impl Transformer<WithSource<&Vec<Keyword>>, Attributes> for AttributesParser {
    fn transform(&self, input: WithSource<&Vec<Keyword>>, config: &Config) -> Result<Attributes> {
        let mut attributes = Attributes::default();
        for keyword in input.ast {
            let attribute = self.transform(input.sub(keyword), config)?;
            attributes.push(attribute);
        }
        Ok(attributes)
    }
} 

impl Transformer<WithSource<&Keyword>, Attribute> for AttributesParser {
    fn transform(&self, input: WithSource<&Keyword>, config: &Config) -> Result<Attribute> {
        let name = input
            .ast
            .arg
            .as_ref()
            .map(|arg| arg.to_string())
            .ok_or_else(|| Error::Message(format!("Failed to parse attribute name: {:?} @ \"{}\"", input.ast.arg, &input.source[input.ast.range.start().to_usize()..input.ast.range.end().to_usize()])))?;
        let identifier = self.identifier_parser.parse(name, config)?;
        let literal = self.literal_parser.transform(&input.ast.value, config)?;
        Ok(Named::new(identifier, literal).into())
    }
}

impl Transformer<WithSource<&Expr>, Attribute> for AttributesParser {
    fn transform(&self, input: WithSource<&Expr>, config: &Config) -> Result<Attribute> {
        match input.ast {
            Expr::Call(expr) => {
                let path = self.path_parser.transform(&*expr.func, config)?;
                let mut attributes = self.transform(input.sub(&expr.args), config).unwrap_or_default(); // TODO: We might want to check for errors here.
                let keywords = self.transform(input.sub(&expr.keywords), config)?;
                attributes.attributes.extend(keywords.attributes);
                Ok(Group::new(path, attributes).into())
            },
            Expr::Name(expr) => {
                let path = self.path_parser.transform(expr, config)?;
                let attributes = Attributes::default();
                Ok(Group::new(path, attributes).into())
            },
            Expr::Attribute(expr) => {
                let name = expr.attr.to_string();
                let identifier = self.identifier_parser.parse(name, config)?;
                let attributes = Attributes::default();
                Ok(Group::new(identifier, attributes).into())
            }
            _ => Err(Error::Message(format!("Invalid attribute {:?}", &input.source[input.ast.start().to_usize()..input.ast.end().to_usize()])))
        }
    }
}
