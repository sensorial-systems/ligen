//! Enumeration variant representation.

use crate::prelude::*;
use ligen::ir::Variant;
use ligen::parser::prelude::*;
use crate::identifier::IdentifierParser;
use crate::macro_attributes::attributes::AttributesParser;

#[derive(Default)]
pub struct VariantParser {
    identifier_parser: IdentifierParser,
    attributes_parser: AttributesParser,
}

impl Transformer<syn::Variant, Variant> for VariantParser {
    fn transform(&self, variant: syn::Variant, config: &Config) -> Result<Variant> {
        let attributes = self.attributes_parser.transform(variant.attrs, config)?;
        let identifier = self.identifier_parser.transform(variant.ident, config)?;
        Ok(Variant { attributes, identifier })
    }
}

impl Transformer<syn::punctuated::Punctuated<syn::Variant, syn::token::Comma>, Vec<Variant>> for VariantParser {
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
    use ligen::parser::prelude::*;
    use syn::parse_quote;
    use ligen::ir::Variant;
    use crate::types::type_definition::enumeration::variant::VariantParser;

    #[test]
    fn parameter_primitive() {
        let enumeration: syn::ItemEnum = parse_quote! {
            enum Enumeration {
                Integer
            }
        };
        let variant = enumeration.variants.into_iter().next().expect("Couldn't get field.");
        assert_eq!(
            VariantParser::default().transform(variant, &Default::default()).expect("Failed to convert field."),
            Variant {
                attributes: Default::default(),
                identifier: "Integer".into(),
            }
        );
    }
}