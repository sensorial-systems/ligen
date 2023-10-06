//! Attribute enumeration.

use syn::__private::ToTokens;
use crate::prelude::*;
use ligen_ir::Attribute;
use crate::parser::Parser;
use crate::parser::universal::identifier::IdentifierParser;
use crate::parser::universal::attributes::AttributesParser;
use crate::parser::universal::literal::LiteralParser;

#[derive(Default)]
pub struct AttributeParser<T: LiteralParser> {
    literal_parser: T
}

impl<T: LiteralParser> Parser<syn::ItemMacro> for AttributeParser<T> {
    type Output = Attribute;
    fn parse(&self, call: syn::ItemMacro) -> Result<Self::Output> {
        let identifier = call
            .mac
            .path
            .segments
            .last()
            .ok_or(Error::Message("Failed to get identifier from syn::ItemMacro".to_string()))?
            .ident
            .clone();
        Ok(Self::Output::Group(IdentifierParser::new().parse(identifier)?, AttributesParser::<T>::default().parse(call.mac.tokens.to_string().as_str())?))
    }
}

impl<T: LiteralParser> Parser<syn::MetaList> for AttributeParser<T> {
    type Output = Attribute;
    fn parse(&self, meta_list: syn::MetaList) -> Result<Self::Output> {
        let identifier = meta_list
            .path
            .segments
            .first()
            .ok_or(Error::Message("Failed to get identifier from syn::MetaList".to_string()))?
            .ident
            .clone();
        Ok(Self::Output::Group(
            IdentifierParser::new().parse(identifier)?,
            AttributesParser::<T>::default().parse(meta_list)?,
        ))
    }
}

impl<T: LiteralParser> Parser<syn::Path> for AttributeParser<T> {
    type Output = Attribute;
    fn parse(&self, path: syn::Path) -> Result<Self::Output> {
        let identifier = path
            .segments
            .last()
            .ok_or(Error::Message("Failed to get identifier from syn::Path".to_string()))?
            .ident
            .clone();
        Ok(Self::Output::Group(IdentifierParser::new().parse(identifier)?, Default::default()))
    }
}


impl<T: LiteralParser> Parser<syn::MetaNameValue> for AttributeParser<T> {
    type Output = Attribute;
    fn parse(&self, meta_name_value: syn::MetaNameValue) -> Result<Self::Output> {
        let identifier = meta_name_value
            .path
            .segments
            .first()
            .ok_or(Error::Message("Failed to get identifier from syn::MetaNameValue".to_string()))?
            .ident
            .clone();
        Ok(Self::Output::Named(IdentifierParser::new().parse(identifier)?, self.literal_parser.parse(meta_name_value.lit.to_token_stream().to_string())?))
    }
}

impl<T: LiteralParser> Parser<syn::Meta> for AttributeParser<T> {
    type Output = Attribute;
    fn parse(&self, meta: syn::Meta) -> Result<Self::Output> {
        match meta {
            syn::Meta::Path(path) => self.parse(path),
            syn::Meta::List(list) => self.parse(list),
            syn::Meta::NameValue(name_value) => self.parse(name_value),
        }
    }
}

impl<T: LiteralParser> Parser<syn::NestedMeta> for AttributeParser<T> {
    type Output = Attribute;
    fn parse(&self, nested_meta: syn::NestedMeta) -> Result<Self::Output> {
        match nested_meta {
            syn::NestedMeta::Meta(meta) => self.parse(meta),
            syn::NestedMeta::Lit(lit) => Ok(Self::Output::Literal(self.literal_parser.parse(lit.to_token_stream().to_string())?)),
        }
    }
}

impl<T: LiteralParser> Parser<syn::Attribute> for AttributeParser<T> {
    type Output = Attribute;
    fn parse(&self, attribute: syn::Attribute) -> Result<Self::Output> {
        attribute
            .parse_meta()
            .map_err(|e| Error::Message(format!("Failed to parse attribute: {:?}", e)))
            .and_then(|attribute| self.parse(attribute))
    }
}

impl<T: LiteralParser> Parser<String> for AttributeParser<T> {
    type Output = Attribute;
    fn parse(&self, input: String) -> Result<Self::Output> {
        self.parse(input.as_str())
    }
}

impl<T: LiteralParser> Parser<&str> for AttributeParser<T> {
    type Output = Attribute;
    fn parse(&self, input: &str) -> Result<Self::Output> {
        syn::parse_str::<syn::NestedMeta>(input)
            .map_err(|e| Error::Message(format!("Failed to parse attribute: {:?}", e)))
            .and_then(|attribute| self.parse(attribute))
    }
}