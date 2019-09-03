use quote::quote;
use quote::{TokenStreamExt, ToTokens};

use crate::ty::Type;
use crate::input::{Inputs};
use crate::output::Output;
use crate::identifier::Identifier;

pub struct Method {
    pub owner: Type,
    pub identifier: Identifier,
    pub inputs : Inputs,
    pub output: Output
}

impl Method {
    pub fn parse(owner: Type, method: syn::ImplItemMethod) -> Self {
        let identifier = Identifier::parse(&method.sig.ident);
        let inputs = Inputs::parse(&method.sig.inputs);
        let output = Output::parse(&method.sig.output);

        Self {
            owner,
            identifier,
            inputs,
            output
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
        tokens.append_all(quote!{
            #[no_mangle]
            pub unsafe extern fn #function_identifier(#parameters) #method_output {
                let value = #method_call;
                #return_value
            }
        });
    }
}
