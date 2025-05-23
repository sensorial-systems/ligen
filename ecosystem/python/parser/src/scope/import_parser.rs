use ligen::transformer::prelude::*;
use ligen::transformer::utils::WithSource;
use ligen::ir::{Import, Path, Identifier, PathSegment};
use rustpython_parser::ast::{StmtImport, StmtImportFrom, Alias};

use crate::PythonParser;

impl Transformer<WithSource<&StmtImport>, Vec<Import>> for PythonParser {
    fn transform(&self, input: WithSource<&StmtImport>, _config: &Config) -> Result<Vec<Import>> {
        let mut imports = Vec::new();
        for import in &input.ast.names {
            imports.push(self.transform(input.sub(import), _config)?);
        }
        Ok(imports)
    }
}

impl Transformer<WithSource<&Alias>, Import> for PythonParser {
    fn transform(&self, input: WithSource<&Alias>, _config: &Config) -> Result<Import> {
        let visibility = Default::default();
        let attributes = Default::default();
        let renaming = input
            .ast
            .asname
            .as_ref()
            .and_then(|asname| self.identifier_parser.parse(asname.as_str(), _config).ok());
        let path = self
            .identifier_parser
            .parse(input.ast.name.as_str(), _config)?
            .into();
        let import = Import { attributes, path, renaming, visibility };
        Ok(import)
    }
}

impl Transformer<WithSource<&StmtImportFrom>, Vec<Import>> for PythonParser {
    fn transform(&self, input: WithSource<&StmtImportFrom>, _config: &Config) -> Result<Vec<Import>> {
        let mut imports = Vec::new();
        let levels = input
            .ast
            .level
            .map(|value| value.to_usize())
            .unwrap_or_default();
        let segments = std::iter::once(Identifier::super_())
            .cycle()
            .take(levels)
            .map(PathSegment::from)
            .collect::<Vec<PathSegment>>();
        let path = Path::from(segments);
        let module_path = input
            .ast
            .module
            .as_ref()
            .map(|module| {
                Path::from_string_with_separator(module.as_str(), ".")
            }).unwrap_or(Path::from(Identifier::self_()));
        let path = path.join(module_path);
        for name in &input.ast.names {
            let mut import = self.transform(input.sub(name), _config)?;
            import.path = path.clone().join(import.path);
            imports.push(import)
        }
        Ok(imports)
    }
}