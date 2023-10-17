//! Enumeration variant representation.

use crate::prelude::*;
use ligen::ir::Variant;
use ligen::parsing::parser::Parser;
use crate::identifier::IdentifierParser;
use crate::macro_attributes::attributes::AttributesParser;

pub struct VariantParser;

impl Parser<syn::Variant> for VariantParser {
    type Output = Variant;
    fn parse(&self, variant: syn::Variant) -> Result<Self::Output> {
        let attributes = AttributesParser::default().parse(variant.attrs)?;
        let identifier = IdentifierParser::new().parse(variant.ident)?;
        Ok(Self::Output { attributes, identifier })
    }
}

#[cfg(test)]
mod tests {
    use quote::quote;
    use syn::parse_quote::parse;
    use ligen::ir::Variant;
    use ligen::parsing::parser::Parser;
    use crate::types::type_definition::enumeration::variant::VariantParser;

    #[test]
    fn parameter_primitive() {
        let enumeration = parse::<syn::ItemEnum>(quote! {
            enum Enumeration {
                Integer
            }
        });
        assert_eq!(
            VariantParser.parse(enumeration.variants.into_iter().next().expect("Couldn't get field.")).expect("Failed to convert field."),
            Variant {
                attributes: Default::default(),
                identifier: "Integer".into(),
            }
        );
    }
}