use crate::ir::Identifier;
use crate::ir::Literal;
use crate::prelude::*;
use crate::procedural_macro;
use proc_macro2::TokenStream;
use quote::{quote, ToTokens, TokenStreamExt};
use std::convert::{TryFrom, TryInto};
use syn::{
    parse::{Parse, ParseStream},
    parse2, AttributeArgs, Meta, MetaList, MetaNameValue, NestedMeta, Path, Token,
};

/// Attribute Enum
#[derive(Debug, PartialEq, Clone)]
pub enum Attribute {
    /// Literal Variant
    Literal(Literal),
    /// Named Variant
    Named(Identifier, Literal),
    /// Group Variant
    Group(Identifier, Attributes),
}

#[derive(Shrinkwrap, Default, Debug, PartialEq, Clone)]
#[shrinkwrap(mutable)]
/// Attributes Struct
pub struct Attributes {
    /// attributes field
    pub attributes: Vec<Attribute>,
}

impl Attributes {
    /// Get named attribute e.g.: name = "literal"
    pub fn get_named(&self, name: &str) -> Option<Literal> {
        self
            .attributes
            .iter()
            .find_map(|attribute| {
                if let Attribute::Named(identifier, literal) = attribute {
                    if identifier.name == name {
                        Some(literal.clone())
                    } else {
                        None
                    }
                } else {
                    None
                }
            })
    }
}

impl From<Vec<Attribute>> for Attributes {
    fn from(attributes: Vec<Attribute>) -> Self {
        Self { attributes }
    }
}

impl From<Attribute> for Attributes {
    fn from(attribute: Attribute) -> Self {
        let attributes = vec![attribute];
        Self { attributes }
    }
}

impl TryFrom<TokenStream> for Attributes {
    type Error = Error;
    fn try_from(tokenstream: TokenStream) -> Result<Self> {
        parse2::<Attributes>(tokenstream).map_err(|_| "Failed to parse Attributes".into())
    }
}

impl TryFrom<procedural_macro::TokenStream> for Attributes {
    type Error = Error;
    fn try_from(tokenstream: procedural_macro::TokenStream) -> Result<Self> {
        let tokenstream: TokenStream = tokenstream.into();
        tokenstream.try_into()
    }
}

impl From<AttributeArgs> for Attributes {
    fn from(attribute_args: AttributeArgs) -> Self {
        let attributes = attribute_args
            .iter()
            .map(|nested_meta| Attribute::from(nested_meta.clone()))
            .collect();
        Self { attributes }
    }
}

impl From<MetaList> for Attribute {
    fn from(meta_list: MetaList) -> Self {
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

impl From<Path> for Attribute {
    fn from(path: Path) -> Self {
        Self::Group(Identifier::from(path.segments.first().unwrap().ident.clone()), Default::default())
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

impl ToTokens for Attributes {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let length = self.attributes.len();
        for (index, attribute) in self.attributes.iter().enumerate() {
            tokens.append_all(quote! { #attribute });
            if index != length - 1 {
                tokens.append_all(quote! { , });
            }
        }
    }
}

impl ToTokens for Attribute {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        match self {
            Attribute::Literal(literal) => {
                tokens.append_all(quote! {#literal})
            }
            Attribute::Named(_, _) => panic!("Named variant should only be used inside groups"),
            Attribute::Group(identifier, group) => {
                let mut attributes = TokenStream::new();
                group
                    .attributes
                    .clone()
                    .into_iter()
                    .enumerate()
                    .for_each(|x| {
                        if let (index, Attribute::Named(identifier, lit)) = x {
                            let name = Identifier::new(&identifier.name);
                            attributes.append_all(quote! {#name = #lit});
                            if index + 1 < group.attributes.len() {
                                attributes.append_all(quote! {, })
                            }
                        } else {
                            panic!("Group contains Non Named variant")
                        }
                    });

                tokens.append_all(quote! {#identifier(#attributes)})
            }
        }
    }
}

impl Parse for Attributes {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut metas: Vec<NestedMeta> = Vec::new();

        while !input.is_empty() {
            let value = input.parse()?;
            metas.push(value);
            if input.is_empty() {
                break;
            }
            input.parse::<Token![,]>()?;
        }
        Ok(Attributes::from(metas))
    }
}

#[cfg(test)]
mod test {
    use crate::ir::{Attribute, Attributes, Identifier, Literal};
    use quote::quote;
    use syn::{parse2, NestedMeta};

    #[test]
    fn attribute_literal() {
        let args: NestedMeta = syn::parse_quote!("C");
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

    #[test]
    fn parse_attributes() {
        assert_eq!(
            Attributes {
                attributes: vec![Attribute::Group(
                    Identifier::new("c"),
                    Attributes {
                        attributes: vec![Attribute::Named(
                            Identifier::new("int"),
                            Literal::String(String::from("sized"))
                        )]
                    }
                )]
            },
            parse2::<Attributes>(quote! {c(int = "sized")}).expect("Failed to parse Attributes")
        );
    }
}
