pub mod parameter;

use crate::prelude::*;
use rustpython_parser::ast::{Arguments, Expr, Ranged, Stmt, StmtAsyncFunctionDef, StmtFunctionDef};
use ligen::ir::{Function, Synchrony, Visibility, Parameter, Type, Attributes};
use crate::function::parameter::ParameterParser;
use crate::identifier::IdentifierParser;
use crate::macro_attributes::attributes::AttributesParser;
use crate::types::type_::TypeParser;

pub struct FunctionParser;

impl Parser<&str> for FunctionParser {
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

impl Parser<WithSource<StmtFunctionDef>> for FunctionParser {
    type Output = Function;
    fn parse(&self, input: WithSource<StmtFunctionDef>) -> Result<Self::Output> {
        let attributes = self.parse_attributes(input.sub(input.ast.decorator_list.clone()))?;
        let visibility = Visibility::Public;
        let synchrony = Synchrony::Synchronous;
        let identifier = IdentifierParser::new().parse(input.ast.name.as_str())?;
        let inputs = self.parse_inputs(*input.ast.args)?;
        let output = self.parse_output(input.ast.returns)?;

        Ok(Self::Output { attributes, visibility, synchrony, identifier, inputs, output })
    }
}

impl Parser<WithSource<StmtAsyncFunctionDef>> for FunctionParser {
    type Output = Function;
    fn parse(&self, input: WithSource<StmtAsyncFunctionDef>) -> Result<Self::Output> {
        let source = input.source;
        let input = input.ast;
        let attributes = self.parse_attributes(WithSource::new(source, input.decorator_list))?;
        let visibility = Visibility::Public;
        let synchrony = Synchrony::Asynchronous;
        let identifier = IdentifierParser::new().parse(input.name.as_str())?;
        let inputs = self.parse_inputs(*input.args)?;
        let output = self.parse_output(input.returns)?;
        Ok(Self::Output { attributes, visibility, synchrony, identifier, inputs, output })
    }
}

impl FunctionParser {
    fn parse_attributes(&self, attributes: WithSource<Vec<Expr>>) -> Result<Attributes> {
        let source = if attributes.ast.is_empty() {
            Default::default()
        } else {
            attributes.source[attributes.ast.first().unwrap().start().to_usize()..attributes.ast.last().unwrap().end().to_usize()].to_string()
        };
        AttributesParser::default().parse(source)
    }
    fn parse_inputs(&self, args: Arguments) -> Result<Vec<Parameter>> {
        let mut parameters = Vec::new();
        for arg in args.args {
            parameters.push(ParameterParser.parse(arg)?);
        }
        Ok(parameters)
    }

    fn parse_output(&self, output: Option<Box<Expr>>) -> Result<Option<Type>> {
        if let Some(expr) = output.and_then(|expr| expr.name_expr()) {
            Ok(Some(TypeParser.parse(expr)?))
        } else {
            Ok(None)
        }
    }
}

#[cfg(test)]
mod test {
    use crate::function::FunctionParser;
    use ligen::prelude::*;
    use ligen::parsing::assert::assert_eq;
    use ligen_ir::function::mock;

    #[test]
    fn function() -> Result<()> {
        assert_eq(FunctionParser, mock::function(), "def test(): pass")
    }

    #[test]
    fn function_async() -> Result<()> {
        assert_eq(FunctionParser, mock::function_async(), "async def test(): pass")
    }

    #[test]
    fn function_input() -> Result<()> {
        assert_eq(FunctionParser, mock::function_input(), "def test(a: int, b: int): pass")
    }

    #[test]
    fn function_input_output() -> Result<()> {
        assert_eq(FunctionParser, mock::function_input_output(), "def test(a: int, b: int) -> int: pass")
    }

    #[test]
    fn function_attribute() -> Result<()> {
        assert_eq(FunctionParser, mock::function_attribute(), "@test(a = 'b')\ndef test(): pass")?;
        assert_eq(FunctionParser, mock::function_attribute(), "@test(a = \"b\")\ndef test(): pass")
    }
}