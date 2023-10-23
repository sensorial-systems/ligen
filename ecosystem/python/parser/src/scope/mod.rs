mod scope_type;

use rustpython_parser::ast::{Arguments, Expr, Stmt};
use ligen::ir::{Interface, Constant, Function, Method, TypeDefinition};
use crate::prelude::*;

pub use scope_type::*;
use crate::parser::PythonParser;

impl Parser<WithSource<&[Stmt]>> for PythonParser {
    type Output = Scope;
    fn parse(&self, input: WithSource<&[Stmt]>) -> Result<Self::Output> {
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

impl PythonParser {
    fn parse_sub_scopes(&self, statements: &WithSource<&[Stmt]>) -> Result<Vec<Scope>> {
        let mut sub_scopes = Vec::new();
        for statement in statements.ast {
            match statement {
                Stmt::If(ast) => {
                    sub_scopes.push(self.parse(statements.sub( ast.body.as_slice()))?);
                    sub_scopes.push(self.parse(statements.sub(ast.orelse.as_slice()))?);
                },
                Stmt::Try(ast) => {
                    sub_scopes.push(self.parse(statements.sub(ast.body.as_slice()))?);
                    sub_scopes.push(self.parse(statements.sub(ast.orelse.as_slice()))?);
                    sub_scopes.push(self.parse(statements.sub(ast.finalbody.as_slice()))?);
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
                        if let Ok(function) = self.function_parser.parse(statements.sub(function.clone())) {
                            functions.push(function)
                        }
                    },
                    Stmt::AsyncFunctionDef(function) => {
                        if let Ok(function) = self.function_parser.parse(statements.sub(function.clone())) {
                            functions.push(function)
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
                        if let Ok(function) = self.method_parser.parse(statements.sub(function.clone())) {
                            methods.push(function)
                        }
                    },
                    Stmt::AsyncFunctionDef(function) => {
                        if let Ok(function) = self.method_parser.parse(statements.sub(function.clone())) {
                            methods.push(function)
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
                if let Ok(type_definition) = self.type_definition_parser.parse(statements.sub(class.clone())) {
                    types.push(type_definition)
                }
            }
        }
        Ok(types)
    }

    fn parse_interfaces(&self, statements: &WithSource<&[Stmt]>) -> Result<Vec<Interface>> {
        let mut interfaces = Vec::new();
        for statement in statements.ast {
            if let Stmt::ClassDef(class) = statement {
                if let Ok(interface) = self.parse(WithSource::new(&statements.source, class)) {
                    interfaces.push(interface)
                }
            }
        }
        Ok(interfaces)
    }

    fn parse_constants(&self, statements: &WithSource<&[Stmt]>) -> Result<Vec<Constant>> {
        let mut constants = Vec::new();
        for statement in statements.ast {
            match statement {
                Stmt::Assign(assign) => {
                    if let Ok(more_constants) = self.constant_parser.parse(assign) {
                        constants.extend(more_constants)
                    }
                },
                Stmt::AnnAssign(assign) => {
                    if let Ok(constant) = self.constant_parser.parse(assign) {
                        constants.push(constant)
                    }
                },
                Stmt::AugAssign(assign) => {
                    if let Ok(constant) = self.constant_parser.parse(assign) {
                        constants.push(constant)
                    }
                },
                _ => ()
            }
        }
        Ok(constants)
    }
}