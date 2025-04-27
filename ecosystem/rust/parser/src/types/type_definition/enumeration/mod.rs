//! Enumeration representation.

pub mod variant;

use crate::prelude::*;
use crate::types::GenericsParser;
use ligen::ir::{Enumeration, TypeDefinition};
use crate::identifier::IdentifierParser;
use crate::macro_attributes::attributes::AttributesParser;
use crate::types::type_definition::enumeration::variant::VariantParser;
use crate::visibility::VisibilityParser;

#[derive(Default)]
pub struct EnumerationParser {
    identifier_parser: IdentifierParser,
    visibility_parser: VisibilityParser,
    attributes_parser: AttributesParser,
    generics_parser: GenericsParser,
    variant_parser: VariantParser,
}

impl EnumerationParser {
    pub fn new() -> Self {
        Default::default()
    }
}

impl Transformer<syn::ItemEnum, TypeDefinition> for EnumerationParser {
    fn transform(&self, enumeration: syn::ItemEnum, config: &Config) -> Result<TypeDefinition> {
        let attributes = self.attributes_parser.transform(enumeration.attrs, config)?;
        let identifier = self.identifier_parser.transform(enumeration.ident, config)?;
        let visibility = self.visibility_parser.transform(enumeration.vis, config)?;
        let interfaces = Default::default();
        let variants = self.variant_parser.transform(enumeration.variants, config)?;
        let definition = Enumeration { variants }.into();
        let generics = self.generics_parser.transform(enumeration.generics, config)?;
        Ok(TypeDefinition { attributes, visibility, identifier, generics, interfaces, definition })
    }
}

impl Transformer<proc_macro::TokenStream, TypeDefinition> for EnumerationParser {
    fn transform(&self, input: proc_macro::TokenStream, config: &Config) -> Result<TypeDefinition> {
        self.transform(proc_macro2::TokenStream::from(input), config)
    }
}

impl Transformer<proc_macro2::TokenStream, TypeDefinition> for EnumerationParser {
    fn transform(&self, input: proc_macro2::TokenStream, config: &Config) -> Result<TypeDefinition> {
        syn::parse2::<syn::ItemEnum>(input)
            .map_err(|e| Error::Message(format!("Failed to parse enumeration: {:?}", e)))
            .and_then(|enumeration| self.transform(enumeration, config))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ligen::ir::enumeration::mock;
    use ligen::transformer::assert::*;

    #[test]
    fn enumeration() -> Result<()> {
        assert_eq(EnumerationParser::default(), mock::enumeration(), quote !{
            pub enum Enumeration {
                Integer,
                Float,
                Boolean
            }
        })
    }
}
