mod attribute;
pub use attribute::*;

use crate::prelude::*;
use ligen::parser::{Parser, ParserConfig};
use attribute::intermediary_attribute::IntermediaryAttribute;
use ligen::ir::{Attribute, Attributes, Literal};

#[derive(Default)]
pub struct AttributesParser {
    attribute_parser: AttributeParser,
}

impl Parser<String> for AttributesParser {
    type Output = Attributes;
    fn parse(&self, input: String, config: &ParserConfig) -> Result<Self::Output> {
        self.parse(input.as_str(), config)
    }
}

impl Parser<&str> for AttributesParser
{
    type Output = Attributes;
    fn parse(&self, input: &str, config: &ParserConfig) -> Result<Self::Output> {
        let attributes = syn::parse_str::<syn2::punctuated::Punctuated<IntermediaryAttribute, syn::token::Comma>>(input)
            .map_err(|e| Error::Message(format!("Failed to parse attributes: {}. Input: {}", e, input)))
            .and_then(|input| self.parse(input.0, config));
        if let Ok(attributes) = attributes {
            Ok(attributes)
        } else {
            Ok(Attributes::from(Attribute::from(Literal::from(input))))
        }
    }
}

impl Parser<Vec<syn::Attribute>> for AttributesParser {
    type Output = Attributes;
    fn parse(&self, in_attributes: Vec<syn::Attribute>, config: &ParserConfig) -> Result<Self::Output> {
        let mut attributes = Vec::new();
        for attribute in in_attributes {
            attributes.push(self.attribute_parser.parse(attribute, config)?);
        }
        Ok(Self::Output { attributes })
    }
}

impl Parser<syn::punctuated::Punctuated<IntermediaryAttribute, syn::token::Comma>> for AttributesParser
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

impl Parser<syn::punctuated::Punctuated<syn::Expr, syn::token::Comma>> for AttributesParser
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

impl Parser<syn::punctuated::Punctuated<syn::Meta, syn::token::Comma>> for AttributesParser {
    type Output = Attributes;
    fn parse(&self, input: syn::punctuated::Punctuated<syn::Meta, syn::token::Comma>, config: &ParserConfig) -> Result<Self::Output> {
        let attributes = input
            .into_iter()
            .map(|nested_meta| self.attribute_parser.parse(nested_meta, config).expect("Failed to parse nested meta."))
            .collect();
        Ok(Self::Output { attributes })
    }
}


#[cfg(test)]
mod test {
    use crate::prelude::*;
    use super::*;
    use ligen::ir::attributes::mock;
    use ligen::parser::assert::assert_eq;

    #[test]
    fn parse_literals() -> Result<()> {
        assert_eq(AttributesParser::default(), mock::parse_literals(), "c(marshal_as(name = \"hello\", uuid = 5), int = \"sized\")")
    }

    #[test]
    fn parse_attributes() -> Result<()> {
        assert_eq(AttributesParser::default(), mock::parse_attributes(), "c(int = \"sized\")")
    }

    #[test]
    fn parse_expressions() -> Result<()> {
        assert_eq(AttributesParser::default(), mock::parse_expressions(), r#"error("the {} field name: '{}' is invalid, path: {:?}", self.0.field_type, self.0.field_name, self.0.path)"#) // we need to make expressions valid.
    }
}
