use ligen::ir::Generics;
use ligen::parsing::parser::Parser;
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
