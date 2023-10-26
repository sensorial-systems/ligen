use crate::{prelude::*, identifier::IdentifierParser};
use ligen::ir::TypeDefinition;
use rustpython_parser::ast::StmtClassDef;

use super::DynamicParser;

#[derive(Default)]
pub struct SymbolParser;

impl<'a> DynamicParser<'a> for SymbolParser {}

impl Parser<WithSource<StmtClassDef>> for SymbolParser {
    type Output = TypeDefinition;
    fn parse(&self, input: WithSource<StmtClassDef>) -> Result<Self::Output> {
        let identifier = IdentifierParser::new().parse(input.ast.name.as_str())?;
        Ok(TypeDefinition { identifier, ..Default::default() })
    }
}