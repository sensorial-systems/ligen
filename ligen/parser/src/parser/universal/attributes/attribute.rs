//! Attribute enumeration.

use ligen_ir::macro_attributes::{Named, Group};
use syn::__private::ToTokens;
use crate::prelude::*;
use ligen_ir::Attribute;
use crate::parser::{Parser, ParserConfig};
use crate::parser::universal::identifier::IdentifierParser;
use crate::parser::universal::attributes::AttributesParser;
use crate::parser::universal::literal::LiteralParser;

#[derive(Default)]
pub struct AttributeParser<T: LiteralParser> {
    literal_parser: T
}

impl<T: LiteralParser> Parser<syn::ItemMacro> for AttributeParser<T> {
    type Output = Attribute;
    fn parse(&self, call: syn::ItemMacro, config: &ParserConfig) -> Result<Self::Output> {
        let identifier = call
            .mac
            .path
            .segments
            .last()
            .ok_or(Error::Message("Failed to get identifier from syn::ItemMacro".to_string()))?
            .ident
            .clone();
        let identifier = IdentifierParser::new().parse(identifier, config)?;
        let attributes = AttributesParser::<T>::default().parse(call.mac.tokens.to_string().as_str(), config)?;
        let group = Group::new(identifier, attributes).into();
        Ok(group)
    }
}

impl<T: LiteralParser> Parser<syn::MetaList> for AttributeParser<T> {
    type Output = Attribute;
    fn parse(&self, meta_list: syn::MetaList, config: &ParserConfig) -> Result<Self::Output> {
        let identifier = meta_list
            .path
            .segments
            .first()
            .ok_or(Error::Message("Failed to get identifier from syn::MetaList".to_string()))?
            .ident
            .clone();
        let identifier = IdentifierParser::new().parse(identifier, config)?;
        let attributes = AttributesParser::<T>::default().parse(meta_list, config)?;
        let group = Group::new(identifier, attributes);
        Ok(group.into())
    }
}

impl<T: LiteralParser> Parser<syn::Path> for AttributeParser<T> {
    type Output = Attribute;
    fn parse(&self, path: syn::Path, config: &ParserConfig) -> Result<Self::Output> {
        let identifier = path
            .segments
            .last()
            .ok_or(Error::Message("Failed to get identifier from syn::Path".to_string()))?
            .ident
            .clone();
        let identifier = IdentifierParser::new().parse(identifier, config)?;
        let attribute = Group::from(identifier).into();
        Ok(attribute)
    }
}


impl<T: LiteralParser> Parser<syn::MetaNameValue> for AttributeParser<T> {
    type Output = Attribute;
    fn parse(&self, meta_name_value: syn::MetaNameValue, config: &ParserConfig) -> Result<Self::Output> {
        let identifier = meta_name_value
            .path
            .segments
            .first()
            .ok_or(Error::Message("Failed to get identifier from syn::MetaNameValue".to_string()))?
            .ident
            .clone();
        let attribute = Named::new(IdentifierParser::new().parse(identifier, config)?, self.literal_parser.parse(meta_name_value.lit.to_token_stream().to_string(), config)?).into();
        Ok(attribute)
    }
}

impl<T: LiteralParser> Parser<syn::Meta> for AttributeParser<T> {
    type Output = Attribute;
    fn parse(&self, meta: syn::Meta, config: &ParserConfig) -> Result<Self::Output> {
        match meta {
            syn::Meta::Path(path) => self.parse(path, config),
            syn::Meta::List(list) => self.parse(list, config),
            syn::Meta::NameValue(name_value) => self.parse(name_value, config),
        }
    }
}

impl<T: LiteralParser> Parser<syn::NestedMeta> for AttributeParser<T> {
    type Output = Attribute;
    fn parse(&self, nested_meta: syn::NestedMeta, config: &ParserConfig) -> Result<Self::Output> {
        match nested_meta {
            syn::NestedMeta::Meta(meta) => self.parse(meta, config),
            syn::NestedMeta::Lit(lit) => Ok(Self::Output::Literal(self.literal_parser.parse(lit.to_token_stream().to_string(), config)?)),
        }
    }
}

impl<T: LiteralParser> Parser<syn::Attribute> for AttributeParser<T> {
    type Output = Attribute;
    fn parse(&self, attribute: syn::Attribute, config: &ParserConfig) -> Result<Self::Output> {
        attribute
            .parse_meta()
            .map_err(|e| Error::Message(format!("Failed to parse attribute: {:?} - {}", e, attribute.to_token_stream().to_string())))
            .and_then(|attribute| self.parse(attribute, config))
    }
}

impl<T: LiteralParser> Parser<String> for AttributeParser<T> {
    type Output = Attribute;
    fn parse(&self, input: String, config: &ParserConfig) -> Result<Self::Output> {
        self.parse(input.as_str(), config)
    }
}

impl<T: LiteralParser> Parser<&str> for AttributeParser<T> {
    type Output = Attribute;
    fn parse(&self, input: &str, config: &ParserConfig) -> Result<Self::Output> {
        syn::parse_str::<syn::NestedMeta>(input)
            .map_err(|e| Error::Message(format!("Failed to parse attribute: {:?} - {}", e, input)))
            .and_then(|attribute| self.parse(attribute, config))
    }
}