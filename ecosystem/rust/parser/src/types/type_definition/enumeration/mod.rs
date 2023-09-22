//! Enumeration representation.

pub mod variant;

use crate::prelude::*;
use ligen_ir::Enumeration;
use ligen_parsing::Parser;
use crate::types::type_definition::enumeration::variant::VariantParser;

pub struct EnumerationParser;

impl Parser<syn::ItemEnum> for EnumerationParser {
    type Output = Enumeration;
    fn parse(&self, enumeration: syn::ItemEnum) -> Result<Self::Output> {
        let mut variants = Vec::new();
        for variant in enumeration.variants {
            variants.push(VariantParser.parse(variant)?);
        }
        Ok(Self::Output { variants })
    }
}

impl Parser<proc_macro::TokenStream> for EnumerationParser {
    type Output = Enumeration;
    fn parse(&self, input: proc_macro::TokenStream) -> Result<Self::Output> {
        self.parse(proc_macro2::TokenStream::from(input))
    }
}

impl Parser<proc_macro2::TokenStream> for EnumerationParser {
    type Output = Enumeration;
    fn parse(&self, input: proc_macro2::TokenStream) -> Result<Self::Output> {
        syn::parse2::<syn::ItemEnum>(input)
            .map_err(|e| Error::Message(format!("Failed to parse enumeration: {:?}", e)))
            .and_then(|enumeration| self.parse(enumeration))
    }
}

#[cfg(test)]
mod tests {
    use ligen_ir::{Enumeration, Variant};
    use ligen_parsing::Parser;
    use crate::prelude::*;
    use crate::types::type_definition::enumeration::EnumerationParser;

    #[test]
    fn enumeration() -> Result<()> {
        let enumeration = quote! {
            enum Enumeration {
                Integer,
                Float,
                Boolean
            }
        };
        assert_eq!(
            EnumerationParser.parse(enumeration)?,
            Enumeration {
                variants: vec! [
                    Variant {
                        attributes: Default::default(),
                        identifier: "Integer".into(),
                    },
                    Variant {
                        attributes: Default::default(),
                        identifier: "Float".into(),
                    },
                    Variant {
                        attributes: Default::default(),
                        identifier: "Boolean".into()
                    }
                ]
            }
        );
        Ok(())
    }
}
