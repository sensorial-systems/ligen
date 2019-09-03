use quote::quote;
use quote::{TokenStreamExt, ToTokens};

use crate::identifier::Identifier;

#[derive(Clone)]
pub struct Type {
    pub is_atomic: bool,
    pub path: Vec<Identifier>,
    pub identifier: Identifier
}

impl Type {
    pub fn parse(ty: &syn::Type) -> Self {
        let ty = match ty {
            syn::Type::Path(path) => Some(Type::parse_path(&path.path)),
            syn::Type::Reference(reference) => Some(Type::parse(&*reference.elem)),
            _ => None
        }.unwrap();
        ty
    }
    pub fn parse_path(path_: &syn::Path) -> Self {
        let mut path = Vec::new();

        for seg in path_.segments.iter() {
            path.push(Identifier::parse(&seg.ident));
        }

        let identifier = path.pop().unwrap();

        let is_atomic = match identifier.name.as_ref() {
            "u64" | "u32" | "u16" | "u8" | "i64" | "i32" | "i16" | "i8" | "f32" | "f64" => true,
            _ => false
        };

        Self {
            is_atomic,
            path,
            identifier
        }
    }
}

impl ToTokens for Type {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let identifier = &self.identifier;
        let modifier = match self.is_atomic {
            true => quote! {},
            false => quote! {
                *mut
            }
        };
        tokens.append_all(quote! {
            #modifier #identifier
        })

    }
}
