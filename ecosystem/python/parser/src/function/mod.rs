use ligen::parsing::dynamic_parser;
use rustpython_parser::ast::{StmtAsyncFunctionDef, StmtFunctionDef};
use ligen::ir::Function;
use crate::prelude::*;

pub mod parameter;
pub mod method;

mod full_parser;
mod symbol_parser;

dynamic_parser!{
    FunctionParser,
    full_parser::FullParser,
    symbol_parser::SymbolParser,
    Function,
    WithSource<StmtFunctionDef>,
    WithSource<StmtAsyncFunctionDef>,
    &str | &'a str
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