//! Structure representation.

pub mod field;
pub use field::*;

use crate::prelude::*;
use crate::Structure;

impl TryFrom<SynItemStruct> for Structure {
    type Error = Error;
    fn try_from(SynItemStruct(structure): SynItemStruct) -> Result<Self> {
        let attributes = (LigenAttributes::try_from(structure.attrs)?).into();
        let identifier = SynIdent(structure.ident).into();
        let visibility = SynVisibility(structure.vis).into();
        let mut fields = Vec::new();
        for field in structure.fields {
            fields.push(SynField(field).try_into()?);
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
    use crate::prelude::SynItemStruct;

    #[test]
    fn structure() {
        let structure = parse::<syn::ItemStruct>(quote! {
            struct Structure {
                integer: i32
            }
        });
        assert_eq!(
            Structure::try_from(SynItemStruct(structure)).expect("Failed to convert structure."),
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
