use crate::prelude::*;
use ligen::parsing::parser::ParserConfig;
use rustpython_parser::ast::StmtClassDef;
use ligen::ir::Interface;
use crate::identifier::IdentifierParser;
use crate::parser::PythonParser;

impl Parser<WithSource<&StmtClassDef>> for PythonParser {
    type Output = Interface;
    fn parse(&self, input: WithSource<&StmtClassDef>, config: &ParserConfig) -> Result<Self::Output> {
        let scope = self.parse(input.sub(input.ast.body.as_slice()), config)?;
        let identifier = IdentifierParser::new().parse(input.ast.name.as_str(), config)?;
        let objects = scope.objects;
        let functions = scope.functions;
        let methods = scope.methods;
        Ok(Interface { identifier, objects, functions, methods, .. Default::default() })
    }
}