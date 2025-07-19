pub mod validator;

use crate::prelude::*;
use rustpython_parser::ast::StmtClassDef;
use ligen::idl::Interface;
use crate::parser::PythonParser;

impl Transformer<WithSource<&StmtClassDef>, Interface> for PythonParser {
    fn transform(&self, input: WithSource<&StmtClassDef>, config: &Config) -> Result<Interface> {
        let scope = self.transform(input.sub(input.ast.body.as_slice()), config)?;
        let identifier = self.identifier_parser.transform(input.ast.name.as_str(), config)?;
        let objects = scope.objects;
        let functions = scope.functions;
        let methods = scope.methods;
        Ok(Interface { identifier, objects, functions, methods, .. Default::default() })
    }
}