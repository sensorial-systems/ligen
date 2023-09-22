//! Module representation.

pub mod import;
pub use import::*;

use crate::prelude::*;
use crate::{Object, Path, Visibility, Attributes, Function, Literal, Constant};

/// Module representation.
#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize)]
pub struct Module {
    /// Attributes.
    pub attributes: Attributes,
    /// Visibility.
    pub visibility: Visibility,
    /// Module path
    pub path: Path,
    /// Imports.
    pub imports: Vec<Import>,
    /// Constants.
    pub constants: Vec<Constant>,
    /// Functions.
    pub functions: Vec<Function>,
    /// Objects.
    pub objects: Vec<Object>,
    /// Sub-modules.
    pub modules: Vec<Module>,
}

impl Module {
    pub fn resolve_paths(&mut self, root: &Self) {
        for import in &mut self.imports {
            println!("{}", import.path);
        }
        for module in &mut self.modules {
            module.resolve_paths(root);
        }
    }

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
        self.attributes.has_ignore_attribute()
    }

    /// Find mutable Object.
    pub fn find_object_mut(&mut self, path: &Path) -> Option<&mut Object> {
        let object = self
            .objects
            .iter_mut()
            .find(|object| object.path == *path);
        if let Some(object) = object {
            Some(object)
        } else {
            self
                .modules
                .iter_mut()
                .filter_map(|module| module.find_object_mut(path))
                .next()
        }
    }

    /// Find Object.
    pub fn find_object(&self, path: &Path) -> Option<&Object> {
        let object = self
            .objects
            .iter()
            .find(|object| object.path == *path);
        if let Some(object) = object {
            Some(object)
        } else {
            self
                .modules
                .iter()
                .filter_map(|module| module.find_object(path))
                .next()
        }
    }
}

impl Module {
    /// Find the module with the specified path.
    pub fn find_module(&self, path: &Path) -> Option<&Module> {
        if self.path == *path {
            Some(self)
        } else {
            self
                .modules
                .iter()
                .filter_map(|module| module.find_module(path))
                .next()
        }
    }

    // FIXME: Move this to `Project`'s post-processing.
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
            println!("ModulePath: {}", module_path);
            if let Some(module) = self.find_module(&module_path) {
                for object in &module.objects {
                    if let Visibility::Public = object.visibility {
                        imports.push(Import {
                            attributes: import.attributes.clone(),
                            visibility: import.visibility.clone(),
                            renaming: import.renaming.clone(),
                            path: object.path.clone()
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

    // FIXME: Move this function to a module containing IR processing functions.
    pub fn guarantee_absolute_paths(&mut self) {
        self.guarantee_absolute_paths_with_parent(Default::default())
    }

    fn guarantee_absolute_paths_with_parent(&mut self, parent: Path) {
        self.path = parent.clone().join(self.path.clone());
        for import in &mut self.imports {
            import.path = self.path.clone().join(import.path.clone());
        }
        for function in &mut self.functions {
            function.path = self.path.clone().join(function.path.clone());
        }
        for module in &mut self.modules {
            module.guarantee_absolute_paths_with_parent(self.path.clone());
        }
        for object in &mut self.objects {
            object.path = self.path.clone().join(object.path.clone());
        }
    }
}
