//! Enumeration variant representation.

use crate::prelude::*;
use ligen::ir::Variant;
use crate::{RustIdentifierParser, RustAttributesParser};

#[derive(Default)]
pub struct RustVariantParser {
    identifier_parser: RustIdentifierParser,
    attributes_parser: RustAttributesParser,
}

impl Transformer<syn::Variant, Variant> for RustVariantParser {
    fn transform(&self, variant: syn::Variant, config: &Config) -> Result<Variant> {
        let attributes = self.attributes_parser.transform(variant.attrs, config)?;
        let identifier = self.identifier_parser.transform(variant.ident, config)?;
        Ok(Variant { attributes, identifier })
    }
}

impl Transformer<syn::punctuated::Punctuated<syn::Variant, syn::token::Comma>, Vec<Variant>> for RustVariantParser {
    fn transform(&self, input: syn::punctuated::Punctuated<syn::Variant, syn::token::Comma>, config: &Config) -> Result<Vec<Variant>> {
        let mut variants = Vec::new();
        for variant in input {
            variants.push(self.transform(variant, config)?);
        }
        Ok(variants)
    }
}

#[cfg(test)]
mod tests {
    use ligen::transformer::prelude::*;
    use syn::parse_quote;
    use ligen::ir::Variant;
    use crate::types::type_definition::enumeration::variant::RustVariantParser;

    #[test]
    fn parameter_primitive() {
        let enumeration: syn::ItemEnum = parse_quote! {
            enum Enumeration {
                Integer
            }
        };
        let variant = enumeration.variants.into_iter().next().expect("Couldn't get field.");
        assert_eq!(
            RustVariantParser::default().transform(variant, &Default::default()).expect("Failed to convert field."),
            Variant {
                attributes: Default::default(),
                identifier: "Integer".into(),
            }
        );
    }
}