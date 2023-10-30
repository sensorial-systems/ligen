pub mod attribute;

use crate::prelude::*;
use crate::literal::LiteralParser;
use ligen::parsing::parser::universal::attributes::AttributesParser as InternalParser;
use ligen::ir::Attributes;
use rustpython_parser::ast::{Expr, Ranged};

#[derive(Default)]
pub struct AttributesParser {
    parser: InternalParser<LiteralParser>
}

impl Parser<WithSource<Vec<Expr>>> for AttributesParser {
    type Output = Attributes;
    fn parse(&self, input: WithSource<Vec<Expr>>) -> Result<Self::Output> {
        let source = if input.ast.is_empty() {
            Default::default()
        } else {
            input.source[input.ast.first().unwrap().start().to_usize()..input.ast.last().unwrap().end().to_usize()].to_string()
        };
        self.parser.parse(source)
    }
}
