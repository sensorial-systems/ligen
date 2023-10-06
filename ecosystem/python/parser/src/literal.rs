use rustpython_parser::ast::{Constant, ExprConstant};
use ligen::ir::Literal;
use ligen::parsing::parser::Parser;
use crate::prelude::*;

#[derive(Default)]
pub struct LiteralParser;

impl ligen::parsing::parser::universal::literal::LiteralParser for LiteralParser {}

impl Parser<String> for LiteralParser {
    type Output = Literal;
    fn parse(&self, input: String) -> Result<Self::Output> {
        self.parse(input.as_str())
    }
}

impl Parser<&str> for LiteralParser {
    type Output = Literal;
    fn parse(&self, input: &str) -> Result<Self::Output> {
        if let Ok(integer) = input.parse::<i64>() {
            Ok(Literal::Integer(integer))
        } else {
            ExprConstant::parse(input, "<embedded>")
                .map_err(|e| Error::Message(format!("Failed to parse literal: {:?}", e)))
                .and_then(|constant| self.parse(constant.value))
        }
    }
}

impl Parser<Constant> for LiteralParser {
    type Output = Literal;
    fn parse(&self, input: Constant) -> Result<Self::Output> {
        match input {
            Constant::Bool(bool) => Ok(Literal::Boolean(bool)),
            Constant::Float(float) => Ok(Literal::Float(float)),
            Constant::Str(string) => Ok(Literal::String(string)),
            _ => Err(Error::Message(format!("Failed to parse literal: {:?}", input)))
        }
    }
}

#[cfg(test)]
mod test {
    use crate::literal::LiteralParser;
    use crate::prelude::*;
    use ligen::ir::literal::mock;
    use ligen::parsing::assert::*;

    #[test]
    fn literal_string() -> Result<()> {
        assert_eq(LiteralParser, mock::literal_string(), "\"string\"")
    }

    #[test]
    fn literal_bool() -> Result<()> {
        assert_eq(LiteralParser, mock::literal_bool(), "False")
    }

    #[test]
    fn literal_integer() -> Result<()> {
        assert_eq(LiteralParser, mock::literal_integer(), "-2")
    }

    #[test]
    fn literal_float() -> Result<()> {
        assert_eq(LiteralParser, mock::literal_float(), "3.5")
    }
}