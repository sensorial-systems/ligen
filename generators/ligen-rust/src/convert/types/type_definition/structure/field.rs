//! Structure field representation.

use crate::prelude::*;
use ligen_ir::{Identifier, Type, Visibility, Attributes, Field};

impl TryFrom<syn::Field> for Field {
    type Error = Error;
    fn try_from(field: syn::Field) -> Result<Self> {
        let attributes = field.attrs.try_into()?;
        let visibility = field.vis.into();
        let identifier = field.ident.map(|identifier| identifier.into());
        let type_ = field.ty.try_into()?;
        Ok(Self { attributes, visibility, identifier, type_ })
    }
}

#[cfg(test)]
mod tests {
    use quote::quote;
    use syn::parse_quote::parse;
    use std::convert::TryFrom;
    use crate::{Field, Visibility, Path};

    #[test]
    fn field() {
        let structure = parse::<syn::ItemStruct>(quote! {
            struct Structure {
                instant: std::time::Instant
            }
        });
        assert_eq!(
            Field::try_from(structure.fields.into_iter().next().expect("Couldn't get field.")).expect("Failed to convert field."),
            Field {
                attributes: Default::default(),
                visibility: Visibility::Inherited,
                identifier: Some("instant".into()),
                type_: Path::from("std::time::Instant").into()
            }
        );
    }
}