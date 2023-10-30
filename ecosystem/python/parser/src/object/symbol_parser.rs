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
    fn parse(&self, input: &StmtAnnAssign) -> Result<Self::Output> {
        self.parse(input.target.as_ref())
    }
}

impl Parser<&StmtAugAssign> for SymbolParser {
    type Output = Object;
    fn parse(&self, input: &StmtAugAssign) -> Result<Self::Output> {
        self.parse(input.target.as_ref())
    }
}

impl Parser<&Expr> for SymbolParser {
    type Output = Object;
    fn parse(&self, expr: &Expr) -> Result<Self::Output> {
        let identifier = expr
            .as_name_expr()
            .ok_or(Error::Message("Expected identifier".into()))?
            .id
            .as_str();
        let identifier = IdentifierParser::new().parse(identifier)?;
        Ok(Object { identifier, ..Default::default() })
    }
}

impl Parser<&StmtAssign> for SymbolParser {
    type Output = Vec<Object>;
    fn parse(&self, input: &StmtAssign) -> Result<Self::Output> {
        let mut objects = Vec::new();
        for target in &input.targets {
            if let Ok(object) = self.parse(target) {
                objects.push(object);
            }
        }
        Ok(objects)
    }
}
