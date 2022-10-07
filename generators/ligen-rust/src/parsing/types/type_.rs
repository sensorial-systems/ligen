use crate::{Atomic, Reference, ReferenceKind, Generics, Mutability, Type};
use crate::prelude::*;
use syn::{TypePath, TypePtr, TypeReference};

impl From<SynPath> for Type {
    fn from(SynPath(path): SynPath) -> Self {
        if Atomic::is_atomic(SynPath(path.clone())) {
            Self::Atomic(SynPath(path).into())
        } else {
            let generics = path
                .segments
                .last()
                .map(|segment| Generics::from(SynPathArguments(segment.arguments.clone())))
                .unwrap_or_default();
            Self::Compound(SynPath(path).into(), generics)
        }
    }
}

impl TryFrom<SynType> for Type {
    type Error = Error;
    fn try_from(SynType(syn_type): SynType) -> Result<Self> {
        if let syn::Type::Path(TypePath { path, .. }) = syn_type {
            Ok(SynPath(path).into())
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
                    let mutability = if mutability.is_none() { Mutability::Constant } else { Mutability::Mutable };
                    let type_ = Box::new(SynPath(path).into());
                    Ok(Self::Reference(Reference { kind, mutability, type_, }))
                } else {
                    Err(Error::Message("Couldn't find path".into()))
                }
            } else {
                Err(Error::Message("Only Path, Reference and Ptr Types are currently supported".into()))
            }
        }
    }
}

impl ToTokens for Type {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        match &self {
            Type::Atomic(atomic) => tokens.append_all(atomic.to_token_stream()),
            Type::Compound(compound, generics) => {
                tokens.append_all(compound.to_token_stream());
                tokens.append_all(generics.to_token_stream());
            },
            Type::Reference(reference) => tokens.append_all(reference.to_token_stream()),
        }
    }
}

impl Display for Type {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let display = match &self {
            Type::Atomic(atomic)               => format!("{}", (atomic as &dyn Display)),
            Type::Compound(compound, generics) => format!("{}{}", (compound as &dyn Display), (generics as &dyn Display)),
            Type::Reference(reference)         => format!("{}", (reference as &dyn Display)),
        };
        f.write_str(&display)
    }
}

#[cfg(test)]
mod test {
    use std::convert::TryInto;

    use quote::quote;
    use syn::parse_quote::parse;

    use crate::{Float, Integer, Mutability, ReferenceKind};
    use crate::prelude::SynType;

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
                SynType(parse::<syn::Type>(x))
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
                SynType(parse::<syn::Type>(x))
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
            SynType(parse::<syn::Type>(quote! {bool}))
                .try_into()
                .expect("Failed to convert from syn::Type")
        );
    }

    #[test]
    fn types_character() {
        assert_eq!(
            Type::Atomic(Character),
            SynType(parse::<syn::Type>(quote! {char}))
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
                    mutability: Mutability::Constant,
                    type_: Box::new(
                        Type::Atomic(
                            Atomic::Integer(
                                Integer::I32
                            )
                        )
                    )
                }
            ),
            SynType(parse::<syn::Type>(quote! {&i32}))
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
                    mutability: Mutability::Mutable,
                    type_: Box::new(
                        Type::Atomic(
                            Atomic::Integer(
                                Integer::I32
                            )
                        )
                    )
                }
            ),
            SynType(parse::<syn::Type>(quote! {&mut i32}))
                .try_into()
                .expect("Failed to convert from syn::Type")
        );
    }

    #[test]
    fn types_pointer_constant() {
        assert_eq!(
            Type::Reference(Reference {
                kind: ReferenceKind::Pointer,
                mutability: Mutability::Constant,
                type_: Box::new(
                    Type::Atomic(
                        Atomic::Integer(
                            Integer::I32
                        )
                    )
                )
            }),
            SynType(parse::<syn::Type>(quote! {*const i32}))
                .try_into()
                .expect("Failed to convert from syn::Type")
        );
    }

    #[test]
    fn types_pointer_mutable() {
        assert_eq!(
            Type::Reference(Reference {
                kind: ReferenceKind::Pointer,
                mutability: Mutability::Mutable,
                type_: Box::new(
                    Type::Atomic(
                        Atomic::Integer(
                            Integer::I32
                        )
                    )
                )
            }),
            SynType(parse::<syn::Type>(quote! {*mut i32}))
                .try_into()
                .expect("Failed to convert from syn::Type")
        );
    }
}
