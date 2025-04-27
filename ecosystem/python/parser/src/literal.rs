use rustpython_parser::ast::{Constant, ExprConstant, Expr};
use ligen::ir::Literal;
use ligen::transformer::prelude::*;
use crate::prelude::*;

#[derive(Default)]
pub struct LiteralParser {}

impl Parser<Literal> for LiteralParser {
    fn parse(&self, input: impl AsRef<str>, config: &Config) -> Result<Literal> {
        let input = input.as_ref();
        if let Ok(integer) = input.parse::<i64>() {
            Ok(Literal::Integer(integer))
        } else {
            ExprConstant::parse(input, "<embedded>")
                .map_err(|e| Error::Message(format!("Failed to parse literal from ExprConstant: {:?}", e)))
                .and_then(|constant| self.transform(&constant, config))
        }
    }
}

impl Transformer<&Constant, Literal> for LiteralParser {
    fn transform(&self, input: &Constant, _config: &Config) -> Result<Literal> {
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
                    result.push(self.transform(element, _config)?);
                }
                Ok(Literal::Tuple(result))
            },
            _ => Err(Error::Message(format!("Failed to parse literal from constant: {:?}", input)))
        }
    }
}

impl Transformer<&ExprConstant, Literal> for LiteralParser {
    fn transform(&self, input: &ExprConstant, config: &Config) -> Result<Literal> {
        self.transform(&input.value, config)
    }
}

impl Transformer<&Expr, Literal> for LiteralParser {
    fn transform(&self, input: &Expr, config: &Config) -> Result<Literal> {
        match input {
            Expr::Constant(constant) => self.transform(constant, config),
            Expr::List(list) => {
                let mut result = Vec::new();
                for element in &list.elts {
                    result.push(self.transform(element, config)?);
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
    use ligen::transformer::assert::*;

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
