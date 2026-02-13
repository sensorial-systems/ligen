use crate::library::RustLibraryParser;
use cargo_toml::Manifest;
use ligen::idl::Registry;
use ligen::prelude::*;

#[derive(Default)]
pub struct RustRegistryParser;

impl RustRegistryParser {
    pub fn new() -> Self {
        Default::default()
    }
}

impl Transformer<std::path::PathBuf, Registry> for RustRegistryParser {
    fn transform(&self, input: std::path::PathBuf, config: &Config) -> Result<Registry> {
        self.transform(input.as_path(), config)
    }

    fn name(&self) -> &str {
        "Rust"
    }

    fn config(&self) -> Config {
        Default::default()
    }
}

impl Transformer<&std::path::Path, Registry> for RustRegistryParser {
    fn transform(&self, input: &std::path::Path, config: &Config) -> Result<Registry> {
        let mut registry = Registry::new();
        let manifest_path = if input.is_dir() {
            input.join("Cargo.toml")
        } else {
            input.to_path_buf()
        };

        let manifest = Manifest::from_path(&manifest_path)
            .map_err(|e| Error::Message(format!("Failed to parse Cargo.toml: {}", e)))?;

        let library_parser = RustLibraryParser::new();

        if let Some(workspace) = manifest.workspace {
            for member in workspace.members {
                // TODO: Support Glob patterns.
                let member_path = input.join(&member);
                if let Ok(library) = library_parser.transform(&member_path, config) {
                    registry.libraries.push(library);
                }
            }
        }

        if manifest.package.is_some() {
            if let Ok(library) = library_parser.transform(input, config) {
                registry.libraries.push(library);
            }
        }

        Ok(registry)
    }

    fn name(&self) -> &str {
        "Rust"
    }

    fn config(&self) -> Config {
        Default::default()
    }
}
