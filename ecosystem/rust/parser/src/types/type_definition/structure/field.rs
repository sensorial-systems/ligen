//! Structure field representation.

use crate::prelude::*;
use ligen_ir::Field;
use ligen_parsing::Parser;
use crate::macro_attributes::attributes::AttributesParser;

impl TryFrom<SynField> for Field {
    type Error = Error;
    fn try_from(SynField(field): SynField) -> Result<Self> {
        let attributes = AttributesParser.parse(field.attrs)?;
        let visibility = SynVisibility(field.vis).into();
        let identifier = field.ident.map(|identifier| SynIdent(identifier).into());
        let type_ = SynType(field.ty).try_into()?;
        Ok(Self { attributes, visibility, identifier, type_ })
    }
}

#[cfg(test)]
mod tests {
    use quote::quote;
    use syn::parse_quote::parse;
    use std::convert::TryFrom;
    use ligen_ir::{Field, Visibility, Path};
    use crate::prelude::SynField;

    #[test]
    fn field() {
        let structure = parse::<syn::ItemStruct>(quote! {
            struct Structure {
                instant: std::time::Instant
            }
        });
        assert_eq!(
            Field::try_from(SynField(structure.fields.into_iter().next().expect("Couldn't get field."))).expect("Failed to convert field."),
            Field {
                attributes: Default::default(),
                visibility: Visibility::Private,
                identifier: Some("instant".into()),
                type_: Path::from("std::time::Instant").into()
            }
        );
    }
}