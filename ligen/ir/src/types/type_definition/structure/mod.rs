//! Structure representation.

mod field;
pub use field::*;

use crate::prelude::*;
use crate::{Attributes, Visibility, Identifier};

/// Structure representation.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Structure {
    /// Attributes field.
    pub attributes: Attributes,
    /// Structure visibility.
    pub visibility: Visibility,
    /// Structure identifier.
    pub identifier: Identifier,
    /// Items field.
    pub fields: Vec<Field>,
}

impl TryFrom<syn::ItemStruct> for Structure {
    type Error = Error;
    fn try_from(structure: syn::ItemStruct) -> Result<Self> {
        let attributes = structure.attrs.try_into()?;
        let identifier = structure.ident.into();
        let visibility = structure.vis.into();
        let mut fields = Vec::new();
        for field in structure.fields {
            fields.push(field.try_into()?);
        }
        Ok(Self { attributes, visibility, identifier, fields })
    }
}

#[cfg(test)]
mod tests {
    use quote::quote;
    use syn::parse_quote::parse;
    use std::convert::TryFrom;
    use crate::{Field, Identifier, Type, Atomic, Integer, Visibility, Structure};

    #[test]
    fn structure() {
        let structure = parse::<syn::ItemStruct>(quote! {
            struct Structure {
                integer: i32
            }
        });
        assert_eq!(
            Structure::try_from(structure).expect("Failed to convert structure."),
            Structure {
                attributes: Default::default(),
                visibility: Visibility::Inherited,
                identifier: Identifier::new("Structure"),
                fields: vec! [
                    Field {
                        attributes: Default::default(),
                        visibility: Visibility::Inherited,
                        identifier: Some("integer".into()),
                        type_: Type::Atomic(Atomic::Integer(Integer::I32))
                    }
                ]
            }
        );
    }
}
