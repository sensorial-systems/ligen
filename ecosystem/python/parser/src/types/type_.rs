use std::collections::HashMap;

use rustpython_parser::ast::{ExprName, Expr, ExprSubscript, ExprTuple};
use ligen::{ir::Type, parsing::parser::ParserConfig};
use crate::prelude::*;

#[derive(Default)]
pub struct TypeParser {
    mapper: HashMap<String, Type>,
}

impl TypeParser {
    pub fn new() -> Self {
        let mapper = HashMap::from([
            ("bool".into(), Type::boolean()),
            ("char".into(), Type::character()),
            ("byte".into(), Type::i8()),
            ("int".into(), Type::i32()),
            ("float".into(), Type::f32()),
        ]);
        Self { mapper }
    }
}

impl Parser<&ExprName> for TypeParser {
    type Output = Type;
    fn parse(&self, input: &ExprName, _config: &ParserConfig) -> Result<Self::Output> {
        let name = input.id.as_str();
        let type_ = self
            .mapper
            .get(name)
            .cloned()
            .unwrap_or(Type::Path(name.into()));
        Ok(type_)
    }
}

impl Parser<&ExprSubscript> for TypeParser {
    type Output = Type;
    fn parse(&self, input: &ExprSubscript, config: &ParserConfig) -> Result<Self::Output> {
        let mut type_ = self.parse(&*input.value, config)?;
        if let Type::Path(path) = &mut type_ {
            let type_ = self.parse(&*input.slice, config)?;
            path.last_mut().generics.types.push(type_);
        }
        Ok(type_)
    }
}

impl Parser<&ExprTuple> for TypeParser {
    type Output = Type;
    fn parse(&self, _input: &ExprTuple, _config: &ParserConfig) -> Result<Self::Output> {
        todo!("Tuple not implemented yet");
    }
}

impl Parser<&Expr> for TypeParser {
    type Output = Type;
    fn parse(&self, input: &Expr, config: &ParserConfig) -> Result<Self::Output> {
        match input {
            Expr::Name(expr) => self.parse(expr, config),
            Expr::Subscript(expr) => self.parse(expr, config),
            Expr::Tuple(expr) => self.parse(expr, config),
            _ => Err(Error::Message("Expected type".into()))
        }
    }
}
