use rustpython_parser::ast::{ExprAttribute, Expr, Identifier, ExprName};

use crate::{prelude::*, identifier::IdentifierParser};

use ligen::ir::Path;
use ligen::parser::prelude::*;

#[derive(Default)]
pub struct PathParser {
    identifier_parser: IdentifierParser,
}

impl Transformer<&ExprAttribute, Path> for PathParser {
    fn transform(&self, input: &ExprAttribute, config: &Config) -> Result<Path> {
        Ok(self.transform(&input.attr, config)?.join(self.transform(&*input.value, config)?))
    }
}

impl Transformer<&ExprName, Path> for PathParser {
    fn transform(&self, input: &ExprName, config: &Config) -> Result<Path> {
        self.transform(&input.id, config)
    }

}

impl Transformer<&Identifier, Path> for PathParser {
    fn transform(&self, input: &Identifier, config: &Config) -> Result<Path> {
        let identifier = self.identifier_parser.parse(input.as_str(), config)?;
        Ok(Path::from(identifier))
    }
}

impl Transformer<&Expr, Path> for PathParser {
    fn transform(&self, input: &Expr, config: &Config) -> Result<Path> {
        match input {
            Expr::Attribute(attribute) => self.transform(attribute, config),
            Expr::Name(name) => self.transform(name, config),
            _ => Err(Error::Message(format!("Failed to parse path from {:?}", input))),
        }
    }
}