use std::collections::HashMap;

use rustpython_parser::ast::{ExprName, Expr, ExprSubscript, ExprTuple, Ranged, ExprList, ExprConstant, Constant};
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
        map.insert("float".into(), Identifier::f64());
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

impl Parser<WithSource<&ExprSubscript>> for TypeParser {
    type Output = Type;
    fn parse(&self, input: WithSource<&ExprSubscript>, config: &ParserConfig) -> Result<Self::Output> {
        let mut type_ = self.parse(input.sub(&*input.ast.value), config)?;
        let path = &mut type_.path;
        if let Expr::Tuple(expr) = &*input.ast.slice {
            let types = &mut path.last_mut().generics.types;
            types.extend(self.parse(input.sub(expr), config)?);
        } else {
            let type_ = self.parse(input.sub(&*input.ast.slice), config)?;
            path.last_mut().generics.types.push(type_);
        }
        Ok(type_)
    }
}

impl Parser<WithSource<&ExprTuple>> for TypeParser {
    type Output = Vec<Type>;
    fn parse(&self, input: WithSource<&ExprTuple>, config: &ParserConfig) -> Result<Self::Output> {
        let mut types = Vec::new();
        for expr in &input.ast.elts {
            types.push(self.parse(input.sub(expr), config)?);
        }
        Ok(types)
    }
}

impl Parser<WithSource<&ExprList>> for TypeParser {
    type Output = Type;
    fn parse(&self, input: WithSource<&ExprList>, config: &ParserConfig) -> Result<Self::Output> {
        let types = input
            .ast
            .elts
            .iter()
            .map(|expr| 
                self.parse(input.sub(expr), config)
            ).collect::<Result<Vec<Type>>>()?;
        if types.len() == 1 {
            Ok(types.into_iter().next().unwrap())
        } else {
            Ok(Type::tuple(types))
        }
    }
}

impl Parser<WithSource<&ExprConstant>> for TypeParser {
    type Output = Type;
    fn parse(&self, input: WithSource<&ExprConstant>, config: &ParserConfig) -> Result<Self::Output> {
        self.parse(&input.ast.value, config)
    }
}

impl Parser<&Constant> for TypeParser {
    type Output = Type;
    fn parse(&self, input: &Constant, _config: &ParserConfig) -> Result<Self::Output> {
        match &input {
            Constant::Ellipsis => Ok(Type::variadic(Type::opaque())),
            Constant::Str(_) => Ok(Type::string()),
            Constant::Bool(_) => Ok(Type::boolean()),
            Constant::Bytes(_) => Ok(Type::vector(Type::u8())),
            Constant::Float(_) => Ok(Type::f64()),
            Constant::Int(_) => Ok(Type::i32()),
            Constant::None => Ok(Type::option(Type::opaque())),
            Constant::Tuple(values) => {
                let types = values
                    .iter()
                    .map(|expr| self.parse(expr, _config))
                    .collect::<Result<Vec<Type>>>()?;
                Ok(Type::tuple(types))
            },
            _ => Err(Error::Message(format!("Failed to parse constant in type: {:?}", input)))
        }
    }
}

impl Parser<WithSource<&Expr>> for TypeParser {
    type Output = Type;
    fn parse(&self, input: WithSource<&Expr>, config: &ParserConfig) -> Result<Self::Output> {
        match &input.ast {
            Expr::Name(expr) => self.parse(expr, config),
            Expr::Subscript(expr) => self.parse(input.sub(expr), config),
            Expr::List(expr) => self.parse(input.sub(expr), config),
            Expr::Constant(expr) => self.parse(input.sub(expr), config),
            _ => Err(Error::Message(format!("Failed to parse type: {}, {:#?}", &input.source[input.ast.start().to_usize()..input.ast.end().to_usize()], input.ast)))
        }
    }
}
