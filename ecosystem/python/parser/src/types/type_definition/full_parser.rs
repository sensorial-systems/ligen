use crate::{prelude::*, identifier::IdentifierParser};
use ligen::ir::{TypeDefinition, Visibility};
use rustpython_parser::ast::StmtClassDef;

use super::DynamicParser;

#[derive(Default)]
pub struct FullParser;

impl<'a> DynamicParser<'a> for FullParser {}

impl Parser<WithSource<StmtClassDef>> for FullParser {
    type Output = TypeDefinition;
    fn parse(&self, input: WithSource<StmtClassDef>) -> Result<Self::Output> {
        let attributes = Err(Error::Message("Not implemented".into()))?;
        let identifier = IdentifierParser::new().parse(input.ast.name.as_str())?;
        let visibility = Visibility::Public;
        let definition = Err(Error::Message("Not implemented".into()))?;
        let interfaces = Err(Error::Message("Not implemented".into()))?;
        Ok(TypeDefinition { attributes, visibility, identifier, definition, interfaces })
    }
}