use crate::ir::Type;
use quote::{ToTokens, quote, TokenStreamExt};
use proc_macro2::TokenStream;

/// Reference kind.
#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
pub enum ReferenceKind {
    /// Borrow reference, denoted with &.
    Borrow,
    /// Pointer reference, denoted with *.
    Pointer
}

/// Reference representation.
#[derive(Debug, Hash, PartialEq, Eq, Clone)]
pub struct Reference {
    /// Indicates the reference kind.
    pub kind: ReferenceKind,
    /// Indicate constness.
    pub is_constant: bool,
    /// The type being referenced.
    pub type_: Box<Type>
}

impl std::fmt::Display for Reference {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self.kind {
            ReferenceKind::Pointer => {
                if self.is_constant {
                    f.write_str("*const ")?;
                } else {
                    f.write_str("*mut ")?;
                }
            },
            ReferenceKind::Borrow => {
                if self.is_constant {
                    f.write_str("&")?;
                } else {
                    f.write_str("&mut ")?;
                }
            }
        }
        f.write_str(&self.type_.to_string())
    }
}

impl ToTokens for Reference {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        match self.kind {
            ReferenceKind::Pointer => {
                if self.is_constant {
                    tokens.append_all(quote! {*const })
                } else {
                    tokens.append_all(quote! {*mut })
                }
            },
            ReferenceKind::Borrow => {
                if self.is_constant {
                    tokens.append_all(quote! {&})
                } else {
                    tokens.append_all(quote! {&mut })
                }
            }
        }
        let type_ = &self.type_;
        tokens.append_all(quote! {#type_});
    }
}