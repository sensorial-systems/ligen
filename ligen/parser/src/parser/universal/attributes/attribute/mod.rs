//! Attribute enumeration.

use intermediary_attribute::IntermediaryAttribute;
use ligen_ir::macro_attributes::{Named, Group};
use syn::__private::ToTokens;
use crate::prelude::*;
use crate::universal::parser::PathParser;
use ligen_ir::{Attribute, Literal};
use crate::parser::{Parser, ParserConfig};
use crate::parser::universal::identifier::IdentifierParser;
use crate::parser::universal::attributes::AttributesParser;
use crate::parser::universal::literal::LiteralParser;

pub(crate) mod intermediary_attribute;

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
        println!("4");
        let path = PathParser::default().parse(meta_list.path.clone(), config)?;
        let inner = meta_list.tokens.into_iter().map(|token| token.to_string()).collect::<Vec<_>>().join("");
        let attributes = AttributesParser::<T>::default().parse(inner.as_str(), config)?;
        let group = Group::new(path, attributes);
        Ok(group.into())
    }
}

impl<T: LiteralParser> Parser<syn::Lit> for AttributeParser<T> {
    type Output = Attribute;
    fn parse(&self, lit: syn::Lit, config: &ParserConfig) -> Result<Self::Output> {
        self.literal_parser.parse(lit.to_token_stream().to_string(), config).map(Attribute::Literal)
    }
}

impl<T: LiteralParser> Parser<syn::ExprCall> for AttributeParser<T> {
    type Output = Attribute;
    fn parse(&self, expr_call: syn::ExprCall, config: &ParserConfig) -> Result<Self::Output> {
        let identifier = expr_call
            .func
            .to_token_stream()
            .to_string();
        let identifier = IdentifierParser::new().parse(identifier, config)?;
        let attributes = AttributesParser::<T>::default().parse(expr_call.args, config)?;
        let group = Group::new(identifier, attributes);
        Ok(group.into())
    }
}

impl<T: LiteralParser> Parser<syn::ExprAssign> for AttributeParser<T> {
    type Output = Attribute;
    fn parse(&self, expr_assign: syn::ExprAssign, config: &ParserConfig) -> Result<Self::Output> {
        let identifier = expr_assign
            .left
            .to_token_stream()
            .to_string();
        let identifier = IdentifierParser::new().parse(identifier, config)?;
        let literal = self.literal_parser.parse(expr_assign.right.to_token_stream().to_string(), config)?;
        let group = Named::new(identifier, literal);
        Ok(group.into())
    }
}

impl<T: LiteralParser> Parser<syn::Expr> for AttributeParser<T> {
    type Output = Attribute;
    fn parse(&self, expr: syn::Expr, config: &ParserConfig) -> Result<Self::Output> {
        match expr {
            syn::Expr::Path(expr) => self.parse(expr, config),
            syn::Expr::Lit(expr) => self.literal_parser.parse(expr.to_token_stream().to_string(), config).map(Attribute::Literal),
            syn::Expr::Call(expr) => self.parse(expr, config),
            syn::Expr::Assign(expr) => self.parse(expr, config),
            _ => Ok(Attribute::Literal(Literal::Unknown(expr.to_token_stream().into_iter().map(|token| token.to_string()).collect::<Vec<_>>().join("")))),
        }
    }
}

impl<T: LiteralParser> Parser<syn::ExprPath> for AttributeParser<T> {
    type Output = Attribute;
    fn parse(&self, input: syn::ExprPath, config: &ParserConfig) -> Result<Self::Output> {
        self.parse(input.path, config)
    }
}

impl<T: LiteralParser> Parser<syn::Path> for AttributeParser<T> {
    type Output = Attribute;
    fn parse(&self, path: syn::Path, config: &ParserConfig) -> Result<Self::Output> {
        let path = PathParser::default().parse(path, config)?;
        let attribute = Group::from(path).into();
        Ok(attribute)
    }
}


impl<T: LiteralParser> Parser<syn::MetaNameValue> for AttributeParser<T> {
    type Output = Attribute;
    fn parse(&self, meta_name_value: syn::MetaNameValue, config: &ParserConfig) -> Result<Self::Output> {
        let path = PathParser::default().parse(meta_name_value.path, config)?;
        let literal = self.literal_parser.parse(meta_name_value.value.to_token_stream().to_string(), config)?;
        let attribute = Named::new(path, literal).into();
        Ok(attribute)
    }
}

impl<T: LiteralParser> Parser<syn::Meta> for AttributeParser<T> {
    type Output = Attribute;
    fn parse(&self, meta: syn::Meta, config: &ParserConfig) -> Result<Self::Output> {
        println!("3");
        match meta {
            syn::Meta::Path(path) => self.parse(path, config),
            syn::Meta::List(list) => self.parse(list, config),
            syn::Meta::NameValue(name_value) => self.parse(name_value, config),
        }
    }
}

impl<T: LiteralParser> Parser<syn::Attribute> for AttributeParser<T> {
    type Output = Attribute;
    fn parse(&self, attribute: syn::Attribute, config: &ParserConfig) -> Result<Self::Output> {
        self.parse(attribute.meta, config)
    }
}

impl<T: LiteralParser> Parser<String> for AttributeParser<T> {
    type Output = Attribute;
    fn parse(&self, input: String, config: &ParserConfig) -> Result<Self::Output> {
        self.parse(input.as_str(), config)
    }
}

impl<T: LiteralParser> Parser<IntermediaryAttribute> for AttributeParser<T> {
    type Output = Attribute;
    fn parse(&self, input: IntermediaryAttribute, config: &ParserConfig) -> Result<Self::Output> {
        println!("2", );
        match input {
            IntermediaryAttribute::Meta(meta) => self.parse(meta, config),
            IntermediaryAttribute::Lit(lit) => self.parse(lit, config),
            IntermediaryAttribute::Expr(expr) => self.parse(expr, config),
            IntermediaryAttribute::Unknown(unknown) => Ok(Attribute::Literal(Literal::Unknown(unknown))),
        }
    }
}

impl<T: LiteralParser> Parser<&str> for AttributeParser<T> {
    type Output = Attribute;
    fn parse(&self, input: &str, config: &ParserConfig) -> Result<Self::Output> {
        let attribute = syn::parse_str::<IntermediaryAttribute>(input)
            .map_err(|e| Error::Message(format!("Failed to parse attribute: {:?} - {}", e, input)))?;
        self.parse(attribute, config)
    }
}
