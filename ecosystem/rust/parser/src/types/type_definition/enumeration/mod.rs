//! Enumeration representation.

pub mod variant;

use crate::prelude::*;
use ligen::ir::{Enumeration, TypeDefinition};
use ligen::parsing::parser::Parser;
use crate::identifier::IdentifierParser;
use crate::macro_attributes::attributes::AttributesParser;
use crate::types::type_definition::enumeration::variant::VariantParser;
use crate::visibility::VisibilityParser;

#[derive(Default)]
pub struct EnumerationParser;

impl EnumerationParser {
    pub fn new() -> Self {
        Self
    }
}

impl Parser<syn::ItemEnum> for EnumerationParser {
    type Output = TypeDefinition;
    fn parse(&self, enumeration: syn::ItemEnum) -> Result<Self::Output> {
        let attributes = AttributesParser::default().parse(enumeration.attrs)?;
        let identifier = IdentifierParser::new().parse(enumeration.ident)?;
        let visibility = VisibilityParser::new().parse(enumeration.vis)?;
        let interfaces = Default::default();
        let variants = VariantParser.parse(enumeration.variants)?;
        let definition = Enumeration { variants }.into();
        Ok(TypeDefinition { attributes, visibility, identifier, interfaces, definition })
    }
}

impl Parser<proc_macro::TokenStream> for EnumerationParser {
    type Output = TypeDefinition;
    fn parse(&self, input: proc_macro::TokenStream) -> Result<Self::Output> {
        self.parse(proc_macro2::TokenStream::from(input))
    }
}

impl Parser<proc_macro2::TokenStream> for EnumerationParser {
    type Output = TypeDefinition;
    fn parse(&self, input: proc_macro2::TokenStream) -> Result<Self::Output> {
        syn::parse2::<syn::ItemEnum>(input)
            .map_err(|e| Error::Message(format!("Failed to parse enumeration: {:?}", e)))
            .and_then(|enumeration| self.parse(enumeration))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ligen::ir::enumeration::mock;
    use ligen::parsing::assert::*;

    #[test]
    fn enumeration() -> Result<()> {
        assert_eq(EnumerationParser, mock::enumeration(), quote !{
            pub enum Enumeration {
                Integer,
                Float,
                Boolean
            }
        })
    }
}
