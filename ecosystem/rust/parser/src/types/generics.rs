use ligen::ir::Generics;
use ligen::parser::{Parser, ParserConfig};
use crate::prelude::*;
use crate::types::type_::TypeParser;

#[derive(Default)]
pub struct GenericsParser {}

impl GenericsParser {
    pub fn new() -> Self {
        Default::default()
    }
}

impl Parser<syn::PathArguments> for GenericsParser {
    type Output = Generics;
    fn parse(&self, input: syn::PathArguments, config: &ParserConfig) -> Result<Self::Output> {
        let types = match input {
            syn::PathArguments::AngleBracketed(arguments) => {
                arguments
                    .args
                    .into_iter()
                    .filter_map(|generic| match generic {
                        syn::GenericArgument::Type(type_) => Some(TypeParser::new().parse(type_, config).expect("Failed to parse generic type.")),
                        _ => None
                    })
                    .collect()
            },
            _ => Default::default()
        };
        Ok(Self::Output { types })
    }
}

impl Parser<syn::Generics> for GenericsParser {
    type Output = Generics;
    fn parse(&self, input: syn::Generics, config: &ParserConfig) -> Result<Self::Output> {
        let mut generics = Generics::default();
        for generic in input.params {
            if let syn::GenericParam::Type(type_) = generic {
                generics.types.push(TypeParser::new().parse(type_.ident, config)?);
            }
        }
        Ok(generics)
    }
}
