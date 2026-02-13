use crate::identifier::IdentifierParser;
use crate::prelude::*;
use crate::types::type_::TypeParser;
use ligen::idl::{Object, Visibility};
use rustpython_parser::ast::{Expr, StmtAnnAssign, StmtAssign, StmtAugAssign};

#[derive(Default)]
pub struct ObjectParser {
    identifier_parser: IdentifierParser,
    type_parser: TypeParser,
}

impl Transformer<WithSource<&StmtAnnAssign>, Object> for ObjectParser {
    fn transform(&self, input: WithSource<&StmtAnnAssign>, config: &Config) -> Result<Object> {
        let mut object = self.transform(input.ast.target.as_ref(), config)?;
        if !config.get_only_parse_symbols() {
            object.type_ = self
                .type_parser
                .transform(input.sub(&*input.ast.annotation), config)?;
        }
        Ok(object)
    }
}

impl Transformer<&StmtAugAssign, Object> for ObjectParser {
    fn transform(&self, input: &StmtAugAssign, config: &Config) -> Result<Object> {
        self.transform(input.target.as_ref(), config)
    }
}

impl Transformer<&Expr, Object> for ObjectParser {
    fn transform(&self, expr: &Expr, config: &Config) -> Result<Object> {
        let identifier = expr
            .as_name_expr()
            .ok_or(Error::Message("Expected identifier".into()))?
            .id
            .as_str();
        let identifier = self.identifier_parser.transform(identifier, config)?;
        let visibility = Visibility::Public;
        if config.get_only_parse_symbols() {
            Ok(Object {
                visibility,
                identifier,
                ..Default::default()
            })
        } else {
            let mutability = self.identifier_parser.get_mutability(&identifier);
            let type_ = Default::default();
            let literal = Default::default();
            Ok(Object {
                visibility,
                identifier,
                mutability,
                literal,
                type_,
            })
        }
    }
}

impl Transformer<&StmtAssign, Vec<Object>> for ObjectParser {
    fn transform(&self, input: &StmtAssign, config: &Config) -> Result<Vec<Object>> {
        let mut objects = Vec::new();
        for target in &input.targets {
            if let Ok(object) = self.transform(target, config) {
                objects.push(object);
            }
        }
        Ok(objects)
    }
}
