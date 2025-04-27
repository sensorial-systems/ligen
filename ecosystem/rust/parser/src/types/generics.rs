use ligen::ir::Generics;
use ligen::parser::prelude::*;
use crate::types::type_::TypeParser;

#[derive(Default)]
pub struct GenericsParser {
    type_parser: TypeParser,
}

impl GenericsParser {
    pub fn new() -> Self {
        Default::default()
    }
}

impl Transformer<syn::PathArguments, Generics> for GenericsParser {
    fn transform(&self, input: syn::PathArguments, config: &Config) -> Result<Generics> {
        let types = match input {
            syn::PathArguments::AngleBracketed(arguments) => {
                arguments
                    .args
                    .into_iter()
                    .filter_map(|generic| match generic {
                        syn::GenericArgument::Type(type_) => Some(type_),
                        _ => None
                    })
                    .map(|type_| self.type_parser.transform(type_, config))
                    .collect::<Result<Vec<_>>>()?
            },
            _ => Default::default()
        };
        Ok(Generics { types })
    }
}

impl Transformer<syn::Generics, Generics> for GenericsParser {
    fn transform(&self, input: syn::Generics, config: &Config) -> Result<Generics> {
        let mut generics = Generics::default();
        for generic in input.params {
            if let syn::GenericParam::Type(type_) = generic {
                generics.types.push(self.type_parser.transform(type_.ident, config)?);
            }
        }
        Ok(generics)
    }
}
