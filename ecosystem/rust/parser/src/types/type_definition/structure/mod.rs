//! Structure representation.

mod field;
pub use field::*;

use crate::prelude::*;
use crate::{RustIdentifierParser, RustAttributesParser, RustVisibilityParser, RustGenericsParser};
use ligen::ir::{Structure, TypeDefinition};

#[derive(Default)]
pub struct RustStructureParser {
    field_parser: RustFieldParser,
    identifier_parser: RustIdentifierParser,
    visibility_parser: RustVisibilityParser,
    attributes_parser: RustAttributesParser,
    generics_parser: RustGenericsParser,
}

impl RustStructureParser {
    pub fn new() -> Self {
        Default::default()
    }
}

impl Transformer<proc_macro::TokenStream, TypeDefinition> for RustStructureParser {
    fn transform(&self, token_stream: proc_macro::TokenStream, config: &Config) -> Result<TypeDefinition> {
        self.transform(proc_macro2::TokenStream::from(token_stream), config)
    }
}

impl Transformer<proc_macro2::TokenStream, TypeDefinition> for RustStructureParser {
    fn transform(&self, tokenstream: proc_macro2::TokenStream, config: &Config) -> Result<TypeDefinition> {
        syn::parse2::<syn::ItemStruct>(tokenstream)
            .map_err(|e| Error::Message(format!("Failed to parse to structure: {e:?}")))
            .and_then(|structure| self.transform(structure, config))
    }
}

impl Transformer<syn::ItemStruct, TypeDefinition> for RustStructureParser {
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
    use crate::prelude::*;
    use crate::RustStructureParser;

    use ligen::transformer::assert::*;
    use ligen::ir::structure::mock;

    #[test]
    fn structure() -> Result<()> {
        assert_eq(RustStructureParser::default(), mock::structure(), quote! {
            pub struct Structure {
                integer: i32
            }
        })
    }
}
