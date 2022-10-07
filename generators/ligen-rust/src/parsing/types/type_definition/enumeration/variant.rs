//! Enumeration variant representation.

use crate::prelude::*;
use crate::Variant;

impl TryFrom<SynVariant> for Variant {
    type Error = Error;
    fn try_from(SynVariant(variant): SynVariant) -> Result<Self> {
        let attributes = (LigenAttributes::try_from(variant.attrs)?).into();
        let identifier = SynIdent(variant.ident).into();
        Ok(Self { attributes, identifier })
    }
}

#[cfg(test)]
mod tests {
    use quote::quote;
    use syn::parse_quote::parse;
    use std::convert::TryFrom;
    use crate::Variant;

    #[test]
    fn parameter_atomic() {
        let enumeration = parse::<syn::ItemEnum>(quote! {
            enum Enumeration {
                Integer
            }
        });
        assert_eq!(
            Variant::try_from(enumeration.variants.into_iter().next().expect("Couldn't get field.")).expect("Failed to convert field."),
            Variant {
                attributes: Default::default(),
                identifier: "Integer".into(),
            }
        );
    }
}