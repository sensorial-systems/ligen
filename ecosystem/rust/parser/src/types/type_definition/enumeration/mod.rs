//! Enumeration representation.

mod variant;
pub use variant::*;

use crate::prelude::*;
use crate::{RustIdentifierParser, RustAttributesParser, RustGenericsParser, RustVisibilityParser};
use ligen::idl::{Enumeration, TypeDefinition};

#[derive(Default)]
pub struct RustEnumerationParser {
    identifier_parser: RustIdentifierParser,
    visibility_parser: RustVisibilityParser,
    attributes_parser: RustAttributesParser,
    generics_parser: RustGenericsParser,
    variant_parser: RustVariantParser,
}

impl RustEnumerationParser {
    pub fn new() -> Self {
        Default::default()
    }
}

impl Transformer<syn::ItemEnum, TypeDefinition> for RustEnumerationParser {
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

impl Transformer<proc_macro::TokenStream, TypeDefinition> for RustEnumerationParser {
    fn transform(&self, input: proc_macro::TokenStream, config: &Config) -> Result<TypeDefinition> {
        self.transform(proc_macro2::TokenStream::from(input), config)
    }
}

impl Transformer<proc_macro2::TokenStream, TypeDefinition> for RustEnumerationParser {
    fn transform(&self, input: proc_macro2::TokenStream, config: &Config) -> Result<TypeDefinition> {
        syn::parse2::<syn::ItemEnum>(input)
            .map_err(|e| Error::Message(format!("Failed to parse enumeration: {e:?}")))
            .and_then(|enumeration| self.transform(enumeration, config))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ligen::idl::enumeration::mock;
    use ligen::transformer::assert::*;

    #[test]
    fn enumeration() -> Result<()> {
        assert_eq(RustEnumerationParser::default(), mock::enumeration(), quote !{
            pub enum Enumeration {
                Integer,
                Float,
                Boolean
            }
        })
    }
}
