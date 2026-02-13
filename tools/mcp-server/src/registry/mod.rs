use ligen_common::Result;
use ligen_idl::Library;
use ligen_idl::prelude::*;
use ligen_rust_parser::RustRegistryParser;
use ligen_transformer::Transformer;
use std::collections::HashMap;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Project {
    pub name: String,
    pub path: PathBuf,
    pub library: Library,
}

pub struct Registry {
    pub projects: HashMap<String, Project>,
}

impl Default for Registry {
    fn default() -> Self {
        Self::new()
    }
}

impl Registry {
    pub fn new() -> Self {
        Self {
            projects: HashMap::new(),
        }
    }

    pub fn add_project(&mut self, path: impl AsRef<Path>) -> Result<String> {
        let path = path.as_ref().to_path_buf();
        let parser = RustRegistryParser;

        // I need to check RustParser transform signature.
        let transformer: &dyn Transformer<&Path, ligen_idl::Registry> = &parser;
        let config = transformer.config();
        let registry = transformer.transform(path.as_path(), &config)?;

        if let Some(library) = registry.libraries.first() {
            let name = library.identifier.to_string();
            let project = Project {
                name: name.clone(),
                path,
                library: library.clone(),
            };
            self.projects.insert(name.clone(), project);
            Ok(name)
        } else {
            Err(anyhow::anyhow!("No libraries found in project").into())
        }
    }

    pub fn remove_project(&mut self, name: &str) -> bool {
        self.projects.remove(name).is_some()
    }

    pub fn list_projects(&self) -> Vec<String> {
        self.projects.keys().cloned().collect()
    }

    pub fn get_project(&self, name: &str) -> Option<&Project> {
        self.projects.get(name)
    }
}
