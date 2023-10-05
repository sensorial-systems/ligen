pub mod attribute;

use rustpython_parser::ast::{Expr, ExprCall, Ranged};
use ligen::ir::Attributes;
use crate::macro_attributes::attributes::attribute::AttributeParser;
use crate::prelude::*;

pub struct AttributesParser;

impl Parser<WithSource<Vec<Expr>>> for AttributesParser {
    type Output = Attributes;
    fn parse(&self, input: WithSource<Vec<Expr>>) -> Result<Self::Output> {
        let source = input.source;
        let input = input.ast;
        if !input.is_empty() {
            println!("{}", &source[input.first().unwrap().start().to_usize()..input.last().unwrap().end().to_usize()]);
        }
        let mut attributes = Attributes::default();
        for expr in input {
            if let Expr::Attribute(expr) = expr {
                attributes.attributes.push(AttributeParser.parse(WithSource::new(&source, expr))?);
            }
        }
        Ok(attributes)
    }
}

impl Parser<WithSource<Box<Expr>>> for AttributesParser {
    type Output = Attributes;
    fn parse(&self, input: WithSource<Box<Expr>>) -> Result<Self::Output> {
        let source = input.source;
        let input = input.ast;
        let mut attributes = Attributes::default();
        match input.as_ref() {
            Expr::Call(call) => attributes.attributes.append(&mut AttributesParser.parse(WithSource::new(&source, call.clone()))?.attributes),
            _ => ()
        }
        Ok(attributes)
    }
}

impl Parser<WithSource<ExprCall>> for AttributesParser {
    type Output = Attributes;
    fn parse(&self, _input: WithSource<ExprCall>) -> Result<Self::Output> {
        Ok(Default::default())
    }
}