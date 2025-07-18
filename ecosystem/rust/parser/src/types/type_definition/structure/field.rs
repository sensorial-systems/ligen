//! Structure field representation.

use crate::prelude::*;
use ligen::ir::Field;
use crate::{RustIdentifierParser, RustAttributesParser, RustTypeParser, RustVisibilityParser};

#[derive(Default)]
pub struct RustFieldParser {
    identifier_parser: RustIdentifierParser,
    visibility_parser: RustVisibilityParser,
    attributes_parser: RustAttributesParser,
    type_parser: RustTypeParser,
}

impl Transformer<syn::Field, Field> for RustFieldParser {
    fn transform(&self, field: syn::Field, config: &Config) -> Result<Field> {
        let attributes = self.attributes_parser.transform(field.attrs, config)?;
        let visibility = self.visibility_parser.transform(field.vis, config)?;
        let identifier = field.ident.map(|identifier| self.identifier_parser.transform(identifier, config)).transpose()?;
        let type_ = self.type_parser.transform(field.ty, config)?;
        Ok(Field { attributes, visibility, identifier, type_ })
    }
}

impl Transformer<syn::Fields, Vec<Field>> for RustFieldParser {
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
    use syn::parse_quote;
    use ligen::ir::{Field, Visibility, Path};
    use crate::RustFieldParser;
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
            RustFieldParser::default().transform(field, &Default::default())?,
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