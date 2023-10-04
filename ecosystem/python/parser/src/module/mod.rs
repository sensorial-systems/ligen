use crate::prelude::*;
use ligen::ir::{Constant, Function, Import, Module, Object};
use rustpython_parser::ast::{ModModule, Stmt};
pub struct ModuleParser;

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
        let attributes = Default::default();
        let visibility = Default::default();
        let identifier = Default::default();
        let modules = Default::default();
        let imports = self.extract_imports(&input);
        let constants = self.extract_constants(&input);
        let functions = self.extract_functions(&input);
        let objects = self.extract_objects(&input);
        Ok(Module { attributes, visibility, identifier, modules, imports, constants, functions, objects })
    }
}

impl ModuleParser {
    fn extract_imports(&self, _input: &ModModule) -> Vec<Import> {
        Default::default()
    }

    fn extract_constants(&self, _input: &ModModule) -> Vec<Constant> {
        Default::default()
    }

    fn extract_functions(&self, _input: &ModModule) -> Vec<Function> {
        Default::default()
    }

    fn extract_objects(&self, _input: &ModModule) -> Vec<Object> {
        Default::default()
    }

}

#[cfg(test)]
mod test {
    #[test]
    fn test_module_parser() {

    }
}