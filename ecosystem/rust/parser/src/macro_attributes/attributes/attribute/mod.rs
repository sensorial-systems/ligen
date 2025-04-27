//! Attribute enumeration.

pub(crate) mod intermediary_attribute;

use intermediary_attribute::IntermediaryAttribute;
use ligen::ir::macro_attributes::{Named, Group};
use syn::__private::ToTokens;
use crate::literal::LiteralParser;
use crate::prelude::*;
use ligen::parser::universal::{PathParser, IdentifierParser};
use ligen::ir::{Attribute, Literal};
use ligen::parser::prelude::*;

use super::AttributesParser;


#[derive(Default)]
pub struct AttributeParser {
    literal_parser: LiteralParser,
}

impl Transformer<syn::ItemMacro, Attribute> for AttributeParser {
    fn transform(&self, call: syn::ItemMacro, config: &Config) -> Result<Attribute> {
        let identifier = call
            .mac
            .path
            .segments
            .last()
            .ok_or(Error::Message("Failed to get identifier from syn::ItemMacro".to_string()))?
            .ident
            .clone();
        let identifier = IdentifierParser::new().transform(identifier, config)?;
        let attributes = AttributesParser::default().transform(call.mac.tokens.to_string().as_str(), config)?;
        let group = Group::new(identifier, attributes).into();
        Ok(group)
    }
}

impl Transformer<syn::MetaList, Attribute> for AttributeParser {
    fn transform(&self, meta_list: syn::MetaList, config: &Config) -> Result<Attribute> {
        let path = PathParser::default().transform(meta_list.path.clone(), config)?;
        let inner = meta_list.tokens.into_iter().map(|token| token.to_string()).collect::<Vec<_>>().join("");
        let attributes = AttributesParser::default().transform(inner.as_str(), config)?;
        let group = Group::new(path, attributes);
        Ok(group.into())
    }
}

impl Transformer<syn::Lit, Attribute> for AttributeParser {
    fn transform(&self, lit: syn::Lit, config: &Config) -> Result<Attribute> {
        self.literal_parser.transform(lit.to_token_stream().to_string(), config).map(Attribute::Literal)
    }
}

impl Transformer<syn::ExprCall, Attribute> for AttributeParser {
    fn transform(&self, expr_call: syn::ExprCall, config: &Config) -> Result<Attribute> {
        let identifier = expr_call
            .func
            .to_token_stream()
            .to_string();
        let identifier = IdentifierParser::new().transform(identifier, config)?;
        let attributes = AttributesParser::default().transform(expr_call.args, config)?;
        let group = Group::new(identifier, attributes);
        Ok(group.into())
    }
}

impl Transformer<syn::ExprAssign, Attribute> for AttributeParser {
    fn transform(&self, expr_assign: syn::ExprAssign, config: &Config) -> Result<Attribute> {
        let identifier = expr_assign
            .left
            .to_token_stream()
            .to_string();
        let identifier = IdentifierParser::new().transform(identifier, config)?;
        let literal = self.literal_parser.transform(expr_assign.right.to_token_stream().to_string(), config)?;
        let group = Named::new(identifier, literal);
        Ok(group.into())
    }
}

impl Transformer<syn::Expr, Attribute> for AttributeParser {
    fn transform(&self, expr: syn::Expr, config: &Config) -> Result<Attribute> {
        match expr {
            syn::Expr::Path(expr) => self.transform(expr, config),
            syn::Expr::Lit(expr) => self.literal_parser.transform(expr.to_token_stream().to_string(), config).map(Attribute::Literal),
            syn::Expr::Call(expr) => self.transform(expr, config),
            syn::Expr::Assign(expr) => self.transform(expr, config),
            _ => Ok(Attribute::Literal(Literal::Unknown(expr.to_token_stream().into_iter().map(|token| token.to_string()).collect::<Vec<_>>().join("")))),
        }
    }
}

impl Transformer<syn::ExprPath, Attribute> for AttributeParser {
    fn transform(&self, input: syn::ExprPath, config: &Config) -> Result<Attribute> {
        self.transform(input.path, config)
    }
}

impl Transformer<syn::Path, Attribute> for AttributeParser {
    fn transform(&self, path: syn::Path, config: &Config) -> Result<Attribute> {
        let path = PathParser::default().transform(path, config)?;
        let attribute = Group::from(path).into();
        Ok(attribute)
    }
}


impl Transformer<syn::MetaNameValue, Attribute> for AttributeParser {
    fn transform(&self, meta_name_value: syn::MetaNameValue, config: &Config) -> Result<Attribute> {
        let path = PathParser::default().transform(meta_name_value.path, config)?;
        let literal = self.literal_parser.transform(meta_name_value.value.to_token_stream().to_string(), config)?;
        let attribute = Named::new(path, literal).into();
        Ok(attribute)
    }
}

impl Transformer<syn::Meta, Attribute> for AttributeParser {
    fn transform(&self, meta: syn::Meta, config: &Config) -> Result<Attribute> {
        match meta {
            syn::Meta::Path(path) => self.transform(path, config),
            syn::Meta::List(list) => self.transform(list, config),
            syn::Meta::NameValue(name_value) => self.transform(name_value, config),
        }
    }
}

impl Transformer<syn::Attribute, Attribute> for AttributeParser {
    fn transform(&self, attribute: syn::Attribute, config: &Config) -> Result<Attribute> {
        self.transform(attribute.meta, config)
    }
}

impl Parser<Attribute> for AttributeParser {
    fn parse(&self, input: impl AsRef<str>, config: &Config) -> Result<Attribute> {
        let input = input.as_ref();
        let attribute = syn::parse_str::<IntermediaryAttribute>(input)
            .map_err(|e| Error::Message(format!("Failed to parse attribute: {:?} - {}", e, input)))?;
        self.transform(attribute, config)
    }
}

impl Transformer<IntermediaryAttribute, Attribute> for AttributeParser {
    fn transform(&self, input: IntermediaryAttribute, config: &Config) -> Result<Attribute> {
        match input {
            IntermediaryAttribute::Meta(meta) => self.transform(meta, config),
            IntermediaryAttribute::Lit(lit) => self.transform(lit, config),
            IntermediaryAttribute::Expr(expr) => self.transform(expr, config),
            IntermediaryAttribute::Unknown(unknown) => Ok(Attribute::Literal(Literal::Unknown(unknown))),
        }
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