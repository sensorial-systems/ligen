//! Structure representation.

pub mod field;

pub use field::*;

use crate::prelude::*;
use crate::types::GenericsParser;
use ligen::ir::{Structure, TypeDefinition};
use ligen::parser::prelude::*;
use crate::identifier::IdentifierParser;
use crate::macro_attributes::attributes::AttributesParser;
use crate::visibility::VisibilityParser;

#[derive(Default)]
pub struct StructureParser {
    field_parser: FieldParser,
    identifier_parser: IdentifierParser,
    visibility_parser: VisibilityParser,
    attributes_parser: AttributesParser,
    generics_parser: GenericsParser,
}

impl StructureParser {
    pub fn new() -> Self {
        Default::default()
    }
}

impl Transformer<proc_macro::TokenStream, TypeDefinition> for StructureParser {
    fn transform(&self, token_stream: proc_macro::TokenStream, config: &Config) -> Result<TypeDefinition> {
        self.transform(proc_macro2::TokenStream::from(token_stream), config)
    }
}

impl Transformer<proc_macro2::TokenStream, TypeDefinition> for StructureParser {
    fn transform(&self, tokenstream: proc_macro2::TokenStream, config: &Config) -> Result<TypeDefinition> {
        syn::parse2::<syn::ItemStruct>(tokenstream)
            .map_err(|e| Error::Message(format!("Failed to parse to structure: {:?}", e)))
            .and_then(|structure| self.transform(structure, config))
    }
}

impl Transformer<syn::ItemStruct, TypeDefinition> for StructureParser {
    fn transform(&self, structure: syn::ItemStruct, config: &Config) -> Result<TypeDefinition> {
        let attributes = self.attributes_parser.transform(structure.attrs, config)?;
        let identifier = self.identifier_parser.transform(structure.ident, config)?;
        let visibility = self.visibility_parser.transform(structure.vis, config)?;
        let interfaces = Default::default();
        let fields = self.field_parser.transform(structure.fields, config)?;
        let definition = Structure { fields }.into();
        let generics = self.generics_parser.transform(structure.generics, config)?;
        Ok(TypeDefinition { attributes, visibility, identifier, generics, interfaces, definition })
    }
}

#[cfg(test)]
mod tests {
    use crate::types::structure::StructureParser;
    use crate::prelude::*;

    use ligen::parser::assert::*;
    use ligen::ir::structure::mock;

    #[test]
    fn structure() -> Result<()> {
        assert_eq(StructureParser::default(), mock::structure(), quote! {
            pub struct Structure {
                integer: i32
            }
        })
    }
}
