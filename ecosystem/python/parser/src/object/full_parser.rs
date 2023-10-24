// FIXME: Duplicated from symbol_parser.

use rustpython_parser::ast::{Expr, StmtAnnAssign, StmtAssign, StmtAugAssign};
use ligen::ir::{Object, Identifier, Mutability};
use crate::identifier::IdentifierParser;
use crate::prelude::*;

use super::DynamicParser;

impl<'a> DynamicParser<'a> for FullParser {}

#[derive(Default)]
pub struct FullParser;

impl Parser<&StmtAnnAssign> for FullParser {
    type Output = Object;
    fn parse(&self, input: &StmtAnnAssign) -> Result<Self::Output> {
        self.parse(input.target.as_ref())
    }
}

impl Parser<&StmtAugAssign> for FullParser {
    type Output = Object;
    fn parse(&self, input: &StmtAugAssign) -> Result<Self::Output> {
        self.parse(input.target.as_ref())
    }
}

impl Parser<&Expr> for FullParser {
    type Output = Object;
    fn parse(&self, expr: &Expr) -> Result<Self::Output> {
        let identifier = expr
            .as_name_expr()
            .ok_or(Error::Message("Expected identifier".into()))?
            .id
            .as_str();
        let identifier = IdentifierParser::new().parse(identifier)?;
        let mutability = self.get_mutability(&identifier);
        Ok(Object { identifier, mutability, ..Default::default() })
    }
}

impl Parser<&StmtAssign> for FullParser {
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

impl FullParser {
    fn get_mutability(&self, identifier: &Identifier) -> Mutability {
        if identifier.name.to_uppercase() == identifier.name {
            Mutability::Constant
        } else {
            Mutability::Mutable
        }
    }
}