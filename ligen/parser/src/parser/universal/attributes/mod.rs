pub mod attribute;

use crate::prelude::*;
use crate::parser::{Parser, ParserConfig};
use attribute::intermediary_attribute::IntermediaryAttribute;
use ligen_ir::Attributes;
use attribute::AttributeParser;
use crate::parser::universal::literal::LiteralParser;

#[derive(Default)]
pub struct AttributesParser<L: LiteralParser> {
    attribute_parser: AttributeParser<L>,
}

impl<L: LiteralParser> AttributesParser<L> {
    pub fn new(attribute_parser: AttributeParser<L>) -> Self {
        Self { attribute_parser }
    }
}

impl<L: LiteralParser> Parser<String> for AttributesParser<L> {
    type Output = Attributes;
    fn parse(&self, input: String, config: &ParserConfig) -> Result<Self::Output> {
        self.parse(input.as_str(), config)
    }
}

impl<L: LiteralParser> Parser<&str> for AttributesParser<L>
{
    type Output = Attributes;
    fn parse(&self, input: &str, config: &ParserConfig) -> Result<Self::Output> {
        syn::parse_str::<syn2::punctuated::Punctuated<IntermediaryAttribute, syn::token::Comma>>(input)
            .map_err(|e| Error::Message(format!("Failed to parse attributes: {}. Input: {}", e, input)))
            .and_then(|input| self.parse(input.0, config))
    }
}

impl<L: LiteralParser> Parser<Vec<syn::Attribute>> for AttributesParser<L> {
    type Output = Attributes;
    fn parse(&self, in_attributes: Vec<syn::Attribute>, config: &ParserConfig) -> Result<Self::Output> {
        let mut attributes = Vec::new();
        for attribute in in_attributes {
            attributes.push(self.attribute_parser.parse(attribute, config)?);
        }
        Ok(Self::Output { attributes })
    }
}

impl<L: LiteralParser> Parser<syn::punctuated::Punctuated<IntermediaryAttribute, syn::token::Comma>> for AttributesParser<L>
{
    type Output = Attributes;
    fn parse(&self, input: syn::punctuated::Punctuated<IntermediaryAttribute, syn::token::Comma>, config: &ParserConfig) -> Result<Self::Output> {
        let mut attributes = Vec::new();
        for attribute in input {
            attributes.push(self.attribute_parser.parse(attribute, config)?);
        }
        Ok(Self::Output { attributes })
    }
}

impl<L: LiteralParser> Parser<syn::punctuated::Punctuated<syn::Expr, syn::token::Comma>> for AttributesParser<L>
{
    type Output = Attributes;
    fn parse(&self, input: syn::punctuated::Punctuated<syn::Expr, syn::token::Comma>, config: &ParserConfig) -> Result<Self::Output> {
        let attributes = input
            .into_iter()
            .map(|input| self.attribute_parser.parse(input, config).expect("Failed to parse nested meta."))
            .collect();
        Ok(Self::Output { attributes })
    }
}

impl<L: LiteralParser> Parser<syn::punctuated::Punctuated<syn::Meta, syn::token::Comma>> for AttributesParser<L> {
    type Output = Attributes;
    fn parse(&self, input: syn::punctuated::Punctuated<syn::Meta, syn::token::Comma>, config: &ParserConfig) -> Result<Self::Output> {
        let attributes = input
            .into_iter()
            .map(|nested_meta| self.attribute_parser.parse(nested_meta, config).expect("Failed to parse nested meta."))
            .collect();
        Ok(Self::Output { attributes })
    }
}
