//! Attribute enumeration.

pub(crate) mod intermediary_attribute;

use intermediary_attribute::IntermediaryAttribute;
use ligen::ir::macro_attributes::{Named, Group};
use syn::__private::ToTokens;
use crate::literal::LiteralParser;
use crate::prelude::*;
use ligen::parser::universal::{PathParser, IdentifierParser};
use ligen::ir::{Attribute, Literal};
use ligen::parser::{Parser, ParserConfig};

use super::AttributesParser;


#[derive(Default)]
pub struct AttributeParser {
    literal_parser: LiteralParser,
}

impl Parser<syn::ItemMacro> for AttributeParser {
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
        let attributes = AttributesParser::default().parse(call.mac.tokens.to_string().as_str(), config)?;
        let group = Group::new(identifier, attributes).into();
        Ok(group)
    }
}

impl Parser<syn::MetaList> for AttributeParser {
    type Output = Attribute;
    fn parse(&self, meta_list: syn::MetaList, config: &ParserConfig) -> Result<Self::Output> {
        let path = PathParser::default().parse(meta_list.path.clone(), config)?;
        let inner = meta_list.tokens.into_iter().map(|token| token.to_string()).collect::<Vec<_>>().join("");
        let attributes = AttributesParser::default().parse(inner.as_str(), config)?;
        let group = Group::new(path, attributes);
        Ok(group.into())
    }
}

impl Parser<syn::Lit> for AttributeParser {
    type Output = Attribute;
    fn parse(&self, lit: syn::Lit, config: &ParserConfig) -> Result<Self::Output> {
        self.literal_parser.parse(lit.to_token_stream().to_string(), config).map(Attribute::Literal)
    }
}

impl Parser<syn::ExprCall> for AttributeParser {
    type Output = Attribute;
    fn parse(&self, expr_call: syn::ExprCall, config: &ParserConfig) -> Result<Self::Output> {
        let identifier = expr_call
            .func
            .to_token_stream()
            .to_string();
        let identifier = IdentifierParser::new().parse(identifier, config)?;
        let attributes = AttributesParser::default().parse(expr_call.args, config)?;
        let group = Group::new(identifier, attributes);
        Ok(group.into())
    }
}

impl Parser<syn::ExprAssign> for AttributeParser {
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

impl Parser<syn::Expr> for AttributeParser {
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

impl Parser<syn::ExprPath> for AttributeParser {
    type Output = Attribute;
    fn parse(&self, input: syn::ExprPath, config: &ParserConfig) -> Result<Self::Output> {
        self.parse(input.path, config)
    }
}

impl Parser<syn::Path> for AttributeParser {
    type Output = Attribute;
    fn parse(&self, path: syn::Path, config: &ParserConfig) -> Result<Self::Output> {
        let path = PathParser::default().parse(path, config)?;
        let attribute = Group::from(path).into();
        Ok(attribute)
    }
}


impl Parser<syn::MetaNameValue> for AttributeParser {
    type Output = Attribute;
    fn parse(&self, meta_name_value: syn::MetaNameValue, config: &ParserConfig) -> Result<Self::Output> {
        let path = PathParser::default().parse(meta_name_value.path, config)?;
        let literal = self.literal_parser.parse(meta_name_value.value.to_token_stream().to_string(), config)?;
        let attribute = Named::new(path, literal).into();
        Ok(attribute)
    }
}

impl Parser<syn::Meta> for AttributeParser {
    type Output = Attribute;
    fn parse(&self, meta: syn::Meta, config: &ParserConfig) -> Result<Self::Output> {
        match meta {
            syn::Meta::Path(path) => self.parse(path, config),
            syn::Meta::List(list) => self.parse(list, config),
            syn::Meta::NameValue(name_value) => self.parse(name_value, config),
        }
    }
}

impl Parser<syn::Attribute> for AttributeParser {
    type Output = Attribute;
    fn parse(&self, attribute: syn::Attribute, config: &ParserConfig) -> Result<Self::Output> {
        self.parse(attribute.meta, config)
    }
}

impl Parser<String> for AttributeParser {
    type Output = Attribute;
    fn parse(&self, input: String, config: &ParserConfig) -> Result<Self::Output> {
        self.parse(input.as_str(), config)
    }
}

impl Parser<IntermediaryAttribute> for AttributeParser {
    type Output = Attribute;
    fn parse(&self, input: IntermediaryAttribute, config: &ParserConfig) -> Result<Self::Output> {
        match input {
            IntermediaryAttribute::Meta(meta) => self.parse(meta, config),
            IntermediaryAttribute::Lit(lit) => self.parse(lit, config),
            IntermediaryAttribute::Expr(expr) => self.parse(expr, config),
            IntermediaryAttribute::Unknown(unknown) => Ok(Attribute::Literal(Literal::Unknown(unknown))),
        }
    }
}

impl Parser<&str> for AttributeParser {
    type Output = Attribute;
    fn parse(&self, input: &str, config: &ParserConfig) -> Result<Self::Output> {
        let attribute = syn::parse_str::<IntermediaryAttribute>(input)
            .map_err(|e| Error::Message(format!("Failed to parse attribute: {:?} - {}", e, input)))?;
        self.parse(attribute, config)
    }
}


#[cfg(test)]
mod test {
    use crate::prelude::*;
    use super::*;
    use ligen::parser::assert::assert_eq;
    use ligen::ir::attribute::mock;

    #[test]
    fn attribute_literal() -> Result<()> {
        assert_eq(AttributeParser::default(), mock::attribute_literal(), "\"c\"")
    }

    #[test]
    fn attribute_named() -> Result<()> {
        assert_eq(AttributeParser::default(), mock::attribute_named(), "int = \"sized\"")
    }

    #[test]
    fn attribute_group() -> Result<()> {
        assert_eq(AttributeParser::default(), mock::attribute_group(), "c(int = \"sized\")")
    }

    #[test]
    fn attribute_empty_group() -> Result<()> {
        assert_eq(AttributeParser::default(), mock::attribute_empty_group(), "c()")?;
        assert_eq(AttributeParser::default(), mock::attribute_empty_group(), "c")
    }
}