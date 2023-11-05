use crate::prelude::*;
use ligen::parsing::parser::ParserConfig;
use rustpython_parser::ast::{Arguments, Expr, Stmt, StmtAsyncFunctionDef, StmtFunctionDef};
use ligen::ir::{Function, Synchrony, Visibility, Parameter, Type};
use crate::function::DynamicParser;
use crate::function::parameter::ParameterParser;
use crate::identifier::IdentifierParser;
use crate::macro_attributes::attributes::AttributesParser;
use crate::types::type_::TypeParser;


#[derive(Default)]
pub struct FullParser;

impl DynamicParser<'_> for FullParser {}

impl Parser<&str> for FullParser {
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

impl Parser<WithSource<StmtFunctionDef>> for FullParser {
    type Output = Function;
    fn parse(&self, input: WithSource<StmtFunctionDef>, config: &ParserConfig) -> Result<Self::Output> {
        let attributes = AttributesParser::default().parse(input.sub(input.ast.decorator_list.clone()), config)?;
        let visibility = Visibility::Public;
        let synchrony = Synchrony::Synchronous;
        let identifier = IdentifierParser::new().parse(input.ast.name.as_str(), config)?;
        let inputs = self.parse_inputs(*input.ast.args, config)?;
        let output = self.parse_output(input.ast.returns, config)?;

        Ok(Self::Output { attributes, visibility, synchrony, identifier, inputs, output })
    }
}

impl Parser<WithSource<StmtAsyncFunctionDef>> for FullParser {
    type Output = Function;
    fn parse(&self, input: WithSource<StmtAsyncFunctionDef>, config: &ParserConfig) -> Result<Self::Output> {
        let source = input.source;
        let input = input.ast;
        let attributes = AttributesParser::default().parse(WithSource::new(source, input.decorator_list), config)?;
        let visibility = Visibility::Public;
        let synchrony = Synchrony::Asynchronous;
        let identifier = IdentifierParser::new().parse(input.name.as_str(), config)?;
        let inputs = self.parse_inputs(*input.args, config)?;
        let output = self.parse_output(input.returns, config)?;
        Ok(Self::Output { attributes, visibility, synchrony, identifier, inputs, output })
    }
}

impl FullParser {
    fn parse_inputs(&self, args: Arguments, config: &ParserConfig) -> Result<Vec<Parameter>> {
        let mut parameters = Vec::new();
        for arg in args.args {
            parameters.push(ParameterParser.parse(arg, config)?);
        }
        Ok(parameters)
    }

    fn parse_output(&self, output: Option<Box<Expr>>, config: &ParserConfig) -> Result<Option<Type>> {
        if let Some(expr) = output.and_then(|expr| expr.name_expr()) {
            Ok(Some(TypeParser::default().parse(&expr, config)?))
        } else {
            Ok(None)
        }
    }
}