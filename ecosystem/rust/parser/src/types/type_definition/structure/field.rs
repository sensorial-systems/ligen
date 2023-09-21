//! Structure field representation.

use crate::prelude::*;
use ligen_ir::Field;
use ligen_parsing::Parser;
use crate::identifier::IdentifierParser;
use crate::macro_attributes::attributes::AttributesParser;
use crate::types::TypeParser;
use crate::visibility::VisibilityParser;

impl TryFrom<SynField> for Field {
    type Error = Error;
    fn try_from(SynField(field): SynField) -> Result<Self> {
        let attributes = AttributesParser.parse(field.attrs)?;
        let visibility = VisibilityParser.parse(field.vis)?;
        let identifier = field.ident.map(|identifier| IdentifierParser.parse(identifier).expect("Failed to parse identifier."));
        let type_ = TypeParser.parse(field.ty)?;
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