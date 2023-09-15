use crate::Literal;
use crate::prelude::*;

impl From<SynLit> for Literal {
    fn from(SynLit(lit): SynLit) -> Self {
        match lit {
            syn::Lit::Str(litstr) => Self::String(litstr.value()),
            syn::Lit::Verbatim(litverb) => Self::String(litverb.to_string()),
            syn::Lit::ByteStr(litbytestr) => Self::String(String::from_utf8_lossy(&litbytestr.value()).into_owned()),
            syn::Lit::Byte(litbyte) => Self::UnsignedInteger(litbyte.value() as u64),
            syn::Lit::Char(litchar) => Self::UnsignedInteger(litchar.value() as u64),
            syn::Lit::Int(litint) => Self::Integer(litint.base10_parse().unwrap()),
            syn::Lit::Float(litfloat) => Self::Float(litfloat.base10_parse().unwrap()),
            syn::Lit::Bool(litbool) => Self::Boolean(litbool.value),
        }
    }
}

impl From<SynIdent> for Literal {
    fn from(SynIdent(ident): SynIdent) -> Self {
        Self::String(ident.to_string())
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
    use quote::quote;
    use syn::parse_quote::parse;
    use crate::prelude::SynLit;

    #[test]
    fn literal_string() {
        let tokenstream = quote! { "value" };
        let lit: syn::Lit = parse(tokenstream);
        let literal: Literal = SynLit(lit).into();
        if let Literal::String(value) = literal {
            assert_eq!(value, "value");
        }
    }

    #[test]
    fn literal_verbatim() {
        let lit = syn::Lit::Verbatim(proc_macro2::Literal::string("verbatim"));
        let literal: Literal = SynLit(lit).into();
        if let Literal::String(value) = literal {
            assert_eq!(value, "\"verbatim\"");
        }
    }

    #[test]
    fn literal_byte() {
        let tokenstream = quote! { b'A' };
        let lit: syn::Lit = parse(tokenstream);
        let literal: Literal = SynLit(lit).into();
        if let Literal::UnsignedInteger(value) = literal {
            assert_eq!(value, b'A' as u64);
        }
    }

    #[test]
    fn literal_byte_str() {
        let tokenstream = quote! { b"bytestr" };
        let lit: syn::Lit = parse(tokenstream);
        let literal: Literal = SynLit(lit).into();
        if let Literal::String(value) = literal {
            assert_eq!(value, "bytestr");
        }
    }

    #[test]
    fn literal_bool() {
        let tokenstream = quote! { true };
        let lit: syn::Lit = parse(tokenstream);
        let literal: Literal = SynLit(lit).into();
        if let Literal::Boolean(value) = literal {
            assert_eq!(value, true);
        }
    }

    #[test]
    fn literal_char() {
        let tokenstream = quote! { 'a' };
        let lit: syn::Lit = parse(tokenstream);
        let literal: Literal = SynLit(lit).into();
        if let Literal::Character(value) = literal {
            assert_eq!(value, 'a');
        }
    }

    #[test]
    fn literal_integer() {
        let tokenstream = quote! { 2 };
        let lit: syn::Lit = parse(tokenstream);
        let literal: Literal = SynLit(lit).into();
        if let Literal::Integer(value) = literal {
            assert_eq!(value, 2);
        }
    }

    #[test]
    fn literal_unsigned_integer() {
        let tokenstream = quote! { 2 };
        let lit: syn::Lit = parse(tokenstream);
        let literal: Literal = SynLit(lit).into();
        if let Literal::UnsignedInteger(value) = literal {
            assert_eq!(value, 2);
        }
    }

    #[test]
    fn literal_float() {
        let tokenstream = quote! { 2.0 };
        let lit: syn::Lit = parse(tokenstream);
        let literal: Literal = SynLit(lit).into();
        if let Literal::Float(value) = literal {
            assert_eq!(value, 2.0);
        }
    }
}
