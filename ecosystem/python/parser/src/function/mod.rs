pub mod parameter;
pub mod method;

use crate::prelude::*;
use ligen::parser::prelude::*;
use rustpython_parser::ast::{Arguments, Expr, Stmt, StmtAsyncFunctionDef, StmtFunctionDef};
use ligen::ir::{Function, Synchrony, Visibility, Parameter, Type};
use crate::function::parameter::ParameterParser;
use crate::identifier::IdentifierParser;
use crate::macro_attributes::attributes::AttributesParser;
use crate::types::type_::TypeParser;


#[derive(Default)]
pub struct FunctionParser {}

impl Parser<&str> for FunctionParser {
    type Output = Function;
    fn parse(&self, input: &str, config: &Config) -> Result<Self::Output> {
        let statement = Stmt::parse(input, "<embedded>")
            .map_err(|error| Error::Message(format!("Failed to parse statement: {}", error)))?;
        match statement {
            Stmt::FunctionDef(function) => self.parse(WithSource::new(input, function), config),
            Stmt::AsyncFunctionDef(function) => self.parse(WithSource::new(input, function), config),
            _ => Err(Error::Message("No function found".into()))
        }
    }
}

impl Parser<WithSource<StmtFunctionDef>> for FunctionParser {
    type Output = Function;
    fn parse(&self, input: WithSource<StmtFunctionDef>, config: &Config) -> Result<Self::Output> {
        let identifier = IdentifierParser::new().parse(input.ast.name.as_str(), config)?;
        if config.get_only_parse_symbols() {
            Ok(Function { identifier, ..Default::default() })
        } else {
            let attributes = AttributesParser::default().parse(input.sub(&input.ast.decorator_list), config)?;
            let visibility = Visibility::Public;
            let synchrony = Synchrony::Synchronous;
            let inputs = self.parse_inputs(*input.ast.args, config)?;
            let output = self.parse_output(input.ast.returns, config)?;
            Ok(Function { attributes, visibility, synchrony, identifier, inputs, output })    
        }
    }
}

impl Parser<WithSource<StmtAsyncFunctionDef>> for FunctionParser {
    type Output = Function;
    fn parse(&self, input: WithSource<StmtAsyncFunctionDef>, config: &Config) -> Result<Self::Output> {
        let identifier = IdentifierParser::new().parse(input.ast.name.as_str(), config)?;
        if config.get_only_parse_symbols() {
            Ok(Function { identifier, ..Default::default() })
        } else {
            let attributes = AttributesParser::default().parse(input.sub(&input.ast.decorator_list), config)?;
            let visibility = Visibility::Public;
            let synchrony = Synchrony::Asynchronous;
            let inputs = self.parse_inputs(*input.ast.args, config)?;
            let output = self.parse_output(input.ast.returns, config)?;    
            Ok(Function { attributes, visibility, synchrony, identifier, inputs, output })
        }
    }
}

impl FunctionParser {
    fn parse_inputs(&self, args: Arguments, config: &Config) -> Result<Vec<Parameter>> {
        let mut parameters = Vec::new();
        for arg in args.args {
            parameters.push(ParameterParser::default().parse(arg, config)?);
        }
        Ok(parameters)
    }

    fn parse_output(&self, output: Option<Box<Expr>>, config: &Config) -> Result<Option<Type>> {
        if let Some(expr) = output.and_then(|expr| expr.name_expr()) {
            Ok(Some(TypeParser::default().parse(&expr, config)?))
        } else {
            Ok(None)
        }
    }
}

#[cfg(test)]
mod test {
    use crate::function::FunctionParser;
    use ligen::prelude::*;
    use ligen::parser::assert::assert_eq;
    use ligen_ir::function::mock;

    #[test]
    fn function() -> Result<()> {
        assert_eq(FunctionParser::default(), mock::function(), "def test(): pass")
    }

    #[test]
    fn function_async() -> Result<()> {
        assert_eq(FunctionParser::default(), mock::function_async(), "async def test(): pass")
    }

    #[test]
    fn function_input() -> Result<()> {
        assert_eq(FunctionParser::default(), mock::function_input(), "def test(a: int, b: int): pass")
    }

    #[test]
    fn function_input_output() -> Result<()> {
        assert_eq(FunctionParser::default(), mock::function_input_output(), "def test(a: int, b: int) -> int: pass")
    }

    #[test]
    fn function_attribute() -> Result<()> {
        assert_eq(FunctionParser::default(), mock::function_attribute(), "@test(a = 'b')\ndef test(): pass")?;
        assert_eq(FunctionParser::default(), mock::function_attribute(), "@test(a = \"b\")\ndef test(): pass")
    }
}