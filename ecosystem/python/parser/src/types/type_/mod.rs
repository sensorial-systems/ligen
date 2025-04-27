use std::collections::HashMap;

pub mod validator;
pub use validator::*;

use rustpython_parser::ast::{ExprName, Expr, ExprSubscript, ExprTuple, Ranged, ExprList, ExprConstant, Constant, ExprAttribute};
use ligen::ir::{Path, Type, Identifier};
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
        map.insert("Dict".into(), Identifier::dictionary());
        map.insert("tuple".into(), Identifier::tuple());
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

impl Transformer<&ExprName, Type> for TypeParser {
    fn transform(&self, input: &ExprName, _config: &Config) -> Result<Type> {
        let name = input.id.as_str();
        let identifier = self
            .mapper
            .to_ligen(&name.into())
            .cloned()
            .unwrap_or(Identifier::from(name));
        let mut type_ = Type::from(identifier);
        if type_.path.last().identifier == Identifier::vector() {
            type_.path.last_mut().generics.types.push(Type::opaque());
        }
        Ok(type_)
    }
}

impl Transformer<WithSource<&ExprSubscript>, Type> for TypeParser {
    fn transform(&self, input: WithSource<&ExprSubscript>, config: &Config) -> Result<Type> {
        let mut type_ = self.transform(input.sub(&*input.ast.value), config)?;
        let path = &mut type_.path;
        if let Expr::Tuple(expr) = &*input.ast.slice {
            let types = &mut path.last_mut().generics.types;
            types.extend(self.transform(input.sub(expr), config)?);
        } else {
            let type_ = self.transform(input.sub(&*input.ast.slice), config)?;
            let last = path.last_mut();
            if last.identifier == Identifier::vector() {
                last.generics.types[0] = type_;
            } else {
                last.generics.types.push(type_);
            }
        }
        Ok(type_)
    }
}

impl Transformer<WithSource<&ExprTuple>, Vec<Type>> for TypeParser {
    fn transform(&self, input: WithSource<&ExprTuple>, config: &Config) -> Result<Vec<Type>> {
        let mut types = Vec::new();
        for expr in &input.ast.elts {
            types.push(self.transform(input.sub(expr), config)?);
        }
        Ok(types)
    }
}

impl Transformer<WithSource<&ExprList>, Type> for TypeParser {
    fn transform(&self, input: WithSource<&ExprList>, config: &Config) -> Result<Type> {
        let types = input
            .ast
            .elts
            .iter()
            .map(|expr| 
                self.transform(input.sub(expr), config)
            ).collect::<Result<Vec<Type>>>()?;
        if types.len() == 1 {
            Ok(types.into_iter().next().unwrap())
        } else {
            Ok(Type::tuple(types))
        }
    }
}

impl Transformer<WithSource<&ExprConstant>, Type> for TypeParser {
    fn transform(&self, input: WithSource<&ExprConstant>, config: &Config) -> Result<Type> {
        self.transform(&input.ast.value, config)
    }
}

impl Transformer<&Constant, Type> for TypeParser {
    fn transform(&self, input: &Constant, _config: &Config) -> Result<Type> {
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
                    .map(|expr| self.transform(expr, _config))
                    .collect::<Result<Vec<Type>>>()?;
                Ok(Type::tuple(types))
            },
            _ => Err(Error::Message(format!("Failed to parse constant in type: {:?}", input)))
        }
    }
}

impl Transformer<WithSource<&ExprAttribute>, Type> for TypeParser {
    fn transform(&self, input: WithSource<&ExprAttribute>, config: &Config) -> Result<Type> {
        let mut type_ = self.transform(input.sub(&*input.ast.value), config)?;
        let name = input.ast.attr.as_str();
        let identifier = self
            .mapper
            .to_ligen(&name.into())
            .cloned()
            .unwrap_or(Identifier::from(name));
        type_.path = Path::from(identifier).join(type_.path);
        Ok(type_)
    }
}

impl Transformer<WithSource<&Expr>, Type> for TypeParser {
    fn transform(&self, input: WithSource<&Expr>, config: &Config) -> Result<Type> {
        match &input.ast {
            Expr::Name(expr) => self.transform(expr, config),
            Expr::Subscript(expr) => self.transform(input.sub(expr), config),
            Expr::List(expr) => self.transform(input.sub(expr), config),
            Expr::Constant(expr) => self.transform(input.sub(expr), config),
            Expr::Attribute(expr) => self.transform(input.sub(expr), config),
            Expr::BinOp(_expr) => Ok(Type::opaque()), // TODO: BinOp (e.g. "int | float") is not supported yet. They can be implemented as enumerations.
            Expr::Call(_expr) => Ok(Type::opaque()), // TODO: Call (e.g. "Annotated[int, Ge(0)]") is not supported yet. They can be implemented as function types.
            _ => Err(Error::Message(format!("Failed to parse type: {}, {:?}", &input.source[input.ast.start().to_usize()..input.ast.end().to_usize()], input.ast)))
        }
    }
}
