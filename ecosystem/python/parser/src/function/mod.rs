pub mod parameter;
pub mod method;

use crate::prelude::*;
use rustpython_parser::ast::{Arguments, Expr, Stmt, StmtAsyncFunctionDef, StmtFunctionDef};
use ligen::idl::{Function, Synchrony, Visibility, Parameter, Type};
use crate::function::parameter::ParameterParser;
use crate::identifier::IdentifierParser;
use crate::macro_attributes::attributes::AttributesParser;
use crate::types::type_::TypeParser;


#[derive(Default)]
pub struct FunctionParser {
    attributes_parser: AttributesParser,
    parameter_parser: ParameterParser,
    type_parser: TypeParser,
    identifier_parser: IdentifierParser,
}

impl Parser<Function> for FunctionParser {
    fn parse(&self, input: impl AsRef<str>, config: &Config) -> Result<Function> {
        let input = input.as_ref();
        let statement = Stmt::parse(input, "<embedded>")
            .map_err(|error| Error::Message(format!("Failed to parse statement: {error}")))?;
        match statement {
            Stmt::FunctionDef(function) => self.transform(WithSource::new(input, function), config),
            Stmt::AsyncFunctionDef(function) => self.transform(WithSource::new(input, function), config),
            _ => Err(Error::Message("No function found".into()))
        }
    }
}

impl Transformer<WithSource<StmtFunctionDef>, Function> for FunctionParser {
    fn transform(&self, input: WithSource<StmtFunctionDef>, config: &Config) -> Result<Function> {
        let identifier = self.identifier_parser.parse(input.ast.name.as_str(), config)?;
        if config.get_only_parse_symbols() {
            Ok(Function { identifier, ..Default::default() })
        } else {
            let attributes = self.attributes_parser.transform(input.sub(&input.ast.decorator_list), config)?;
            let visibility = Visibility::Public;
            let synchrony = Synchrony::Synchronous;
            let inputs = self.parse_inputs(*input.ast.args, config)?;
            let output = self.parse_output(input.ast.returns, config)?;
            let body = Default::default();
            Ok(Function { attributes, visibility, synchrony, identifier, inputs, output, body })    
        }
    }
}

impl Transformer<WithSource<StmtAsyncFunctionDef>, Function> for FunctionParser {
    fn transform(&self, input: WithSource<StmtAsyncFunctionDef>, config: &Config) -> Result<Function> {
        let identifier = self.identifier_parser.parse(input.ast.name.as_str(), config)?;
        if config.get_only_parse_symbols() {
            Ok(Function { identifier, ..Default::default() })
        } else {
            let attributes = self.attributes_parser.transform(input.sub(&input.ast.decorator_list), config)?;
            let visibility = Visibility::Public;
            let synchrony = Synchrony::Asynchronous;
            let inputs = self.parse_inputs(*input.ast.args, config)?;
            let output = self.parse_output(input.ast.returns, config)?;    
            let body = Default::default();
            Ok(Function { attributes, visibility, synchrony, identifier, inputs, output, body })
        }
    }
}

impl FunctionParser {
    fn parse_inputs(&self, args: Arguments, config: &Config) -> Result<Vec<Parameter>> {
        let mut parameters = Vec::new();
        for arg in args.args {
            parameters.push(self.parameter_parser.transform(arg, config)?);
        }
        Ok(parameters)
    }

    fn parse_output(&self, output: Option<Box<Expr>>, config: &Config) -> Result<Option<Type>> {
        if let Some(expr) = output.and_then(|expr| expr.name_expr()) {
            Ok(Some(self.type_parser.transform(&expr, config)?))
        } else {
            Ok(None)
        }
    }
}

#[cfg(test)]
mod test {
    use crate::function::FunctionParser;
    use ligen::prelude::*;
    use ligen::transformer::assert::assert_eq;
    use ligen_idl::function::mock;

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