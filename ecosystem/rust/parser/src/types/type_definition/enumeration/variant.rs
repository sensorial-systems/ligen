//! Enumeration variant representation.

use crate::prelude::*;
use ligen_ir::Variant;
use ligen_parsing::Parser;
use crate::macro_attributes::attributes::AttributesParser;

impl TryFrom<SynVariant> for Variant {
    type Error = Error;
    fn try_from(SynVariant(variant): SynVariant) -> Result<Self> {
        let attributes = AttributesParser.parse(variant.attrs)?;
        let identifier = SynIdent(variant.ident).into();
        Ok(Self { attributes, identifier })
    }
}

#[cfg(test)]
mod tests {
    use quote::quote;
    use syn::parse_quote::parse;
    use std::convert::TryFrom;
    use crate::prelude::SynVariant;
    use ligen_ir::Variant;

    #[test]
    fn parameter_primitive() {
        let enumeration = parse::<syn::ItemEnum>(quote! {
            enum Enumeration {
                Integer
            }
        });
        assert_eq!(
            Variant::try_from(SynVariant(enumeration.variants.into_iter().next().expect("Couldn't get field."))).expect("Failed to convert field."),
            Variant {
                attributes: Default::default(),
                identifier: "Integer".into(),
            }
        );
    }
}