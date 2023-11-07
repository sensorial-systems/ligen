//! Structure field representation.

use crate::prelude::*;
use ligen::ir::Field;
use ligen::parser::{Parser, ParserConfig};
use crate::identifier::IdentifierParser;
use crate::macro_attributes::attributes::AttributesParser;
use crate::types::TypeParser;
use crate::visibility::VisibilityParser;

pub struct FieldParser;

impl Parser<syn::Field> for FieldParser {
    type Output = Field;
    fn parse(&self, field: syn::Field, config: &ParserConfig) -> Result<Self::Output> {
        let attributes = AttributesParser::default().parse(field.attrs, config)?;
        let visibility = VisibilityParser.parse(field.vis, config)?;
        let identifier = field.ident.map(|identifier| IdentifierParser::new().parse(identifier, config).expect("Failed to parse identifier."));
        let type_ = TypeParser.parse(field.ty, config)?;
        Ok(Self::Output { attributes, visibility, identifier, type_ })
    }
}

impl Parser<syn::Fields> for FieldParser {
    type Output = Vec<Field>;
    fn parse(&self, input: syn::Fields, config: &ParserConfig) -> Result<Self::Output> {
        let mut fields = Vec::new();
        for field in input {
            fields.push(self.parse(field, config)?);
        }
        Ok(fields)
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
        let field = structure.fields.into_iter().next().expect("Couldn't get field.");
        assert_eq!(
            FieldParser.parse(field, &Default::default())?,
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