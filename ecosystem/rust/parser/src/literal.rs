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
    fn to_tokens(&self, tokens: &mut TokenStream) {
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
    use super::Literal;
    use ligen_parsing::Parser;
    use crate::literal::LiteralParser;
    use crate::prelude::*;

    #[test]
    fn literal_verbatim() -> Result<()> {
        let lit = syn::Lit::Verbatim(proc_macro2::Literal::string("verbatim"));
        let literal = LiteralParser.parse(lit)?;
        assert_eq!(literal, Literal::String("\"verbatim\"".into()));
        Ok(())
    }

    #[test]
    fn literal_string() -> Result<()> {
        let literal = LiteralParser.parse(quote! { "value" })?;
        assert_eq!(literal, Literal::String("value".into()));
        Ok(())
    }

    #[test]
    fn literal_byte() -> Result<()> {
        let literal = LiteralParser.parse(quote! { b'A' })?;
        assert_eq!(literal, Literal::UnsignedInteger(b'A' as u64));
        Ok(())
    }

    #[test]
    fn literal_byte_str() -> Result<()> {
        let literal = LiteralParser.parse(quote! { b"bytestr" })?;
        assert_eq!(literal, Literal::String("bytestr".into()));
        Ok(())
    }

    #[test]
    fn literal_bool() -> Result<()> {
        let literal = LiteralParser.parse(quote! { true })?;
        assert_eq!(literal, Literal::Boolean(true));
        Ok(())
    }

    #[test]
    fn literal_char() -> Result<()> {
        let literal = LiteralParser.parse(quote! { 'a' })?;
        assert_eq!(literal, Literal::Character('a'));
        Ok(())
    }

    #[test]
    fn literal_integer() -> Result<()> {
        let literal = LiteralParser.parse(quote! { -2 })?;
        assert_eq!(literal, Literal::Integer(-2));
        Ok(())
    }

    #[test]
    fn literal_float() -> Result<()> {
        let literal = LiteralParser.parse(quote! { 3.5 })?;
        assert_eq!(literal, Literal::Float(3.5));
        Ok(())
    }
}
