use ligen::parser::prelude::*;
use rustpython_parser::ast::{Expr, StmtAnnAssign, StmtAssign, StmtAugAssign};
use ligen::ir::Object;
use crate::identifier::IdentifierParser;
use crate::prelude::*;
use crate::types::type_::TypeParser;

#[derive(Default)]
pub struct ObjectParser;

impl Parser<WithSource<&StmtAnnAssign>> for ObjectParser {
    type Output = Object;
    fn parse(&self, input: WithSource<&StmtAnnAssign>, config: &Config) -> Result<Self::Output> {
        let mut object = self.parse(input.ast.target.as_ref(), config)?;
        if !config.get_only_parse_symbols() {
            object.type_ = TypeParser::new().parse(input.sub(&*input.ast.annotation), config)?;
        }
        Ok(object)
    }
}

impl Parser<&StmtAugAssign> for ObjectParser {
    type Output = Object;
    fn parse(&self, input: &StmtAugAssign, config: &Config) -> Result<Self::Output> {
        self.parse(input.target.as_ref(), config)
    }
}

impl Parser<&Expr> for ObjectParser {
    type Output = Object;
    fn parse(&self, expr: &Expr, config: &Config) -> Result<Self::Output> {
        let identifier = expr
            .as_name_expr()
            .ok_or(Error::Message("Expected identifier".into()))?
            .id
            .as_str();
        let identifier_parser = IdentifierParser::new();
        let identifier = identifier_parser.parse(identifier, config)?;
        if config.get_only_parse_symbols() {
            Ok(Object { identifier, ..Default::default() })
        } else {
            let mutability = identifier_parser.get_mutability(&identifier);
            let type_ = Default::default();
            let literal = Default::default();
            Ok(Object { identifier, mutability, literal, type_ })
        }
    }
}

impl Parser<&StmtAssign> for ObjectParser {
    type Output = Vec<Object>;
    fn parse(&self, input: &StmtAssign, config: &Config) -> Result<Self::Output> {
        let mut objects = Vec::new();
        for target in &input.targets {
            if let Ok(object) = self.parse(target, config) {
                objects.push(object);
            }
        }
        Ok(objects)
    }
}

