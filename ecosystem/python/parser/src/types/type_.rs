use std::collections::HashMap;

use rustpython_parser::ast::{ExprName, Expr, ExprSubscript, ExprTuple};
use ligen::{ir::{Type, Identifier}, parser::ParserConfig};
use crate::prelude::*;

pub struct PythonMapper {
    map: HashMap<Identifier, Identifier>
}

impl Default for PythonMapper {
    fn default() -> Self {
        let mut map = HashMap::new();
        map.insert("bool".into(), Identifier::boolean(), );
        map.insert("int".into(), Identifier::i32());
        map.insert("float".into(), Identifier::f32());
        map.insert("str".into(), Identifier::string());
        map.insert("Optional".into(), Identifier::option());
        map.insert("Any".into(), Identifier::opaque());
        map.insert("byte".into(), Identifier::i8());
        map.insert("datetime".into(), Identifier::date_time());
        map.insert("List".into(), Identifier::vector());
        map.insert("list".into(), Identifier::vector());
        map.insert("dict".into(), Identifier::dictionary());
        Self { map }
    }
}

impl PythonMapper {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn to_ligen(&self, identifier: &Identifier) -> Option<&Identifier> {
        self.map.get(identifier)
    }
}

pub struct TypeParser {
    mapper: PythonMapper
}

impl Default for TypeParser {
    fn default() -> Self {
        let mapper = PythonMapper::new();
        Self { mapper }
    }
}

impl TypeParser {
    pub fn new() -> Self {
        Default::default()
    }
}

impl Parser<&ExprName> for TypeParser {
    type Output = Type;
    fn parse(&self, input: &ExprName, _config: &ParserConfig) -> Result<Self::Output> {
        let name = input.id.as_str();
        let identifier = self
            .mapper
            .to_ligen(&name.into())
            .cloned()
            .unwrap_or(Identifier::from(name));
        let type_ = identifier.into();
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
        println!("{:#?}", _input);
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
