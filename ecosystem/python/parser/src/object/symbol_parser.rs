use ligen::parsing::parser::ParserConfig;
use rustpython_parser::ast::{Expr, StmtAnnAssign, StmtAssign, StmtAugAssign};
use ligen::ir::Object;
use crate::identifier::IdentifierParser;
use crate::prelude::*;

use super::DynamicParser;

impl<'a> DynamicParser<'a> for SymbolParser {}

#[derive(Default)]
pub struct SymbolParser;

impl Parser<&StmtAnnAssign> for SymbolParser {
    type Output = Object;
    fn parse(&self, input: &StmtAnnAssign, config: &ParserConfig) -> Result<Self::Output> {
        self.parse(input.target.as_ref(), config)
    }
}

impl Parser<&StmtAugAssign> for SymbolParser {
    type Output = Object;
    fn parse(&self, input: &StmtAugAssign, config: &ParserConfig) -> Result<Self::Output> {
        self.parse(input.target.as_ref(), config)
    }
}

impl Parser<&Expr> for SymbolParser {
    type Output = Object;
    fn parse(&self, expr: &Expr, config: &ParserConfig) -> Result<Self::Output> {
        let identifier = expr
            .as_name_expr()
            .ok_or(Error::Message("Expected identifier".into()))?
            .id
            .as_str();
        let identifier = IdentifierParser::new().parse(identifier, config)?;
        Ok(Object { identifier, ..Default::default() })
    }
}

impl Parser<&StmtAssign> for SymbolParser {
    type Output = Vec<Object>;
    fn parse(&self, input: &StmtAssign, config: &ParserConfig) -> Result<Self::Output> {
        let mut objects = Vec::new();
        for target in &input.targets {
            if let Ok(object) = self.parse(target, config) {
                objects.push(object);
            }
        }
        Ok(objects)
    }
}
