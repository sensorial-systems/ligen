//! Import representation.

use crate::prelude::*;
use ligen::ir::{Path, Attributes, Visibility, Import};
use ligen::parser::prelude::*;
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

#[derive(Default)]
pub struct ImportsParser {
    attributes_parser: AttributesParser,
    visibility_parser: VisibilityParser,
    identifier_parser: IdentifierParser
}


impl Transformer<syn::ItemUse, Vec<Import>> for ImportsParser {
    fn transform(&self, import: syn::ItemUse, config: &Config) -> Result<Vec<Import>> {
        let attributes = self.attributes_parser.transform(import.attrs, config)?;
        let visibility = self.visibility_parser.transform(import.vis, config)?;
        let path = Path::default();
        let tree = import.tree;
        self.transform(ImportsBuilder { attributes, visibility, path, tree }, config)
    }
}

impl Transformer<proc_macro::TokenStream, Vec<Import>> for ImportsParser {
    fn transform(&self, input: proc_macro::TokenStream, config: &Config) -> Result<Vec<Import>> {
        self.transform(proc_macro2::TokenStream::from(input), config)
    }
}

impl Transformer<proc_macro2::TokenStream, Vec<Import>> for ImportsParser {
    fn transform(&self, input: proc_macro2::TokenStream, config: &Config) -> Result<Vec<Import>> {
        syn::parse2::<syn::ItemUse>(input)
            .map_err(|e| Error::Message(format!("Failed to parse imports: {:?}", e)))
            .and_then(|imports| self.transform(imports, config))
    }
}


impl Transformer<ImportsBuilder, Vec<Import>> for ImportsParser {
    fn transform(&self, builder: ImportsBuilder, config: &Config) -> Result<Vec<Import>> {
        let mut builder = builder;
        match builder.tree {
            syn::UseTree::Path(use_path) => {
                builder.path = builder.path.join(self.identifier_parser.transform(use_path.ident, config)?);
                builder.tree = (*use_path.tree).clone();
                self.transform(builder, config)
            },
            syn::UseTree::Name(name) => {
                builder.path = builder.path.join(self.identifier_parser.transform(name.ident, config)?);
                Ok(vec![Import {
                    attributes: builder.attributes,
                    visibility: builder.visibility,
                    path: builder.path,
                    renaming: None
                }])
            },
            syn::UseTree::Rename(rename) => {
                builder.path = builder.path.join(self.identifier_parser.transform(rename.ident, config)?);
                Ok(vec![Import {
                    attributes: builder.attributes,
                    visibility: builder.visibility,
                    path: builder.path,
                    renaming: Some(self.identifier_parser.transform(rename.rename, config)?)
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
                    let mut child_imports = self.transform(builder.clone(), config)?;
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
        assert_eq(ImportsParser::default(), mock::import(), quote! {
            #[custom(attribute)]
            pub use std::collections::HashMap;
        })
    }

    #[test]
    fn glob_import() -> Result<()> {
        assert_eq(ImportsParser::default(), mock::glob_import(), quote! {
            #[custom(attribute)]
            pub use std::collections::*;
        })
    }

    #[test]
    fn renamed_import() -> Result<()> {
        assert_eq(ImportsParser::default(), mock::renamed_import(), quote !{
            #[custom(attribute)]
            pub use std::collections::HashMap as Map;
        })
    }

    #[test]
    fn group_import() -> Result<()> {
        assert_eq(ImportsParser::default(), mock::group_import(), quote! {
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