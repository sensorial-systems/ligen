use proc_macro2::Ident;

use crate::ir::Identifier;

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Integer {
    U8,
    U16,
    U32,
    U64,
    U128,
    USize,
    I8,
    I16,
    I32,
    I64,
    I128,
    ISize,
}

#[derive(Debug, PartialEq)]
pub enum Float {
    F32,
    F64,
}

#[derive(Debug, PartialEq)]
pub enum Atomic {
    Integer(Integer),
    Float(Float),
    Boolean,
    Character,
}

#[derive(Debug, PartialEq)]
pub enum Type {
    Atomic(Atomic),
    Compound(Identifier),
}

pub enum Borrowed {
    Exclusive(Type),
    Shared(Type),
}

pub enum TypeOwnership {
    Owned(Type),
    Borrowed(Borrowed),
}

impl From<Ident> for Type {
    fn from(ident: Ident) -> Self {
        match ident.to_string().as_str() {
            "u8" => Self::Atomic(Atomic::Integer(Integer::U8)),
            "u16" => Self::Atomic(Atomic::Integer(Integer::U16)),
            "u32" => Self::Atomic(Atomic::Integer(Integer::U32)),
            "u64" => Self::Atomic(Atomic::Integer(Integer::U64)),
            "u128" => Self::Atomic(Atomic::Integer(Integer::U128)),
            "usize" => Self::Atomic(Atomic::Integer(Integer::USize)),
            "i8" => Self::Atomic(Atomic::Integer(Integer::I8)),
            "i16" => Self::Atomic(Atomic::Integer(Integer::I16)),
            "i32" => Self::Atomic(Atomic::Integer(Integer::I32)),
            "i64" => Self::Atomic(Atomic::Integer(Integer::I64)),
            "i128" => Self::Atomic(Atomic::Integer(Integer::I128)),
            "isize" => Self::Atomic(Atomic::Integer(Integer::ISize)),
            "f32" => Self::Atomic(Atomic::Float(Float::F32)),
            "f64" => Self::Atomic(Atomic::Float(Float::F64)),
            "bool" => Self::Atomic(Atomic::Boolean),
            "char" => Self::Atomic(Atomic::Character),
            _ => Self::Atomic(Atomic::Integer(Integer::U64)),
        }
    }
}

#[cfg(test)]
mod test {
    use std::any::type_name;

    use super::{Atomic, Float, Integer, Type};
    use quote::quote;
    use syn::{parse_quote::parse, Ident};

    #[test]
    fn types_integer() {
        let mut vec: Vec<Type> = vec![
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
        .map(|x| parse::<Ident>(x).into())
        .collect();
        let mut expected = vec![
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
        ];

        let mut iter = vec.iter().zip(expected.iter());

        while let Some((Type::Atomic(Atomic::Integer(value)), expected_value)) = iter.next() {
            assert_eq!(value, expected_value);
        }
    }

    #[test]
    fn types_float() {
        let mut vec: Vec<Type> = vec![quote! { f32 }, quote! { f64 }]
            .into_iter()
            .map(|x| parse::<Ident>(x).into())
            .collect();
        let mut expected = vec![Float::F32, Float::F64];

        let mut iter = vec.iter().zip(expected.iter());

        while let Some((Type::Atomic(Atomic::Float(value)), expected_value)) = iter.next() {
            assert_eq!(value, expected_value);
        }
    }

    #[test]
    fn types_boolean() {
        assert_eq!(
            Type::Atomic(Atomic::Boolean),
            parse::<Ident>(quote! {bool}).into()
        );
    }

    #[test]
    fn types_character() {
        assert_eq!(
            Type::Atomic(Atomic::Character),
            parse::<Ident>(quote! {char}).into()
        );
    }
}
