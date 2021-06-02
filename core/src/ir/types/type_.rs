use crate::ir::{Atomic, Identifier, Reference, Borrow, Pointer};
use std::convert::TryFrom;
use syn::{TypePath, TypeReference, TypePtr};
use quote::{ToTokens, quote, TokenStreamExt};
use proc_macro2::TokenStream;

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
