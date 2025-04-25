use ligen::ir::Literal;
use ligen::parser::{Parser, ParserConfig};
use crate::prelude::*;

#[derive(Default)]
pub struct LiteralParser;

impl Parser<syn::Lit> for LiteralParser {
    type Output = Literal;
    fn parse(&self, lit: syn::Lit, _config: &ParserConfig) -> Result<Self::Output> {
        Ok(match lit {
            syn::Lit::Str(litstr) => Self::Output::String(litstr.value()),
            syn::Lit::Verbatim(litverb) => Self::Output::String(litverb.to_string()),
            syn::Lit::ByteStr(litbytestr) => Self::Output::String(String::from_utf8_lossy(&litbytestr.value()).into_owned()),
            syn::Lit::Byte(litbyte) => Self::Output::UnsignedInteger(litbyte.value() as u64),
            syn::Lit::Char(litchar) => Self::Output::Character(litchar.value()),
            syn::Lit::Int(litint) => Self::Output::Integer(litint.base10_parse().unwrap()),
            syn::Lit::Float(litfloat) => Self::Output::Float(litfloat.base10_parse().unwrap()),
            syn::Lit::Bool(litbool) => Self::Output::Boolean(litbool.value),
            syn::Lit::CStr(litcstr) => Self::Output::String(litcstr.value().to_str().unwrap().to_string()),
            _ => return Err(Error::Message("Failed to parse literal".into())),
        })
    }
}

impl Parser<syn::Ident> for LiteralParser {
    type Output = Literal;
    fn parse(&self, input: syn::Ident, _config: &ParserConfig) -> Result<Self::Output> {
        Ok(Self::Output::String(input.to_string()))
    }
}

impl Parser<syn::Expr> for LiteralParser {
    type Output = Literal;
    fn parse(&self, input: syn::Expr, config: &ParserConfig) -> Result<Self::Output> {
        match input {
            syn::Expr::Lit(lit) => self.parse(lit, config),
            _ => Err(Error::Message("Failed to parse literal from expression".into())),
        }
    }
}

impl Parser<syn::ExprLit> for LiteralParser {
    type Output = Literal;
    fn parse(&self, input: syn::ExprLit, config: &ParserConfig) -> Result<Self::Output> {
        self.parse(input.lit, config)
    }
}

impl Parser<proc_macro::TokenStream> for LiteralParser {
    type Output = Literal;
    fn parse(&self, input: proc_macro::TokenStream, config: &ParserConfig) -> Result<Self::Output> {
        self.parse(proc_macro2::TokenStream::from(input), config)
    }
}

impl Parser<proc_macro2::TokenStream> for LiteralParser {
    type Output = Literal;
    fn parse(&self, input: proc_macro2::TokenStream, config: &ParserConfig) -> Result<Self::Output> {
        syn::parse2::<syn::Lit>(input)
            .map_err(|e| Error::Message(format!("Failed to parse literal: {:?}", e)))
            .and_then(|literal| self.parse(literal, config))
    }
}

impl Parser<String> for LiteralParser {
    type Output = Literal;
    fn parse(&self, input: String, config: &ParserConfig) -> Result<Self::Output> {
        self.parse(input.as_str(), config)
    }
}

impl Parser<&str> for LiteralParser {
    type Output = Literal;
    fn parse(&self, input: &str, config: &ParserConfig) -> Result<Self::Output> {
        if let Ok(lit) = syn::parse_str::<syn::Lit>(input) {
            Ok(self.parse(lit, config)?)
        } else {
            Ok(Literal::Unknown(input.to_string()))
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
    fn literal_verbatim() -> Result<()> {
        assert_eq(LiteralParser, mock::literal_verbatim(), syn::Lit::Verbatim(proc_macro2::Literal::string("verbatim")))
    }

    #[test]
    fn literal_string() -> Result<()> {
        assert_eq(LiteralParser, mock::literal_string(), "\"string\"")
    }

    #[test]
    fn literal_byte_str() -> Result<()> {
        assert_eq(LiteralParser, mock::literal_string(), "b\"string\"")
    }

    #[test]
    fn literal_byte() -> Result<()> {
        assert_eq(LiteralParser, mock::literal_byte(), "b'A'")
    }

    #[test]
    fn literal_bool() -> Result<()> {
        assert_eq(LiteralParser, mock::literal_bool(), "false")
    }

    #[test]
    fn literal_character() -> Result<()> {
        assert_eq(LiteralParser, mock::literal_character(), "'A'")
    }

    #[test]
    fn literal_integer() -> Result<()> {
        assert_eq(LiteralParser, mock::literal_integer(), "-2")
    }

    #[test]
    fn literal_float() -> Result<()> {
        assert_eq(LiteralParser, mock::literal_float(), "3.5")
    }

    #[test]
    fn literal_unknown() -> Result<()> {
        assert_eq(LiteralParser, mock::literal_unknown(), ".0") // FIXME: This is actually an expression.
    }
}
