use rustpython_parser::ast::{ModModule, Stmt};
use rustpython_parser::parse;
use ligen::symbols::interface::Interface;
use ligen::symbols::module::Module;
use crate::prelude::*;
use crate::symbols::interface::InterfaceParser;
use crate::symbols::scope::ScopeParser;

pub struct ModuleParser;

impl ModuleParser {
    pub fn new() -> Self {
        Self
    }
}

impl Parser<&str> for ModuleParser {
    type Output = Module;
    fn parse(&self, input: &str) -> Result<Self::Output> {
        let module = parse(input, Mode::Module, "<embedded>")
            .map_err(|error| Error::Message(format!("Failed to parse module: {}", error)))?
            .module()
            .ok_or(Error::Message("No module found".into()))?;
        self.parse(module)
    }
}

impl Parser<ModModule> for ModuleParser {
    type Output = Module;
    fn parse(&self, input: ModModule) -> Result<Self::Output> {
        let scope_parser = ScopeParser::new();
        let identifier = Default::default();
        let constants = scope_parser.parse_constants(&input.body)?;
        let types = scope_parser.parse_types(&input.body)?;
        let functions = scope_parser.parse_functions(&input.body)?;
        let interfaces = self.parse_interfaces(&input.body)?;
        let modules = Default::default();
        Ok(Module { identifier, constants, types, functions, interfaces, modules })
    }
}

impl ModuleParser {
    fn parse_interfaces(&self, statements: &[Stmt]) -> Result<Vec<Interface>> {
        let mut interfaces = Vec::new();
        for statement in statements {
            match statement {
                Stmt::ClassDef(class) => interfaces.push(InterfaceParser::new().parse(class)?),
                _ => ()
            }
        }
        Ok(interfaces)
    }
}