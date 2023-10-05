//! Attribute enumeration.

use proc_macro::TokenStream;
use crate::prelude::*;
use ligen::ir::Attribute;
use ligen::parsing::parser::Parser;
use crate::literal::LiteralParser;

type InnerAttributeParser = ligen::parsing::parser::universal::attributes::attribute::AttributeParser<LiteralParser>;
pub struct AttributeParser;

impl Parser<syn::ItemMacro> for AttributeParser {
    type Output = Attribute;
    fn parse(&self, input: syn::ItemMacro) -> Result<Self::Output> {
        InnerAttributeParser::default().parse(input)
    }
}

impl Parser<syn::MetaList> for AttributeParser {
    type Output = Attribute;
    fn parse(&self, input: syn::MetaList) -> Result<Self::Output> {
        InnerAttributeParser::default().parse(input)
    }
}

impl Parser<syn::Path> for AttributeParser {
    type Output = Attribute;
    fn parse(&self, input: syn::Path) -> Result<Self::Output> {
        InnerAttributeParser::default().parse(input)
    }
}


impl Parser<syn::MetaNameValue> for AttributeParser {
    type Output = Attribute;
    fn parse(&self, input: syn::MetaNameValue) -> Result<Self::Output> {
        InnerAttributeParser::default().parse(input)
    }
}

impl Parser<syn::Meta> for AttributeParser {
    type Output = Attribute;
    fn parse(&self, input: syn::Meta) -> Result<Self::Output> {
        InnerAttributeParser::default().parse(input)
    }
}

impl Parser<syn::NestedMeta> for AttributeParser {
    type Output = Attribute;
    fn parse(&self, input: syn::NestedMeta) -> Result<Self::Output> {
        InnerAttributeParser::default().parse(input)
    }
}

impl Parser<syn::Attribute> for AttributeParser {
    type Output = Attribute;
    fn parse(&self, input: syn::Attribute) -> Result<Self::Output> {
        InnerAttributeParser::default().parse(input)
    }
}

impl Parser<&str> for AttributeParser {
    type Output = Attribute;
    fn parse(&self, input: &str) -> Result<Self::Output> {
        InnerAttributeParser::default().parse(input)
    }
}

impl Parser<proc_macro::TokenStream> for AttributeParser {
    type Output = Attribute;
    fn parse(&self, token_stream: TokenStream) -> Result<Self::Output> {
        let token_stream = proc_macro2::TokenStream::from(token_stream);
        self.parse(token_stream)
    }
}

impl Parser<proc_macro2::TokenStream> for AttributeParser {
    type Output = Attribute;
    fn parse(&self, input: proc_macro2::TokenStream) -> Result<Self::Output> {
        syn::parse2::<syn::NestedMeta>(input)
            .map_err(|e| Error::Message(format!("Failed to parse attribute: {:?}", e)))
            .and_then(|nested_meta| self.parse(nested_meta))
    }
}

#[cfg(test)]
mod test {
    use ligen::parsing::assert::assert_eq;
    use super::*;

    use ligen::ir::attribute::mock;

    #[test]
    fn attribute_literal() -> Result<()> {
        assert_eq(AttributeParser, mock::attribute_literal(), "\"c\"")
    }

    #[test]
    fn attribute_named() -> Result<()> {
        assert_eq(AttributeParser, mock::attribute_named(), "int = \"sized\"")
    }

    #[test]
    fn attribute_group() -> Result<()> {
        assert_eq(AttributeParser, mock::attribute_group(), "c(int = \"sized\")")
    }

    #[test]
    fn attribute_empty_group() -> Result<()> {
        assert_eq(AttributeParser, mock::attribute_empty_group(), "c()")?;
        assert_eq(AttributeParser, mock::attribute_empty_group(), "c")
    }
}