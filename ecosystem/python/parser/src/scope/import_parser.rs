use ligen::parser::ParserConfig;
use ligen::parser::{Parser, utils::WithSource};
use ligen::ir::{Import, Path, Identifier, PathSegment};
use rustpython_parser::ast::{StmtImport, StmtImportFrom, Alias};

use crate::prelude::*;
use crate::PythonParser;

impl Parser<WithSource<&StmtImport>> for PythonParser {
    type Output = Vec<Import>;
    fn parse(&self, input: WithSource<&StmtImport>, _config: &ParserConfig) -> Result<Self::Output> {
        let mut imports = Vec::new();
        for import in &input.ast.names {
            imports.push(self.parse(input.sub(import), _config)?);
        }
        Ok(imports)
    }
}

impl Parser<WithSource<&Alias>> for PythonParser {
    type Output = Import;
    fn parse(&self, input: WithSource<&Alias>, _config: &ParserConfig) -> Result<Self::Output> {
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

impl Parser<WithSource<&StmtImportFrom>> for PythonParser {
    type Output = Vec<Import>;
    fn parse(&self, input: WithSource<&StmtImportFrom>, _config: &ParserConfig) -> Result<Self::Output> {
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
            let mut import = self.parse(input.sub(name), _config)?;
            import.path = path.clone().join(import.path);
            imports.push(import)
        }
        Ok(imports)
    }
}