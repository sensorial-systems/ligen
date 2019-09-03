use quote::quote;
use quote::{TokenStreamExt, ToTokens};

use crate::method::Method;
use crate::ty::Type;

pub struct Object {
    pub ty: Type,
    pub methods: Vec<Method>
}

impl Object {
    pub fn parse(impl_: syn::ItemImpl) -> Object {
        let ty = Type::parse(&*impl_.self_ty);

        let mut methods = Vec::new();

        for item in impl_.items {
            match item {
                syn::ImplItem::Method(method) => {
                    if let syn::Visibility::Public(_) = &method.vis {
                        methods.push(Method::parse(ty.clone(), method));
                    }
                },
                _ => ()
            }
        }

        Object {
            ty,
            methods
        }
    }
}

impl ToTokens for Object {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let identifier = &self.ty.identifier;
        let struct_destroy = proc_macro2::Ident::new(&format!("{}_destroy", self.ty.identifier.name), proc_macro2::Span::call_site());

        tokens.append_all(quote! {
            #[no_mangle]
            pub unsafe extern fn #struct_destroy(object: *mut #identifier) {
                Box::from_raw(object);
            }
        });

        for method in &self.methods {
            tokens.append_all(quote!{
                #method
            });
        }
    }
}
