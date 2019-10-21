use quote::quote;
use quote::{TokenStreamExt, ToTokens};

use crate::{Type, Inputs, Output, Identifier, Attribute};

pub struct Method {
    pub attribute : Attribute,
    pub owner: Type,
    pub identifier: Identifier,
    pub inputs : Inputs,
    pub output: Output,
    pub code : Option<proc_macro2::TokenStream>
}

impl Method {
    pub fn new(owner : Type, attribute : Attribute, identifier : Identifier, inputs : Inputs, output : Output, code : Option<proc_macro2::TokenStream>) -> Self {
        Self {
            attribute,
            owner,
            identifier,
            inputs,
            output,
            code
        }
    }

    pub fn parse(owner: Type, method: syn::ImplItemMethod) -> Self {
        let attribute = Attribute::parse_attributes(&method.attrs);
        let identifier = Identifier::parse(&method.sig.ident);
        let inputs = Inputs::parse(&owner, &method.sig.inputs);
        let output = Output::parse(&method.sig.output);

        Self {
            attribute,
            owner,
            identifier,
            inputs,
            output,
            code : None
        }
    }

    pub fn get_extern_name(&self) -> String {
        format!("{}_{}", self.owner.identifier.name, self.identifier.name)
    }
}

impl ToTokens for Method {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let (method_output, return_value) = self.output.get_tokens();
        let (parameters, _args, method_call) = self.inputs.get_tokens(&self);

        let function_identifier = proc_macro2::Ident::new(&self.get_extern_name(), proc_macro2::Span::call_site());

        let default_code = quote! {
            let value = #method_call;
            #return_value
        };

        let code = match &self.code {
                Some(code) => code,
                None => &default_code
        };

        let signature = quote! {
            pub unsafe extern fn #function_identifier(#parameters) #method_output
        };

        tokens.append_all(quote!{
            #[no_mangle]
            #signature {
                #code
            }
        });
    }
}
