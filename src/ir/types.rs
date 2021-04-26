use proc_macro2::Ident;
use syn::{Path, TypePath, TypeReference};

use crate::ir::Identifier;

#[derive(Debug, Copy, Clone, PartialEq)]
/// Integer Enum
pub enum Integer {
    /// u8 variant
    U8,
    /// u16 variant
    U16,
    /// u32 variant
    U32,
    /// u64 variant
    U64,
    /// u128 variant
    U128,
    /// usize variant
    USize,
    /// i8 variant
    I8,
    /// i16 variant
    I16,
    /// i32 variant
    I32,
    /// i64 variant
    I64,
    /// i128 variant
    I128,
    /// isize variant
    ISize,
}

#[derive(Debug, PartialEq, Copy, Clone)]
/// Float Enum
pub enum Float {
    /// f32 variant
    F32,
    /// f64 variant
    F64,
}

#[derive(Debug, PartialEq, Clone, Copy)]
/// Atomic Enum
pub enum Atomic {
    /// Integer variant
    Integer(Integer),
    /// Float variant
    Float(Float),
    /// Boolean variant
    Boolean,
    /// Character variant
    Character,
}

#[derive(Debug, PartialEq)]
// TODO: Find better name for enum
/// Atom Enum
pub enum Atom {
    /// Atomic variant
    Atomic(Atomic),
    /// Compound variant
    Compound(Identifier),
}

#[derive(Debug, PartialEq)]
/// Borrowed Enum
pub enum Borrowed {
    /// Shared variant
    Shared(Atom),
    /// Exclusive variant
    Exclusive(Atom),
}

#[derive(Debug, PartialEq)]
/// Type Enum
pub enum Type {
    /// Owned variant
    Owned(Atom),
    /// Borrowed variant
    Borrowed(Borrowed),
}

impl From<Ident> for Atom {
    fn from(ident: Ident) -> Self {
        println!("ident: {:#?}", ident);
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
            _ => panic!("Unknown Ident"),
        }
    }
}

impl From<Path> for Atom {
    fn from(path: Path) -> Self {
        match path {
            Path { segments, .. } => Self::from(segments[0].ident.clone()),
        }
    }
}

impl From<syn::Type> for Type {
    fn from(syn_type: syn::Type) -> Self {
        println!("syn_type: {:#?}", syn_type);
        match syn_type {
            syn::Type::Path(TypePath { path, .. }) => Self::Owned(Atom::from(path)),
            syn::Type::Reference(TypeReference {
                elem, mutability, ..
            }) => {
                if let syn::Type::Path(TypePath { path, .. }) = *elem {
                    match mutability {
                        Some(_m) => Self::Borrowed(Borrowed::Exclusive(Atom::from(path))),
                        None => Self::Borrowed(Borrowed::Shared(Atom::from(path))),
                    }
                } else {
                    panic!("Unknown type");
                }
            }

            _ => panic!("Unknown Type"),
        }
    }
}

#[cfg(test)]
mod test {

    use super::{Atom, Atomic, Borrowed, Float, Integer, Type};
    use quote::quote;
    use syn::parse_quote::parse;

    #[test]
    fn types_array() {
        let a: Type = parse::<syn::Type>(quote! {Vec<i32>}).into();
    }

    #[test]
    fn types_integer() {
        let vec: Vec<Type> = vec![
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
        .map(|x| parse::<syn::Type>(x).into())
        .collect();
        let expected: Vec<Type> = vec![
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
        .map(|x| Type::Owned(Atom::Atomic(Atomic::Integer(x))))
        .collect();

        let mut iter = vec.iter().zip(expected.iter());

        while let Some((value, expected_value)) = iter.next() {
            assert_eq!(value, expected_value);
        }
    }

    #[test]
    fn types_float() {
        let vec: Vec<Type> = vec![quote! { f32 }, quote! { f64 }]
            .into_iter()
            .map(|x| parse::<syn::Type>(x).into())
            .collect();
        let expected: Vec<Type> = vec![Float::F32, Float::F64]
            .into_iter()
            .map(|x| Type::Owned(Atom::Atomic(Atomic::Float(x))))
            .collect();

        let mut iter = vec.iter().zip(expected.iter());

        while let Some((value, expected_value)) = iter.next() {
            assert_eq!(value, expected_value);
        }
    }

    #[test]
    fn types_boolean() {
        assert_eq!(
            Type::Owned(Atom::Atomic(Atomic::Boolean)),
            parse::<syn::Type>(quote! {bool}).into()
        );
    }

    #[test]
    fn types_character() {
        assert_eq!(
            Type::Owned(Atom::Atomic(Atomic::Character)),
            parse::<syn::Type>(quote! {char}).into()
        );
    }

    #[test]
    fn types_borrowed_shared() {
        assert_eq!(
            Type::Borrowed(Borrowed::Shared(Atom::Atomic(Atomic::Integer(
                Integer::I32
            )))),
            parse::<syn::Type>(quote! {&i32}).into()
        );
    }

    #[test]
    fn types_borrowed_exclusive() {
        assert_eq!(
            Type::Borrowed(Borrowed::Exclusive(Atom::Atomic(Atomic::Integer(
                Integer::I32
            )))),
            parse::<syn::Type>(quote! {&mut i32}).into()
        );
    }
}
