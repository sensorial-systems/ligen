use crate::prelude::*;
use ligen::ir::{Constant, Function, Import, Module, TypeDefinition, Interface};
use rustpython_parser::ast::{ModModule, Stmt};
use crate::function::FunctionParser;

pub struct ModuleParser;

impl Parser<&str> for ModuleParser {
    type Output = Module;
    fn parse(&self, input: &str) -> Result<Self::Output> {
        let module = parse(input, Mode::Module, "<embedded>")
            .map_err(|error| Error::Message(format!("Failed to parse module: {}", error)))?
            .module()
            .ok_or(Error::Message("No module found".into()))?;
        self.parse(WithSource::new(input, module))
    }
}

impl Parser<WithSource<ModModule>> for ModuleParser {
    type Output = Module;
    fn parse(&self, input: WithSource<ModModule>) -> Result<Self::Output> {
        let attributes = Default::default();
        let visibility = Default::default();
        let identifier = Default::default();
        let modules = Default::default();
        let imports = self.extract_imports(&input)?;
        let constants = self.extract_constants(&input)?;
        let functions = self.extract_functions(&input)?;
        let types = self.extract_types(&input)?;
        let interfaces = self.extract_interfaces(&input)?;
        Ok(Module { attributes, visibility, identifier, modules, imports, constants, functions, types, interfaces })
    }
}

impl ModuleParser {
    fn extract_imports(&self, _input: &WithSource<ModModule>) -> Result<Vec<Import>> {
        Ok(Default::default())
    }

    fn extract_constants(&self, _input: &WithSource<ModModule>) -> Result<Vec<Constant>> {
        Ok(Default::default())
    }

    fn extract_functions(&self, input: &WithSource<ModModule>) -> Result<Vec<Function>> {
        let mut functions = Vec::new();
        for statement in &input.ast.body {
            match statement {
                Stmt::FunctionDef(function) => functions.push(FunctionParser.parse(input.sub(function.clone()))?),
                Stmt::AsyncFunctionDef(function) => functions.push(FunctionParser.parse(input.sub(function.clone()))?),
                _ => ()
            }
        }
        Ok(functions)
    }

    fn extract_types(&self, _input: &WithSource<ModModule>) -> Result<Vec<TypeDefinition>> {
        Ok(Default::default())
    }

    fn extract_interfaces(&self, _input: &WithSource<ModModule>) -> Result<Vec<Interface>> {
        Ok(Default::default())
    }
}

#[cfg(test)]
mod test {

    // #[test]
    // fn sub_modules() -> Result<()> {
    //     assert_eq(ModuleParser, mock::sub_modules(), quote! {
    //         pub mod root {
    //             pub mod branch {
    //                 pub mod leaf {}
    //             }
    //         }
    //     })
    // }

    // #[test]
    // fn module_objects() -> Result<()> {
    //     assert_eq(ModuleParser, mock::module_objects(), quote! {
    //         pub mod objects {
    //             pub struct Structure;
    //             pub enum Enumeration {}
    //             pub const CONSTANT: bool = false;
    //             pub fn function() {}
    //         }
    //     })
    // }
}
