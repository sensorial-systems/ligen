use rustpython_parser::ast::StmtClassDef;
use ligen::ir::{TypeDefinition, Structure};
use crate::identifier::IdentifierParser;
use crate::prelude::*;

#[derive(Default)]
pub struct TypeDefinitionParser;

impl TypeDefinitionParser {
    pub fn full() -> Self {
        Default::default()
    }

    pub fn symbol() -> Self {
        Self
    }
}

impl Parser<WithSource<StmtClassDef>> for TypeDefinitionParser {
    type Output = TypeDefinition;
    fn parse(&self, input: WithSource<StmtClassDef>) -> Result<Self::Output> {
        let identifier = IdentifierParser::new().parse(input.ast.name.as_str())?;
        let structure = Structure { identifier, ..Default::default() };
        Ok(structure.into())
    }
}