use crate::prelude::*;
use rustpython_parser::ast::StmtClassDef;
use ligen::ir::Interface;
use crate::identifier::IdentifierParser;
use crate::symbols::scope::ScopeParser;

#[derive(Default)]
pub struct InterfaceParser;

impl InterfaceParser {
    pub fn new() -> Self {
        Default::default()
    }
}

impl Parser<WithSource<&StmtClassDef>> for InterfaceParser {
    type Output = Interface;
    fn parse(&self, input: WithSource<&StmtClassDef>) -> Result<Self::Output> {
        self.parse_symbols(input)
    }
    fn parse_symbols(&self, input: WithSource<&StmtClassDef>) -> Result<Self::Output> {
        let scope = ScopeParser::new().parse(input.sub(&input.ast.body))?;
        let identifier = IdentifierParser::new().parse(input.ast.name.as_str())?;
        let constants = scope.constants;
        let functions = scope.functions;
        let methods = scope.methods;
        Ok(Interface { identifier, constants, functions, methods, .. Default::default() })
    }
}