use syn::{Ident, Lit};

/// Literal Enum
#[derive(Debug, PartialEq)]
pub enum Literal {
    /// String variant
    String(String),
    /// Bool variant
    Bool(bool),
    /// Char variant
    Char(char),
    /// Integer variant
    Integer(i64),
    /// UnsignedInteger variant
    UnsignedInteger(u64),
    /// Float variant
    Float(f64),
}

impl From<Lit> for Literal {
    fn from(lit: Lit) -> Self {
        match lit {
            Lit::Str(litstr) => Self::String(litstr.value()),
            Lit::Verbatim(litverb) => Self::String(litverb.to_string()),
            Lit::ByteStr(litbytestr) => {
                Self::String(String::from_utf8_lossy(&litbytestr.value()).into_owned())
            }
            Lit::Byte(litbyte) => Self::UnsignedInteger(litbyte.value() as u64),
            Lit::Char(litchar) => Self::UnsignedInteger(litchar.value() as u64),
            Lit::Int(litint) => Self::Integer(litint.base10_parse().unwrap()),
            Lit::Float(litfloat) => Self::Float(litfloat.base10_parse().unwrap()),
            Lit::Bool(litbool) => Self::Bool(litbool.value),
        }
    }
}

impl From<Ident> for Literal {
    fn from(ident: Ident) -> Self {
        Self::String(ident.to_string())
    }
}

impl std::fmt::Display for Literal {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Literal::String(value) => write!(f, "{}", value),
            Literal::Bool(value) => write!(f, "{}", value),
            Literal::Char(value) => write!(f, "{}", value),
            Literal::Integer(value) => write!(f, "{}", value),
            Literal::UnsignedInteger(value) => write!(f, "{}", value),
            Literal::Float(value) => write!(f, "{}", value),
        }
    }
}

#[cfg(test)]
mod test {
    use super::Literal;
    use quote::quote;
    use syn::parse_quote::parse;

    #[test]
    fn literal_string() {
        let tokenstream = quote! { "value" };
        let lit: syn::Lit = parse(tokenstream);
        let literal: Literal = lit.into();
        if let Literal::String(value) = literal {
            assert_eq!(value, "value");
        }
    }

    #[test]
    fn literal_verbatim() {
        let lit = syn::Lit::Verbatim(proc_macro2::Literal::string("verbatim"));
        let literal: Literal = lit.into();
        if let Literal::String(value) = literal {
            assert_eq!(value, "\"verbatim\"");
        }
    }

    #[test]
    fn literal_byte() {
        let tokenstream = quote! { b'A' };
        let lit: syn::Lit = parse(tokenstream);
        let literal: Literal = lit.into();
        if let Literal::UnsignedInteger(value) = literal {
            assert_eq!(value, b'A' as u64);
        }
    }

    #[test]
    fn literal_byte_str() {
        let tokenstream = quote! { b"bytestr" };
        let lit: syn::Lit = parse(tokenstream);
        let literal: Literal = lit.into();
        if let Literal::String(value) = literal {
            assert_eq!(value, "bytestr");
        }
    }

    #[test]
    fn literal_bool() {
        let tokenstream = quote! { true };
        let lit: syn::Lit = parse(tokenstream);
        let literal: Literal = lit.into();
        if let Literal::Bool(value) = literal {
            assert_eq!(value, true);
        }
    }

    #[test]
    fn literal_char() {
        let tokenstream = quote! { 'a' };
        let lit: syn::Lit = parse(tokenstream);
        let literal: Literal = lit.into();
        if let Literal::Char(value) = literal {
            assert_eq!(value, 'a');
        }
    }

    #[test]
    fn literal_integer() {
        let tokenstream = quote! { 2 };
        let lit: syn::Lit = parse(tokenstream);
        let literal: Literal = lit.into();
        if let Literal::Integer(value) = literal {
            assert_eq!(value, 2);
        }
    }

    #[test]
    fn literal_unsigned_integer() {
        let tokenstream = quote! { 2 };
        let lit: syn::Lit = parse(tokenstream);
        let literal: Literal = lit.into();
        if let Literal::UnsignedInteger(value) = literal {
            assert_eq!(value, 2);
        }
    }

    #[test]
    fn literal_float() {
        let tokenstream = quote! { 2.0 };
        let lit: syn::Lit = parse(tokenstream);
        let literal: Literal = lit.into();
        if let Literal::Float(value) = literal {
            assert_eq!(value, 2.0);
        }
    }
}
