//! Attribute enumeration.

use crate::prelude::*;
use crate::{Literal, Identifier, Attributes, Attribute};

impl TryFrom<SynItemMacro> for Attribute {
    type Error = Error;
    fn try_from(SynItemMacro(call): SynItemMacro) -> Result<Self> {
        Ok(Self::Group(SynIdent(call.mac.path.segments.last().expect("Failed to get identifier from syn::ItemMacro").ident.clone()).into(), ProcMacro2TokenStream::from(call.mac.tokens).try_into()?))
    }
}

impl From<SynMetaList> for Attribute {
    fn from(SynMetaList(meta_list): SynMetaList) -> Self {
        Self::Group(
            Identifier::from(SynIdent::from(meta_list.path.segments.first().unwrap().ident.clone())),
            Attributes {
                attributes: meta_list
                    .nested
                    .into_iter()
                    .map(|nested_meta| Attribute::from(SynNestedMeta::from(nested_meta)))
                    .collect(),
            },
        )
    }
}

impl From<SynPath> for Attribute {
    fn from(SynPath(path): SynPath) -> Self {
        Self::Group(Identifier::from(SynIdent::from(path.segments.first().unwrap().ident.clone())), Default::default())
    }
}

impl From<SynMeta> for Attribute {
    fn from(SynMeta(meta): SynMeta) -> Self {
        match meta {
            syn::Meta::Path(path) => Self::from(SynPath::from(path)),
            syn::Meta::List(list) => Self::from(SynMetaList::from(list)),
            syn::Meta::NameValue(name_value) => Self::from(SynMetaNameValue::from(name_value)),
        }
    }
}

impl From<SynMetaNameValue> for Attribute {
    fn from(SynMetaNameValue(meta_name_value): SynMetaNameValue) -> Self {
        Self::Named(
            Identifier::from(SynIdent::from(meta_name_value.path.segments.first().unwrap().ident.clone())),
            Literal::from(SynLit::from(meta_name_value.lit)),
        )
    }
}

impl From<SynNestedMeta> for Attribute {
    fn from(SynNestedMeta(nested_meta): SynNestedMeta) -> Self {
        match nested_meta {
            syn::NestedMeta::Meta(meta) => Self::from(SynMeta::from(meta)),
            syn::NestedMeta::Lit(lit) => Self::Literal(Literal::from(SynLit::from(lit))),
        }
    }
}

impl TryFrom<SynAttribute> for Attribute {
    type Error = Error;
    fn try_from(SynAttribute(attribute): SynAttribute) -> Result<Self> {
        let meta = SynMeta::from(attribute.parse_meta().map_err(|e| Error::Generic(Box::new(e)))?);
        Ok(meta.into())
    }
}
