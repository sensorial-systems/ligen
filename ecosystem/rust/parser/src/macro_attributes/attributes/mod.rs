mod attribute;

pub use attribute::*;

use crate::prelude::*;
use ligen::ir::{Attribute, Attributes, Identifier};
use ligen::parsing::Parser;

pub struct AttributesParser;

impl Parser<Vec<syn::Attribute>> for AttributesParser {
    type Output = Attributes;
    fn parse(&self, in_attributes: Vec<syn::Attribute>) -> Result<Self::Output> {
        let mut attributes = Vec::new();
        for attribute in in_attributes {
            attributes.push(AttributeParser.parse(attribute)?);
        }
        Ok(Self::Output { attributes })
    }
}

impl Parser<proc_macro2::TokenStream> for AttributesParser {
    type Output = Attributes;
    fn parse(&self, tokenstream: proc_macro2::TokenStream) -> Result<Self::Output> {
        syn::parse2::<syn2::punctuated::Punctuated<syn::NestedMeta, syn::token::Comma>>(tokenstream)
            .map_err(|e| Error::Message(format!("Failed to parse attributes: {:?}", e)))
            .and_then(|nested_metas| self.parse(nested_metas.0))
    }
}

impl Parser<syn::punctuated::Punctuated<syn::NestedMeta, syn::token::Comma>> for AttributesParser {
    type Output = Attributes;
    fn parse(&self, input: syn::punctuated::Punctuated<syn::NestedMeta, syn::token::Comma>) -> Result<Self::Output> {
        let attributes = input
            .into_iter()
            .map(|nested_meta| AttributeParser.parse(nested_meta).expect("Failed to parse nested meta."))
            .collect();
        Ok(Self::Output { attributes })
    }
}

impl Parser<proc_macro::TokenStream> for AttributesParser {
    type Output = Attributes;
    fn parse(&self, token_stream: proc_macro::TokenStream) -> Result<Self::Output> {
        self.parse(proc_macro2::TokenStream::from(token_stream))
    }
}

impl Parser<syn::AttributeArgs> for AttributesParser {
    type Output = Attributes;
    fn parse(&self, attribute_args: syn::AttributeArgs) -> Result<Self::Output> {
        let attributes = attribute_args
            .iter()
            .map(|nested_meta| AttributeParser.parse(nested_meta.clone()).expect("Failed to parse nested meta."))
            .collect();
        Ok(Self::Output { attributes })
    }
}

impl Parser<syn::MetaList> for AttributesParser {
    type Output = Attributes;
    fn parse(&self, input: syn::MetaList) -> Result<Self::Output> {
        Ok(Self::Output {
            attributes: input
                .nested
                .into_iter()
                .map(|nested_meta| AttributeParser.parse(nested_meta).expect("Failed to parse nested meta."))
                .collect(),
        })
    }
}


impl ToTokens for Attributes {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let length = self.attributes.len();
        for (index, attribute) in self.attributes.iter().enumerate() {
            let attribute = attribute.to_token_stream();
            tokens.append_all(quote! { #attribute });
            if index != length - 1 {
                tokens.append_all(quote! { , });
            }
        }
    }
}

impl ToTokens for Attribute {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        match self {
            Attribute::Literal(literal) => {
                let literal = literal.to_token_stream();
                tokens.append_all(quote! {#literal})
            }
            Attribute::Named(_, _) => panic!("Named variant should only be used inside groups"),
            Attribute::Group(identifier, group) => {
                let mut attributes = proc_macro2::TokenStream::new();
                group
                    .attributes
                    .clone()
                    .into_iter()
                    .enumerate()
                    .for_each(|x| {
                        if let (index, Attribute::Named(identifier, lit)) = x {
                            let name = Identifier::new(&identifier.name).to_token_stream();
                            let lit = lit.to_token_stream();
                            attributes.append_all(quote! {#name = #lit});
                            if index + 1 < group.attributes.len() {
                                attributes.append_all(quote! {, })
                            }
                        } else {
                            panic!("Group contains Non Named variant")
                        }
                    });

                let identifier = identifier.to_token_stream();
                tokens.append_all(quote! {#identifier(#attributes)})
            }
        }
    }
}

#[cfg(test)]
mod test {
    use quote::quote;
    use super::*;
    use ligen::ir::attributes::mock;
    use ligen::parsing::assert::assert_eq;

    #[test]
    fn parse_literals() -> Result<()> {
        assert_eq(AttributesParser, mock::parse_literals(), quote! {
            c(marshal_as(name = "hello", uuid = 5), int = "sized")
        })
    }

    #[test]
    fn parse_attributes() -> Result<()> {
        assert_eq(AttributesParser, mock::parse_attributes(), quote! {
            c(int = "sized")
        })
    }
}
