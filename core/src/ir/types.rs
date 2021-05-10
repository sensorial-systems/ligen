use crate::ir::Atomic;
use crate::ir::Identifier;
use proc_macro2::TokenStream;
use quote::{quote, ToTokens, TokenStreamExt};
use std::convert::TryFrom;
use syn::{TypePath, TypePtr, TypeReference};

#[derive(Debug, PartialEq, Clone)]
/// Type Enum
pub enum Type {
    /// Atomic variant
    Atomic(Atomic),
    /// Compound variant
    Compound(Identifier),
    /// Reference variant
    Reference(Reference),
}

#[derive(Debug, PartialEq, Clone)]
/// Reference Enum
pub enum Reference {
    /// Borrow variant
    Borrow(Borrow),
    /// Pointer variant
    Pointer(Pointer),
}

#[derive(Debug, PartialEq, Clone)]
/// Borrow Enum
pub enum Borrow {
    /// Constant variant
    Constant(Box<Type>),
    /// Mutable variant
    Mutable(Box<Type>),
}

#[derive(Debug, PartialEq, Clone)]
/// Pointer Enum
pub enum Pointer {
    /// Constant variant
    Constant(Box<Type>),
    /// Mutable variant
    Mutable(Box<Type>),
}

impl From<syn::Path> for Type {
    fn from(path: syn::Path) -> Self {
        match path.clone() {
            syn::Path { segments, .. } => match segments[0].ident.clone().to_string().as_str() {
                "u8" | "u16" | "u32" | "u64" | "u128" | "usize" | "i8" | "i16" | "i32" | "i64"
                | "i128" | "isize" | "f32" | "f64" | "bool" | "char" => Self::Atomic(path.into()),
                _ => Self::Compound(segments[0].ident.clone().into()),
            },
        }
    }
}

impl TryFrom<syn::Type> for Type {
    type Error = &'static str;
    fn try_from(syn_type: syn::Type) -> Result<Self, Self::Error> {
        match syn_type {
            syn::Type::Path(TypePath { path, .. }) => Ok(path.into()),
            syn::Type::Reference(TypeReference {
                elem, mutability, ..
            }) => {
                if let syn::Type::Path(TypePath { path, .. }) = *elem {
                    match mutability {
                        Some(_m) => Ok(Self::Reference(Reference::Borrow(Borrow::Mutable(
                            Box::new(path.into()),
                        )))),
                        None => Ok(Self::Reference(Reference::Borrow(Borrow::Constant(
                            Box::new(path.into()),
                        )))),
                    }
                } else {
                    Err("Couldn't find path")
                }
            }
            syn::Type::Ptr(TypePtr {
                elem, mutability, ..
            }) => {
                if let syn::Type::Path(TypePath { path, .. }) = *elem {
                    match mutability {
                        Some(_m) => Ok(Self::Reference(Reference::Pointer(Pointer::Mutable(
                            Box::new(path.into()),
                        )))),
                        None => Ok(Self::Reference(Reference::Pointer(Pointer::Constant(
                            Box::new(path.into()),
                        )))),
                    }
                } else {
                    Err("Couldn't find path")
                }
            }

            _ => Err("Only Path, Reference and Ptr Types are currently supported"),
        }
    }
}

impl ToTokens for Type {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        match &self {
            Type::Atomic(atomic) => tokens.append_all(atomic.to_token_stream()),
            Type::Compound(compound) => tokens.append_all(compound.to_token_stream()),
            Type::Reference(reference) => match reference {
                Reference::Borrow(borrow) => match borrow {
                    Borrow::Constant(constant) => {
                        let typ = &**constant;
                        let type_tokens = typ.to_token_stream();
                        tokens.append_all(quote! {&#type_tokens});
                    }
                    Borrow::Mutable(mutable) => {
                        let typ = &**mutable;
                        let type_tokens = typ.to_token_stream();
                        tokens.append_all(quote! {&mut #type_tokens});
                    }
                },
                Reference::Pointer(pointer) => match pointer {
                    Pointer::Constant(constant) => {
                        let typ = &**constant;
                        let type_tokens = typ.to_token_stream();
                        tokens.append_all(quote! {&#type_tokens});
                    }
                    Pointer::Mutable(mutable) => {
                        let typ = &**mutable;
                        let type_tokens = typ.to_token_stream();
                        tokens.append_all(quote! {&mut #type_tokens});
                    }
                },
            },
        }
    }
}

#[cfg(test)]
mod test {
    use super::{
        Atomic::{self, Boolean, Character},
        Borrow, Pointer, Reference, Type,
    };
    use crate::ir::{Float, Integer};
    use quote::quote;
    use std::convert::TryInto;
    use syn::parse_quote::parse;

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
