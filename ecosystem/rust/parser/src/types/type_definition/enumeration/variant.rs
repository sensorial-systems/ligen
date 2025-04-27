//! Enumeration variant representation.

use crate::prelude::*;
use ligen::ir::Variant;
use ligen::parser::prelude::*;
use crate::identifier::IdentifierParser;
use crate::macro_attributes::attributes::AttributesParser;

pub struct VariantParser;

impl Parser<syn::Variant> for VariantParser {
    type Output = Variant;
    fn parse(&self, variant: syn::Variant, config: &Config) -> Result<Self::Output> {
        let attributes = AttributesParser::default().parse(variant.attrs, config)?;
        let identifier = IdentifierParser::new().parse(variant.ident, config)?;
        Ok(Self::Output { attributes, identifier })
    }
}

impl Parser<syn::punctuated::Punctuated<syn::Variant, syn::token::Comma>> for VariantParser {
    type Output = Vec<Variant>;
    fn parse(&self, input: syn::punctuated::Punctuated<syn::Variant, syn::token::Comma>, config: &Config) -> Result<Self::Output> {
        let mut variants = Vec::new();
        for variant in input {
            variants.push(self.parse(variant, config)?);
        }
        Ok(variants)
    }
}

#[cfg(test)]
mod tests {
    use syn::parse_quote;
    use ligen::ir::Variant;
    use ligen::parser::Parser;
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
            VariantParser.parse(variant, &Default::default()).expect("Failed to convert field."),
            Variant {
                attributes: Default::default(),
                identifier: "Integer".into(),
            }
        );
    }
}