use crate::prelude::*;
use rustpython_parser::ast::StmtClassDef;
use ligen::ir::Interface;
use crate::identifier::IdentifierParser;
use crate::scope::ScopeParser;

#[derive(Default)]
pub struct InterfaceParser {
    // scope_parser: Box<ScopeParser>
}

impl InterfaceParser {
    pub fn full() -> Self {
        Default::default()
    }

    pub fn symbol() -> Self {
        // let scope_parser = Box::new(ScopeParser::symbol());
        // Self { scope_parser }
        Default::default()
    }
}

impl Parser<WithSource<&StmtClassDef>> for InterfaceParser {
    type Output = Interface;
    fn parse(&self, input: WithSource<&StmtClassDef>) -> Result<Self::Output> {
        let scope = ScopeParser::symbol().parse(input.sub(&input.ast.body))?;
        let identifier = IdentifierParser::new().parse(input.ast.name.as_str())?;
        let constants = scope.constants;
        let functions = scope.functions;
        let methods = scope.methods;
        Ok(Interface { identifier, constants, functions, methods, .. Default::default() })
    }
}