//! Enumeration variant representation.

use crate::prelude::*;
use crate::{Attributes, Identifier};

/// Enumeration representation.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Variant {
    /// Attributes field.
    pub attributes: Attributes,
    /// Variant identifier.
    pub identifier: Identifier
}

impl TryFrom<syn::Variant> for Variant {
    type Error = Error;
    fn try_from(variant: syn::Variant) -> Result<Self> {
        let attributes = variant.attrs.try_into()?;
        let identifier = variant.ident.into();
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