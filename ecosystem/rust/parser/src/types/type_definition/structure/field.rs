//! Structure field representation.

use crate::prelude::*;
use ligen::ir::Field;
use ligen::parser::prelude::*;
use crate::identifier::IdentifierParser;
use crate::macro_attributes::attributes::AttributesParser;
use crate::types::TypeParser;
use crate::visibility::VisibilityParser;

#[derive(Default)]
pub struct FieldParser {
    identifier_parser: IdentifierParser,
    visibility_parser: VisibilityParser,
    attributes_parser: AttributesParser,
    type_parser: TypeParser,
}

impl Transformer<syn::Field, Field> for FieldParser {
    fn transform(&self, field: syn::Field, config: &Config) -> Result<Field> {
        let attributes = self.attributes_parser.transform(field.attrs, config)?;
        let visibility = self.visibility_parser.transform(field.vis, config)?;
        let identifier = field.ident.map(|identifier| self.identifier_parser.transform(identifier, config)).transpose()?;
        let type_ = self.type_parser.transform(field.ty, config)?;
        Ok(Field { attributes, visibility, identifier, type_ })
    }
}

impl Transformer<syn::Fields, Vec<Field>> for FieldParser {
    fn transform(&self, input: syn::Fields, config: &Config) -> Result<Vec<Field>> {
        let mut fields = Vec::new();
        for field in input {
            fields.push(self.transform(field, config)?);
        }
        Ok(fields)
    }
}

#[cfg(test)]
mod tests {
    use ligen::parser::prelude::Transformer;
    use syn::parse_quote;
    use ligen::ir::{Field, Visibility, Path};
    use crate::types::structure::FieldParser;
    use crate::prelude::*;

    #[test]
    fn field() -> Result<()> {
        let structure: syn::ItemStruct = parse_quote! {
            struct Structure {
                instant: std::time::Instant
            }
        };
        let field = structure.fields.into_iter().next().expect("Couldn't get field.");
        assert_eq!(
            FieldParser::default().transform(field, &Default::default())?,
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