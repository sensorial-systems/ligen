use quote::{quote, ToTokens, TokenStreamExt};

extern crate darling;
extern crate syn;

extern crate proc_macro;

use darling::{FromVariant, FromMeta, *};
use syn::{AttributeArgs, ItemFn};
use proc_macro::TokenStream;

/// Identifier structure
#[derive(Clone, Debug, FromMeta, Default)]
pub struct Identifier {
    #[darling(default)]
    /// Name field of Identifier
    pub name: String,
}

impl Identifier {
    /// Create a new Identifier
    pub fn new(name: &str) -> Identifier {
        Identifier {
            name: String::from(name),
        }
    }

    /// Parse Identifier
    pub fn parse(ident: &syn::Ident) -> Identifier {
        Identifier {
            name: ident.to_string(),
        }
    }
}

impl ToTokens for Identifier {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let identifier = proc_macro2::Ident::new(&self.name, proc_macro2::Span::call_site());
        tokens.append_all(quote! {
            #identifier
        });
    }
}
