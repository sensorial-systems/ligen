//! Attribute enumeration.

use crate::prelude::*;
use ligen_ir::Attribute;
use ligen_parsing::Parser;
use crate::identifier::IdentifierParser;
use crate::literal::LiteralParser;
use crate::macro_attributes::attributes::AttributesParser;

pub struct AttributeParser;

impl Parser<syn::ItemMacro> for AttributeParser {
    type Output = Attribute;
    fn parse(&self, call: syn::ItemMacro) -> Result<Self::Output> {
        Ok(Self::Output::Group(IdentifierParser.parse(call.mac.path.segments.last().expect("Failed to get identifier from syn::ItemMacro").ident.clone())?, AttributesParser.parse(call.mac.tokens)?))
    }
}

impl Parser<syn::MetaList> for AttributeParser {
    type Output = Attribute;
    fn parse(&self, meta_list: syn::MetaList) -> Result<Self::Output> {
        Ok(Self::Output::Group(
            IdentifierParser.parse(meta_list.path.segments.first().unwrap().ident.clone())?,
            AttributesParser.parse(meta_list)?,
        ))
    }
}

impl Parser<syn::Path> for AttributeParser {
    type Output = Attribute;
    fn parse(&self, path: syn::Path) -> Result<Self::Output> {
        Ok(Self::Output::Group(IdentifierParser.parse(path.segments.first().unwrap().ident.clone())?, Default::default()))
    }
}


impl Parser<syn::MetaNameValue> for AttributeParser {
    type Output = Attribute;
    fn parse(&self, meta_name_value: syn::MetaNameValue) -> Result<Self::Output> {
        Ok(Self::Output::Named(
            IdentifierParser.parse(meta_name_value.path.segments.first().unwrap().ident.clone())?,
            LiteralParser.parse(meta_name_value.lit)?,
        ))
    }
}

impl Parser<syn::Meta> for AttributeParser {
    type Output = Attribute;
    fn parse(&self, meta: syn::Meta) -> Result<Self::Output> {
        match meta {
            syn::Meta::Path(path) => self.parse(path),
            syn::Meta::List(list) => self.parse(list),
            syn::Meta::NameValue(name_value) => self.parse(name_value),
        }
    }
}

impl Parser<syn::NestedMeta> for AttributeParser {
    type Output = Attribute;
    fn parse(&self, nested_meta: syn::NestedMeta) -> Result<Self::Output> {
        match nested_meta {
            syn::NestedMeta::Meta(meta) => self.parse(meta),
            syn::NestedMeta::Lit(lit) => Ok(Self::Output::Literal(LiteralParser.parse(lit)?)),
        }
    }
}

impl Parser<syn::Attribute> for AttributeParser {
    type Output = Attribute;
    fn parse(&self, attribute: syn::Attribute) -> Result<Self::Output> {
        self.parse(attribute.parse_meta().map_err(|e| Error::Generic(Box::new(e)))?)
    }
}
