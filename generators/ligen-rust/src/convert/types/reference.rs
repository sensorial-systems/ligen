use ligen_ir::{Type, ReferenceKind};
use crate::prelude::*;
use ligen_ir::Reference;

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