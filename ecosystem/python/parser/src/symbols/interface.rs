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
        let scope = ScopeParser::new().parse(&input.body)?;
        let identifier = IdentifierParser::new().parse(input.name.as_str())?;
        let constants = scope.constants;
        let functions = scope.functions;
        let methods = scope.methods;
        Ok(Interface { identifier, constants, functions, methods })
    }
}