//! Structure field representation.

use crate::prelude::*;
use crate::ir::{Identifier, Type, Visibility, Attributes};
use std::convert::{TryFrom, TryInto};

/// Property representation.
#[derive(Debug, Clone, PartialEq)]
pub struct Field {
    /// Field attributes.
    pub attributes: Attributes,
    /// Field visibility.
    pub visibility: Visibility,
    /// Field identifier.
    pub identifier: Identifier,
    /// Field type.
    pub type_: Type
}

impl TryFrom<syn::Field> for Field {
    type Error = Error;
    fn try_from(field: syn::Field) -> Result<Self> {
        let attributes = field.attrs.try_into()?;
        let visibility = field.vis.into();
        let identifier = field.ident.unwrap().into();
        let type_ = field.ty.try_into()?;
        Ok(Self { attributes, visibility, identifier, type_ })
    }
}

#[cfg(test)]
mod tests {
    use quote::quote;
    use syn::parse_quote::parse;
    use std::convert::TryFrom;
    use crate::ir::{Field, Identifier, Type, Atomic, Integer, Visibility};

    #[test]
    fn parameter_atomic() {
        let structure = parse::<syn::ItemStruct>(quote! {
            struct Structure {
                integer: i32
            }
        });
        assert_eq!(
            Field::try_from(structure.fields.into_iter().next().expect("Couldn't get field.")).expect("Failed to convert field."),
            Field {
                attributes: Default::default(),
                visibility: Visibility::Inherited,
                identifier: Identifier::new("integer"),
                type_: Type::Atomic(Atomic::Integer(Integer::I32))
            }
        );
    }
}