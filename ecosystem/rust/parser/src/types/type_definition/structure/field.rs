//! Structure field representation.

use crate::prelude::*;
use ligen::ir::Field;
use ligen::parsing::Parser;
use crate::identifier::IdentifierParser;
use crate::macro_attributes::attributes::AttributesParser;
use crate::types::TypeParser;
use crate::visibility::VisibilityParser;

pub struct FieldParser;

impl Parser<syn::Field> for FieldParser {
    type Output = Field;
    fn parse(&self, field: syn::Field) -> Result<Self::Output> {
        let attributes = AttributesParser.parse(field.attrs)?;
        let visibility = VisibilityParser.parse(field.vis)?;
        let identifier = field.ident.map(|identifier| IdentifierParser.parse(identifier).expect("Failed to parse identifier."));
        let type_ = TypeParser.parse(field.ty)?;
        Ok(Self::Output { attributes, visibility, identifier, type_ })
    }
}

#[cfg(test)]
mod tests {
    use syn::parse_quote::parse;
    use ligen::ir::{Field, Visibility, Path};
    use crate::types::structure::FieldParser;
    use crate::prelude::*;

    #[test]
    fn field() -> Result<()> {
        let structure = parse::<syn::ItemStruct>(quote! {
            struct Structure {
                instant: std::time::Instant
            }
        });
        assert_eq!(
            FieldParser.parse(structure.fields.into_iter().next().expect("Couldn't get field."))?,
            Field {
                attributes: Default::default(),
                visibility: Visibility::Private,
                identifier: Some("instant".into()),
                type_: Path::from("std::time::Instant").into()
            }
        );
        Ok(())
    }
}