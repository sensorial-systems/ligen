use ligen_ir::{Atomic, Reference, ReferenceKind, Generics, Type};
use crate::prelude::*;
use syn::{TypePath, TypePtr, TypeReference};
use crate::traits::AsRust;

impl From<syn::Path> for Type {
    fn from(path: syn::Path) -> Self {
        if Atomic::is_atomic(path.clone()) {
            Self::Atomic(path.into())
        } else {
            let generics = path
                .segments
                .last()
                .map(|segment| Generics::from(segment.arguments.clone()))
                .unwrap_or_default();
            Self::Compound(path.into(), generics)
        }
    }
}

impl TryFrom<syn::Type> for Type {
    type Error = Error;
    fn try_from(syn_type: syn::Type) -> Result<Self> {
        if let syn::Type::Path(TypePath { path, .. }) = syn_type {
            Ok(path.into())
        } else {
            let reference = match &syn_type {
                syn::Type::Reference(TypeReference {
                    elem, mutability, ..
                }) => Some((ReferenceKind::Borrow, elem, mutability)),
                syn::Type::Ptr(TypePtr {
                    elem, mutability, ..
                }) => Some((ReferenceKind::Pointer, elem, mutability)),
                _ => None,
            };
            if let Some((kind, elem, mutability)) = reference {
                if let syn::Type::Path(TypePath { path, .. }) = *elem.clone() {
                    let is_constant = mutability.is_none();
                    let type_ = Box::new(path.into());
                    Ok(Self::Reference(Reference {
                        kind,
                        is_constant,
                        type_,
                    }))
                } else {
                    Err(Error::Message("Couldn't find path".into()))
                }
            } else {
                Err(Error::Message("Only Path, Reference and Ptr Types are currently supported".into()))
            }
        }
    }
}

impl AsRust for Type {
    fn as_rust(&self) -> String {
        match &self {
            Type::Atomic(atomic)               => format!("{}", atomic),
            Type::Compound(compound, generics) => format!("{}{}", compound, generics),
            Type::Reference(reference)         => format!("{}", reference),
        }
    }
}

#[cfg(test)]
mod test {
    use std::convert::TryInto;

    use quote::quote;
    use syn::parse_quote::parse;

    use crate::{Float, Integer, ReferenceKind};

    use super::{
        Atomic::{self, Boolean, Character},
        Reference, Type,
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
            Type::Reference(
                Reference {
                    kind: ReferenceKind::Borrow,
                    is_constant: true,
                    type_: Box::new(
                        Type::Atomic(
                            Atomic::Integer(
                                Integer::I32
                            )
                        )
                    )
                }
            ),
            parse::<syn::Type>(quote! {&i32})
                .try_into()
                .expect("Failed to convert from syn::Type")
        );
    }

    #[test]
    fn types_borrow_mutable() {
        assert_eq!(
            Type::Reference(
                Reference {
                    kind: ReferenceKind::Borrow,
                    is_constant: false,
                    type_: Box::new(
                        Type::Atomic(
                            Atomic::Integer(
                                Integer::I32
                            )
                        )
                    )
                }
            ),
            parse::<syn::Type>(quote! {&mut i32})
                .try_into()
                .expect("Failed to convert from syn::Type")
        );
    }

    #[test]
    fn types_pointer_constant() {
        assert_eq!(
            Type::Reference(Reference {
                kind: ReferenceKind::Pointer,
                is_constant: true,
                type_: Box::new(
                    Type::Atomic(
                        Atomic::Integer(
                            Integer::I32
                        )
                    )
                )
            }),
            parse::<syn::Type>(quote! {*const i32})
                .try_into()
                .expect("Failed to convert from syn::Type")
        );
    }

    #[test]
    fn types_pointer_mutable() {
        assert_eq!(
            Type::Reference(Reference {
                kind: ReferenceKind::Pointer,
                is_constant: false,
                type_: Box::new(
                    Type::Atomic(
                        Atomic::Integer(
                            Integer::I32
                        )
                    )
                )
            }),
            parse::<syn::Type>(quote! {*mut i32})
                .try_into()
                .expect("Failed to convert from syn::Type")
        );
    }
}
