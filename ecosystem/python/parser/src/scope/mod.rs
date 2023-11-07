mod scope_type;

use rustpython_parser::ast::{Arguments, Expr, Stmt};
use ligen::{ir::{Interface, Object, Function, Method, TypeDefinition}, parser::ParserConfig};
use crate::{prelude::*, parser::PythonParserConfig};

pub use scope_type::*;
use crate::parser::PythonParser;

impl Parser<WithSource<&[Stmt]>> for PythonParser {
    type Output = Scope;
    fn parse(&self, input: WithSource<&[Stmt]>, config: &ParserConfig) -> Result<Self::Output> {
        let objects = self.parse_objects(&input, config)?;
        let types = self.parse_types(&input, config)?;
        let functions = self.parse_functions(&input, config)?;
        let interfaces = self.parse_interfaces(&input, config)?;
        let methods = self.parse_methods(&input, config)?;
        let scope = Scope { objects, types, functions, methods, interfaces };
        let sub_scopes = self.parse_sub_scopes(&input, config)?;
        let scope = self.join_scopes(scope, sub_scopes);
        Ok(scope)
    }
}

impl PythonParser {
    fn join_scopes(&self, mut scope: Scope, sub_scopes: Vec<Scope>) -> Scope {
        for sub_scope in sub_scopes {
            scope.join(sub_scope)
        }
        scope.objects = self.deduplicate_objects(scope.objects);
        scope
    }

    fn deduplicate_objects(&self, objects: Vec<Object>) -> Vec<Object> {
        let mut deduplicated_objects: Vec<Object> = Vec::new();
        for object in objects.into_iter().rev() {
            if !deduplicated_objects.iter().any(|deduplicated_object| deduplicated_object.identifier == object.identifier) {
                deduplicated_objects.push(object)
            }
        }
        deduplicated_objects
    }

    fn parse_sub_scopes(&self, statements: &WithSource<&[Stmt]>, config: &ParserConfig) -> Result<Vec<Scope>> {
        let mut sub_scopes = Vec::new();
        for statement in statements.ast {
            match statement {
                Stmt::If(ast) => {
                    sub_scopes.push(self.parse(statements.sub( ast.body.as_slice()), config)?);
                    sub_scopes.push(self.parse(statements.sub(ast.orelse.as_slice()), config)?);
                },
                Stmt::Try(ast) => {
                    sub_scopes.push(self.parse(statements.sub(ast.body.as_slice()), config)?);
                    sub_scopes.push(self.parse(statements.sub(ast.orelse.as_slice()), config)?);
                    sub_scopes.push(self.parse(statements.sub(ast.finalbody.as_slice()), config)?);
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

    fn parse_functions(&self, statements: &WithSource<&[Stmt]>, config: &ParserConfig) -> Result<Vec<Function>> {
        let mut functions = Vec::new();
        for statement in statements.ast {
            if self.is_static_method(statements.sub(statement)) {
                match statement {
                    Stmt::FunctionDef(function) => {
                        if let Ok(function) = self.function_parser.parse(statements.sub(function.clone()), config) {
                            functions.push(function)
                        }
                    },
                    Stmt::AsyncFunctionDef(function) => {
                        if let Ok(function) = self.function_parser.parse(statements.sub(function.clone()), config) {
                            functions.push(function)
                        }
                    },
                    _ => (),
                }
            }
        }
        Ok(functions)
    }

    fn parse_methods(&self, statements: &WithSource<&[Stmt]>, config: &ParserConfig) -> Result<Vec<Method>> {
        let mut methods = Vec::new();
        for statement in statements.ast {
            if !self.is_static_method(statements.sub(statement)) {
                match statement {
                    Stmt::FunctionDef(function) => {
                        if let Ok(function) = self.parse(statements.sub(function.clone()), config) {
                            methods.push(function)
                        }
                    },
                    Stmt::AsyncFunctionDef(function) => {
                        if let Ok(function) = self.parse(statements.sub(function.clone()), config) {
                            methods.push(function)
                        }
                    },
                    _ => (),
                }
            }
        }
        Ok(methods)
    }

    fn parse_types(&self, statements: &WithSource<&[Stmt]>, config: &ParserConfig) -> Result<Vec<TypeDefinition>> {
        let mut types = Vec::new();
        for statement in statements.ast {
            if let Stmt::ClassDef(class) = statement {
                if let Ok(type_definition) = self.type_definition_parser.parse(statements.sub(class.clone()), config) {
                    types.push(type_definition)
                }
            }
        }
        Ok(types)
    }

    fn parse_interfaces(&self, statements: &WithSource<&[Stmt]>, config: &ParserConfig) -> Result<Vec<Interface>> {
        let mut interfaces = Vec::new();
        for statement in statements.ast {
            if let Stmt::ClassDef(class) = statement {
                if let Ok(interface) = self.parse(WithSource::new(&statements.source, class), config) {
                    interfaces.push(interface)
                }
            }
        }
        Ok(interfaces)
    }

    fn parse_objects(&self, statements: &WithSource<&[Stmt]>, config: &ParserConfig) -> Result<Vec<Object>> {
        let mut objects = Vec::new();
        let class_variables_as_properties = PythonParserConfig::from(config).get_class_variables_as_properties();
        if !class_variables_as_properties {
            for statement in statements.ast {
                match statement {
                    Stmt::Assign(assign) => {
                        if let Ok(more_objects) = self.object_parser.parse(assign, config) {
                            objects.extend(more_objects)
                        }
                    },
                    Stmt::AnnAssign(assign) => {
                        if let Ok(object) = self.object_parser.parse(assign, config) {
                            objects.push(object)
                        }
                    },
                    Stmt::AugAssign(assign) => {
                        if let Ok(object) = self.object_parser.parse(assign, config) {
                            objects.push(object)
                        }
                    },
                    _ => ()
                }
            }
        }
        Ok(objects)
    }
}