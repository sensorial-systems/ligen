pub mod validator;

use ligen::parser::prelude::*;
use rustpython_parser::ast::{StmtAsyncFunctionDef, StmtFunctionDef};
use ligen::ir::{Method, Mutability};
use crate::parser::PythonParser;
use crate::prelude::*;

impl Parser<WithSource<StmtFunctionDef>> for PythonParser {
    type Output = Method;
    fn parse(&self, input: WithSource<StmtFunctionDef>, config: &Config) -> Result<Self::Output> {
        let function = self.function_parser.parse(input, config)?;
        let attributes = function.attributes;
        let visibility = function.visibility;
        let synchrony = function.synchrony;
        let identifier = function.identifier;
        let inputs = function.inputs;
        let output = function.output;
        let mutability = Mutability::Mutable;
        Ok(Self::Output { attributes, visibility, synchrony, mutability, identifier, inputs, output })
    }
}

impl Parser<WithSource<StmtAsyncFunctionDef>> for PythonParser {
    type Output = Method;
    fn parse(&self, input: WithSource<StmtAsyncFunctionDef>, config: &Config) -> Result<Self::Output> {
        let function = self.function_parser.parse(input, config)?;
        let attributes = function.attributes;
        let visibility = function.visibility;
        let synchrony = function.synchrony;
        let identifier = function.identifier;
        let inputs = function.inputs;
        let output = function.output;
        let mutability = Mutability::Mutable;
        Ok(Self::Output { attributes, visibility, synchrony, mutability, identifier, inputs, output })
    }
}
