use quote::quote;
use quote::{TokenStreamExt, ToTokens};

use crate::identifier::Identifier;

#[derive(Clone, Debug)]
pub struct Reference {
    pub is_mutable : bool
}

impl Reference {
    pub fn new(is_mutable : bool) -> Reference {
        Reference {
            is_mutable
        }
    }
}

#[derive(Clone)]
pub enum TypeModifier {
    Reference(Reference),
    Pointer(Reference),
    None
}

#[derive(Clone)]
pub struct Type {
    pub modifier : TypeModifier,
    pub path: Vec<Identifier>,
    pub identifier: Identifier
}

impl Type {
    pub fn new(modifier : TypeModifier, path : Vec<Identifier>, identifier : Identifier) -> Type {
        Type {
            modifier,
            path,
            identifier
        }
    }

    pub fn parse(ty: &syn::Type) -> Self {
        let ty = match ty {
            syn::Type::Path(path) => Some(Type::parse_path(&path.path)),
            syn::Type::Ptr(ptr) => {
                let mut is_mutable = false;
                if let Some(_mutability) = ptr.mutability {
                    is_mutable = true;
                }
                Some(Type {
                    modifier : TypeModifier::Pointer(Reference::new(is_mutable)),
                    ..Type::parse(&*ptr.elem)
                })
            },
            syn::Type::Reference(reference) => {
                let mut is_mutable = false;
                if let Some(_mutability) = reference.mutability {
                    is_mutable = true;
                }
                Some(Type {
                    modifier : TypeModifier::Reference(Reference::new(is_mutable)),
                    ..Type::parse(&*reference.elem)
                })
            },
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

        Self {
            modifier : TypeModifier::None,
            path,
            identifier
        }
    }

    pub fn is_atomic(&self) -> bool {
        match self.identifier.name.as_ref() {
            "u64" | "u32" | "u16" | "u8" | "i64" | "i32" | "i16" | "i8" | "f32" | "f64" | "bool" | "usize" | "isize" => true,
            _ => false
        }
    }
}

impl ToTokens for Type {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let identifier = &self.identifier;
        let modifier = match &self.modifier {
            TypeModifier::Reference(reference) => if reference.is_mutable { quote! { *mut } } else { quote! { *const } },
            TypeModifier::Pointer(reference) => if reference.is_mutable { quote! { *mut } } else { quote! { *const } },
            TypeModifier::None => if self.is_atomic() { quote! {} } else { quote! { *mut } }
        };
        let mut path = quote! {};
        for identifier in &self.path {
            path = quote! { #path#identifier:: };
        }
        path = quote! { #path#identifier };
        tokens.append_all(quote! {
            #modifier #path
        })

    }
}
