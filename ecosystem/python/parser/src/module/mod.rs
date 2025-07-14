use crate::prelude::*;
use ligen::ir::Module;
use rustpython_parser::ast::ModModule;
use crate::parser::PythonParser;

#[derive(Default)]
pub struct ModuleParser;

impl Parser<WithSource<ModModule>> for ModuleParser {
    fn parse(&self, input: impl AsRef<str>, _config: &Config) -> Result<WithSource<ModModule>> {
        let input = input.as_ref();
        let module = parse(input, Mode::Module, "<embedded>")
            .map_err(|error| Error::Message(format!("Failed to parse module: {error}")))?
            .module()
            .ok_or(Error::Message("No module found".into()))?;
        Ok(WithSource::new(input, module))
    }
}

impl Transformer<WithSource<ModModule>, Module> for PythonParser {
    fn transform(&self, input: WithSource<ModModule>, config: &Config) -> Result<Module> {
        let scope = self.transform(input.sub(input.ast.body.as_slice()), config)?;
        let imports = scope.imports;
        let objects = scope.objects;
        let types = scope.types;
        let functions = scope.functions;
        let interfaces = scope.interfaces;
        Ok(Module { objects, functions, types, interfaces, imports, .. Default::default() })
    }
}

pub(crate) struct Directory<'a>(pub &'a std::path::Path);
pub(crate) struct File<'a>(pub &'a std::path::Path);
pub(crate) struct SubPath<'a>(pub &'a std::path::Path);

impl Transformer<File<'_>, Module> for PythonParser {
    fn transform(&self, File(input): File<'_>, config: &Config) -> Result<Module> {
        let content = std::fs::read_to_string(input)?;
        let module = ModuleParser.parse(content.as_str(), config)?;
        let mut module: Module = self.transform(module, config)?;
        module.identifier = self.identifier_parser.transform(input, config)?;
        Ok(module)
    }
}

impl Transformer<Directory<'_>, Module> for PythonParser {
    fn transform(&self, Directory(input): Directory<'_>, config: &Config) -> Result<Module> {
        let identifier = self.identifier_parser.transform(input, config)?;
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
            if extension == "py" || extension == "pyi" || path.is_dir() {
                if let Ok(module) = self.transform(SubPath(path.as_path()), config) {
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

impl Transformer<SubPath<'_>, Module> for PythonParser {
    fn transform(&self, SubPath(input): SubPath<'_>, config: &Config) -> Result<Module> {
        let input = if input.with_extension("py").exists() {
            input.with_extension("py")
        } else {
            input.to_path_buf()
        };

        let input = input.as_path();

        if input.is_dir() {
            self.transform(Directory(input), config)
        } else {
            self.transform(File(input), config)
                .map_err(|error| Error::Message(format!("Failed to read {}. Cause: {:?}", input.display(), error)))
        }        
    }
}
