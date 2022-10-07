use crate::Generics;
use crate::prelude::*;

impl From<SynPathArguments> for Generics {
    fn from(SynPathArguments(from): SynPathArguments) -> Self {
        let types = match from {
            syn::PathArguments::AngleBracketed(arguments) => {
                arguments
                    .args
                    .into_iter()
                    .filter_map(|generic| match generic {
                        syn::GenericArgument::Type(type_) => SynType(type_).try_into().ok(),
                        _ => None
                    })
                    .collect()
            },
            _ => Default::default()
        };
        Self { types }
    }
}

impl ToTokens for Generics {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        if !self.types.is_empty() {
            tokens.append_separated(self.types.iter().map(|x| x.to_token_stream()), quote! {,})
        }
    }
}
