//! Enumeration representation.

mod variant;
pub use variant::*;

use crate::prelude::*;
use ligen_ir::Enumeration;

impl TryFrom<syn::ItemEnum> for Enumeration {
    type Error = Error;
    fn try_from(enumeration: syn::ItemEnum) -> Result<Self> {
        let attributes = enumeration.attrs.try_into()?;
        let identifier = enumeration.ident.into();
        let visibility = enumeration.vis.into();
        let mut variants = Vec::new();
        for variant in enumeration.variants {
            variants.push(variant.try_into()?);
        }
        Ok(Self { attributes, visibility, identifier, variants })
    }
}

#[cfg(test)]
mod tests {
    use quote::quote;
    use syn::parse_quote::parse;
    use std::convert::TryFrom;
    use crate::{Visibility, Enumeration, Variant};

    #[test]
    fn enumeration() {
        let enumeration = parse::<syn::ItemEnum>(quote! {
            enum Enumeration {
                Integer,
                Float,
                Boolean
            }
        });
        assert_eq!(
            Enumeration::try_from(enumeration).expect("Failed to convert structure."),
            Enumeration {
                attributes: Default::default(),
                visibility: Visibility::Inherited,
                identifier: "Enumeration".into(),
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
    }
}
