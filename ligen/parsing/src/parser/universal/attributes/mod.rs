pub mod attribute;

use crate::prelude::*;
use crate::parser::Parser;
use ligen_ir::Attributes;
use attribute::AttributeParser;
use crate::parser::universal::attributes::attribute::LiteralParser;

#[derive(Default)]
pub struct AttributesParser<L: LiteralParser> {
    attribute_parser: AttributeParser<L>,
}

impl<L: LiteralParser> AttributesParser<L> {
    pub fn new(attribute_parser: AttributeParser<L>) -> Self {
        Self { attribute_parser }
    }
}

impl<L: LiteralParser> Parser<&str> for AttributesParser<L> {
    type Output = Attributes;
    fn parse(&self, input: &str) -> Result<Self::Output> {
        syn::parse_str::<syn2::punctuated::Punctuated<syn::NestedMeta, syn::token::Comma>>(input)
            .map_err(|e| Error::Message(format!("Failed to parse attributes: {:?}", e)))
            .and_then(|nested_metas| self.parse(nested_metas.0))
    }
}

impl<L: LiteralParser> Parser<Vec<syn::Attribute>> for AttributesParser<L> {
    type Output = Attributes;
    fn parse(&self, in_attributes: Vec<syn::Attribute>) -> Result<Self::Output> {
        let mut attributes = Vec::new();
        for attribute in in_attributes {
            attributes.push(self.attribute_parser.parse(attribute)?);
        }
        Ok(Self::Output { attributes })
    }
}

impl<L: LiteralParser> Parser<syn::punctuated::Punctuated<syn::NestedMeta, syn::token::Comma>> for AttributesParser<L> {
    type Output = Attributes;
    fn parse(&self, input: syn::punctuated::Punctuated<syn::NestedMeta, syn::token::Comma>) -> Result<Self::Output> {
        let attributes = input
            .into_iter()
            .map(|nested_meta| self.attribute_parser.parse(nested_meta).expect("Failed to parse nested meta."))
            .collect();
        Ok(Self::Output { attributes })
    }
}

impl<L: LiteralParser> Parser<syn::AttributeArgs> for AttributesParser<L> {
    type Output = Attributes;
    fn parse(&self, attribute_args: syn::AttributeArgs) -> Result<Self::Output> {
        let attributes = attribute_args
            .iter()
            .map(|nested_meta| self.attribute_parser.parse(nested_meta.clone()).expect("Failed to parse nested meta."))
            .collect();
        Ok(Self::Output { attributes })
    }
}

impl<L: LiteralParser> Parser<syn::MetaList> for AttributesParser<L> {
    type Output = Attributes;
    fn parse(&self, input: syn::MetaList) -> Result<Self::Output> {
        Ok(Self::Output {
            attributes: input
                .nested
                .into_iter()
                .map(|nested_meta| self.attribute_parser.parse(nested_meta).expect("Failed to parse nested meta."))
                .collect(),
        })
    }
}
