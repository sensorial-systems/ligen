pub mod validator;

use ligen::transformer::prelude::*;
use rustpython_parser::ast::{StmtAsyncFunctionDef, StmtFunctionDef};
use ligen::idl::{Method, Mutability, Function};
use crate::parser::PythonParser;
use crate::prelude::*;

impl<Body: Default> Transformer<WithSource<StmtFunctionDef>, Method<Body>> for PythonParser {
    fn transform(&self, input: WithSource<StmtFunctionDef>, config: &Config) -> Result<Method<Body>> {
        let function: Function<Body> = self.function_parser.transform(input, config)?;
        let attributes = function.attributes;
        let visibility = function.visibility;
        let synchrony = function.synchrony;
        let identifier = function.identifier;
        let inputs = function.inputs;
        let output = function.output;
        let mutability = Mutability::Mutable;
        let body = Default::default();
        Ok(Method { attributes, visibility, synchrony, mutability, identifier, inputs, output, body })
    }
}

impl<Body: Default> Transformer<WithSource<StmtAsyncFunctionDef>, Method<Body>> for PythonParser {
    fn transform(&self, input: WithSource<StmtAsyncFunctionDef>, config: &Config) -> Result<Method<Body>> {
        let function: Function<Body> = self.function_parser.transform(input, config)?;
        let attributes = function.attributes;
        let visibility = function.visibility;
        let synchrony = function.synchrony;
        let identifier = function.identifier;
        let inputs = function.inputs;
        let output = function.output;
        let mutability = Mutability::Mutable;
        let body = Default::default();
        Ok(Method { attributes, visibility, synchrony, mutability, identifier, inputs, output, body })
    }
}
