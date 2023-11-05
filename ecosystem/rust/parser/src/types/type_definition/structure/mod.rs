//! Structure representation.

pub mod field;

pub use field::*;

use crate::prelude::*;
use crate::types::GenericsParser;
use ligen::ir::{Structure, TypeDefinition};
use ligen::parsing::parser::{Parser, ParserConfig};
use crate::identifier::IdentifierParser;
use crate::macro_attributes::attributes::AttributesParser;
use crate::visibility::VisibilityParser;

#[derive(Default)]
pub struct StructureParser;

impl StructureParser {
    pub fn new() -> Self {
        Default::default()
    }
}

impl Parser<proc_macro::TokenStream> for StructureParser {
    type Output = TypeDefinition;
    fn parse(&self, token_stream: proc_macro::TokenStream, config: &ParserConfig) -> Result<Self::Output> {
        self.parse(proc_macro2::TokenStream::from(token_stream), config)
    }
}

impl Parser<proc_macro2::TokenStream> for StructureParser {
    type Output = TypeDefinition;
    fn parse(&self, tokenstream: proc_macro2::TokenStream, config: &ParserConfig) -> Result<Self::Output> {
        syn::parse2::<syn::ItemStruct>(tokenstream)
            .map_err(|e| Error::Message(format!("Failed to parse to structure: {:?}", e)))
            .and_then(|structure| self.parse(structure, config))
    }
}

impl Parser<syn::ItemStruct> for StructureParser {
    type Output = TypeDefinition;
    fn parse(&self, structure: syn::ItemStruct, config: &ParserConfig) -> Result<Self::Output> {
        let attributes = AttributesParser::default().parse(structure.attrs, config)?;
        let identifier = IdentifierParser::new().parse(structure.ident, config)?;
        let visibility = VisibilityParser::new().parse(structure.vis, config)?;
        let interfaces = Default::default();
        let fields = FieldParser.parse(structure.fields, config)?;
        let definition = Structure { fields }.into();
        let generics = GenericsParser::default().parse(structure.generics, config)?;
        Ok(Self::Output { attributes, visibility, identifier, generics, interfaces, definition })
    }
}

#[cfg(test)]
mod tests {
    use crate::types::structure::StructureParser;
    use crate::prelude::*;

    use ligen::parsing::assert::*;
    use ligen::ir::structure::mock;

    #[test]
    fn structure() -> Result<()> {
        assert_eq(StructureParser, mock::structure(), quote! {
            pub struct Structure {
                integer: i32
            }
        })
    }
}
