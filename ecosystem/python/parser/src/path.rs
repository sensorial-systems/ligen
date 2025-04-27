use rustpython_parser::ast::{ExprAttribute, Expr, Identifier, ExprName};

use crate::{prelude::*, identifier::IdentifierParser};

use ligen::ir::Path;
use ligen::parser::prelude::*;

#[derive(Default)]
pub struct PathParser {
    identifier_parser: IdentifierParser,
}

impl Parser<&ExprAttribute> for PathParser {
    type Output = Path;
    fn parse(&self, input: &ExprAttribute, config: &Config) -> Result<Self::Output> {
        Ok(self.parse(&input.attr, config)?.join(self.parse(&*input.value, config)?))
    }
}

impl Parser<&ExprName> for PathParser {
    type Output = Path;
    fn parse(&self, input: &ExprName, config: &Config) -> Result<Self::Output> {
        self.parse(&input.id, config)
    }

}

impl Parser<&Identifier> for PathParser {
    type Output = Path;
    fn parse(&self, input: &Identifier, config: &Config) -> Result<Self::Output> {
        let identifier = self.identifier_parser.parse(input.as_str(), config)?;
        Ok(Path::from(identifier))
    }
}

impl Parser<&Expr> for PathParser {
    type Output = Path;
    fn parse(&self, input: &Expr, config: &Config) -> Result<Self::Output> {
        match input {
            Expr::Attribute(attribute) => self.parse(attribute, config),
            Expr::Name(name) => self.parse(name, config),
            _ => Err(Error::Message(format!("Failed to parse path from {:?}", input))),
        }
    }
}