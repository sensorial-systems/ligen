//! Module representation.

mod import;
pub use import::*;

use crate::prelude::*;
use crate::{Object, Path, Visibility, Identifier, TypeDefinition, Attributes, Attribute, Function, Literal};

/// Module representation.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Module {
    /// Attributes.
    pub attributes: Attributes,
    /// Visibility.
    pub visibility: Visibility,
    /// Module path
    pub path: Path,
    /// Module name.
    pub name: Identifier,
    /// Imports.
    pub imports: Vec<Import>,
    /// Sub-modules.
    pub modules: Vec<Module>,
    /// Functions.
    pub functions: Vec<Function>,
    /// Objects.
    pub objects: Vec<Object>
}

impl Module {
    /// FIXME: This is a temporary workaround.
    pub fn get_attributes_from_path<P: Into<Path>>(&self, path: P) -> Option<&Attributes> {
        let path = path.into();
        if let Some(attributes) = self.attributes.get_subgroup(path.clone()) {
            Some(attributes)
        } else {
            self
                .modules
                .iter()
                .find_map(|module| module.get_attributes_from_path(path.clone()))
        }
    }

    /// FIXME: This is a temporary workaround.
    pub fn get_literal_from_path<P: Into<Path>>(&self, path: P) -> Option<&Literal> {
        let path = path.into();
        if let Some(literal) = self.attributes.get_literal_from_path(path.clone()) {
            Some(literal)
        } else {
            self
                .modules
                .iter()
                .find_map(|module| module.get_literal_from_path(path.clone()))
        }
    }

    /// Tells if ligen is ignoring this module.
    pub fn ignored(&self) -> bool {
        Self::ignored_from_attributes(&self.attributes)
    }

    /// Find the Type definition.
    pub fn find_definition(&self, path: &Path) -> Option<TypeDefinition> {
        if let Some(identifier) = path.segments.first() {
            if *identifier == self.name {
                let mut path = path.clone();
                path.segments.remove(0);
                if path.segments.len() > 1 {
                    self
                        .modules
                        .iter()
                        .filter_map(|module| module.find_definition(&path))
                        .next()
                } else {
                    if let Some(identifier) = path.segments.first() {
                        self
                            .objects
                            .iter()
                            .filter(|object| object.definition.identifier() == identifier)
                            .map(|object| object.definition.clone())
                            .next()
                    } else {
                        None
                    }
                }
            } else {
                None
            }
        } else {
            None
        }
    }

    fn ignored_from_attributes(attributes: &Attributes) -> bool {
        attributes.contains(&Attribute::Group("ligen".into(), Attribute::Group("ignore".into(), Default::default()).into()))
    }
}

impl Module {
    /// Find the module with the specified path.
    pub fn find_module(&self, path: &Path) -> Option<&Module> {
        let mut path = path.clone();
        if let Some(identifier) = path.pop_front() {
            let module = self
                .modules
                .iter()
                .find(|module| identifier == module.name);
            if let Some(module) = module {
                if path.segments.is_empty() {
                    Some(module)
                } else {
                    module.find_module(&path)
                }
            } else {
                None
            }
        } else {
            None
        }
    }

    // FIXME: Rusty
    /// Replace wild card imports with actual imports.
    pub fn replace_wildcard_imports(&mut self) {
        for module in &mut self.modules {
            module.replace_wildcard_imports();
        }
        let wildcard_imports: Vec<_> = self
            .imports
            .iter()
            .filter(|import| import.path.last() == "*".into())
            .cloned()
            .collect();
        let mut imports: Vec<_> = self
            .imports
            .iter()
            .filter(|import| import.path.last() != "*".into())
            .cloned()
            .collect();
        for import in wildcard_imports {
            let module_path = import.path.clone().without_last();
            if let Some(module) = self.find_module(&module_path) {
                for object in &module.objects {
                    if let Visibility::Public = object.definition.visibility() {
                        imports.push(Import {
                            attributes: import.attributes.clone(),
                            visibility: import.visibility.clone(),
                            renaming: import.renaming.clone(),
                            path: module_path.clone().join(object.definition.identifier().clone())
                        })
                    }
                }
                for internal_import in &module.imports {
                    if let Visibility::Public = internal_import.visibility {
                        let identifier = if let Some(renaming) = &internal_import.renaming {
                            renaming.clone()
                        } else {
                            internal_import.path.last()
                        };
                        imports.push(Import {
                            attributes: import.attributes.clone(),
                            visibility: import.visibility.clone(),
                            renaming: import.renaming.clone(),
                            path: module_path.clone().join(identifier)
                        })
                    }
                }
            }
        }
        self.imports = imports;
    }
}