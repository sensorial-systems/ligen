use rustpython_parser::ast::{Constant, ExprConstant, Expr};
use ligen::ir::Literal;
use ligen::parser::{Parser, ParserConfig};
use crate::prelude::*;

#[derive(Default)]
pub struct LiteralParser {}

impl Parser<String> for LiteralParser {
    type Output = Literal;
    fn parse(&self, input: String, config: &ParserConfig) -> Result<Self::Output> {
        self.parse(input.as_str(), config)
    }
}

impl Parser<&str> for LiteralParser {
    type Output = Literal;
    fn parse(&self, input: &str, config: &ParserConfig) -> Result<Self::Output> {
        if let Ok(integer) = input.parse::<i64>() {
            Ok(Literal::Integer(integer))
        } else {
            ExprConstant::parse(input, "<embedded>")
                .map_err(|e| Error::Message(format!("Failed to parse literal from ExprConstant: {:?}", e)))
                .and_then(|constant| self.parse(&constant, config))
        }
    }
}

impl Parser<&Constant> for LiteralParser {
    type Output = Literal;
    fn parse(&self, input: &Constant, _config: &ParserConfig) -> Result<Self::Output> {
        match input {
            Constant::Bool(bool) => Ok(Literal::Boolean(*bool)),
            Constant::Float(float) => Ok(Literal::Float(*float)),
            Constant::Str(string) => Ok(Literal::String(string.clone())),
            Constant::Int(big_int) => Ok(
                Literal::Integer(
                    big_int
                        .try_into()
                        .map_err(|_| Error::Message("Failed to convert BigInt to usize".into()))?
                    )
                ),
            Constant::None => Ok(Literal::None),
            Constant::Tuple(tuple) => {
                let mut result = Vec::new();
                for element in tuple {
                    result.push(self.parse(element, _config)?);
                }
                Ok(Literal::Tuple(result))
            },
            _ => Err(Error::Message(format!("Failed to parse literal from constant: {:?}", input)))
        }
    }
}

impl Parser<&ExprConstant> for LiteralParser {
    type Output = Literal;
    fn parse(&self, input: &ExprConstant, config: &ParserConfig) -> Result<Self::Output> {
        self.parse(&input.value, config)
    }
}

impl Parser<&Expr> for LiteralParser {
    type Output = Literal;
    fn parse(&self, input: &Expr, config: &ParserConfig) -> Result<Self::Output> {
        match input {
            Expr::Constant(constant) => self.parse(constant, config),
            Expr::List(list) => {
                let mut result = Vec::new();
                for element in &list.elts {
                    result.push(self.parse(element, config)?);
                }
                Ok(Literal::Vector(result))
            },
            _ => Ok(Literal::Unknown("Unimplemented".into()))
        }
    }
}

#[cfg(test)]
mod test {
    use crate::literal::LiteralParser;
    use crate::prelude::*;
    use ligen::ir::literal::mock;
    use ligen::parser::assert::*;

    #[test]
    fn literal_string() -> Result<()> {
        assert_eq(LiteralParser::default(), mock::literal_string(), "\"string\"")
    }

    #[test]
    fn literal_bool() -> Result<()> {
        assert_eq(LiteralParser::default(), mock::literal_bool(), "False")
    }

    #[test]
    fn literal_integer() -> Result<()> {
        assert_eq(LiteralParser::default(), mock::literal_integer(), "-2")
    }

    #[test]
    fn literal_float() -> Result<()> {
        assert_eq(LiteralParser::default(), mock::literal_float(), "3.5")
    }
}
