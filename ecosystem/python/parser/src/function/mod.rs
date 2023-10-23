use rustpython_parser::ast::{StmtAsyncFunctionDef, StmtFunctionDef};
use ligen::ir::Function;
use crate::function::full_parser::FullParser;
use crate::function::symbol_parser::SymbolParser;
use crate::prelude::*;

pub mod parameter;
pub mod method;

mod symbol_parser;
mod full_parser;

trait DynamicParser<'a>:
  Parser<WithSource<StmtFunctionDef>, Output = Function>
+ Parser<WithSource<StmtAsyncFunctionDef>, Output = Function>
+ Parser<&'a str, Output = Function>
{}

pub struct FunctionParser {
    parser: Box<dyn for<'a> DynamicParser<'a>>
}

impl Default for FunctionParser {
    fn default() -> Self {
        let parser = Box::new(FullParser::default());
        Self { parser }
    }
}

impl Parser<WithSource<StmtFunctionDef>> for FunctionParser {
    type Output = Function;
    fn parse(&self, input: WithSource<StmtFunctionDef>) -> Result<Self::Output> {
        self.parser.parse(input)
    }
}

impl Parser<WithSource<StmtAsyncFunctionDef>> for FunctionParser {
    type Output = Function;
    fn parse(&self, input: WithSource<StmtAsyncFunctionDef>) -> Result<Self::Output> {
        self.parser.parse(input)
    }
}

impl Parser<&str> for FunctionParser {
    type Output = Function;
    fn parse(&self, input: &str) -> Result<Self::Output> {
        self.parser.parse(input)
    }
}

impl FunctionParser {
    pub fn full() -> Self {
        Self::default()
    }

    pub fn symbol() -> Self {
        let parser = Box::new(SymbolParser::default());
        Self { parser }
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