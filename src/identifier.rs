use quote::quote;
use quote::{TokenStreamExt, ToTokens};

#[derive(Clone)]
pub struct Identifier {
    pub name : String
}

impl Identifier {
    pub fn parse(ident: &syn::Ident) -> Identifier {
        Identifier {
            name: ident.to_string()
        }
    }
}

impl ToTokens for Identifier {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let identifier = proc_macro2::Ident::new(&format!("{}", self.name), proc_macro2::Span::call_site());
        tokens.append_all(quote!{
            #identifier
        });
    }
}
