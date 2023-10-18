use rustpython_parser::ast::{Arguments, Expr, Stmt};
use ligen::ir::{Identifier, Interface, Constant, Function, Method, TypeDefinition, Structure};
use crate::identifier::IdentifierParser;
use crate::prelude::*;
use crate::symbols::interface::InterfaceParser;

mod scope_type;

pub use scope_type::*;

#[derive(Default)]
pub struct ScopeParser;

impl ScopeParser {
    pub fn new() -> Self {
        Default::default()
    }
}

impl Parser<WithSource<&[Stmt]>> for ScopeParser {
    type Output = Scope;
    fn parse(&self, input: WithSource<&[Stmt]>) -> Result<Self::Output> {
        self.parse_symbols(input)
    }
    fn parse_symbols(&self, input: WithSource<&[Stmt]>) -> Result<Self::Output> {
        let constants = self.parse_constants(&input)?;
        let types = self.parse_types(&input)?;
        let functions = self.parse_functions(&input)?;
        let interfaces = self.parse_interfaces(&input)?;
        let methods = self.parse_methods(&input)?;
        let mut scope = Scope { constants, types, functions, methods, interfaces };
        let sub_scopes = self.parse_sub_scopes(&input)?;
        for sub_scope in sub_scopes {
            scope.join(sub_scope);
        }
        Ok(scope)
    }
}

impl ScopeParser {
    fn parse_sub_scopes(&self, statements: &WithSource<&[Stmt]>) -> Result<Vec<Scope>> {
        let mut sub_scopes = Vec::new();
        for statement in statements.ast {
            match statement {
                Stmt::If(ast) => {
                    sub_scopes.push(self.parse(statements.sub( &ast.body))?);
                    sub_scopes.push(self.parse(statements.sub(&ast.orelse))?);
                },
                Stmt::Try(ast) => {
                    sub_scopes.push(self.parse(statements.sub(&ast.body))?);
                    sub_scopes.push(self.parse(statements.sub(&ast.orelse))?);
                    sub_scopes.push(self.parse(statements.sub(&ast.finalbody))?);
                },
                _ => ()
            }
        }
        Ok(sub_scopes)
    }

    fn has_static_decorator(&self, decorator_list: WithSource<&[Expr]>) -> bool {
        decorator_list
            .ast
            .iter()
            .filter_map(|expr| match expr {
                Expr::Call(call) => call.func.as_name_expr(),
                _ => None
            })
            .any(|decorator| decorator.id.as_str() == "staticmethod")
    }

    fn has_self(&self, arguments: WithSource<&Arguments>) -> bool {
        arguments
            .ast
            .args
            .first()
            .map(|argument| argument.def.arg.as_str() == "self")
            .unwrap_or(false)
    }

    fn is_static_method(&self, statement: WithSource<&Stmt>) -> bool {
        match statement.ast {
            Stmt::FunctionDef(function) => {
                self.has_static_decorator(statement.sub(&function.decorator_list)) || !self.has_self(statement.sub(&function.args))
            },
            Stmt::AsyncFunctionDef(function) => {
                self.has_static_decorator(statement.sub(&function.decorator_list)) || !self.has_self(statement.sub(&function.args))
            },
            _ => false
        }
    }

    fn parse_functions(&self, statements: &WithSource<&[Stmt]>) -> Result<Vec<Function>> {
        let mut functions = Vec::new();
        for statement in statements.ast {
            if self.is_static_method(statements.sub(statement)) {
                match statement {
                    Stmt::FunctionDef(function) => {
                        if let Ok(identifier) = IdentifierParser::new().parse(function.name.as_str()) {
                            functions.push(Function { identifier, ..Default::default() })
                        }
                    },
                    Stmt::AsyncFunctionDef(function) => {
                        if let Ok(identifier) = IdentifierParser::new().parse(function.name.as_str()) {
                            functions.push(Function { identifier, ..Default::default() })
                        }

                    },
                    _ => (),
                }
            }
        }
        Ok(functions)
    }

    fn parse_methods(&self, statements: &WithSource<&[Stmt]>) -> Result<Vec<Method>> {
        let mut methods = Vec::new();
        for statement in statements.ast {
            if !self.is_static_method(statements.sub(statement)) {
                match statement {
                    Stmt::FunctionDef(function) => {
                        if let Ok(identifier) = IdentifierParser::new().parse(function.name.as_str()) {
                            methods.push(Method { identifier, ..Default::default() })
                        }
                    },
                    Stmt::AsyncFunctionDef(function) => {
                        if let Ok(identifier) = IdentifierParser::new().parse(function.name.as_str()) {
                            methods.push(Method { identifier, ..Default::default() })
                        }
                    },
                    _ => (),
                }
            }
        }
        Ok(methods)
    }

    fn parse_types(&self, statements: &WithSource<&[Stmt]>) -> Result<Vec<TypeDefinition>> {
        let mut types = Vec::new();
        for statement in statements.ast {
            if let Stmt::ClassDef(class) = statement {
                if let Ok(identifier) = IdentifierParser::new().parse(class.name.as_str()) {
                    types.push(Structure { identifier, .. Default::default() }.into())
                }
            }
        }
        Ok(types)
    }

    fn parse_interfaces(&self, statements: &WithSource<&[Stmt]>) -> Result<Vec<Interface>> {
        let mut interfaces = Vec::new();
        for statement in statements.ast {
            if let Stmt::ClassDef(class) = statement {
                if let Ok(interface) = InterfaceParser::new().parse(WithSource::new(&statements.source, class)) {
                    interfaces.push(interface)
                }
            }
        }
        Ok(interfaces)
    }

    fn is_constant(&self, identifier: &Identifier) -> bool {
        identifier.name.to_uppercase() == identifier.name
    }

    fn parse_constants(&self, statements: &WithSource<&[Stmt]>) -> Result<Vec<Constant>> {
        let mut constants = Vec::new();
        for statement in statements.ast {
            match statement {
                Stmt::Assign(assign) => {
                    for target in &assign.targets {
                        if let Ok(identifier) = self.parse_expr(target) {
                            if self.is_constant(&identifier) {
                                constants.push(Constant { identifier, .. Default::default() })
                            }
                        }
                    }
                },
                Stmt::AnnAssign(assign) => {
                    if let Ok(identifier) = self.parse_expr(&assign.target) {
                        if self.is_constant(&identifier) {
                            constants.push(Constant { identifier, .. Default::default() })
                        }
                    }
                },
                Stmt::AugAssign(assign) => {
                    if let Ok(identifier) = self.parse_expr(&assign.target) {
                        if self.is_constant(&identifier) {
                            constants.push(Constant { identifier, .. Default::default() })
                        }
                    }
                },
                _ => ()
            }
        }
        Ok(constants)
    }

    fn parse_expr(&self, expr: &Expr) -> Result<Identifier> {
        let identifier = expr
            .as_name_expr()
            .ok_or(Error::Message("Expected identifier".into()))?
            .id
            .as_str();
        let identifier = IdentifierParser::new().parse(identifier)?;
        Ok(identifier)
    }
}