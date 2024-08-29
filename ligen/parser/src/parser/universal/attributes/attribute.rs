//! Attribute enumeration.

use ligen_ir::macro_attributes::{Named, Group};
use syn::__private::ToTokens;
use crate::prelude::*;
use ligen_ir::{Attribute, Path, PathSegment};
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
            syn::Expr::Field(_) => Err(Error::Message(format!("Failed to parse attributes: Field is not supported"))),
            syn::Expr::Array(_) => Err(Error::Message(format!("Failed to parse attributes: Array is not supported"))),
            syn::Expr::Tuple(_) => Err(Error::Message(format!("Failed to parse attributes: Tuple is not supported"))),
            syn::Expr::Paren(_) => Err(Error::Message(format!("Failed to parse attributes: Paren is not supported"))),
            syn::Expr::Group(_) => Err(Error::Message(format!("Failed to parse attributes: Group is not supported"))),
            syn::Expr::AssignOp(_) => Err(Error::Message(format!("Failed to parse attributes: AssignOp is not supported"))),
            syn::Expr::Index(_) => Err(Error::Message(format!("Failed to parse attributes: Index is not supported"))),
            syn::Expr::Range(_) => Err(Error::Message(format!("Failed to parse attributes: Range is not supported"))),
            syn::Expr::Async(_) => Err(Error::Message(format!("Failed to parse attributes: Async is not supported"))),
            syn::Expr::Try(_) => Err(Error::Message(format!("Failed to parse attributes: Try is not supported"))),
            syn::Expr::TryBlock(_) => Err(Error::Message(format!("Failed to parse attributes: TryBlock is not supported"))),
            syn::Expr::Yield(_) => Err(Error::Message(format!("Failed to parse attributes: Yield is not supported"))),
            syn::Expr::Verbatim(_) => Err(Error::Message(format!("Failed to parse attributes: Verbatim is not supported"))),
            syn::Expr::Await(_) => Err(Error::Message(format!("Failed to parse attributes: Await is not supported"))),
            syn::Expr::Closure(_) => Err(Error::Message(format!("Failed to parse attributes: Closure is not supported"))),
            syn::Expr::Unsafe(_) => Err(Error::Message(format!("Failed to parse attributes: Unsafe is not supported"))),
            syn::Expr::Block(_) => Err(Error::Message(format!("Failed to parse attributes: Block is not supported"))),
            syn::Expr::If(_) => Err(Error::Message(format!("Failed to parse attributes: If is not supported"))),
            syn::Expr::Binary(_) => Err(Error::Message(format!("Failed to parse attributes: Binary is not supported"))),
            syn::Expr::Cast(_) => Err(Error::Message(format!("Failed to parse attributes: Cast is not supported"))),
            syn::Expr::Type(_) => Err(Error::Message(format!("Failed to parse attributes: Type is not supported"))),
            syn::Expr::Repeat(_) => Err(Error::Message(format!("Failed to parse attributes: Repeat is not supported"))),
            syn::Expr::Struct(_) => Err(Error::Message(format!("Failed to parse attributes: Struct is not supported"))),
            syn::Expr::MethodCall(_) => Err(Error::Message(format!("Failed to parse attributes: MethodCall is not supported"))),
            _ => Err(Error::Message("Failed to parse attribute: Unsupported expression type".to_string())),
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
        // TODO: This is duplicated from PathParser.
        let segments = path
            .segments
            .iter()
            // FIXME: This isn't parsing generics, just the identifiers.
            .map(|segment| IdentifierParser::new().parse(segment.ident.clone(), config).expect("Failed to parse segment."))
            .map(PathSegment::from)
            .collect();
        let path = Path { segments };
        let attribute = Group::from(path).into();
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