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
        let token_stream = proc_macro2::TokenStream::from(input);
        self.parse(token_stream)
    }
}

impl Parser<proc_macro2::TokenStream> for ImportsParser {
    type Output = Imports;
    fn parse(&self, input: proc_macro2::TokenStream) -> Result<Self::Output> {
        let import = syn::parse2::<syn::ItemUse>(input).expect("Failed to parse import.");
        self.parse(import)
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
    use ligen_ir::Attribute;

    fn attributes() -> Attributes {
        Attribute::Group("custom".into(), Attribute::Group("attribute".into(), Default::default()).into()).into()
    }

    #[test]
    fn import() -> Result<()> {
        let import = quote! {
            #[custom(attribute)]
            pub use std::collections::HashMap;
        };
        let imports = ImportsParser.parse(import)?;
        assert_eq!(imports, Imports(vec![
            Import {
                attributes: attributes(),
                visibility: Visibility::Public,
                path: Path::from("std::collections::HashMap"),
                renaming: None
            }
        ]));
        Ok(())
    }

    #[test]
    fn glob_import() -> Result<()> {
        let import = quote! {
            #[custom(attribute)]
            pub use std::collections::*;
        };
        let imports = ImportsParser.parse(import)?;
        assert_eq!(imports, Imports(vec![
            Import {
                attributes: attributes(),
                visibility: Visibility::Public,
                path: Path::from("std::collections::*"),
                renaming: None
            }
        ]));
        Ok(())
    }

    #[test]
    fn renamed_import() -> Result<()> {
        let import = quote! {
            #[custom(attribute)]
            pub use std::collections::HashMap as Map;
        };
        let imports = ImportsParser.parse(import)?;
        assert_eq!(imports, Imports(vec![
            Import {
                attributes: attributes(),
                visibility: Visibility::Public,
                path: Path::from("std::collections::HashMap"),
                renaming: Some("Map".into())
            }
        ]));
        Ok(())
    }

    #[test]
    fn group_import() -> Result<()> {
        let import = quote! {
            #[custom(attribute)]
            pub use std::{
                collections::{
                    BinaryHeap as Heap,
                    HashMap
                },
                rc::Rc
            };
        };
        let imports = ImportsParser.parse(import)?;
        assert_eq!(imports, Imports(vec![
            Import {
                attributes: attributes(),
                visibility: Visibility::Public,
                path: Path::from("std::collections::BinaryHeap"),
                renaming: Some("Heap".into())
            },
            Import {
                attributes: attributes(),
                visibility: Visibility::Public,
                path: Path::from("std::collections::HashMap"),
                renaming: None
            },
            Import {
                attributes: attributes(),
                visibility: Visibility::Public,
                path: Path::from("std::rc::Rc"),
                renaming: None
            },
        ]));
        Ok(())
    }
}