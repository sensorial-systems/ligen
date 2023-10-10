use crate::prelude::*;
use rustpython_parser::ast::StmtClassDef;
use ligen::symbols::interface::Interface;
use crate::identifier::IdentifierParser;
use crate::symbols::scope::ScopeParser;

pub struct InterfaceParser;

impl InterfaceParser {
    pub fn new() -> Self {
        Self
    }
}

impl<T> Parser<&StmtClassDef<T>> for InterfaceParser {
    type Output = Interface;
    fn parse(&self, input: &StmtClassDef<T>) -> Result<Self::Output> {
        let scope_parser = ScopeParser::new();
        let identifier = IdentifierParser::new().parse(input.name.as_str())?;
        let constants = scope_parser.parse_constants(&input.body)?;
        let functions = scope_parser.parse_functions(&input.body)?;
        let methods   = scope_parser.parse_methods(&input.body)?;
        Ok(Interface { identifier, constants, functions, methods })
    }
}