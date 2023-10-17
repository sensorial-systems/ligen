use rustpython_parser::ast::ModModule;
use rustpython_parser::parse;
use ligen::symbols::Identifier;
use ligen::symbols::Module;
use crate::identifier::IdentifierParser;
use crate::prelude::*;
use crate::symbols::scope::ScopeParser;

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
        let scope = ScopeParser::new().parse(input.sub(&input.ast.body))?;
        let identifier = Default::default();
        let modules = Default::default();
        let constants = scope.constants;
        let types = scope.types;
        let functions = scope.functions;
        let interfaces = scope.interfaces;
        Ok(Module { identifier, constants, types, functions, interfaces, modules })
    }
}

struct Directory<'a>(pub &'a std::path::Path);
struct File<'a>(pub &'a std::path::Path);

impl Parser<File<'_>> for ModuleParser {
    type Output = Module;
    fn parse(&self, File(input): File<'_>) -> Result<Self::Output> {
        let content = std::fs::read_to_string(input)?;
        let mut module = self.parse(content.as_str())?;
        module.identifier = self.parse_identifier(input)?;
        Ok(module)
    }
}

impl Parser<Directory<'_>> for ModuleParser {
    type Output = Module;
    fn parse(&self, Directory(input): Directory<'_>) -> Result<Self::Output> {
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
                if let Ok(module) = self.parse(path.as_path()) {
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
        if input.is_dir() {
            self.parse(Directory(input))
        } else {
            self.parse(File(input)).map_err(|error| Error::Message(format!("Failed to read {}. Cause: {:?}", input.display(), error)))
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