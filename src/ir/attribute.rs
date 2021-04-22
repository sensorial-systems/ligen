use crate::prelude::*;

use crate::ir::Identifier;
use crate::ir::Literal;

use syn::{AttributeArgs, Meta, MetaList, MetaNameValue, NestedMeta, Path};

/// Attribute Enum
#[derive(Debug)]
pub enum Attribute {
    /// Literal Variant
    Literal(Literal),
    /// Named Variant
    Named(Identifier, Literal),
    /// Group Variant
    Group(Identifier, Attributes),
}

#[derive(Shrinkwrap, Default, Debug)]
#[shrinkwrap(mutable)]
/// Attributes Struct
pub struct Attributes {
    /// attributes field
    pub attributes: Vec<Attribute>,
}

impl From<AttributeArgs> for Attributes {
    fn from(attribute_args: AttributeArgs) -> Self {
        println!("attribute_args: {:#?}", attribute_args);
        let mut vec: Vec<Attribute> = vec![];
        for arg in attribute_args {
            vec.push(Attribute::from(arg));
        }
        Self { attributes: vec }
    }
}

impl From<MetaList> for Attribute {
    fn from(meta_list: MetaList) -> Self {
        Self::Group(
            Identifier::from(meta_list.path.segments.first().unwrap().ident.clone()),
            Attributes {
                attributes: vec![Attribute::from(meta_list.nested.first().unwrap().clone())],
            },
        )
    }
}

impl From<Path> for Attribute {
    fn from(path: Path) -> Self {
        Self::Literal(Literal::from(path.segments.first().unwrap().ident.clone()))
    }
}

impl From<Meta> for Attribute {
    fn from(meta: Meta) -> Self {
        match meta {
            syn::Meta::Path(path) => Self::from(path),
            syn::Meta::List(list) => Self::from(list),
            syn::Meta::NameValue(name_value) => Self::from(name_value),
        }
    }
}

impl From<MetaNameValue> for Attribute {
    fn from(meta_name_value: MetaNameValue) -> Self {
        Self::Named(
            Identifier::from(meta_name_value.path.segments.first().unwrap().ident.clone()),
            Literal::from(meta_name_value.lit),
        )
    }
}

impl From<NestedMeta> for Attribute {
    fn from(nested_meta: NestedMeta) -> Self {
        match nested_meta {
            NestedMeta::Meta(meta) => Self::from(meta),
            NestedMeta::Lit(lit) => Self::Literal(Literal::from(lit)),
        }
    }
}

#[cfg(test)]
mod test {

    #[test]
    fn attribute_literal() {}

    #[test]
    fn attribute_named() {}

    #[test]
    fn attribute_group() {}
}
