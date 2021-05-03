use crate::ir::Identifier;
use crate::ir::Literal;
use crate::prelude::*;
use syn::{AttributeArgs, Meta, MetaList, MetaNameValue, NestedMeta, Path};

/// Attribute Enum
#[derive(Debug, PartialEq)]
pub enum Attribute {
    /// Literal Variant
    Literal(Literal),
    /// Named Variant
    Named(Identifier, Literal),
    /// Group Variant
    Group(Identifier, Attributes),
}

#[derive(Shrinkwrap, Default, Debug, PartialEq)]
#[shrinkwrap(mutable)]
/// Attributes Struct
pub struct Attributes {
    /// attributes field
    pub attributes: Vec<Attribute>,
}

impl From<AttributeArgs> for Attributes {
    fn from(attribute_args: AttributeArgs) -> Self {
        let attributes = attribute_args
            .iter()
            .map(|a| Attribute::from(a.clone()))
            .collect();
        Self { attributes }
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
    use crate::ir::{Attribute, Attributes, Identifier, Literal};
    use syn::NestedMeta;

    #[test]
    fn attribute_literal() {
        let args: NestedMeta = syn::parse_quote!(C);
        let attr: Attribute = args.into();
        assert_eq!(attr, Attribute::Literal(Literal::String(String::from("C"))))
    }

    #[test]
    fn attribute_named() {
        let args: NestedMeta = syn::parse_quote!(int = "sized");
        let attr: Attribute = args.into();
        assert_eq!(
            attr,
            Attribute::Named(
                Identifier::new("int"),
                Literal::String(String::from("sized"))
            )
        )
    }

    #[test]
    fn attribute_group() {
        let args: NestedMeta = syn::parse_quote!(C(int = "sized"));
        let attr: Attribute = args.into();
        assert_eq!(
            attr,
            Attribute::Group(
                Identifier::new("C"),
                Attributes {
                    attributes: vec![Attribute::Named(
                        Identifier::new("int"),
                        Literal::String(String::from("sized"))
                    )]
                }
            )
        )
    }
}
