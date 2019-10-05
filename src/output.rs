use quote::quote;
use std::ops::Deref;
use proc_macro2::TokenStream;

use crate::ty::Type;

pub struct Output {
    pub typ: Option<Type>
}

impl Output {
    pub fn new(typ : Option<Type>) -> Self {
        Self {
            typ
        }
    }

    pub fn parse(ret_type: &syn::ReturnType) -> Output {
        let typ = match ret_type {
            syn::ReturnType::Default => None,
            syn::ReturnType::Type(_, box_ty) => {
                Some(Type::parse(&(box_ty.deref())))
            }
        };
        Output {
            typ
        }
    }

    pub fn get_tokens(&self) -> (TokenStream, TokenStream) {
        let (method_output, output_is_atomic) = match &self.typ {
            Some(typ) => (quote!{-> #typ}, typ.is_atomic()),
            None => (quote!{}, true)
        };
        let return_value = match output_is_atomic {
            true => quote! { value },
            false => quote! {
                Box::into_raw(
                    Box::new(
                        value
                    )
                )
            }
        };
        (method_output, return_value)
    }
}
