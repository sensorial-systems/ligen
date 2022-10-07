use crate::prelude::*;

pub mod integer;
pub mod float;

pub use integer::*;
pub use float::*;
use crate::{Primitive, Float, Integer};

impl TryFrom<SynIdent> for Primitive {
    type Error = Error;
    fn try_from(SynIdent(ident): SynIdent) -> Result<Self> {
        match ident.to_string().as_str() {
            "u8" => Ok(Self::Integer(Integer::U8)),
            "u16" => Ok(Self::Integer(Integer::U16)),
            "u32" => Ok(Self::Integer(Integer::U32)),
            "u64" => Ok(Self::Integer(Integer::U64)),
            "u128" => Ok(Self::Integer(Integer::U128)),
            "usize" => Ok(Self::Integer(Integer::USize)),
            "i8" => Ok(Self::Integer(Integer::I8)),
            "i16" => Ok(Self::Integer(Integer::I16)),
            "i32" => Ok(Self::Integer(Integer::I32)),
            "i64" => Ok(Self::Integer(Integer::I64)),
            "i128" => Ok(Self::Integer(Integer::I128)),
            "isize" => Ok(Self::Integer(Integer::ISize)),
            "c_char" => Ok(Self::Integer(Integer::I8)),
            "c_uchar" => Ok(Self::Integer(Integer::U8)),
            "f32" => Ok(Self::Float(Float::F32)),
            "f64" => Ok(Self::Float(Float::F64)),
            "bool" => Ok(Self::Boolean),
            "char" => Ok(Self::Character),
            _ => Err(Error::Message("Unknown Ident".into())),
        }
    }
}

// TODO: Why is it not a TryFrom?
impl From<SynPath> for Primitive {
    fn from(SynPath(path): SynPath) -> Self {
        match path {
            syn::Path { segments, .. } => {
                Self::try_from(SynIdent(segments.last().unwrap().ident.clone())).expect("Failed to convert from Ident")
            }
        }
    }
}

impl ToTokens for Primitive {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        match &self {
            Primitive::Integer(integer) => integer.to_tokens(tokens),
            Primitive::Float(float) => float.to_tokens(tokens),
            Primitive::Boolean => tokens.append_all(quote! {bool}),
            Primitive::Character => tokens.append_all(quote! {char}),
        }
    }
}

#[cfg(test)]
mod test {

    use std::convert::TryInto;

    use super::{Primitive, Float, Integer};
    use quote::quote;
    use syn::parse_quote::parse;
    use crate::prelude::SynIdent;

    #[test]
    fn primitive_integer() {
        let vec: Vec<Primitive> = vec![
            quote! { u8 },
            quote! { u16 },
            quote! { u32 },
            quote! { u64 },
            quote! { u128 },
            quote! { usize },
            quote! { i8 },
            quote! { i16 },
            quote! { i32 },
            quote! { i64 },
            quote! { i128 },
            quote! { isize },
        ]
            .into_iter()
            .map(|x| SynIdent(parse::<syn::Ident>(x)).try_into().expect("Failed to parse"))
            .collect();
        let expected: Vec<Integer> = vec![
            Integer::U8,
            Integer::U16,
            Integer::U32,
            Integer::U64,
            Integer::U128,
            Integer::USize,
            Integer::I8,
            Integer::I16,
            Integer::I32,
            Integer::I64,
            Integer::I128,
            Integer::ISize,
        ]
            .into_iter()
            .collect();

        let mut iter = vec.iter().zip(expected.iter());

        while let Some((Primitive::Integer(value), expected_value)) = iter.next() {
            assert_eq!(value, expected_value);
        }
    }

    #[test]
    fn primitive_float() {
        let vec: Vec<Primitive> = vec![quote! { f32 }, quote! { f64 }]
            .into_iter()
            .map(|x| SynIdent(parse::<syn::Ident>(x)).try_into().expect("Failed to parse"))
            .collect();
        let expected: Vec<Float> = vec![Float::F32, Float::F64].into_iter().collect();

        let mut iter = vec.iter().zip(expected.iter());

        while let Some((Primitive::Float(value), expected_value)) = iter.next() {
            assert_eq!(value, expected_value);
        }
    }

    #[test]
    fn primitive_boolean() {
        assert_eq!(
            Primitive::Boolean,
            SynIdent(parse::<syn::Ident>(quote! {bool}))
                .try_into()
                .expect("Failed to parse")
        );
    }

    #[test]
    fn primitive_character() {
        assert_eq!(
            Primitive::Character,
            SynIdent(parse::<syn::Ident>(quote! {char}))
                .try_into()
                .expect("Failed to parse")
        );
    }
}
