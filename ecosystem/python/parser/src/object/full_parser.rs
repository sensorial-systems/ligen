use rustpython_parser::ast::{Expr, StmtAnnAssign, StmtAssign, StmtAugAssign};
use ligen::ir::Object;
use crate::identifier::IdentifierParser;
use crate::prelude::*;
use crate::types::type_::TypeParser;

use super::DynamicParser;

impl<'a> DynamicParser<'a> for FullParser {}

#[derive(Default)]
pub struct FullParser;

impl Parser<&StmtAnnAssign> for FullParser {
    type Output = Object;
    fn parse(&self, input: &StmtAnnAssign) -> Result<Self::Output> {
        let mut object = self.parse(input.target.as_ref())?;
        object.type_ = TypeParser::new().parse(&*input.annotation)?;
        Ok(object)
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
        let identifier_parser = IdentifierParser::new();
        let identifier = identifier_parser.parse(identifier)?;
        let mutability = identifier_parser.get_mutability(&identifier);
        let type_ = Default::default();
        let literal = Default::default();
        Ok(Object { identifier, mutability, literal, type_ })
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

