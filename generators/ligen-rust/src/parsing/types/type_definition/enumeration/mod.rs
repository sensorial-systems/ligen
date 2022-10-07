//! Enumeration representation.

pub mod variant;

use crate::prelude::*;
use crate::Enumeration;

impl TryFrom<SynItemEnum> for Enumeration {
    type Error = Error;
    fn try_from(SynItemEnum(enumeration): SynItemEnum) -> Result<Self> {
        let attributes = (LigenAttributes::try_from(enumeration.attrs)?).into();
        let identifier = SynIdent(enumeration.ident).into();
        let visibility = SynVisibility(enumeration.vis).into();
        let mut variants = Vec::new();
        for variant in enumeration.variants {
            variants.push(SynVariant(variant).try_into()?);
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
