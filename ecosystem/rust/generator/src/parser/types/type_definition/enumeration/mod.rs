//! Enumeration representation.

pub mod variant;

use crate::prelude::*;
use crate::Enumeration;

impl TryFrom<SynItemEnum> for Enumeration {
    type Error = Error;
    fn try_from(SynItemEnum(enumeration): SynItemEnum) -> Result<Self> {
        let mut variants = Vec::new();
        for variant in enumeration.variants {
            variants.push(SynVariant(variant).try_into()?);
        }
        Ok(Self { variants })
    }
}

#[cfg(test)]
mod tests {
    use quote::quote;
    use syn::parse_quote::parse;
    use std::convert::TryFrom;
    use crate::{Enumeration, Variant};
    use crate::prelude::SynItemEnum;

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
            Enumeration::try_from(SynItemEnum(enumeration)).expect("Failed to convert structure."),
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
    }
}
