mod import_parser;
mod scope_type;

use rustpython_parser::ast::{Arguments, Expr, Stmt};
use ligen::ir::{Interface, Object, Function, Method, Import, TypeDefinition};
use crate::{prelude::*, parser::PythonParserConfig};

// TODO: REMOVE THIS.
// pub use import_parser::*;
pub use scope_type::*;
use crate::parser::PythonParser;

impl Transformer<WithSource<&[Stmt]>, Scope> for PythonParser {
    fn transform(&self, input: WithSource<&[Stmt]>, config: &Config) -> Result<Scope> {
        let imports = self.parse_imports(&input, config)?;
        let objects = self.parse_objects(&input, config)?;
        let types = self.parse_types(&input, config)?;
        let functions = self.parse_functions(&input, config)?;
        let interfaces = self.parse_interfaces(&input, config)?;
        let methods = self.parse_methods(&input, config)?;
        let scope = Scope { imports, objects, types, functions, methods, interfaces };
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

    fn parse_sub_scopes(&self, statements: &WithSource<&[Stmt]>, config: &Config) -> Result<Vec<Scope>> {
        let mut sub_scopes = Vec::new();
        for statement in statements.ast {
            match statement {
                Stmt::If(ast) => {
                    sub_scopes.push(self.transform(statements.sub( ast.body.as_slice()), config)?);
                    sub_scopes.push(self.transform(statements.sub(ast.orelse.as_slice()), config)?);
                },
                Stmt::Try(ast) => {
                    sub_scopes.push(self.transform(statements.sub(ast.body.as_slice()), config)?);
                    sub_scopes.push(self.transform(statements.sub(ast.orelse.as_slice()), config)?);
                    sub_scopes.push(self.transform(statements.sub(ast.finalbody.as_slice()), config)?);
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

    fn parse_functions(&self, statements: &WithSource<&[Stmt]>, config: &Config) -> Result<Vec<Function>> {
        let mut functions = Vec::new();
        for statement in statements.ast {
            if self.is_static_method(statements.sub(statement)) {
                match statement {
                    Stmt::FunctionDef(function) => {
                        if let Ok(function) = self.function_parser.transform(statements.sub(function.clone()), config) {
                            functions.push(function)
                        }
                    },
                    Stmt::AsyncFunctionDef(function) => {
                        if let Ok(function) = self.function_parser.transform(statements.sub(function.clone()), config) {
                            functions.push(function)
                        }
                    },
                    _ => (),
                }
            }
        }
        Ok(functions)
    }

    fn parse_methods(&self, statements: &WithSource<&[Stmt]>, config: &Config) -> Result<Vec<Method>> {
        let mut methods = Vec::new();
        for statement in statements.ast {
            if !self.is_static_method(statements.sub(statement)) {
                match statement {
                    Stmt::FunctionDef(function) => {
                        if let Ok(function) = self.transform(statements.sub(function.clone()), config) {
                            methods.push(function)
                        }
                    },
                    Stmt::AsyncFunctionDef(function) => {
                        if let Ok(function) = self.transform(statements.sub(function.clone()), config) {
                            methods.push(function)
                        }
                    },
                    _ => (),
                }
            }
        }
        Ok(methods)
    }

    fn parse_types(&self, statements: &WithSource<&[Stmt]>, config: &Config) -> Result<Vec<TypeDefinition>> {
        let mut types = Vec::new();
        for statement in statements.ast {
            if let Stmt::ClassDef(class) = statement {
                match self.type_definition_parser.transform(statements.sub(class.clone()), config) {
                    Ok(type_definition) => types.push(type_definition),
                    Err(error) => todo!("Failed to parse type definition: {:?}", error)
                }
            }
        }
        Ok(types)
    }

    fn parse_interfaces(&self, statements: &WithSource<&[Stmt]>, config: &Config) -> Result<Vec<Interface>> {
        let mut interfaces = Vec::new();
        for statement in statements.ast {
            if let Stmt::ClassDef(class) = statement {
                if let Ok(interface) = self.transform(WithSource::new(&statements.source, class), config) {
                    interfaces.push(interface)
                }
            }
        }
        Ok(interfaces)
    }

    fn parse_imports(&self, statements: &WithSource<&[Stmt]>, config: &Config) -> Result<Vec<Import>> {
        let mut imports = Vec::new();
        for statement in statements.ast {
            match statement {
                Stmt::Import(import) => {
                    if let Ok(parsed_imports) = self.transform(statements.sub(import), config) {
                        imports.extend(parsed_imports);
                    }
                },
                Stmt::ImportFrom(import) => {
                    if let Ok(parsed_imports) = self.transform(statements.sub(import), config) {
                        imports.extend(parsed_imports);
                    }
                },
                _ => ()
            }
        }
        Ok(imports)
    }

    fn parse_objects(&self, statements: &WithSource<&[Stmt]>, config: &Config) -> Result<Vec<Object>> {
        let mut objects = Vec::new();
        let class_variables_as_properties = PythonParserConfig::from(config).get_class_variables_as_properties();
        if !class_variables_as_properties {
            for statement in statements.ast {
                match statement {
                    Stmt::Assign(assign) => {
                        if let Ok(more_objects) = self.object_parser.transform(assign, config) {
                            objects.extend(more_objects)
                        }
                    },
                    Stmt::AnnAssign(assign) => {
                        if let Ok(object) = self.object_parser.transform(statements.sub(assign), config) {
                            objects.push(object)
                        }
                    },
                    Stmt::AugAssign(assign) => {
                        if let Ok(object) = self.object_parser.transform(assign, config) {
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