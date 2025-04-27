//! Structure representation.

use crate::prelude::*;
use crate::types::{GenericsParser, TypeParser};
use ligen::ir::{TypeAlias, TypeDefinition};
use ligen::parser::prelude::*;
use crate::identifier::IdentifierParser;
use crate::macro_attributes::attributes::AttributesParser;
use crate::visibility::VisibilityParser;

#[derive(Default)]
pub struct TypeAliasParser {
    identifier_parser: IdentifierParser,
    visibility_parser: VisibilityParser,
    attributes_parser: AttributesParser,
    generics_parser: GenericsParser,
    type_parser: TypeParser,
}

impl TypeAliasParser {
    pub fn new() -> Self {
        Default::default()
    }
}

impl Transformer<proc_macro::TokenStream, TypeDefinition> for TypeAliasParser {
    fn transform(&self, token_stream: proc_macro::TokenStream, config: &Config) -> Result<TypeDefinition> {
        self.transform(proc_macro2::TokenStream::from(token_stream), config)
    }
}

impl Transformer<proc_macro2::TokenStream, TypeDefinition> for TypeAliasParser {
    fn transform(&self, tokenstream: proc_macro2::TokenStream, config: &Config) -> Result<TypeDefinition> {
        syn::parse2::<syn::ItemType>(tokenstream)
            .map_err(|e| Error::Message(format!("Failed to parse to structure: {:?}", e)))
            .and_then(|structure| self.transform(structure, config))
    }
}

impl Transformer<syn::ItemType, TypeDefinition> for TypeAliasParser {
    fn transform(&self, type_alias: syn::ItemType, config: &Config) -> Result<TypeDefinition> {
        let attributes = self.attributes_parser.transform(type_alias.attrs, config)?;
        let identifier = self.identifier_parser.transform(type_alias.ident, config)?;
        let visibility = self.visibility_parser.transform(type_alias.vis, config)?;
        let interfaces = Default::default();
        let type_ = self.type_parser.transform(*type_alias.ty, config)?;
        let definition = TypeAlias { type_ }.into();
        let generics = self.generics_parser.transform(type_alias.generics, config)?;
        Ok(TypeDefinition { attributes, visibility, identifier, generics, interfaces, definition })
    }
}

#[cfg(test)]
mod tests {
    use crate::prelude::*;

    use ligen::parser::assert::*;
    use ligen::ir::type_alias::mock;

    use super::TypeAliasParser;

    #[test]
    fn type_alias() -> Result<()> {
        assert_eq(TypeAliasParser::default(), mock::type_alias(), quote! {
            pub type Integer = i32;
        })
    }
}
