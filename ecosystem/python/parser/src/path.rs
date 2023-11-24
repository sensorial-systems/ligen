use rustpython_parser::ast::{ExprAttribute, Expr, Identifier, ExprName};

use crate::{prelude::*, identifier::IdentifierParser};

use ligen::{ir::Path, parser::ParserConfig};

#[derive(Default)]
pub struct PathParser {
    identifier_parser: IdentifierParser,
}

impl Parser<&ExprAttribute> for PathParser {
    type Output = Path;
    fn parse(&self, input: &ExprAttribute, config: &ParserConfig) -> Result<Self::Output> {
        Ok(self.parse(&input.attr, config)?.join(self.parse(&*input.value, config)?))
    }
}

impl Parser<&ExprName> for PathParser {
    type Output = Path;
    fn parse(&self, input: &ExprName, config: &ParserConfig) -> Result<Self::Output> {
        self.parse(&input.id, config)
    }

}

impl Parser<&Identifier> for PathParser {
    type Output = Path;
    fn parse(&self, input: &Identifier, config: &ParserConfig) -> Result<Self::Output> {
        let identifier = self.identifier_parser.parse(input.as_str(), config)?;
        Ok(Path::from(identifier))
    }
}

impl Parser<&Expr> for PathParser {
    type Output = Path;
    fn parse(&self, input: &Expr, config: &ParserConfig) -> Result<Self::Output> {
        match input {
            Expr::Attribute(attribute) => self.parse(attribute, config),
            Expr::Name(name) => self.parse(name, config),
            _ => Err(Error::Message(format!("Failed to parse path from {:?}", input))),
        }
    }
}