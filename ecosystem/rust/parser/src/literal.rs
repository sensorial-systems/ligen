use ligen_ir::Literal;
use ligen_parsing::Parser;
use crate::prelude::*;

pub struct LiteralParser;

impl Parser<syn::Lit> for LiteralParser {
    type Output = Literal;
    fn parse(&self, lit: syn::Lit) -> Result<Self::Output> {
        Ok(match lit {
            syn::Lit::Str(litstr) => Self::Output::String(litstr.value()),
            syn::Lit::Verbatim(litverb) => Self::Output::String(litverb.to_string()),
            syn::Lit::ByteStr(litbytestr) => Self::Output::String(String::from_utf8_lossy(&litbytestr.value()).into_owned()),
            syn::Lit::Byte(litbyte) => Self::Output::UnsignedInteger(litbyte.value() as u64),
            syn::Lit::Char(litchar) => Self::Output::Character(litchar.value()),
            syn::Lit::Int(litint) => Self::Output::Integer(litint.base10_parse().unwrap()),
            syn::Lit::Float(litfloat) => Self::Output::Float(litfloat.base10_parse().unwrap()),
            syn::Lit::Bool(litbool) => Self::Output::Boolean(litbool.value),
        })
    }
}

impl Parser<syn::Ident> for LiteralParser {
    type Output = Literal;
    fn parse(&self, input: syn::Ident) -> Result<Self::Output> {
        Ok(Self::Output::String(input.to_string()))
    }
}

impl Parser<proc_macro::TokenStream> for LiteralParser {
    type Output = Literal;
    fn parse(&self, input: proc_macro::TokenStream) -> Result<Self::Output> {
        self.parse(proc_macro2::TokenStream::from(input))
    }
}

impl Parser<proc_macro2::TokenStream> for LiteralParser {
    type Output = Literal;
    fn parse(&self, input: proc_macro2::TokenStream) -> Result<Self::Output> {
        syn::parse2::<syn::Lit>(input)
            .map_err(|e| Error::Message(format!("Failed to parse literal: {:?}", e)))
            .and_then(|literal| self.parse(literal))
    }
}

impl ToTokens for Literal {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        match self.clone() {
            Literal::String(x) => {
                let y = proc_macro2::Literal::string(&x);
                tokens.append_all(quote! {#y})
            }
            Literal::Boolean(x) => {
                let y = proc_macro2::Ident::new(&x.to_string(), proc_macro2::Span::call_site());
                tokens.append_all(quote! {#y})
            }
            Literal::Character(x) => {
                let y = proc_macro2::Literal::character(x);
                tokens.append_all(quote! {#y})
            }
            Literal::Integer(x) => {
                let y = proc_macro2::Literal::i64_unsuffixed(x);
                tokens.append_all(quote! {#y})
            }
            Literal::UnsignedInteger(x) => {
                let y = proc_macro2::Literal::u64_unsuffixed(x);
                tokens.append_all(quote! {#y})
            }
            Literal::Float(x) => {
                let y = proc_macro2::Literal::f64_unsuffixed(x);
                tokens.append_all(quote! {#y})
            }
        };
    }
}

#[cfg(test)]
mod test {
    use crate::literal::LiteralParser;
    use crate::prelude::*;
    use ligen_ir::literal::mock;
    use ligen_parsing::assert::*;

    #[test]
    fn literal_verbatim() -> Result<()> {
        assert_eq(LiteralParser, mock::literal_verbatim(), syn::Lit::Verbatim(proc_macro2::Literal::string("verbatim")))
    }

    #[test]
    fn literal_string() -> Result<()> {
        assert_eq(LiteralParser, mock::literal_string(), quote!{
            "string"
        })
    }

    #[test]
    fn literal_byte_str() -> Result<()> {
        assert_eq(LiteralParser, mock::literal_string(), quote!{
            b"string"
        })
    }

    #[test]
    fn literal_byte() -> Result<()> {
        assert_eq(LiteralParser, mock::literal_byte(), quote!{
            b'A'
        })
    }

    #[test]
    fn literal_bool() -> Result<()> {
        assert_eq(LiteralParser, mock::literal_bool(), quote!{
            false
        })
    }

    #[test]
    fn literal_character() -> Result<()> {
        assert_eq(LiteralParser, mock::literal_character(), quote!{
            'A'
        })
    }

    #[test]
    fn literal_integer() -> Result<()> {
        assert_eq(LiteralParser, mock::literal_integer(), quote!{
            -2
        })
    }

    #[test]
    fn literal_float() -> Result<()> {
        assert_eq(LiteralParser, mock::literal_float(), quote!{
            3.5
        })
    }
}
