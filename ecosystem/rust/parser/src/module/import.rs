//! Import representation.

use crate::prelude::*;
use ligen::ir::{Path, Attributes, Visibility, Import};
use ligen::parser::{Parser, ParserConfig};
use crate::identifier::IdentifierParser;
use crate::macro_attributes::attributes::AttributesParser;
use crate::visibility::VisibilityParser;

#[derive(Clone)]
struct ImportsBuilder {
    pub attributes: Attributes,
    pub visibility: Visibility,
    pub path: Path,
    pub tree: syn::UseTree
}

pub struct ImportsParser;

impl Parser<syn::ItemUse> for ImportsParser {
    type Output = Vec<Import>;
    fn parse(&self, import: syn::ItemUse, config: &ParserConfig) -> Result<Self::Output> {
        let attributes = AttributesParser::default().parse(import.attrs, config)?;
        let visibility = VisibilityParser.parse(import.vis, config)?;
        let path = Path::default();
        let tree = import.tree;
        self.parse(ImportsBuilder { attributes, visibility, path, tree }, config)
    }
}

impl Parser<proc_macro::TokenStream> for ImportsParser {
    type Output = Vec<Import>;
    fn parse(&self, input: proc_macro::TokenStream, config: &ParserConfig) -> Result<Self::Output> {
        self.parse(proc_macro2::TokenStream::from(input), config)
    }
}

impl Parser<proc_macro2::TokenStream> for ImportsParser {
    type Output = Vec<Import>;
    fn parse(&self, input: proc_macro2::TokenStream, config: &ParserConfig) -> Result<Self::Output> {
        syn::parse2::<syn::ItemUse>(input)
            .map_err(|e| Error::Message(format!("Failed to parse imports: {:?}", e)))
            .and_then(|imports| self.parse(imports, config))
    }
}


impl Parser<ImportsBuilder> for ImportsParser {
    type Output = Vec<Import>;
    fn parse(&self, builder: ImportsBuilder, config: &ParserConfig) -> Result<Self::Output> {
        let mut builder = builder;
        match builder.tree {
            syn::UseTree::Path(use_path) => {
                builder.path = builder.path.join(IdentifierParser::new().parse(use_path.ident, config)?);
                builder.tree = (*use_path.tree).clone();
                self.parse(builder, config)
            },
            syn::UseTree::Name(name) => {
                builder.path = builder.path.join(IdentifierParser::new().parse(name.ident, config)?);
                Ok(vec![Import {
                    attributes: builder.attributes,
                    visibility: builder.visibility,
                    path: builder.path,
                    renaming: None
                }])
            },
            syn::UseTree::Rename(rename) => {
                builder.path = builder.path.join(IdentifierParser::new().parse(rename.ident, config)?);
                Ok(vec![Import {
                    attributes: builder.attributes,
                    visibility: builder.visibility,
                    path: builder.path,
                    renaming: Some(IdentifierParser::new().parse(rename.rename, config)?)
                }])
            },
            syn::UseTree::Glob(_) => {
                builder.path = builder.path.join("*");
                Ok(vec![Import {
                    attributes: builder.attributes,
                    visibility: builder.visibility,
                    path: builder.path,
                    renaming: None
                }])
            },
            syn::UseTree::Group(group) => {
                let mut imports: Vec<Import> = Default::default();
                for tree in group.items {
                    builder.tree = tree;
                    let mut child_imports = self.parse(builder.clone(), config)?;
                    imports.append(&mut child_imports);
                }
                Ok(imports)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ligen::parser::assert::*;
    use ligen::ir::module::import::mock;

    #[test]
    fn import() -> Result<()> {
        assert_eq(ImportsParser, mock::import(), quote! {
            #[custom(attribute)]
            pub use std::collections::HashMap;
        })
    }

    #[test]
    fn glob_import() -> Result<()> {
        assert_eq(ImportsParser, mock::glob_import(), quote! {
            #[custom(attribute)]
            pub use std::collections::*;
        })
    }

    #[test]
    fn renamed_import() -> Result<()> {
        assert_eq(ImportsParser, mock::renamed_import(), quote !{
            #[custom(attribute)]
            pub use std::collections::HashMap as Map;
        })
    }

    #[test]
    fn group_import() -> Result<()> {
        assert_eq(ImportsParser, mock::group_import(), quote! {
            #[custom(attribute)]
            pub use std::{
                collections::{
                    BinaryHeap as Heap,
                    HashMap
                },
                rc::Rc
            };
        })
    }
}