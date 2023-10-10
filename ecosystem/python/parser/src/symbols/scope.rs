use rustpython_parser::ast::{Arguments, Expr, Stmt};
use ligen::symbols::identifier::Identifier;
use ligen::symbols::interface::Interface;
use crate::identifier::IdentifierParser;
use crate::prelude::*;
use crate::symbols::interface::InterfaceParser;

pub struct ScopeParser;

impl ScopeParser {
    pub fn new() -> Self {
        Self
    }
}

pub struct Scope {
    pub constants: Vec<Identifier>,
    pub types: Vec<Identifier>,
    pub functions: Vec<Identifier>,
    pub interfaces: Vec<Interface>
}

impl<T> Parser<&[Stmt<T>]> for ScopeParser {
    type Output = Scope;
    fn parse(&self, input: &[Stmt<T>]) -> Result<Self::Output> {
        let constants = self.parse_constants(input)?;
        let types = self.parse_types(input)?;
        let functions = self.parse_functions(input)?;
        let interfaces = self.parse_interfaces(input)?;
        Ok(Scope { constants, types, functions, interfaces })
    }
}

impl ScopeParser {

    pub fn has_static_decorator<T>(&self, decorator_list: &[Expr<T>]) -> bool {
        decorator_list
            .iter()
            .filter_map(|expr| match expr {
                Expr::Call(call) => call.func.as_name_expr(),
                _ => None
            })
            .any(|decorator| decorator.id.as_str() == "staticmethod")
    }

    pub fn has_self<T>(&self, arguments: &Arguments<T>) -> bool {
        arguments
            .args
            .first()
            .map(|argument| argument.def.arg.as_str() == "self")
            .unwrap_or(false)
    }

    pub fn is_static_method<T>(&self, statement: &Stmt<T>) -> bool {
        match statement {
            Stmt::FunctionDef(function) => {
                self.has_static_decorator(&function.decorator_list) || !self.has_self(&function.args)
            },
            Stmt::AsyncFunctionDef(function) => {
                self.has_static_decorator(&function.decorator_list) || !self.has_self(&function.args)
            },
            _ => false
        }
    }

    pub(crate) fn parse_functions<T>(&self, statements: &[Stmt<T>]) -> Result<Vec<Identifier>> {
        let mut functions = Vec::new();
        for statement in statements {
            if self.is_static_method(statement) {
                match statement {
                    Stmt::FunctionDef(function) => functions.push(IdentifierParser::new().parse(function.name.as_str())?),
                    Stmt::AsyncFunctionDef(function) => functions.push(IdentifierParser::new().parse(function.name.as_str())?),
                    _ => (),
                }
            }
        }
        Ok(functions)
    }

    pub(crate) fn parse_methods<T>(&self, statements: &[Stmt<T>]) -> Result<Vec<Identifier>> {
        let mut methods = Vec::new();
        for statement in statements {
            if !self.is_static_method(statement) {
                match statement {
                    Stmt::FunctionDef(function) => methods.push(IdentifierParser::new().parse(function.name.as_str())?),
                    Stmt::AsyncFunctionDef(function) => methods.push(IdentifierParser::new().parse(function.name.as_str())?),
                    _ => (),
                }
            }
        }
        Ok(methods)
    }

    pub(crate) fn parse_types<T>(&self, statements: &[Stmt<T>]) -> Result<Vec<Identifier>> {
        let mut interfaces = Vec::new();
        for statement in statements {
            match statement {
                Stmt::ClassDef(class) => interfaces.push(IdentifierParser::new().parse(class.name.as_str())?),
                // TODO: How can we detect and parse a type alias? Only ClassDefs are being parsed.
                _ => ()
            }
        }
        Ok(interfaces)
    }

    pub(crate) fn parse_interfaces<T>(&self, statements: &[Stmt<T>]) -> Result<Vec<Interface>> {
        let mut interfaces = Vec::new();
        for statement in statements {
            match statement {
                Stmt::ClassDef(class) => interfaces.push(InterfaceParser::new().parse(class)?),
                _ => ()
            }
        }
        Ok(interfaces)
    }
    pub(crate) fn parse_constants<T>(&self, statements: &[Stmt<T>]) -> Result<Vec<Identifier>> {
        let mut constants = Vec::new();
        for statement in statements {
            match statement {
                Stmt::Assign(assign) =>
                    for target in &assign.targets {
                        constants.push(self.parse_expr(target)?);
                    },
                Stmt::AnnAssign(assign) =>
                    constants.push(self.parse_expr(&assign.target)?),
                Stmt::AugAssign(assign) =>
                    constants.push(self.parse_expr(&assign.target)?),
                _ => ()
            }
        }
        Ok(constants)
    }

    pub(crate) fn parse_expr<T>(&self, expr: &Expr<T>) -> Result<Identifier> {
        let identifier = expr
            .as_name_expr()
            .ok_or(Error::Message("Expected identifier".into()))?
            .id
            .as_str();
        let identifier = IdentifierParser::new().parse(identifier)?;
        Ok(identifier)
    }
}