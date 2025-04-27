mod attribute;
pub use attribute::*;

use crate::prelude::*;
use ligen::parser::prelude::*;
use attribute::intermediary_attribute::IntermediaryAttribute;
use ligen::ir::{Attribute, Attributes, Literal};

#[derive(Default)]
pub struct AttributesParser {
    attribute_parser: AttributeParser,
}

impl Parser<Attributes> for AttributesParser {
    fn parse(&self, input: impl AsRef<str>, config: &Config) -> Result<Attributes> {
        let input = input.as_ref();
        let attributes = syn::parse_str::<syn2::punctuated::Punctuated<IntermediaryAttribute, syn::token::Comma>>(input)
            .map_err(|e| Error::Message(format!("Failed to parse attributes: {}. Input: {}", e, input)))
            .and_then(|input| self.transform(input.0, config));
        if let Ok(attributes) = attributes {
            Ok(attributes)
        } else {
            Ok(Attributes::from(Attribute::from(Literal::from(input))))
        }
    }
}

impl Transformer<Vec<syn::Attribute>, Attributes> for AttributesParser {
    fn transform(&self, in_attributes: Vec<syn::Attribute>, config: &Config) -> Result<Attributes> {
        let mut attributes = Vec::new();
        for attribute in in_attributes {
            attributes.push(self.attribute_parser.transform(attribute, config)?);
        }
        Ok(Attributes { attributes })
    }
}

impl Transformer<syn::punctuated::Punctuated<IntermediaryAttribute, syn::token::Comma>, Attributes> for AttributesParser {
    fn transform(&self, input: syn::punctuated::Punctuated<IntermediaryAttribute, syn::token::Comma>, config: &Config) -> Result<Attributes> {
        let mut attributes = Vec::new();
        for attribute in input {
            attributes.push(self.attribute_parser.transform(attribute, config)?);
        }
        Ok(Attributes { attributes })
    }
}

impl Transformer<syn::punctuated::Punctuated<syn::Expr, syn::token::Comma>, Attributes> for AttributesParser {
    fn transform(&self, input: syn::punctuated::Punctuated<syn::Expr, syn::token::Comma>, config: &Config) -> Result<Attributes> {
        let attributes = input
            .into_iter()
            .map(|input| self.attribute_parser.transform(input, config).expect("Failed to parse nested meta."))
            .collect();
        Ok(Attributes { attributes })
    }
}

impl Transformer<syn::punctuated::Punctuated<syn::Meta, syn::token::Comma>, Attributes> for AttributesParser {
    fn transform(&self, input: syn::punctuated::Punctuated<syn::Meta, syn::token::Comma>, config: &Config) -> Result<Attributes> {
        let attributes = input
            .into_iter()
            .map(|nested_meta| self.attribute_parser.transform(nested_meta, config).expect("Failed to parse nested meta."))
            .collect();
        Ok(Attributes { attributes })
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
