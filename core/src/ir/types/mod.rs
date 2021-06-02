mod type_;
mod borrow;
mod reference;
mod pointer;
mod atomic;

pub use type_::*;
pub use borrow::*;
pub use reference::*;
pub use pointer::*;
pub use atomic::*;

// TODO: Can these tests be moved to the modules in the lines above?
#[cfg(test)]
mod test {
    use std::convert::TryInto;

    use quote::quote;
    use syn::parse_quote::parse;

    use crate::ir::{Float, Integer};

    use super::{
        Atomic::{self, Boolean, Character},
        Borrow, Pointer, Reference, Type,
    };

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
        .map(|x| {
            parse::<syn::Type>(x)
                .try_into()
                .expect("Failed to convert from syn::Type")
        })
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
        .map(|x| Type::Atomic(Atomic::Integer(x)))
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
            .map(|x| {
                parse::<syn::Type>(x)
                    .try_into()
                    .expect("Failed to convert from syn::Type")
            })
            .collect();
        let expected: Vec<Type> = vec![Float::F32, Float::F64]
            .into_iter()
            .map(|x| Type::Atomic(Atomic::Float(x)))
            .collect();

        let mut iter = vec.iter().zip(expected.iter());

        while let Some((value, expected_value)) = iter.next() {
            assert_eq!(value, expected_value);
        }
    }

    #[test]
    fn types_boolean() {
        assert_eq!(
            Type::Atomic(Boolean),
            parse::<syn::Type>(quote! {bool})
                .try_into()
                .expect("Failed to convert from syn::Type")
        );
    }

    #[test]
    fn types_character() {
        assert_eq!(
            Type::Atomic(Character),
            parse::<syn::Type>(quote! {char})
                .try_into()
                .expect("Failed to convert from syn::Type")
        );
    }

    #[test]
    fn types_borrow_constant() {
        assert_eq!(
            Type::Reference(Reference::Borrow(Borrow::Constant(Box::new(Type::Atomic(
                Atomic::Integer(Integer::I32)
            ))))),
            parse::<syn::Type>(quote! {&i32})
                .try_into()
                .expect("Failed to convert from syn::Type")
        );
    }

    #[test]
    fn types_borrow_mutable() {
        assert_eq!(
            Type::Reference(Reference::Borrow(Borrow::Mutable(Box::new(Type::Atomic(
                Atomic::Integer(Integer::I32)
            ))))),
            parse::<syn::Type>(quote! {&mut i32})
                .try_into()
                .expect("Failed to convert from syn::Type")
        );
    }

    #[test]
    fn types_pointer_constant() {
        assert_eq!(
            Type::Reference(Reference::Pointer(Pointer::Constant(Box::new(
                Type::Atomic(Atomic::Integer(Integer::I32))
            )))),
            parse::<syn::Type>(quote! {*const i32})
                .try_into()
                .expect("Failed to convert from syn::Type")
        );
    }

    #[test]
    fn types_pointer_mutable() {
        assert_eq!(
            Type::Reference(Reference::Pointer(Pointer::Mutable(Box::new(
                Type::Atomic(Atomic::Integer(Integer::I32))
            )))),
            parse::<syn::Type>(quote! {*mut i32})
                .try_into()
                .expect("Failed to convert from syn::Type")
        );
    }
}
