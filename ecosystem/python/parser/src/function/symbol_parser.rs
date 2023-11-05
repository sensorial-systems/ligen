use crate::prelude::*;
use ligen::parsing::parser::ParserConfig;
use rustpython_parser::ast::{Stmt, StmtAsyncFunctionDef, StmtFunctionDef};
use ligen::ir::Function;
use crate::function::DynamicParser;
use crate::identifier::IdentifierParser;


#[derive(Default)]
pub struct SymbolParser;

impl DynamicParser<'_> for SymbolParser {}

impl Parser<WithSource<StmtFunctionDef>> for SymbolParser {
    type Output = Function;
    fn parse(&self, input: WithSource<StmtFunctionDef>, config: &ParserConfig) -> Result<Self::Output> {
        let identifier = IdentifierParser::new().parse(input.ast.name.as_str(), config)?;
        Ok(Self::Output { identifier, ..Default::default() })
    }
}

impl Parser<WithSource<StmtAsyncFunctionDef>> for SymbolParser {
    type Output = Function;
    fn parse(&self, input: WithSource<StmtAsyncFunctionDef>, config: &ParserConfig) -> Result<Self::Output> {
        let identifier = IdentifierParser::new().parse(input.ast.name.as_str(), config)?;
        Ok(Self::Output { identifier, ..Default::default() })
    }
}

impl Parser<&str> for SymbolParser {
    type Output = Function;
    fn parse(&self, input: &str, config: &ParserConfig) -> Result<Self::Output> {
        let statement = Stmt::parse(input, "<embedded>")
            .map_err(|error| Error::Message(format!("Failed to parse statement: {}", error)))?;
        match statement {
            Stmt::FunctionDef(function) => self.parse(WithSource::new(input, function), config),
            Stmt::AsyncFunctionDef(function) => self.parse(WithSource::new(input, function), config),
            _ => Err(Error::Message("No function found".into()))
        }
    }
}
