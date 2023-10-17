use rustpython_parser::ast::ExprName;
use ligen::ir::{Type, Primitive, Integer, Float};
use crate::prelude::*;

pub struct TypeParser;

impl Parser<ExprName> for TypeParser {
    type Output = Type;
    fn parse(&self, input: ExprName) -> Result<Self::Output> {
        let name = input.id;
        match name.as_str() {
            "bool"  => Ok(Primitive::Boolean.into()),
            "char"  => Ok(Primitive::Character.into()),
            "byte"    => Ok(Integer::I8.into()),
            "int"   => Ok(Integer::I32.into()),
            "float"   => Ok(Float::F32.into()),
            _ => Err(Error::Message(format!("Unknown type: {}", name)))
        }
    }
}