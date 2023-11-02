use rustpython_parser::ast::{ExprName, Expr, ExprSubscript, ExprTuple};
use ligen::ir::Type;
use crate::prelude::*;

#[derive(Default)]
pub struct TypeParser;

impl TypeParser {
    pub fn new() -> Self {
        Default::default()
    }
}

impl Parser<&ExprName> for TypeParser {
    type Output = Type;
    fn parse(&self, input: &ExprName) -> Result<Self::Output> {
        match input.id.as_str() {
            "bool"  => Ok(Type::boolean()),
            "char"  => Ok(Type::character()),
            "byte"    => Ok(Type::i8()),
            "int"   => Ok(Type::i32()),
            "float"   => Ok(Type::f32()),
            name => Ok(Type::Path(name.into()))
        }
    }
}

impl Parser<&ExprSubscript> for TypeParser {
    type Output = Type;
    fn parse(&self, input: &ExprSubscript) -> Result<Self::Output> {
        let mut type_ = self.parse(&*input.value)?;
        if let Type::Path(path) = &mut type_ {
            let type_ = self.parse(&*input.slice)?;
            path.last_mut().generics.types.push(type_);
        }
        Ok(type_)
    }
}

impl Parser<&ExprTuple> for TypeParser {
    type Output = Type;
    fn parse(&self, _input: &ExprTuple) -> Result<Self::Output> {
        todo!("Tuple not implemented yet");
    }
}

impl Parser<&Expr> for TypeParser {
    type Output = Type;
    fn parse(&self, input: &Expr) -> Result<Self::Output> {
        match input {
            Expr::Name(expr) => self.parse(expr),
            Expr::Subscript(expr) => self.parse(expr),
            Expr::Tuple(expr) => self.parse(expr),
            _ => Err(Error::Message("Expected type".into()))
        }
    }
}