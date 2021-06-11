use crate::ir::{Atomic, Identifier, Reference, ReferenceKind};
use proc_macro2::TokenStream;
use quote::{ToTokens, TokenStreamExt};
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

impl From<syn::Path> for Type {
    fn from(path: syn::Path) -> Self {
        if Atomic::is_atomic(path.segments[0].ident.clone().to_string().as_str()) {
            Self::Atomic(path.into())
        } else {
            Self::Compound(path.segments[0].ident.clone().into())
        }
    }
}

impl TryFrom<syn::Type> for Type {
    type Error = &'static str;
    fn try_from(syn_type: syn::Type) -> Result<Self, Self::Error> {
        if let syn::Type::Path(TypePath { path, .. }) = syn_type {
            Ok(path.into())
        } else {
            let reference = match syn_type {
                syn::Type::Reference(TypeReference {
                    elem, mutability, ..
                }) => Some((ReferenceKind::Borrow, elem, mutability)),
                syn::Type::Ptr(TypePtr {
                    elem, mutability, ..
                }) => Some((ReferenceKind::Pointer, elem, mutability)),
                _ => None,
            };
            if let Some((kind, elem, mutability)) = reference {
                if let syn::Type::Path(TypePath { path, .. }) = *elem {
                    let is_constant = mutability.is_none();
                    let type_ = Box::new(path.into());
                    Ok(Self::Reference(Reference {
                        kind,
                        is_constant,
                        type_,
                    }))
                } else {
                    Err("Couldn't find path")
                }
            } else {
                Err("Only Path, Reference and Ptr Types are currently supported")
            }
        }
    }
}

impl ToTokens for Type {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        match &self {
            Type::Atomic(atomic) => tokens.append_all(atomic.to_token_stream()),
            Type::Compound(compound) => tokens.append_all(compound.to_token_stream()),
            Type::Reference(reference) => tokens.append_all(reference.to_token_stream()),
        }
    }
}

impl Type {
    /// Checks if type is not a built-in rust type and returns it.
    pub fn is_dep(&self) -> Option<&Identifier> {
        if let Self::Compound(ident) = self {
            match ident.name.as_str() {
                // TODO: Add all built-in types
                "String" | "Self" | "Vec" | "Box" => None,
                _ => Some(ident),
            }
        } else {
            None
        }
    }
}
