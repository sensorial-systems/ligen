use crate::Type;
use crate::prelude::*;

/// Generic arguments list.
#[derive(Debug, Hash, PartialEq, Eq, Clone, Default, Serialize, Deserialize)]
pub struct Generics {
    /// Generic types.
    pub types: Vec<Type>
}

impl From<syn::PathArguments> for Generics {
    fn from(from: syn::PathArguments) -> Self {
        let types = match from {
            syn::PathArguments::AngleBracketed(arguments) => {
                arguments
                    .args
                    .into_iter()
                    .filter_map(|generic| match generic {
                        syn::GenericArgument::Type(type_) => type_.try_into().ok(),
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
            tokens.append_separated(self.types.iter(), quote! {,})
        }
    }
}

impl std::fmt::Display for Generics {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        if self.types.is_empty() {
            f.write_str("")
        } else {
            let generics = self
                .types
                .iter()
                .map(|generic| format!("{}", generic))
                .collect::<Vec<String>>()
                .join(", ");
            f.write_str(&format!("<{}>", generics))
        }
    }
}
