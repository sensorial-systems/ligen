use crate::prelude::*;
use rustpython_parser::ast::{Stmt, StmtAsyncFunctionDef, StmtFunctionDef};
use ligen::ir::Function;
use crate::function::DynamicParser;
use crate::identifier::IdentifierParser;


#[derive(Default)]
pub struct SymbolParser;

impl DynamicParser<'_> for SymbolParser {}

impl Parser<WithSource<StmtFunctionDef>> for SymbolParser {
    type Output = Function;
    fn parse(&self, input: WithSource<StmtFunctionDef>) -> Result<Self::Output> {
        let identifier = IdentifierParser::new().parse(input.ast.name.as_str())?;
        Ok(Self::Output { identifier, ..Default::default() })
    }
}

impl Parser<WithSource<StmtAsyncFunctionDef>> for SymbolParser {
    type Output = Function;
    fn parse(&self, input: WithSource<StmtAsyncFunctionDef>) -> Result<Self::Output> {
        let identifier = IdentifierParser::new().parse(input.ast.name.as_str())?;
        Ok(Self::Output { identifier, ..Default::default() })
    }
}

impl Parser<&str> for SymbolParser {
    type Output = Function;
    fn parse(&self, input: &str) -> Result<Self::Output> {
        let statement = Stmt::parse(input, "<embedded>")
            .map_err(|error| Error::Message(format!("Failed to parse statement: {}", error)))?;
        match statement {
            Stmt::FunctionDef(function) => self.parse(WithSource::new(input, function)),
            Stmt::AsyncFunctionDef(function) => self.parse(WithSource::new(input, function)),
            _ => Err(Error::Message("No function found".into()))
        }
    }
}
