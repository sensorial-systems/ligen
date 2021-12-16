//! Attribute enumeration.

use crate::prelude::*;
use ligen_ir::{Literal, Identifier, Attributes, Attribute};

impl TryFrom<syn::ItemMacro> for Attribute {
    type Error = Error;
    fn try_from(call: syn::ItemMacro) -> Result<Self> {
        Ok(Self::Group(call.mac.path.segments.last().expect("Failed to get identifier from syn::ItemMacro").ident.clone().into(), call.mac.tokens.try_into()?))
    }
}

impl From<syn::MetaList> for Attribute {
    fn from(meta_list: syn::MetaList) -> Self {
        Self::Group(
            Identifier::from(meta_list.path.segments.first().unwrap().ident.clone()),
            Attributes {
                attributes: meta_list
                    .nested
                    .into_iter()
                    .map(|nested_meta| Attribute::from(nested_meta))
                    .collect(),
            },
        )
    }
}

impl From<syn::Path> for Attribute {
    fn from(path: syn::Path) -> Self {
        Self::Group(Identifier::from(path.segments.first().unwrap().ident.clone()), Default::default())
    }
}

impl From<syn::Meta> for Attribute {
    fn from(meta: syn::Meta) -> Self {
        match meta {
            syn::Meta::Path(path) => Self::from(path),
            syn::Meta::List(list) => Self::from(list),
            syn::Meta::NameValue(name_value) => Self::from(name_value),
        }
    }
}

impl From<syn::MetaNameValue> for Attribute {
    fn from(meta_name_value: syn::MetaNameValue) -> Self {
        Self::Named(
            Identifier::from(meta_name_value.path.segments.first().unwrap().ident.clone()),
            Literal::from(meta_name_value.lit),
        )
    }
}

impl From<syn::NestedMeta> for Attribute {
    fn from(nested_meta: syn::NestedMeta) -> Self {
        match nested_meta {
            syn::NestedMeta::Meta(meta) => Self::from(meta),
            syn::NestedMeta::Lit(lit) => Self::Literal(Literal::from(lit)),
        }
    }
}

impl TryFrom<syn::Attribute> for Attribute {
    type Error = Error;
    fn try_from(attribute: syn::Attribute) -> Result<Self> {
        let meta = attribute.parse_meta()?;
        Ok(meta.into())
    }
}
