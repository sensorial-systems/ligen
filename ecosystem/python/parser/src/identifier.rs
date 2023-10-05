use ligen::ir::Identifier;
use crate::prelude::*;

pub struct IdentifierParser;

impl Parser<rustpython_parser::ast::Identifier> for IdentifierParser {
    type Output = Identifier;
    fn parse(&self, input: rustpython_parser::ast::Identifier) -> Result<Self::Output> {
        Ok(Identifier::new(input.to_string()))
    }
}