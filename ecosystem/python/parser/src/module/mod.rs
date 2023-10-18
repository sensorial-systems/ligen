use crate::prelude::*;
use ligen::ir::{Constant, Function, Import, Module, TypeDefinition, Interface, Identifier};
use rustpython_parser::ast::{ModModule, Stmt};
use crate::function::FunctionParser;
use crate::identifier::IdentifierParser;
use crate::scope::ScopeParser;

#[derive(Default)]
pub struct ModuleParser;

impl ModuleParser {
    pub fn new() -> Self {
        Default::default()
    }
}

impl Parser<&str> for ModuleParser {
    type Output = Module;
    fn parse(&self, input: &str) -> Result<Self::Output> {
        self.parse_symbols(input)
    }
    fn parse_symbols(&self, input: &str) -> Result<Self::Output> {
        let module = parse(input, Mode::Module, "<embedded>")
            .map_err(|error| Error::Message(format!("Failed to parse module: {}", error)))?
            .module()
            .ok_or(Error::Message("No module found".into()))?;
        self.parse_symbols(WithSource::new(input, module))
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

    fn parse_symbols(&self, input: WithSource<ModModule>) -> Result<Self::Output> {
        let scope = ScopeParser::new().parse_symbols(input.sub(&input.ast.body))?;
        let identifier = Default::default();
        let modules = Default::default();
        let constants = scope.constants;
        let types = scope.types;
        let functions = scope.functions;
        let interfaces = scope.interfaces;
        Ok(Module { identifier, constants, types, functions, interfaces, modules, .. Default::default() })
    }
}

struct Directory<'a>(pub &'a std::path::Path);
struct File<'a>(pub &'a std::path::Path);

impl Parser<File<'_>> for ModuleParser {
    type Output = Module;
    fn parse(&self, input: File<'_>) -> Result<Self::Output> {
        self.parse_symbols(input)
    }

    fn parse_symbols(&self, File(input): File<'_>) -> Result<Self::Output> {
        let content = std::fs::read_to_string(input)?;
        let mut module = self.parse_symbols(content.as_str())?;
        module.identifier = self.parse_identifier(input)?;
        Ok(module)
    }
}

impl Parser<Directory<'_>> for ModuleParser {
    type Output = Module;
    fn parse(&self, input: Directory<'_>) -> Result<Self::Output> {
        self.parse_symbols(input)
    }
    fn parse_symbols(&self, Directory(input): Directory<'_>) -> Result<Self::Output> {
        let identifier = self.parse_identifier(input)?;
        let mut module = Module { identifier, .. Default::default() };
        let mut modules: Vec<Module> = Vec::new();
        for entry in input.read_dir()? {
            let entry = entry?;
            let path = entry.path();
            let extension = path
                .extension()
                .and_then(|extension| extension.to_str())
                .map(String::from)
                .unwrap_or_default();
            if extension == "py" || path.is_dir() {
                if let Ok(module) = self.parse_symbols(path.as_path()) {
                    if let Some(existing) = modules
                        .iter_mut()
                        .find(|existing| existing.identifier == module.identifier)
                    {
                        existing.join(module)
                    } else {
                        modules.push(module);
                    }
                }
            }
        }
        if let Some((index, _)) = modules
            .iter()
            .enumerate()
            .find(|(_, sub_module)| sub_module.identifier.name == "__init__")
        {
            let identifier = module.identifier;
            module = modules.remove(index);
            module.identifier = identifier;
        }
        module.modules = modules;
        Ok(module)
    }
}

impl Parser<&std::path::Path> for ModuleParser {
    type Output = Module;
    fn parse(&self, input: &std::path::Path) -> Result<Self::Output> {
        self.parse_symbols(input)
    }
    fn parse_symbols(&self, input: &std::path::Path) -> Result<Self::Output> {
        if input.is_dir() {
            self.parse_symbols(Directory(input))
        } else {
            self.parse_symbols(File(input)).map_err(|error| Error::Message(format!("Failed to read {}. Cause: {:?}", input.display(), error)))
        }
    }
}

impl ModuleParser {
    fn parse_identifier(&self, input: &std::path::Path) -> Result<Identifier> {
        let identifier = input
            .file_stem()
            .ok_or(Error::Message(format!("Failed to parse file stem from path: {}", input.display())))?
            .to_str()
            .ok_or(Error::Message(format!("Failed to parse file stem to string: {}", input.display())))?;
        IdentifierParser::new().parse(identifier)
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
