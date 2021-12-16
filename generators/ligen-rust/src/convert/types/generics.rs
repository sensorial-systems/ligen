use crate::prelude::*;
use ligen_ir::Generics;
use crate::traits::AsRust;

impl From<syn::PathArguments> for Generics {
    fn from(from: syn::PathArguments) -> Self {
        let types = match from.0 {
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

impl AsRust for Generics {
    fn as_rust(&self) -> String {
        if self.types.is_empty() {
            "".into()
        } else {
            let generics = self
                .types
                .iter()
                .map(|generic| format!("{}", generic.as_rust()))
                .collect::<Vec<String>>()
                .join(", ");
            format!("<{}>", generics)
        }
    }
}
