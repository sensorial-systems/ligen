//! Structure representation.

use crate::prelude::*;
use crate::types::{GenericsParser, TypeParser};
use ligen::ir::{TypeAlias, TypeDefinition};
use ligen::parser::prelude::*;
use crate::identifier::IdentifierParser;
use crate::macro_attributes::attributes::AttributesParser;
use crate::visibility::VisibilityParser;

#[derive(Default)]
pub struct TypeAliasParser;

impl TypeAliasParser {
    pub fn new() -> Self {
        Default::default()
    }
}

impl Parser<proc_macro::TokenStream> for TypeAliasParser {
    type Output = TypeDefinition;
    fn parse(&self, token_stream: proc_macro::TokenStream, config: &Config) -> Result<Self::Output> {
        self.parse(proc_macro2::TokenStream::from(token_stream), config)
    }
}

impl Parser<proc_macro2::TokenStream> for TypeAliasParser {
    type Output = TypeDefinition;
    fn parse(&self, tokenstream: proc_macro2::TokenStream, config: &Config) -> Result<Self::Output> {
        syn::parse2::<syn::ItemType>(tokenstream)
            .map_err(|e| Error::Message(format!("Failed to parse to structure: {:?}", e)))
            .and_then(|structure| self.parse(structure, config))
    }
}

impl Parser<syn::ItemType> for TypeAliasParser {
    type Output = TypeDefinition;
    fn parse(&self, type_alias: syn::ItemType, config: &Config) -> Result<Self::Output> {
        let attributes = AttributesParser::default().parse(type_alias.attrs, config)?;
        let identifier = IdentifierParser::new().parse(type_alias.ident, config)?;
        let visibility = VisibilityParser::new().parse(type_alias.vis, config)?;
        let interfaces = Default::default();
        let type_ = TypeParser::default().parse(*type_alias.ty, config)?;
        let definition = TypeAlias { type_ }.into();
        let generics = GenericsParser::default().parse(type_alias.generics, config)?;
        Ok(Self::Output { attributes, visibility, identifier, generics, interfaces, definition })
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
        assert_eq(TypeAliasParser, mock::type_alias(), quote! {
            pub type Integer = i32;
        })
    }
}
