mod attribute;
pub use attribute::*;

use crate::ir::Identifier;
use crate::ir::Literal;
use crate::prelude::*;
use proc_macro2::TokenStream;
use quote::{quote, ToTokens, TokenStreamExt};
use std::convert::{TryFrom, TryInto};
use syn::{
    parse::{Parse, ParseStream},
    parse2, AttributeArgs, NestedMeta, Token,
};

#[derive(Shrinkwrap, Default, Debug, PartialEq, Clone)]
#[shrinkwrap(mutable)]
/// Attributes representation.
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

impl TryFrom<Vec<syn::Attribute>> for Attributes {
    type Error = Error;
    fn try_from(in_attributes: Vec<syn::Attribute>) -> Result<Self> {
        let mut attributes = Vec::new();
        for attribute in in_attributes {
            attributes.push(attribute.try_into()?);
        }
        Ok(Self { attributes })
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

impl TryFrom<proc_macro::TokenStream> for Attributes {
    type Error = Error;
    fn try_from(tokenstream: proc_macro::TokenStream) -> Result<Self> {
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
            if !input.is_empty() {
                input.parse::<Token![,]>()?;
            }
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
