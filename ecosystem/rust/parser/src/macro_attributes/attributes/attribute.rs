//! Attribute enumeration.

use crate::prelude::*;
use ligen_ir::{Literal, Identifier, Attributes, Attribute};
use ligen_parsing::Parser;
use crate::macro_attributes::attributes::AttributesParser;

pub struct AttributeParser;

impl Parser<syn::ItemMacro> for AttributeParser {
    type Output = Attribute;
    fn parse(&self, call: syn::ItemMacro) -> Result<Self::Output> {
        Ok(Self::Output::Group(SynIdent(call.mac.path.segments.last().expect("Failed to get identifier from syn::ItemMacro").ident.clone()).into(), AttributesParser.parse(call.mac.tokens)?))
    }
}

impl Parser<syn::MetaList> for AttributeParser {
    type Output = Attribute;
    fn parse(&self, meta_list: syn::MetaList) -> Result<Self::Output> {
        Ok(Self::Output::Group(
            Identifier::from(SynIdent::from(meta_list.path.segments.first().unwrap().ident.clone())),
            Attributes {
                attributes: meta_list
                    .nested
                    .into_iter()
                    .map(|nested_meta| AttributeParser.parse(nested_meta).expect("Failed to parse nested meta."))
                    .collect(),
            },
        ))
    }
}

impl Parser<syn::Path> for AttributeParser {
    type Output = Attribute;
    fn parse(&self, path: syn::Path) -> Result<Self::Output> {
        Ok(Self::Output::Group(Identifier::from(SynIdent::from(path.segments.first().unwrap().ident.clone())), Default::default()))
    }
}


impl Parser<syn::MetaNameValue> for AttributeParser {
    type Output = Attribute;
    fn parse(&self, meta_name_value: syn::MetaNameValue) -> Result<Self::Output> {
        Ok(Self::Output::Named(
            Identifier::from(SynIdent::from(meta_name_value.path.segments.first().unwrap().ident.clone())),
            Literal::from(SynLit::from(meta_name_value.lit)),
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
            syn::NestedMeta::Lit(lit) => Ok(Self::Output::Literal(Literal::from(SynLit::from(lit)))),
        }
    }
}

impl Parser<syn::Attribute> for AttributeParser {
    type Output = Attribute;
    fn parse(&self, attribute: syn::Attribute) -> Result<Self::Output> {
        self.parse(attribute.parse_meta().map_err(|e| Error::Generic(Box::new(e)))?)
    }
}
