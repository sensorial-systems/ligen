use ligen_ir::Generics;
use ligen_parsing::Parser;
use crate::prelude::*;
use crate::types::type_::TypeParser;

pub struct GenericsParser;

impl Parser<syn::PathArguments> for GenericsParser {
    type Output = Generics;
    fn parse(&self, input: syn::PathArguments) -> Result<Self::Output> {
        let types = match input {
            syn::PathArguments::AngleBracketed(arguments) => {
                arguments
                    .args
                    .into_iter()
                    .filter_map(|generic| match generic {
                        syn::GenericArgument::Type(type_) => Some(TypeParser.parse(type_).expect("Failed to parse generic type.")),
                        _ => None
                    })
                    .collect()
            },
            _ => Default::default()
        };
        Ok(Self::Output { types })
    }
}

impl ToTokens for Generics {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        if !self.types.is_empty() {
            tokens.append_separated(self.types.iter().map(|x| x.to_token_stream()), quote! {,})
        }
    }
}
