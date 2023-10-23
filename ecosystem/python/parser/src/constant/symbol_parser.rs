use rustpython_parser::ast::{Expr, StmtAnnAssign, StmtAssign, StmtAugAssign};
use ligen::ir::{Constant, Identifier};
use crate::identifier::IdentifierParser;
use crate::prelude::*;

use super::DynamicParser;

impl<'a> DynamicParser<'a> for SymbolParser {}

#[derive(Default)]
pub struct SymbolParser;

impl Parser<&StmtAnnAssign> for SymbolParser {
    type Output = Constant;
    fn parse(&self, input: &StmtAnnAssign) -> Result<Self::Output> {
        self.parse(input.target.as_ref())
    }
}

impl Parser<&StmtAugAssign> for SymbolParser {
    type Output = Constant;
    fn parse(&self, input: &StmtAugAssign) -> Result<Self::Output> {
        self.parse(input.target.as_ref())
    }
}

impl Parser<&Expr> for SymbolParser {
    type Output = Constant;
    fn parse(&self, expr: &Expr) -> Result<Self::Output> {
        let identifier = expr
            .as_name_expr()
            .ok_or(Error::Message("Expected identifier".into()))?
            .id
            .as_str();
        let identifier = IdentifierParser::new().parse(identifier)?;
        if self.is_constant(&identifier) {
            Ok(Constant { identifier, ..Default::default() })
        } else {
            Err(Error::Message("Expected constant".into()))
        }
    }
}

impl Parser<&StmtAssign> for SymbolParser {
    type Output = Vec<Constant>;
    fn parse(&self, input: &StmtAssign) -> Result<Self::Output> {
        let mut constants = Vec::new();
        for target in &input.targets {
            if let Ok(constant) = self.parse(target) {
                constants.push(constant);
            }
        }
        Ok(constants)
    }
}

impl SymbolParser {
    fn is_constant(&self, identifier: &Identifier) -> bool {
        identifier.name.to_uppercase() == identifier.name
    }
}