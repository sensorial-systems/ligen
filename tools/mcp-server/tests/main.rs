use std::{collections::HashMap, path::PathBuf};

use ligen_idl::{Interface, Library, Object, Registry, TypeDefinition};
use ligen_rust_parser::{RustRegistryParser, prelude::*};

type Parsers = HashMap<String, Box<dyn Transformer<PathBuf, Registry>>>;

pub struct Service {
    registry: HashMap<PathBuf, Library>,
    parsers: Parsers,
}

impl Default for Service {
    fn default() -> Self {
        let registry = Default::default();
        let mut parsers: Parsers = Default::default();
        parsers.insert("rust".to_string(), Box::new(RustRegistryParser));
        Self { registry, parsers }
    }
}

impl Service {
    /// Create a new service.
    pub fn new() -> Self {
        Default::default()
    }

    /// List of supported languages.
    pub fn languages(&self) -> Vec<String> {
        self.parsers.keys().cloned().collect()
    }

    /// Get library description.
    pub fn get_library_description(&self, path: &PathBuf) -> Result<String> {
        self.get_library(path)?
            .metadata
            .description
            .clone()
            .ok_or(anyhow::anyhow!("Description not found").into())
    }

    /// Get library path.
    pub fn get_library_path(&self, name: &str) -> Result<PathBuf> {
        self.registry
            .iter()
            .find(|(_, library)| library.identifier.name == name)
            .map(|(path, _)| path.clone())
            .ok_or(anyhow::anyhow!("Library not found").into())
    }

    /// Get library by path.
    pub fn get_library(&self, path: &PathBuf) -> Result<&Library> {
        self.registry
            .get(path)
            .ok_or(anyhow::anyhow!("Library not found").into())
    }

    /// Add library to the registry.
    pub fn add_library(&mut self, path: PathBuf) -> Result<()> {
        if !path.exists() {
            Err(anyhow::anyhow!("Path does not exist"))?;
        }
        if self.registry.contains_key(&path) {
            Err(anyhow::anyhow!("Library already exists"))?;
        }
        let parser = self.parsers.get("rust").ok_or("No parser found")?;
        let registry = parser
            .transform(path.clone(), &Default::default())
            .map_err(|e| e.to_string())?;
        for library in registry.libraries.into_iter() {
            self.registry.insert(path.clone(), library);
        }
        Ok(())
    }

    /// Get library type definitions.
    pub fn get_library_type_definitions(&self, path: &PathBuf) -> Result<Vec<TypeDefinition>> {
        let library = self.get_library(path)?;
        let mut definitions = Vec::new();
        let mut stack = vec![&library.root_module];
        while let Some(module) = stack.pop() {
            for type_ in &module.types {
                if type_.visibility.is_public() {
                    definitions.push(type_.clone());
                }
            }
            for module in &module.modules {
                if module.visibility.is_public() {
                    stack.push(module);
                }
            }
        }
        Ok(definitions)
    }

    /// Get library interfaces.
    pub fn get_library_interfaces(&self, path: &PathBuf) -> Result<Vec<Interface>> {
        let library = self.get_library(path)?;
        let mut interfaces = Vec::new();
        let mut stack = vec![&library.root_module];
        while let Some(module) = stack.pop() {
            for interface in &module.interfaces {
                if interface.visibility.is_public() {
                    interfaces.push(interface.clone());
                }
            }
            for module in &module.modules {
                if module.visibility.is_public() {
                    stack.push(module);
                }
            }
        }
        Ok(interfaces)
    }

    /// Get library objects.
    pub fn get_library_objects(&self, path: &PathBuf) -> Result<Vec<Object>> {
        let library = self.get_library(path)?;
        let mut objects = Vec::new();
        let mut stack = vec![&library.root_module];
        while let Some(module) = stack.pop() {
            for object in &module.objects {
                if object.visibility.is_public() {
                    objects.push(object.clone());
                }
            }
            for interface in &module.interfaces {
                for object in &interface.objects {
                    if object.visibility.is_public() {
                        objects.push(object.clone());
                    }
                }
            }
            for module in &module.modules {
                stack.push(module);
            }
        }
        Ok(objects)
    }
}

#[test]
fn usual_flow() -> Result<()> {
    let mut service = Service::new();
    if service
        .add_library(PathBuf::from(
            "D:\\dev\\sensorial\\systems\\metadata-service\\crates\\metadata-service",
        ))
        .is_err()
    {
        println!("Skipping test because this is local while in WIP");
        return Ok(());
    }
    let path = service.get_library_path("metadata-service")?;
    let description = service.get_library_description(&path)?;
    let types = service.get_library_type_definitions(&path)?;
    let interfaces = service.get_library_interfaces(&path)?;
    let objects = service.get_library_objects(&path)?;
    println!("Description: {}", description);
    println!("Types: {:#?}", types);
    println!("Interfaces: {:#?}", interfaces);
    println!("Objects: {:#?}", objects);
    Ok(())
}
