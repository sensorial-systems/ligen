use quote::quote;
use quote::{TokenStreamExt, ToTokens};

use crate::{Method, Type, Attribute, Identifier, Inputs, Output, Reference, Attributes};

pub struct Object {
    pub typ: Type,
    pub methods: Vec<Method>
}

impl Object {
    pub fn parse(impl_: syn::ItemImpl) -> Object {
        let typ = Type::parse(&*impl_.self_ty);

        let mut methods = Vec::new();

        for item in impl_.items {
            match item {
                syn::ImplItem::Method(method) => {
                    if let syn::Visibility::Public(_) = &method.vis {
                        methods.push(Method::parse(typ.clone(), method));
                    }
                },
                _ => ()
            }
        }

        let destroy_attribute = Attribute::Group(Identifier::new(""), Attributes::new());
        let destroy_identifier = Identifier::new("destroy");
        let destroy_inputs = Inputs::new(Some(Type::new(Some(Reference::new(true)), Vec::new(), Identifier::new(&typ.identifier.name))), Vec::new());
        let destroy_output = Output::new(None);
        let destroy_code = Some(quote! { Box::from_raw(self_object); });
        let destroy_method = Method::new(typ.clone(), destroy_attribute, destroy_identifier, destroy_inputs, destroy_output, destroy_code);

        methods.push(destroy_method);

        Object {
            typ,
            methods
        }
    }
}

impl ToTokens for Object {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        for method in &self.methods {
            let method = quote! { #method };
            tokens.append_all(method);
        }
    }
}
