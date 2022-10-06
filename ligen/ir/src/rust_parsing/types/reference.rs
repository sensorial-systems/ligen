use crate::{Mutability, Reference, ReferenceKind};
use crate::prelude::*;

impl std::fmt::Display for Reference {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self.kind {
            ReferenceKind::Pointer => {
                match self.mutability {
                    Mutability::Constant => f.write_str("*const ")?,
                    Mutability::Mutable => f.write_str("*mut ")?
                }
            },
            ReferenceKind::Borrow => {
                match self.mutability {
                    Mutability::Constant => f.write_str("&")?,
                    Mutability::Mutable => f.write_str("&mut ")?
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
                match self.mutability {
                    Mutability::Constant => tokens.append_all(quote! {*const }),
                    Mutability::Mutable => tokens.append_all(quote! {*mut })
                }
            },
            ReferenceKind::Borrow => {
                match self.mutability {
                    Mutability::Constant => tokens.append_all(quote! {&}),
                    Mutability::Mutable => tokens.append_all(quote! {&mut })
                }
            }
        }
        let type_ = &self.type_;
        tokens.append_all(quote! {#type_});
    }
}
