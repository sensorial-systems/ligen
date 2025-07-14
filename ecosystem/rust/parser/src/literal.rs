use ligen::ir::Literal;
use ligen::transformer::prelude::*;

#[derive(Default)]
pub struct LiteralParser;

impl Transformer<syn::Lit, Literal> for LiteralParser {
    fn transform(&self, lit: syn::Lit, _config: &Config) -> Result<Literal> {
        Ok(match lit {
            syn::Lit::Str(litstr) => Literal::String(litstr.value()),
            syn::Lit::Verbatim(litverb) => Literal::String(litverb.to_string()),
            syn::Lit::ByteStr(litbytestr) => Literal::String(String::from_utf8_lossy(&litbytestr.value()).into_owned()),
            syn::Lit::Byte(litbyte) => Literal::UnsignedInteger(litbyte.value() as u64),
            syn::Lit::Char(litchar) => Literal::Character(litchar.value()),
            syn::Lit::Int(litint) => Literal::Integer(litint.base10_parse().unwrap()),
            syn::Lit::Float(litfloat) => Literal::Float(litfloat.base10_parse().unwrap()),
            syn::Lit::Bool(litbool) => Literal::Boolean(litbool.value),
            syn::Lit::CStr(litcstr) => Literal::String(litcstr.value().to_str().unwrap().to_string()),
            _ => return Err(Error::Message("Failed to parse literal".into())),
        })
    }
}

impl Transformer<syn::Ident, Literal> for LiteralParser {
    fn transform(&self, input: syn::Ident, _config: &Config) -> Result<Literal> {
        Ok(Literal::String(input.to_string()))
    }
}

impl Transformer<syn::Expr, Literal> for LiteralParser {
    fn transform(&self, input: syn::Expr, config: &Config) -> Result<Literal> {
        match input {
            syn::Expr::Lit(lit) => self.transform(lit, config),
            _ => Err(Error::Message("Failed to parse literal from expression".into())),
        }
    }
}

impl Transformer<syn::ExprLit, Literal> for LiteralParser {
    fn transform(&self, input: syn::ExprLit, config: &Config) -> Result<Literal> {
        self.transform(input.lit, config)
    }
}

impl Transformer<proc_macro::TokenStream, Literal> for LiteralParser {
    fn transform(&self, input: proc_macro::TokenStream, config: &Config) -> Result<Literal> {
        self.transform(proc_macro2::TokenStream::from(input), config)
    }
}

impl Transformer<proc_macro2::TokenStream, Literal> for LiteralParser {
    fn transform(&self, input: proc_macro2::TokenStream, config: &Config) -> Result<Literal> {
        syn::parse2::<syn::Lit>(input)
            .map_err(|e| Error::Message(format!("Failed to parse literal: {e:?}")))
            .and_then(|literal| self.transform(literal, config))
    }
}

impl Parser<Literal> for LiteralParser {
    fn parse(&self, input: impl AsRef<str>, config: &Config) -> Result<Literal> {
        let input = input.as_ref();
        if let Ok(lit) = syn::parse_str::<syn::Lit>(input) {
            Ok(self.transform(lit, config)?)
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
    use ligen::transformer::assert::*;

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
