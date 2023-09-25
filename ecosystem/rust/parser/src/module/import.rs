//! Import representation.

use crate::prelude::*;
use ligen_ir::{Path, Attributes, Visibility, Imports, Import};
use ligen_parsing::Parser;
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

impl Parser<&[syn::Item]> for ImportsParser {
    type Output = Imports;
    fn parse(&self, items: &[syn::Item]) -> Result<Self::Output> {
        let mut imports = Imports::default();
        for item in items {
            if let syn::Item::Use(import) = item {
                imports.0.append(&mut (ImportsParser.parse(import.clone())?).0);
            }
        }
        Ok(imports)
    }
}

pub struct ImportsParser;

impl Parser<syn::ItemUse> for ImportsParser {
    type Output = Imports;
    fn parse(&self, import: syn::ItemUse) -> Result<Self::Output> {
        let attributes = AttributesParser.parse(import.attrs)?;
        let visibility = VisibilityParser.parse(import.vis)?;
        let path = Path::default();
        ImportsBuilder { attributes, visibility, path, tree: import.tree }.try_into()
    }
}

impl Parser<proc_macro::TokenStream> for ImportsParser {
    type Output = Imports;
    fn parse(&self, input: proc_macro::TokenStream) -> Result<Self::Output> {
        self.parse(proc_macro2::TokenStream::from(input))
    }
}

impl Parser<proc_macro2::TokenStream> for ImportsParser {
    type Output = Imports;
    fn parse(&self, input: proc_macro2::TokenStream) -> Result<Self::Output> {
        syn::parse2::<syn::ItemUse>(input)
            .map_err(|e| Error::Message(format!("Failed to parse imports: {:?}", e)))
            .and_then(|imports| self.parse(imports))
    }
}


impl TryFrom<ImportsBuilder> for Imports {
    type Error = Error;
    fn try_from(builder: ImportsBuilder) -> Result<Self> {
        let mut builder = builder;
        match builder.tree {
            syn::UseTree::Path(use_path) => {
                builder.path = builder.path.join(IdentifierParser.parse(use_path.ident)?);
                builder.tree = (*use_path.tree).clone();
                builder.try_into()
            },
            syn::UseTree::Name(name) => {
                builder.path = builder.path.join(IdentifierParser.parse(name.ident)?);
                Ok(Self(vec![Import {
                    attributes: builder.attributes,
                    visibility: builder.visibility,
                    path: builder.path,
                    renaming: None
                }]))
            },
            syn::UseTree::Rename(rename) => {
                builder.path = builder.path.join(IdentifierParser.parse(rename.ident)?);
                Ok(Self(vec![Import {
                    attributes: builder.attributes,
                    visibility: builder.visibility,
                    path: builder.path,
                    renaming: Some(IdentifierParser.parse(rename.rename)?)
                }]))
            },
            syn::UseTree::Glob(_) => {
                builder.path = builder.path.join("*");
                Ok(Self(vec![Import {
                    attributes: builder.attributes,
                    visibility: builder.visibility,
                    path: builder.path,
                    renaming: None
                }]))
            },
            syn::UseTree::Group(group) => {
                let mut imports = Imports::default();
                for tree in group.items {
                    builder.tree = tree;
                    let mut child_imports = Imports::try_from(builder.clone())?;
                    imports.0.append(&mut child_imports.0);
                }
                Ok(imports)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ligen_parsing::assert::*;
    use ligen_ir::module::import::mock;

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