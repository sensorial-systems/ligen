use rustpython_parser::ast::{StmtAsyncFunctionDef, StmtFunctionDef};
use ligen::ir::{Method, Mutability};
use crate::function::FunctionParser;
use crate::prelude::*;

#[derive(Default)]
pub struct MethodParser {
    function_parser: FunctionParser
}

impl MethodParser {
    pub fn full() -> Self {
        Default::default()
    }

    pub fn symbol() -> Self {
        let function_parser = FunctionParser::symbol();
        Self { function_parser }
    }
}

impl Parser<WithSource<StmtFunctionDef>> for MethodParser {
    type Output = Method;
    fn parse(&self, input: WithSource<StmtFunctionDef>) -> Result<Self::Output> {
        let function = self.function_parser.parse(input)?;
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

impl Parser<WithSource<StmtAsyncFunctionDef>> for MethodParser {
    type Output = Method;
    fn parse(&self, input: WithSource<StmtAsyncFunctionDef>) -> Result<Self::Output> {
        let function = self.function_parser.parse(input)?;
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
