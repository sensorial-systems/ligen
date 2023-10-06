use proc_macro2::Ident;
use crate::prelude::*;

use ligen::ir::{Primitive, Float, Integer};
use ligen::parsing::parser::Parser;

pub struct PrimitiveParser;

impl Parser<syn::Ident> for PrimitiveParser {
    type Output = Primitive;
    fn parse(&self, ident: Ident) -> Result<Self::Output> {
        match ident.to_string().as_str() {
            "u8"      => Ok(Self::Output::Integer(Integer::U8)),
            "u16"     => Ok(Self::Output::Integer(Integer::U16)),
            "u32"     => Ok(Self::Output::Integer(Integer::U32)),
            "u64"     => Ok(Self::Output::Integer(Integer::U64)),
            "u128"    => Ok(Self::Output::Integer(Integer::U128)),
            "usize"   => Ok(Self::Output::Integer(Integer::USize)),
            "i8"      => Ok(Self::Output::Integer(Integer::I8)),
            "i16"     => Ok(Self::Output::Integer(Integer::I16)),
            "i32"     => Ok(Self::Output::Integer(Integer::I32)),
            "i64"     => Ok(Self::Output::Integer(Integer::I64)),
            "i128"    => Ok(Self::Output::Integer(Integer::I128)),
            "isize"   => Ok(Self::Output::Integer(Integer::ISize)),
            "c_char"  => Ok(Self::Output::Integer(Integer::I8)),
            "c_uchar" => Ok(Self::Output::Integer(Integer::U8)),
            "f32"     => Ok(Self::Output::Float(Float::F32)),
            "f64"     => Ok(Self::Output::Float(Float::F64)),
            "bool"    => Ok(Self::Output::Boolean),
            "char"    => Ok(Self::Output::Character),
            _ => Err(Error::Message("Unknown Ident".into())),
        }
    }
}

impl Parser<syn::Path> for PrimitiveParser {
    type Output = Primitive;
    fn parse(&self, path: syn::Path) -> Result<Self::Output> {
        self.parse(path.segments.last().unwrap().ident.clone())
    }
}

impl Parser<proc_macro::TokenStream> for PrimitiveParser {
    type Output = Primitive;
    fn parse(&self, input: proc_macro::TokenStream) -> Result<Self::Output> {
        self.parse(proc_macro2::TokenStream::from(input))
    }
}

impl Parser<proc_macro2::TokenStream> for PrimitiveParser {
    type Output = Primitive;
    fn parse(&self, input: proc_macro2::TokenStream) -> Result<Self::Output> {
        syn::parse2::<syn::Path>(input)
            .map_err(|e| Error::Message(format!("Failed to parse primitive: {}", e)))
            .and_then(|path| self.parse(path))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    use ligen::ir::primitive::mock;
    use ligen::parsing::assert::*;

    #[test]
    fn primitive_integer() -> Result<()> {
        let vec: Vec<Result<Primitive>> = vec![
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
            .map(|x| PrimitiveParser.parse(x))
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

        let iter = vec
            .into_iter()
            .zip(expected.into_iter());

        for (result, expected) in iter {
            core::assert_eq!(result?, Primitive::Integer(expected));
        }
        Ok(())
    }

    #[test]
    fn float32() -> Result<()> {
        assert_eq(PrimitiveParser, mock::float32(), quote! {
            f32
        })
    }

    #[test]
    fn float64() -> Result<()> {
        assert_eq(PrimitiveParser, mock::float64(), quote! {
            f64
        })
    }

    #[test]
    fn boolean() -> Result<()> {
        assert_eq(PrimitiveParser, mock::boolean(), quote! {
            bool
        })
    }

    #[test]
    fn character() -> Result<()> {
        assert_eq(PrimitiveParser, mock::character(), quote! {
            char
        })
    }
}
